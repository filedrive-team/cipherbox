use super::*;

impl App {
    pub fn add_backup_tasks(&self, box_id: i64, targets: Vec<String>) -> Result<(), Error> {
        if !self.has_connection() {
            return Err(Error::NoDBConnection);
        }
        let mut obj_list = Vec::<CBoxObj>::with_capacity(targets.len());
        let mut dir_list = Vec::<CBoxObj>::new();
        for p in targets.iter() {
            let meta = std::fs::metadata(p)?;
            let mut obj = CBoxObj::default();
            obj.box_id = box_id;

            match std::path::Path::new(p).file_name() {
                Some(n) => match n.to_str() {
                    Some(n) => obj.name = n.to_owned(),
                    None => {
                        return Err(Error::Other(format!("failed to read file name for {}", p)))
                    }
                },
                None => return Err(Error::Other(format!("failed to read file name for {}", p))),
            }
            obj.path = obj.name.clone();
            obj.size = meta.len();
            obj.origin_path = p.clone();

            obj.nonce = gen_nonce(12);
            obj.create_at = current()?;
            obj.modify_at = obj.create_at;
            obj.status = 0;
            match meta.is_dir() {
                true => {
                    obj.obj_type = 1;
                    obj.task_type = 1;
                    dir_list.push(obj);
                }
                false => {
                    obj.obj_type = 0;
                    obj.task_type = 0;
                    obj_list.push(obj);
                }
            };
        }
        // add single tasks
        for obj in obj_list.iter() {
            self.create_cbox_obj(obj)?;
        }
        // TODO:
        // add dir tasks
        _ = dir_list;
        Ok(())
    }
    pub fn create_cbox_obj(&self, par: &CBoxObj) -> Result<i64, Error> {
        if !self.has_connection() {
            return Err(Error::NoDBConnection);
        }
        let mut insertId = 0i64;
        if let Some(c) = &*self.conn.lock().unwrap() {
            c.execute(
                r#"
                insert into cbox_obj (box_id, name, path, size, origin_path, obj_type, task_type, create_at, modify_at, nonce, status) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
            "#,
                params![par.box_id, par.name, par.path, par.size, par.origin_path, par.obj_type, par.task_type, par.create_at, par.modify_at, par.nonce, par.status],
            )?;
            insertId = c.last_insert_rowid();
        }
        match insertId {
            0 => Err(Error::Other("sqlite error: failed to get insert id".into())),
            _ => Ok(insertId),
        }
    }
    pub fn list_cbox_obj(&self) -> Result<Vec<CBoxObj>, Error> {
        if let Some(c) = &*self.conn.lock().unwrap() {
            let mut stmt = c
                .prepare("SELECT id, box_id, name, path, size, origin_path, obj_type FROM cbox_obj")
                .unwrap();
            let box_iter = stmt
                .query_map([], |row| {
                    let mut b = CBoxObj::default();
                    b.id = row.get(0)?;
                    b.box_id = row.get(1)?;
                    b.name = row.get(2)?;
                    b.path = row.get(3)?;
                    b.size = row.get(4)?;
                    b.origin_path = row.get(5)?;
                    b.obj_type = row.get(6)?;
                    Ok(b)
                })
                .unwrap();
            let mut list: Vec<CBoxObj> = Vec::new();
            for b in box_iter {
                list.push(b.unwrap())
            }
            Ok(list)
        } else {
            Err(Error::NoDBConnection)
        }
    }
}
