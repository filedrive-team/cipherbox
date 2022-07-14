use crate::db::{Connection, DB_FILE_NAME};
use std::{
    time::{SystemTime, SystemTimeError},
    path::{PathBuf},
    sync::{Mutex},
    ffi::{OsString},
};


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
    pub fn has_connection(&self) -> bool {
        match *self.conn.lock().unwrap() {
            Some(_) => true,
            None => false
        }
    }
}