use super::*;

impl App {
    pub fn connect_db(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut dbfile = PathBuf::from(self.app_dir.clone());
        dbfile.push(DB_FILE_NAME);
        let conn = Connection::open(dbfile)?;
        self.conn = Some(conn);
        Ok(())
    }
    pub fn init_db(&mut self) -> Result<(), Box<dyn std::error::Error>> {
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
                    active INTEGER DEFAULT 0,
                    create_at INTEGER,
                    modify_at INTEGER
                );
                CREATE UNIQUE INDEX IF NOT EXISTS index_cbox_name ON cbox (name);
                CREATE TABLE IF NOT EXISTS cbox_obj (
                    id    INTEGER PRIMARY KEY AUTOINCREMENT,
                    box_id INTEGER,
                    cid TEXT,
                    hash TEXT,
                    nonce BLOB,
                    size INTEGER,
                    name TEXT,
                    path TEXT,
                    origin_path TEXT,
                    status INTEGER,
                    obj_type INTEGER,
                    create_at INTEGER,
                    modify_at INTEGER,
                    parent_id INTEGER,
                    task_type INTEGER,
                    err TEXT
                );
                CREATE TABLE IF NOT EXISTS cbox_task (
                    id    INTEGER PRIMARY KEY AUTOINCREMENT,
                    box_id INTEGER,
                    obj_id INTEGER,
                    nonce BLOB,
                    origin_path TEXT,
                    target_path TEXT,
                    status INTEGER,
                    create_at INTEGER,
                    modify_at INTEGER,
                    task_type INTEGER,
                    err TEXT
                );
                CREATE TABLE IF NOT EXISTS identity (
                    id    INTEGER PRIMARY KEY AUTOINCREMENT,
                    secret BLOB
                );
                CREATE TABLE IF NOT EXISTS provider (
                    id    INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT,
                    put_api TEXT,
                    get_api TEXT
                );
                COMMIT;"#,
        )?;
        self.conn = Some(conn);
        Ok(())
    }
    pub fn has_connection(&self) -> bool {
        match self.conn {
            Some(_) => true,
            None => false,
        }
    }
}
