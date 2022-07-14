use std::{
    time::{SystemTime, SystemTimeError},
    path::{PathBuf},
    sync::{Mutex},
    ffi::{OsString},
};
use serde::{Serialize, Deserialize};
use crate::{
    db::{DB_FILE_NAME},
    models::{
        CBox,
    },
};
use rusqlite::{Connection, params};

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
                    path TEXT
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
        "#).expect("failed tp do json deserialize");
        let new_box02 = app.create_cbox(cbpa02).expect("failed to create cbox");
        dbg!(new_box02);
        // query Cbox
        let list = app.list_cbox().unwrap();
        dbg!(&list);
        let list_json: String = serde_json::to_string(&list).unwrap();
        dbg!(&list_json);
    }
}