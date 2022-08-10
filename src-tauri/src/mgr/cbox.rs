use super::*;

impl App {
    pub fn set_active_box(&mut self, box_id: i64) -> Result<CBox, Error> {
        let b = self.get_cbox_by_id(box_id)?;
        self.kv_cache.active_box_id = box_id;
        // let mut mg = self.kv_cache.lock().unwrap();
        // (*mg).active_box_id = box_id;
        // drop(mg);
        self.flush_cache()?;
        Ok(b)
    }
    pub fn create_cbox(&mut self, par: CreateCboxParams) -> Result<CBox, Error> {
        if !self.has_connection() {
            return Err(Error::SessionExpired);
        }
        let mut cbox = CBox::default();
        cbox.name = par.name;
        cbox.encrypt_data = par.encrypt_data;
        cbox.provider = par.provider;
        cbox.access_token = par.access_token;
        match self.box_secret() {
            Err(err) => return Err(err),
            Ok(s) => {
                cbox.secret = s;
            }
        };
        cbox.create_at = current()?;
        cbox.modify_at = cbox.create_at;
        let c = self.conn.as_ref().unwrap();

        c.execute(r#"
                insert into cbox (name, encrypt_data, provider, access_token, secret, create_at, modify_at) values (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            "#, params![cbox.name, cbox.encrypt_data, cbox.provider, cbox.access_token, cbox.secret, cbox.create_at, cbox.modify_at])?;
        let id = c.last_insert_rowid();
        cbox.id = id;

        self.set_active_box(cbox.id)?;

        Ok(cbox)
    }
    fn box_secret(&self) -> Result<Vec<u8>, Error> {
        match self.user_key {
            Some(uk) => {
                let mut bs = gen_nonce(32);
                for (i, v) in bs.iter_mut().enumerate() {
                    *v = uk[i] ^ *v
                }
                Ok(bs)
            }
            None => Err(Error::SessionExpired),
        }
    }
    pub fn list_cbox(&self) -> Result<Vec<CBox>, Error> {
        if let Some(c) = &self.conn {
            let mut stmt = c
                .prepare("SELECT id, name, encrypt_data, provider, access_token FROM cbox")
                .unwrap();
            let box_iter = stmt.query_map([], |row| {
                let mut b = CBox::default();
                b.id = row.get(0)?;
                b.name = row.get(1)?;
                b.encrypt_data = row.get(2)?;
                b.provider = row.get(3)?;
                b.access_token = row.get(4)?;
                Ok(b)
            })?;

            let mut list: Vec<CBox> = Vec::new();
            for b in box_iter {
                list.push(b.unwrap())
            }
            Ok(list)
        } else {
            Err(Error::SessionExpired)
        }
    }
    pub fn get_cbox_by_id(&self, box_id: i64) -> Result<CBox, Error> {
        if let Some(c) = &self.conn {
            let b = c.query_row(
                "SELECT id, name, encrypt_data, provider, access_token FROM cbox where id = ?1",
                params![box_id],
                |row| {
                    let mut b = CBox::default();
                    b.id = row.get(0)?;
                    b.name = row.get(1)?;
                    b.encrypt_data = row.get(2)?;
                    b.provider = row.get(3)?;
                    b.access_token = row.get(4)?;
                    Ok(b)
                },
            )?;

            Ok(b)
        } else {
            Err(Error::SessionExpired)
        }
    }
}
