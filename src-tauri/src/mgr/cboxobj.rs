use super::*;

impl App {
    pub fn create_cbox_obj(&self, par: &CBoxObj) -> Result<(), String>{
        if !self.has_connection() {
            return Err("no db connection yet".to_owned())
        }
        if let Some(c) = &*self.conn.lock().unwrap() {
            c.execute(r#"
                insert into cbox_obj (box_id, provider, name, path, obj_type) values (?1, ?2, ?3, ?4, ?5)
            "#, params![par.box_id, par.provider, par.name, par.path, par.obj_type])
            .map_err(|err| format!("failed to create cbox: {}", err))?;
        }
        Ok(())
    }
    pub fn list_cbox_obj(&self) -> Result<Vec<CBoxObj>, String> {
        if let Some(c) = &*self.conn.lock().unwrap() {
            let mut stmt = c.prepare("SELECT id, box_id, provider, name, path, obj_type FROM cbox_obj").unwrap();
            let box_iter = stmt.query_map([], |row| {
                let mut b = CBoxObj::default();
                b.id = row.get(0)?;
                b.box_id = row.get(1)?;
                b.provider = row.get(2)?;
                b.name = row.get(3)?;
                b.path = row.get(4)?;
                b.obj_type = row.get(5)?;
                Ok(b)
            }).unwrap();
            let mut list: Vec<CBoxObj> = Vec::new();
            for b in box_iter {
                list.push(b.unwrap())
            }
            Ok(list)
        } else {
            Err("no db connection yet".to_owned())
        }
    }
}