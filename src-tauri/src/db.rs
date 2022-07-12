use std::sync::{Mutex};
use rusqlite::{Connection};
static DB_FILE_NAME: &str = "cipherbox.db";

pub(crate) struct DB(Mutex<Connection>);

    
pub(crate) fn db_init(app_dir: &mut std::path::PathBuf) -> Result<DB, Box<dyn std::error::Error>> {
    // make sure directories has been created
    std::fs::create_dir_all(app_dir.clone())?;
    app_dir.push(DB_FILE_NAME);
    let conn = Connection::open(app_dir)?;
    
    conn.execute_batch(
    r#"BEGIN;
        CREATE TABLE IF NOT EXISTS cbox (
            id    INTEGER PRIMARY KEY AUTOINCREMENT,
            name  TEXT NOT NULL,
            encrypt_data INTEGER,
            obj_total INTEGER,
            size_total INTEGER,
            secret BLOB,
            provider  INTEGER
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
    Ok(DB(Mutex::new(conn)))
}


