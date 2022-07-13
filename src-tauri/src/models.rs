use std::ffi::OsString;

#[derive(Debug)]
pub struct CBox {
    id: i32,
    name: OsString,
    // most of time backup should be encrypt unless user intentionly set it false, maybe for public share
    encrypt_data: bool,
    // total objects in the box
    obj_total: u64,
    // total size of objects in the box
    size_total: u64,
    // the key use to do encrypt works
    secret: Vec<u8>,
    // the storage provider, like web3.storage
    provider: i32,
    // the current showing box for user
    active: u8,
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