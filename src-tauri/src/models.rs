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
    // the current showing box for user
    pub active: u8,
}

#[derive(Debug)]
pub struct CBoxObj {
    id: i32,
    box_id: i32,
    provider: i32,
    // encrypted data cid 
    cid: String,
    nonce: Vec<u8>,
    size: u64,
    // filename
    name: OsString,
    // relative path
    path: OsString,
    // path of file in host file system
    origin_path: OsString,
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