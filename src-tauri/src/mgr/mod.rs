use crate::{cipher::gen_nonce, errors::Error};
use async_std::channel::bounded;
use async_std::io::Cursor;
use async_std::sync::RwLock;
use cid::{
    multihash::{Code::Blake2b256, MultihashDigest},
    Cid,
};
use fvm_ipld_blockstore::{Blockstore, MemoryBlockstore};
use fvm_ipld_car::{load_car, Block, CarHeader, CarReader};
use fvm_ipld_encoding::{from_slice, to_vec, DAG_CBOR};
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};
use std::{
    ffi::OsString,
    fs::{read, write},
    future::Future,
    path::PathBuf,
};

mod cbox;
mod cboxobj;
mod db;
mod tasks;
mod typs;
mod userkey;
pub use typs::*;
//pub use userkey::*;

pub async fn web3storage_upload(data: Vec<u8>, cbox: &CBox) -> Result<Cid, Error> {
    let rawdata = to_vec(&RawBlock { data }).unwrap();
    let buffer: Arc<RwLock<Vec<u8>>> = Default::default();
    let cid = Cid::new_v1(DAG_CBOR, Blake2b256.digest(&rawdata));
    let header = CarHeader {
        roots: vec![cid],
        version: 1,
    };

    assert_eq!(to_vec(&header).unwrap().len(), 60);

    let (tx, mut rx) = bounded(10);

    let buffer_cloned = buffer.clone();
    let write_task = async_std::task::spawn(async move {
        header
            .write_stream_async(&mut *buffer_cloned.write().await, &mut rx)
            .await
            .unwrap()
    });

    tx.send((cid, rawdata.clone())).await.unwrap();
    drop(tx);
    write_task.await;

    let buffer: Vec<_> = buffer.read().await.clone();
    let client = reqwest::blocking::Client::new();
    let res = client
        .post("https://api.web3.storage/car")
        .header(reqwest::header::CONTENT_TYPE, "application/vnd.ipld.car")
        .header("Authorization", format!("Bearer {}", &cbox.access_token))
        .body(buffer)
        .send()
        .unwrap();

    if !res.status().is_success() {
        eprintln!("upload failed");
        return Err(Error::Other(format!("{:?}", &res.bytes().unwrap())));
    }
    println!("{:?}, expected cid: {}", &res.bytes().unwrap(), cid);
    Ok(cid)
}

pub fn init_task_record(
    task: &CBoxTask,
    cipherbox_app: Arc<Mutex<App>>,
) -> Result<TaskRecord, Error> {
    if task.task_type != 0 {
        return Err(Error::Other("current only support backup task".into()));
    }

    let meta = std::fs::metadata(&task.origin_path)?;
    if meta.is_dir() {
        return Err(Error::Other("dir backup task not supported".into()));
    };
    let fsize = meta.len();
    if fsize == 0 {
        return Err(Error::Other("zero file size".into()));
    }
    let chunck_count = (fsize - 1) / (CHUNK_SIZE as u64) + 1;
    Ok(TaskRecord {
        box_id: task.box_id,
        task_id: task.id,
        backup: true,
        recover: false,
        total_size: fsize,
        total: 1,
        finished: 0,
        finished_size: 0,
        upload_list: vec![ChoreUploadRecord {
            path: task.origin_path.clone(),
            size: fsize,
            uploaded_size: 0,
            chunk_count: chunck_count,
            chunk_uploaded: 0,
            chunks: Vec::new(),
            chunks_ref: String::new(),
        }],
        err: None,
    })
}

pub fn spawn_and_log_error<F>(fut: F) -> async_std::task::JoinHandle<()>
where
    F: Future<Output = std::result::Result<(), Error>> + Send + 'static,
{
    async_std::task::spawn(async move {
        if let Err(e) = fut.await {
            eprintln!("{}", e)
        }
    })
}

