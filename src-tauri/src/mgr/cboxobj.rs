use super::*;

impl App {
    pub fn add_backup_tasks(&self, box_id: i64, targets: Vec<String>) -> Result<(), Error> {
        if !self.has_connection() {
            return Err(Error::NoDBConnection);
        }
        let mut obj_list = Vec::<CBoxObj>::with_capacity(targets.len());
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
            obj.obj_type = match meta.is_dir() {
                true => 1,
                false => 0,
            };
            obj_list.push(obj);
        }
        for obj in obj_list.iter() {
            self.create_cbox_obj(obj)?;
        }
        Ok(())
    }
    pub fn create_cbox_obj(&self, par: &CBoxObj) -> Result<(), Error> {
        if !self.has_connection() {
            return Err(Error::NoDBConnection);
        }
        if let Some(c) = &*self.conn.lock().unwrap() {
            c.execute(
                r#"
                insert into cbox_obj (box_id, name, path, size, origin_path, obj_type) values (?1, ?2, ?3, ?4, ?5, ?6)
            "#,
                params![par.box_id, par.name, par.path, par.size, par.origin_path, par.obj_type],
            )?;
        }
        Ok(())
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
