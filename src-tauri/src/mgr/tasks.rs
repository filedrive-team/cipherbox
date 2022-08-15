use super::*;

impl App {
    pub fn list_task(&self, status: Vec<i32>) -> Result<Vec<CBoxTask>, Error> {
        if let Some(c) = &self.conn {
            let sqlstr = match status.len() {
                0 => "SELECT id, box_id, origin_path, target_path, task_type, create_at, modify_at, status, err, total, total_size, finished, finished_size, obj_id FROM cbox_task order by id desc".to_string(),
                _ => {
                    let mut ss = String::from("SELECT id, box_id, origin_path, target_path, task_type, create_at, modify_at, status, err, total, total_size, finished, finished_size, obj_id FROM cbox_task where status in ( ");
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
                b.total = row.get(9)?;
                b.total_size = row.get(10)?;
                b.finished = row.get(11)?;
                b.finished_size = row.get(12)?;
                b.obj_id = row.get(13)?;
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
                .prepare("SELECT id, box_id, nonce, origin_path, target_path, task_type, obj_id FROM cbox_task where status = 0 order by id asc limit 1")
                .unwrap();
            let box_iter = match stmt.query_map([], |row| {
                let mut b = CBoxTask::default();
                b.id = row.get(0)?;
                b.box_id = row.get(1)?;
                b.nonce = row.get(2)?;
                b.origin_path = row.get(3)?;
                b.target_path = row.get(4)?;
                b.task_type = row.get(5)?;
                b.obj_id = row.get(6)?;
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
                // let status = match task.task_type {
                //     0 => 1,
                //     1 => 3,
                //     _ => 1,
                // };
                // update task state
                match self.update_task_status(task.id, 1) {
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
    pub fn resume_task(&self, id: i64) {
        if let Some(ta) = self.get_task_by_id(id) {
            match ta.status {
                6 | 7 | 9 => {
                    self.update_task_status(ta.id, 0)
                        .unwrap_or_else(|e| eprint!("failed to resume task: {}, {}", ta.id, e));
                }
                _ => {
                    eprint!("resume a task in unexpected status: {}", ta.status);
                }
            }
        }
    }
    pub fn get_task_by_id(&self, id: i64) -> Option<CBoxTask> {
        if let Some(c) = &self.conn {
            let mut stmt = c
                .prepare("SELECT id, box_id, nonce, origin_path, target_path, task_type, status FROM cbox_task where id = ?1")
                .unwrap();
            let box_iter = match stmt.query_map([id], |row| {
                let mut b = CBoxTask::default();
                b.id = row.get(0)?;
                b.box_id = row.get(1)?;
                b.nonce = row.get(2)?;
                b.origin_path = row.get(3)?;
                b.target_path = row.get(4)?;
                b.task_type = row.get(5)?;
                b.status = row.get(6)?;
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
                Some(task)
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
    pub fn resume_tasks(&self) {
        if !self.has_connection() {
            return;
        }

        let c = self.conn.as_ref().unwrap();
        c.execute(
            r#"
            update cbox_task set status = 0 where status = 1
        "#,
            [],
        )
        .unwrap_or_default();
    }
    pub fn update_task_progress(
        &self,
        id: i64,
        total: u64,
        total_size: u64,
        finished: u64,
        finished_size: u64,
    ) -> Result<(), Error> {
        if !self.has_connection() {
            return Err(Error::NoDBConnection);
        }
        println!(
            "progress updated: id: {} total: {} total_size: {} finished: {} finished_size: {}",
            id, total, total_size, finished, finished_size
        );
        let c = self.conn.as_ref().unwrap();
        c.execute(
            r#"
            update cbox_task set total = ?1, total_size = ?2, finished = ?3, finished_size = ?4  where id = ?5
        "#,
            params![total, total_size, finished, finished_size, id],
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
            insert into cbox_task (box_id, obj_id, nonce, origin_path, target_path, task_type, create_at, modify_at, status, err, total, total_size, finished, finished_size) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)
        "#,
            params![par.box_id, par.obj_id, par.nonce, par.origin_path, par.target_path, par.task_type, par.create_at, par.modify_at, par.status, par.err, par.total, par.total_size, par.finished, par.finished_size],
        )?;
        Ok(c.last_insert_rowid())
    }
    pub fn add_recover_tasks(
        &self,
        box_id: i64,
        target_dir: String,
        obj_ids: Vec<i64>,
    ) -> Result<(), Error> {
        if !self.has_connection() {
            return Err(Error::NoDBConnection);
        }
        let mut task_list = Vec::<CBoxTask>::with_capacity(obj_ids.len());

        for id in obj_ids.into_iter() {
            //let meta = std::fs::metadata(p)?;
            let mut obj = CBoxTask::default();
            obj.box_id = box_id;
            obj.obj_id = id;
            obj.status = 0;
            obj.task_type = 1;
            obj.nonce = Vec::new();
            obj.target_path = target_dir.clone();
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
            obj.origin_path = p.clone();
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
