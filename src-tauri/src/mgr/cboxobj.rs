use super::*;

impl App {
    pub fn create_cbox_obj(&self, par: &CBoxObj) -> Result<i64, Error> {
        if !self.has_connection() {
            return Err(Error::NoDBConnection);
        }
        let c = self.conn.as_ref().unwrap();

        c.execute(
            r#"
            insert into cbox_obj (box_id, name, path, size, origin_path, obj_type, task_type, create_at, modify_at, nonce, status) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
        "#,
            params![par.box_id, par.name, par.path, par.size, par.origin_path, par.obj_type, par.task_type, par.create_at, par.modify_at, par.nonce, par.status],
        )?;
        Ok(c.last_insert_rowid())
    }
    pub fn list_cbox_obj(&self) -> Result<Vec<CBoxObj>, Error> {
        if let Some(c) = &self.conn {
            let mut stmt = c
                .prepare("SELECT id, box_id, name, path, size, origin_path, obj_type FROM cbox_obj")
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
}