impl App {
    pub fn setup(&mut self, app_dir: OsString) {
        self.app_dir = app_dir;
        self.providers = vec![Provider {
            id: 1,
            name: "web3storage".into(),
            put_api: "{}://api.web3.storage/{}".into(),
            get_api: "{}://dweb.link/ipfs/{}?{}".into(),
        }];
    }
    pub fn app_info(&self) -> AppInfo {
        let mut info = AppInfo::default();

        info.has_password_set = self.has_set_password();
        info.session_expired = !self.is_user_key_set();
        let active_box_id = self.kv_cache.active_box_id;
        if active_box_id > 0 {
            if let Ok(b) = self.get_cbox_by_id(active_box_id) {
                info.active_box = Some(b);
            }
        }
        info
    }
    pub fn read_cache(&mut self) -> Result<(), Error> {
        let mut cache_path = PathBuf::from(&self.app_dir);
        cache_path.push(KV_FILE_NAME);
        let d = read(cache_path)?;
        let c: KVCache = toml::from_slice(&d)?;
        self.kv_cache = c;
        Ok(())
    }
    pub fn flush_cache(&self) -> Result<(), Error> {
        let mut cache_path = PathBuf::from(&self.app_dir);
        cache_path.push(KV_FILE_NAME);

        let c = toml::to_string(&self.kv_cache)?;

        write(cache_path, c)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_car_head() {
        let cid = Cid::new_v1(DAG_CBOR, Blake2b256.digest(b"test"));

        let header = CarHeader {
            roots: vec![cid],
            version: 1,
        };

        let bytes = to_vec(&header).unwrap();
        assert_eq!(from_slice::<CarHeader>(&bytes).unwrap(), header);
    }
    #[async_std::test]
    async fn test_car_read_write() {
        let buffer: Arc<RwLock<Vec<u8>>> = Default::default();
        let cid = Cid::new_v1(DAG_CBOR, Blake2b256.digest(b"test"));
        let header = CarHeader {
            roots: vec![cid],
            version: 1,
        };
        assert_eq!(to_vec(&header).unwrap().len(), 60);

        let (tx, mut rx) = bounded(10);

        let buffer_cloned = buffer.clone();
        let write_task = async_std::task::spawn(async move {
            header
                .write_stream_async(&mut *buffer_cloned.write().await, &mut rx)
                .await
                .unwrap()
        });

        tx.send((cid, b"test".to_vec())).await.unwrap();
        drop(tx);
        write_task.await;

        let buffer: Vec<_> = buffer.read().await.clone();

        let reader = Cursor::new(&buffer);

        let bs = MemoryBlockstore::default();
        load_car(&bs, reader).await.unwrap();

        assert_eq!(bs.get(&cid).unwrap(), Some(b"test".to_vec()));
    }

    #[test]
    fn test_download() {
        let client = reqwest::blocking::Client::new();

        let res = client.get("https://bafybeiedjtdnqo4terwb3peodgo46ueetdvpvaqietlz43s3brbg4ysxgq.ipfs.dweb.link/upload_test.txt").send().unwrap();
    }

    #[test]
    fn test_upload() {
        // let client = reqwest::blocking::Client::new();
        // let res = client.post("https://api.web3.storage/upload")
        //     //.header(reqwest::header::CONTENT_TYPE, "multipart/form-data")
        //     .header("Authorization", "Bearer ...")
        //     .body(b"it should work".to_vec())
        //     .send()
        //     .unwrap();
        // dbg!(&res);
        // dbg!(&res.bytes().unwrap());
    }

    // #[async_std::test]
    // async fn test_upload_car() {
    //     let rawdata = to_vec(&RawBlock {
    //         data: b"Hush little baby don't say a word".to_vec(),
    //     })
    //     .unwrap();
    //     let buffer: Arc<RwLock<Vec<u8>>> = Default::default();
    //     let cid = Cid::new_v1(DAG_CBOR, Blake2b256.digest(&rawdata));
    //     let header = CarHeader {
    //         roots: vec![cid],
    //         version: 1,
    //     };

    //     assert_eq!(to_vec(&header).unwrap().len(), 60);

    //     let (tx, mut rx) = bounded(10);

    //     let buffer_cloned = buffer.clone();
    //     let write_task = async_std::task::spawn(async move {
    //         header
    //             .write_stream_async(&mut *buffer_cloned.write().await, &mut rx)
    //             .await
    //             .unwrap()
    //     });

    //     tx.send((cid, rawdata.clone())).await.unwrap();
    //     drop(tx);
    //     write_task.await;

    //     let buffer: Vec<_> = buffer.read().await.clone();

    //     let client = reqwest::blocking::Client::new();
    //     let res = client
    //         .post("https://api.web3.storage/car")
    //         .header(reqwest::header::CONTENT_TYPE, "application/vnd.ipld.car")
    //         .header("Authorization", "Bearer ...")
    //         .body(buffer)
    //         .send()
    //         .unwrap();

    //     if res.status().is_success() {
    //         eprintln!("upload success");
    //     } else {
    //         eprintln!("upload failed");
    //     }
    //     println!("{:?}, expected cid: {}", &res.bytes().unwrap(), cid);
    // }
}
