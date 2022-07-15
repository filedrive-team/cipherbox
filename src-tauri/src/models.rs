use std::ffi::OsString;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CBox {
    pub id: i32,
    pub name: String,
    // most of time backup should be encrypt unless user intentionly set it false, maybe for public share
    pub encrypt_data: bool,
    // total objects in the box
    pub obj_total: u64,
    // total size of objects in the box
    pub size_total: u64,
    // the key use to do encrypt works
    pub secret: Vec<u8>,
    // the storage provider, like web3.storage
    pub provider: i32,
    // access token for provider api
    pub access_token: String,
    // the current showing box for user
    pub active: u8,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CBoxObj {
    pub id: i32,
    pub box_id: i32,
    pub provider: i32,
    // encrypted data cid 
    pub cid: String,
    pub nonce: Vec<u8>,
    pub size: u64,
    // filename
    pub name: String,
    // relative path
    pub path: String,
    // path of file in host file system
    pub origin_path: String,
    // object type - 0 file | 1 directory
    pub obj_type: u8,
}

#[derive(Debug)]
pub struct Identity {
    id: i32,
    secret: Vec<u8>
}

#[derive(Debug)]
pub struct Provider {
    id: i32,
    name: String,
    access_token: String,
    put_api: String,
    get_api: String,
}