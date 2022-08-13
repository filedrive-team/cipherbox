use super::*;

impl App {
    pub fn get_parent_id(&self, box_id: i64, path: &str) -> Result<i64, Error> {
        let mut pa = PathBuf::from(path);
        let mut dir_list = Vec::<String>::new();
        let mut parent_id = 0i64;
        loop {
            match pa.parent() {
                Some(p) => {
                    dir_list.push(
                        p.as_os_str()
                            .to_str()
                            .expect("&OsStr => &Str failed")
                            .into(),
                    );
                    pa = PathBuf::from(p);
                }
                None => break,
            };
        }
        for p in dir_list.into_iter().rev() {
            if p == "" {
                continue;
            }
            match self.get_cbox_obj(box_id, &p) {
                Some(obj) => {
                    parent_id = obj.id;
                }
                None => {
                    let mut obj = CBoxObj::default();
                    obj.box_id = box_id;
                    obj.path = p;
                    obj.obj_type = 1;
                    obj.create_at = current().expect("failed to get current timestamp");
                    obj.modify_at = obj.create_at;
                    obj.name = match PathBuf::from(&obj.path).file_name() {
                        Some(n) => n.to_str().expect("&OsStr => &Str failed").into(),
                        None => "".into(),
                    };
                    match self.create_cbox_obj(&obj) {
                        Ok(id) => {
                            parent_id = id;
                        }
                        Err(err) => return Err(err),
                    };
                }
            }
        }
        Ok(parent_id)
    }
    pub fn create_cbox_obj(&self, par: &CBoxObj) -> Result<i64, Error> {
        if !self.has_connection() {
            return Err(Error::NoDBConnection);
        }
        let c = self.conn.as_ref().unwrap();

        c.execute(
            r#"
            insert into cbox_obj (box_id, cid, hash, name, path, size, origin_path, obj_type, create_at, modify_at, nonce, parent_id) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
        "#,
            params![par.box_id, par.cid, par.hash, par.name, par.path, par.size, par.origin_path, par.obj_type, par.create_at, par.modify_at, par.nonce, par.parent_id],
        )?;
        Ok(c.last_insert_rowid())
    }
    pub fn list_cbox_obj(&self) -> Result<Vec<CBoxObj>, Error> {
        if let Some(c) = &self.conn {
            let mut stmt = c
                .prepare("SELECT id, box_id, name, path, size, origin_path, obj_type, create_at, modify_at, parent_id FROM cbox_obj")
                .unwrap();
            let box_iter = stmt.query_map([], |row| {
                let mut b = CBoxObj::default();
                b.id = row.get(0)?;
                b.box_id = row.get(1)?;
                b.name = row.get(2)?;
                b.path = row.get(3)?;
                b.size = row.get(4)?;
                b.origin_path = row.get(5)?;
                b.obj_type = row.get(6)?;
                b.create_at = row.get(7)?;
                b.modify_at = row.get(8)?;
                b.parent_id = row.get(9)?;
                Ok(b)
            })?;

            let mut list: Vec<CBoxObj> = Vec::new();
            for b in box_iter {
                list.push(b.unwrap())
            }
            Ok(list)
        } else {
            Err(Error::NoDBConnection)
        }
    }
    pub fn get_cbox_obj(&self, box_id: i64, path: &str) -> Option<CBoxObj> {
        if let Some(c) = &self.conn {
            let mut stmt = c
                .prepare("id, box_id, cid, hash, name, path, size, origin_path, obj_type, create_at, modify_at, nonce FROM cbox_obj where box_id = ?1 and path = ?2")
                .unwrap();
            match stmt.execute(params![box_id, path]) {
                Ok(_) => {}
                Err(e) => {
                    eprint!("{}", e);
                    return None;
                }
            };
            let box_iter = match stmt.query_map([], |row| {
                let mut b = CBoxObj::default();
                b.id = row.get(0)?;
                b.box_id = row.get(1)?;
                b.cid = row.get(2)?;
                b.hash = row.get(3)?;
                b.name = row.get(4)?;
                b.path = row.get(5)?;
                b.size = row.get(6)?;
                b.origin_path = row.get(7)?;
                b.obj_type = row.get(8)?;
                b.create_at = row.get(9)?;
                b.modify_at = row.get(10)?;
                b.nonce = row.get(11)?;
                Ok(b)
            }) {
                Ok(it) => it,
                Err(err) => {
                    eprint!("{}", err);
                    return None;
                }
            };

            let mut list: Vec<CBoxObj> = Vec::new();
            for b in box_iter {
                list.push(b.unwrap())
            }
            if list.len() == 0 {
                return None;
            }
            Some(list.remove(0))
        } else {
            None
        }
    }
    pub fn get_cbox_obj_by_id(&self, id: i64) -> Option<CBoxObj> {
        if let Some(c) = &self.conn {
            let b = match c.query_row(
                "SELECT id, box_id, cid, hash, name, path, size, origin_path, obj_type, create_at, modify_at, nonce FROM cbox_obj where id = ?1",
                params![id],
                |row| {
                    let mut b = CBoxObj::default();
                    b.id = row.get(0)?;
                    b.box_id = row.get(1)?;
                    b.cid = row.get(2)?;
                    b.hash = row.get(3)?;
                    b.name = row.get(4)?;
                    b.path = row.get(5)?;
                    b.size = row.get(6)?;
                    b.origin_path = row.get(7)?;
                    b.obj_type = row.get(8)?;
                    b.create_at = row.get(9)?;
                    b.modify_at = row.get(10)?;
                    b.nonce = row.get(11)?;
                    Ok(b)
                },
            ){
                Ok(obj) => obj,
                Err(_) => return None  
            };

            Some(b)
        } else {
            None
        }
    }
}
