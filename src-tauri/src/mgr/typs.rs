use crate::errors::Error;
use cid::Cid;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::{
    ffi::OsString,
    time::{SystemTime, SystemTimeError},
};

pub const CHUNK_SIZE: usize = 1048576;
pub static DB_FILE_NAME: &str = "cipherbox.db";
pub static CIPHER_MESSAGE_NAME: &str = "cipher_message";
pub static KV_FILE_NAME: &str = "cipherbox.kv.toml";

#[derive(Debug)]
pub enum ControlEvent {
    LoopStart,
    Resume(i64),
    Pause(i64),
    PauseAll,
    Cancel(i64),
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    // indicate whether user has set password or not
    pub has_password_set: bool,
    // valid session period after password been verified
    // will expire in a centain time, currently not implemented
    pub session_expired: bool,
    // active cbox
    pub active_box: Option<CBox>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct KVCache {
    pub active_box_id: i64,
}

#[derive(Debug, Default)]
pub struct App {
    pub conn: Option<Connection>,
    pub user_key: Option<[u8; 32]>,
    pub session_start: u64,
    pub app_dir: OsString,
    pub providers: Vec<Provider>,
    pub kv_cache: KVCache,
    pub tauri_handle: Option<tauri::AppHandle>,
    pub task_trigger: Option<async_std::channel::Sender<ControlEvent>>,
    pub running_task_num: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCboxParams {
    pub name: String,
    pub encrypt_data: bool,
    pub provider: i64,
    pub access_token: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CommonRes<T> {
    pub error: String,
    pub result: Option<T>,
}

impl<T> CommonRes<T> {
    pub fn error(err: Error) -> Self {
        CommonRes {
            error: format!("{}", err),
            result: None,
        }
    }
}

impl<T: Serialize> CommonRes<T> {
    pub fn ok(d: T) -> Self {
        CommonRes {
            error: "".into(),
            result: Some(d),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CBox {
    pub id: i64,
    pub name: String,
    // most of time backup should be encrypt unless user intentionly set it false, maybe for public share
    pub encrypt_data: bool,
    // total objects in the box
    pub obj_total: u64,
    // total size of objects in the box
    pub size_total: u64,
    // the key use to do encrypt works
    #[serde(skip_deserializing)]
    pub secret: Vec<u8>,
    // the storage provider, like web3.storage
    pub provider: i64,
    // access token for provider api
    pub access_token: String,
    // the current showing box for user
    pub active: u8,
    pub create_at: u64,
    pub modify_at: u64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CBoxObj {
    pub id: i64,
    pub box_id: i64,
    // encrypted data cid
    pub cid: String,
    // hex string of sha256 sum
    pub hash: String,
    #[serde(skip_serializing)]
    pub nonce: Vec<u8>,
    pub size: u64,
    // filename
    pub name: String,
    // relative path
    pub path: String,
    // path of file in host file system
    pub origin_path: String,
    // // backup status - 0 in queue | 1 uploading | 2 uploaded | 3 finished | 9 failed
    // pub status: u8,
    // object type - 0 file | 1 directory
    pub obj_type: u8,
    pub create_at: u64,
    pub modify_at: u64,
    pub parent_id: i64,
    // // task type - 0 single task | 1 parent task (has children tasks) | 2 child task
    // pub task_type: u8,
    // pub err: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CBoxTask {
    pub id: i64,
    pub box_id: i64,
    pub obj_id: i64,
    // path of file in host file system
    pub origin_path: String,
    // target path to do recover
    pub target_path: String,
    // status - 0 in queue | 1 processing  | 5 finished | 6 paused | 7 canceled | 9 failed
    pub status: u8,
    pub total: u64,
    pub total_size: u64,
    pub finished: u64,
    pub finished_size: u64,
    pub create_at: u64,
    pub modify_at: u64,
    // task type - 0 backup task | 1 recover task
    pub task_type: u8,
    pub err: String,
    #[serde(skip_serializing)]
    pub nonce: Vec<u8>,
}

#[derive(Debug)]
pub struct Identity {
    id: i32,
    secret: Vec<u8>,
}

#[derive(Debug)]
pub struct Provider {
    pub id: i32,
    pub name: String,
    pub put_api: String,
    pub get_api: String,
}

pub fn current() -> Result<u64, SystemTimeError> {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(d) => Ok(d.as_secs()),
        Err(e) => Err(e),
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ChoreProgress {
    pub box_id: i64,
    pub task_id: i64,
    pub total: u64,
    pub total_size: u64,
    pub finished: u64,
    pub finished_size: u64,
    pub backup: bool,
    pub recover: bool,
    pub err: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TaskRecord {
    pub box_id: i64,
    pub task_id: i64,
    pub total: u64,
    pub total_size: u64,
    pub finished: u64,
    pub finished_size: u64,
    pub backup: bool,
    pub recover: bool,
    pub upload_list: Vec<ChoreUploadRecord>,
    pub download_list: Vec<ChoreDownloadRecord>,
    pub err: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ChoreDownloadRecord {
    pub path: String,
    pub hash: String,
    pub size: u64,
    pub chunk_count: u64,
    pub chunk_downloaded: u64,
    pub downloaded_size: u64,
    pub chunks: Vec<Cid>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ChoreUploadRecord {
    pub path: String,
    pub size: u64,
    pub chunk_count: u64,
    pub chunk_uploaded: u64,
    pub uploaded_size: u64,
    pub chunks: Vec<Cid>,
    pub chunks_ref: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Chunks {
    pub chunk_size: u64,
    pub chunk_count: u64,
    pub chunks: Vec<Cid>,
}

#[derive(Serialize, Deserialize)]
pub struct RawBlock {
    pub data: Vec<u8>,
}
