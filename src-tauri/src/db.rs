use std::sync::{Mutex};
use rusqlite::{Connection};
static DB_FILE_NAME: &str = "cipherbox.db";

pub(crate) struct DB(Mutex<Connection>);


pub(crate) fn db_init(app_dir: &mut std::path::PathBuf) -> Result<DB, Box<dyn std::error::Error>> {
    app_dir.push(DB_FILE_NAME);
    let conn = Connection::open(app_dir)?;
    Ok(DB(Mutex::new(conn)))
}
