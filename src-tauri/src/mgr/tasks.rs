use super::*;
use tauri::Manager;

impl App {
    pub async fn setup_chore_loop(&'static self) {
        // async_std::task::spawn(async move {
        //     self.emit_all(ChoreProgress::default());
        // });

        // async_std::task::spawn(async {
        //     self.emit_all(ChoreProgress::default());
        // });
    }
    pub fn emit_all(&self, cp: ChoreProgress) {
        if let Some(ref h) = self.tauri_handle {
            match h.emit_all("progress", cp) {
                Ok(_) => {}
                Err(_e) => {}
            }
        }
    }
    pub fn list_task(&self, status: Vec<i32>) -> Result<Vec<CBoxTask>, Error> {
        if let Some(c) = &self.conn {
            let sqlstr = match status.len() {
                0 => "SELECT id, box_id, origin_path, target_path, task_type, create_at, modify_at, status, err FROM cbox_task order by id desc".to_string(),
                _ => {
                    let mut ss = String::from("SELECT id, box_id, origin_path, target_path, task_type, create_at, modify_at, status, err FROM cbox_task where status in ( ");
                    for sta in status.into_iter().enumerate() {
                        if sta.0 == 0 {
                            ss = format!("{}{}", ss, sta.1)
                        } else {
                            ss = format!("{},{}", ss, sta.1)
                        }
                    }
                    ss = format!("{} ) order by id desc", ss);
                    ss
                }
            };
            let mut stmt = c.prepare(&sqlstr).unwrap();
            let box_iter = stmt.query_map([], |row| {
                let mut b = CBoxTask::default();
                b.id = row.get(0)?;
                b.box_id = row.get(1)?;
                b.origin_path = row.get(2)?;
                b.target_path = row.get(3)?;
                b.task_type = row.get(4)?;
                b.create_at = row.get(5)?;
                b.modify_at = row.get(6)?;
                b.status = row.get(7)?;
                b.err = row.get(8)?;
                Ok(b)
            })?;

            let mut list: Vec<CBoxTask> = Vec::new();
            for b in box_iter {
                list.push(b.unwrap())
            }
            Ok(list)
        } else {
            Err(Error::NoDBConnection)
        }
    }
    pub fn get_pending_task(&self) -> Option<CBoxTask> {
        if let Some(c) = &self.conn {
            let mut stmt = c
                .prepare("SELECT id, box_id, nonce, origin_path, target_path, task_type FROM cbox_task where status = 0 order by id asc limit 1")
                .unwrap();
            let box_iter = match stmt.query_map([], |row| {
                let mut b = CBoxTask::default();
                b.id = row.get(0)?;
                b.box_id = row.get(1)?;
                b.nonce = row.get(2)?;
                b.origin_path = row.get(3)?;
                b.target_path = row.get(4)?;
                b.task_type = row.get(5)?;
                Ok(b)
            }) {
                Ok(item) => item,
                Err(_) => return None,
            };

            let mut list: Vec<CBoxTask> = Vec::new();
            for b in box_iter {
                if let Ok(record) = b {
                    list.push(record);
                }
            }
            if list.len() == 0 {
                None
            } else {
                let task = list.remove(0);
                let status = match task.task_type {
                    0 => 1,
                    1 => 3,
                    _ => 1,
                };
                // update task state
                match self.update_task_status(task.id, status) {
                    Ok(_) => Some(task),
                    Err(err) => {
                        eprint!("{:?}", err);
                        None
                    }
                }
            }
        } else {
            None
        }
    }
    pub fn update_task_status(&self, id: i64, status: isize) -> Result<(), Error> {
        if !self.has_connection() {
            return Err(Error::NoDBConnection);
        }

        let c = self.conn.as_ref().unwrap();
        c.execute(
            r#"
            update cbox_task set status = ?1 where id = ?2
        "#,
            params![status, id],
        )?;
        Ok(())
    }
    pub fn record_task_err(&self, id: i64, err: Error) -> Result<(), Error> {
        if !self.has_connection() {
            return Err(Error::NoDBConnection);
        }

        let c = self.conn.as_ref().unwrap();
        c.execute(
            r#"
            update cbox_task set status = ?1, err = ?2 where id = ?3
        "#,
            params![9, err.to_string(), id],
        )?;
        Ok(())
    }
    pub fn create_cbox_task(&self, par: &CBoxTask) -> Result<i64, Error> {
        if !self.has_connection() {
            return Err(Error::NoDBConnection);
        }

        let c = self.conn.as_ref().unwrap();
        c.execute(
            r#"
            insert into cbox_task (box_id, nonce, origin_path, target_path, task_type, create_at, modify_at, status) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
        "#,
            params![par.box_id, par.nonce, par.origin_path, par.target_path, par.task_type, par.create_at, par.modify_at, par.status],
        )?;
        Ok(c.last_insert_rowid())
    }
    pub fn add_backup_tasks(&self, box_id: i64, targets: Vec<String>) -> Result<(), Error> {
        if !self.has_connection() {
            return Err(Error::NoDBConnection);
        }
        let mut task_list = Vec::<CBoxTask>::with_capacity(targets.len());

        for p in targets.iter() {
            //let meta = std::fs::metadata(p)?;
            let mut obj = CBoxTask::default();
            obj.box_id = box_id;
            obj.status = 0;
            obj.task_type = 0;
            obj.nonce = gen_nonce(12);

            // match std::path::Path::new(p).file_name() {
            //     Some(n) => match n.to_str() {
            //         Some(n) => obj.name = n.to_owned(),
            //         None => {
            //             return Err(Error::Other(format!("failed to read file name for {}", p)))
            //         }
            //     },
            //     None => return Err(Error::Other(format!("failed to read file name for {}", p))),
            // }

            obj.origin_path = p.clone();

            //obj.nonce = gen_nonce(12);
            obj.create_at = current()?;
            obj.modify_at = obj.create_at;
            task_list.push(obj);
        }
        // add single tasks
        for obj in task_list.iter() {
            self.create_cbox_task(obj)?;
        }

        // TODO:
        // trigger async backup or recover task
        if let Some(s) = &self.task_trigger {
            match async_std::task::block_on(s.send(ControlEvent::LoopStart)) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("failed to send LoopStart message: {}", e);
                }
            }
        }
        Ok(())
    }
}
