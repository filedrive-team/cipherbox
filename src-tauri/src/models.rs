use std::ffi::OsString;

#[derive(Debug)]
pub struct CBox {
    id: i32,
    name: OsString,
    encrypt_data: bool,
    obj_total: u64,
    size_total: u64,
    secret: Vec<u8>,
    provider: i32,
}

#[derive(Debug)]
pub struct CBoxObj {
    id: i32,
    box_id: i32,
    provider: i32,
    cid: String,
    nonce: Vec<u8>,
    size: u64,
    name: OsString,
    path: OsString,
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