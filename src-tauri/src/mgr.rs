use std::{
    time::{SystemTime, SystemTimeError},
    path::{PathBuf},
    sync::{Mutex},
    ffi::{OsString},
};
use serde::{Serialize, Deserialize};
use crate::{
    models::{
        CBox, CBoxObj,
    },
};
use rusqlite::{
    Connection, params,
};
pub static DB_FILE_NAME: &str = "cipherbox.db";

#[derive(Debug, Default)]
pub struct App {
    pub conn: Mutex<Option<Connection>>,
    pub user_key: Mutex<Option<[u8;32]>>,
    pub session_start: u64,
    pub app_dir: OsString,
}
pub fn current() -> Result<u64, SystemTimeError>{
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(d) => Ok(d.as_secs()),
        Err(e) => Err(e)
    }
}

impl App {
    pub fn new(app_dir: OsString) -> Self {
        let mut app = App::default();
        app.app_dir = app_dir;
        app
    }
    // pub fn set_user_key(&mut self, key:[u8;32]) {
    //     *self.user_key.lock().unwrap() = Some(key);
    // }
    pub fn release_user_key(&mut self) {
        *self.user_key.lock().unwrap() = None;
    }
    pub fn is_key_set(&self) -> bool {
        if let None = *self.user_key.lock().unwrap() {
            return false
        }
        true 
    }
    pub fn connect_db(&mut self) -> Result<(), Box<dyn std::error::Error>>{
        let mut dbfile = PathBuf::from(self.app_dir.clone());
        dbfile.push(DB_FILE_NAME);
        let conn = Connection::open(dbfile)?;
        *self.conn.lock().unwrap() = Some(conn);
        Ok(())
    }
    pub fn init_db(&mut self) -> Result<(), Box<dyn std::error::Error>>{
        let mut dbfile = PathBuf::from(self.app_dir.clone());
        dbfile.push(DB_FILE_NAME);
        dbg!(dbfile.as_os_str());
        let conn = Connection::open(dbfile)?;
        
        conn.execute_batch(
            r#"BEGIN;
                CREATE TABLE IF NOT EXISTS cbox (
                    id    INTEGER PRIMARY KEY AUTOINCREMENT,
                    name  TEXT NOT NULL,
                    encrypt_data INTEGER,
                    obj_total INTEGER DEFAULT 0,
                    size_total INTEGER DEFAULT 0,
                    secret BLOB,
                    provider  INTEGER,
                    access_token TEXT NOT NULL,
                    active INTEGER DEFAULT 0
                );
                CREATE TABLE IF NOT EXISTS cbox_obj (
                    id    INTEGER PRIMARY KEY AUTOINCREMENT,
                    box_id INTEGER,
                    provider  INTEGER,
                    cid TEXT,
                    nonce BLOB,
                    size INTEGER,
                    name TEXT,
                    path TEXT,
                    obj_type INTEGER
                );
                CREATE TABLE IF NOT EXISTS identity (
                    id    INTEGER PRIMARY KEY AUTOINCREMENT,
                    secret BLOB
                );
                CREATE TABLE IF NOT EXISTS provider (
                    id    INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT,
                    access_token TEXT,
                    put_api TEXT,
                    get_api TEXT
                );
                COMMIT;"#, 
            )?;
        *self.conn.lock().unwrap() = Some(conn);
        Ok(())
    }
    pub fn has_connection(&self) -> bool {
        match *self.conn.lock().unwrap() {
            Some(_) => true,
            None => false
        }
    }
    pub fn create_cbox(&self, par: CreateCboxParams) -> Result<CBox, String> {
        let mut cbox = CBox::default();
        cbox.name = par.name;
        cbox.encrypt_data = par.encrypt_data;
        cbox.provider = par.provider;
        cbox.access_token = par.access_token;
        if !self.has_connection() {
            return Err("no db connection yet".to_owned())
        }
        if let Some(c) = &*self.conn.lock().unwrap() {
            c.execute(r#"
                insert into cbox (name, encrypt_data, provider, access_token) values (?1, ?2, ?3, ?4)
            "#, params![cbox.name, cbox.encrypt_data, cbox.provider, cbox.access_token])
            .map_err(|err| format!("failed to create cbox: {}", err))?;
        }
        
        Ok(cbox)
    }
    pub fn list_cbox(&self) -> Result<Vec<CBox>, String> {
        if let Some(c) = &*self.conn.lock().unwrap() {
            let mut stmt = c.prepare("SELECT id, name, encrypt_data, provider, access_token FROM cbox").unwrap();
            let box_iter = stmt.query_map([], |row| {
                let mut b = CBox::default();
                b.id = row.get(0)?;
                b.name = row.get(1)?;
                b.encrypt_data = row.get(2)?;
                b.provider = row.get(3)?;
                b.access_token = row.get(4)?;
                Ok(b)
            }).unwrap();
            let mut list: Vec<CBox> = Vec::new();
            for b in box_iter {
                list.push(b.unwrap())
            }
            Ok(list)
        } else {
            Err("no db connection yet".to_owned())
        }
    }
    pub fn create_cbox_obj(&self, par: &CBoxObj) -> Result<(), String>{
        if !self.has_connection() {
            return Err("no db connection yet".to_owned())
        }
        if let Some(c) = &*self.conn.lock().unwrap() {
            c.execute(r#"
                insert into cbox_obj (box_id, provider, name, path, obj_type) values (?1, ?2, ?3, ?4, ?5)
            "#, params![par.box_id, par.provider, par.name, par.path, par.obj_type])
            .map_err(|err| format!("failed to create cbox: {}", err))?;
        }
        Ok(())
    }
    pub fn list_cbox_obj(&self) -> Result<Vec<CBoxObj>, String> {
        if let Some(c) = &*self.conn.lock().unwrap() {
            let mut stmt = c.prepare("SELECT id, box_id, provider, name, path, obj_type FROM cbox_obj").unwrap();
            let box_iter = stmt.query_map([], |row| {
                let mut b = CBoxObj::default();
                b.id = row.get(0)?;
                b.box_id = row.get(1)?;
                b.provider = row.get(2)?;
                b.name = row.get(3)?;
                b.path = row.get(4)?;
                b.obj_type = row.get(5)?;
                Ok(b)
            }).unwrap();
            let mut list: Vec<CBoxObj> = Vec::new();
            for b in box_iter {
                list.push(b.unwrap())
            }
            Ok(list)
        } else {
            Err("no db connection yet".to_owned())
        }
    }
}




#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCboxParams {
    pub name: String,
    pub encrypt_data: bool,
    pub provider: i32,
    pub access_token: String,
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_data_flow () {
        // get sys temp dir
        let temp_dir = std::env::temp_dir();
        // init a App
        let mut app = App::new(temp_dir.as_os_str().to_owned());
        // init db
        app.init_db().expect("failed to init sqlite");
        // create a Cbox
        let cbpa01: CreateCboxParams = serde_json::from_str(r#"
            {
                "name": "cbox_x_00001",
                "encrypt_data": true,
                "provider": 1,
                "access_token": "token:for:web3.storage"
            }
        "#).expect("failed tp do json deserialize");
        let new_box01 = app.create_cbox(cbpa01).expect("failed to create cbox");
        dbg!(new_box01);
        // create another Cbox
        let cbpa02: CreateCboxParams = serde_json::from_str(r#"
            {
                "name": "cbox_x_00002",
                "encrypt_data": false,
                "provider": 1,
                "access_token": "token:for:nft.storage"
            }
        "#).expect("failed to do json deserialize");
        let new_box02 = app.create_cbox(cbpa02).expect("failed to create cbox");
        dbg!(new_box02);
        // query Cbox
        let list = app.list_cbox().unwrap();
        dbg!(&list);
        let list_json: String = serde_json::to_string(&list).unwrap();
        dbg!(&list_json);
        let mut obj01 = CBoxObj::default();
        obj01.box_id = 1;
        obj01.name = "cbox_obj_o1".into();
        obj01.provider = 1;
        obj01.obj_type = 0;
        
        // create cbox obj
        app.create_cbox_obj(&obj01).unwrap();
        // query CboxObj
        let objlist = app.list_cbox_obj().unwrap();
        dbg!(&objlist);
        let objlist_json = serde_json::to_string(&objlist).unwrap();
        dbg!(objlist_json);
    }

    use fvm_ipld_car::{Block, CarHeader, CarReader, load_car};
    use cid::{
        Cid,
        multihash::{
            Code::Blake2b256,
            MultihashDigest,
        },
    };
    use fvm_ipld_encoding::{from_slice, to_vec, DAG_CBOR};
    use async_std::channel::bounded;
    use async_std::io::Cursor;
    use async_std::sync::RwLock;
    use fvm_ipld_blockstore::{Blockstore, MemoryBlockstore};
    use std::sync::Arc;

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
    fn test_download(){
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
    use serde::{Serialize};

    #[derive(Serialize)]
    pub struct TCarGen {
        pub name: String,
        pub data: Vec<u8>,
    }
    #[async_std::test]
    async fn test_upload_car() {
        // let rawdata = to_vec(&TCarGen {
        //     name: "ii".into(),
        //     data: b"Hush little baby don't say a word".to_vec(),
        // }).unwrap();
        // let buffer: Arc<RwLock<Vec<u8>>> = Default::default();
        // let cid = Cid::new_v1(DAG_CBOR, Blake2b256.digest(&rawdata));
        // let header = CarHeader {
        //     roots: vec![cid],
        //     version: 1,
        // };
        // assert_eq!(to_vec(&header).unwrap().len(), 60);

        // let (tx, mut rx) = bounded(10);

        // let buffer_cloned = buffer.clone();
        // let write_task = async_std::task::spawn(async move {
        //     header
        //         .write_stream_async(&mut *buffer_cloned.write().await, &mut rx)
        //         .await
        //         .unwrap()
        // });

        // tx.send((cid, rawdata.clone())).await.unwrap();
        // drop(tx);
        // write_task.await;

        // let buffer: Vec<_> = buffer.read().await.clone();
        
        // let client = reqwest::blocking::Client::new();
        // let res = client.post("https://api.web3.storage/car")
        //     .header(reqwest::header::CONTENT_TYPE, "application/vnd.ipld.car")
        //     .header("Authorization", "Bearer ...")
        //     .body(buffer)
        //     .send()
        //     .unwrap();
        // dbg!(&res);
        // dbg!(&res.bytes().unwrap());
        
    }
}