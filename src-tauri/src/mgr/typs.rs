use std::{
    time::{SystemTime, SystemTimeError},
    sync::{Mutex},
    ffi::{OsString},
};
use serde::{Serialize, Deserialize};
use crate::{
    models::{
        CBox, CBoxObj, Provider,
    },
    errors::Error,
};
use rusqlite::{
    Connection,
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
    pub active_box_id: i32
}

#[derive(Debug, Default)]
pub struct App {
    pub conn: Mutex<Option<Connection>>,
    pub user_key: Mutex<Option<[u8;32]>>,
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
    pub provider: i32,
    pub access_token: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CommonRes<T> {
    pub error: String,
    pub result: Option<T>,
}

impl <T> CommonRes<T> {
    pub fn error(err: Error) -> Self {
        CommonRes { error: format!("{}", err), result: None }
    }
}

impl<T: Serialize> CommonRes<T> {
    pub fn ok(d: T) -> Self {
        CommonRes { error: "".into(), result: Some(d) }
    }
}

pub fn current() -> Result<u64, SystemTimeError>{
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(d) => Ok(d.as_secs()),
        Err(e) => Err(e)
    }
}