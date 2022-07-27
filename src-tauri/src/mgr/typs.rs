use crate::errors::Error;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::{
    ffi::OsString,
    sync::Mutex,
    time::{SystemTime, SystemTimeError},
};

pub static DB_FILE_NAME: &str = "cipherbox.db";
pub static CIPHER_MESSAGE_NAME: &str = "cipher_message";
pub static KV_FILE_NAME: &str = "cipherbox.kv.toml";

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
    pub conn: Mutex<Option<Connection>>,
    pub user_key: Mutex<Option<[u8; 32]>>,
    pub session_start: u64,
    pub app_dir: OsString,
    pub providers: Vec<Provider>,
    pub kv_cache: Mutex<KVCache>,
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
    #[serde(skip_deserializing)]
    pub nonce: Vec<u8>,
    pub size: u64,
    // filename
    pub name: String,
    // relative path
    pub path: String,
    // path of file in host file system
    pub origin_path: String,
    // backup status - 0 in queue | 1 uploading | 2 uploaded | 3 finished
    pub status: u8,
    // object type - 0 file | 1 directory
    pub obj_type: u8,
    pub create_at: u64,
    pub modify_at: u64,
    pub parent_id: i64,
    // task type - 0 single task | 1 has children tasks
    pub task_type: u8,
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
