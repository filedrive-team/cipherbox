use super::*;
use async_std::channel::Sender;
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
    pub async fn trigger_task(&self) {
        if self.processing {
            // already in backup task processing
            // just ignore request
            return;
        }
        // get pending task record from table CBoxObj
    }
    pub fn create_cbox_task(&self, par: &CBoxTask) -> Result<i64, Error> {
        if !self.has_connection() {
            return Err(Error::NoDBConnection);
        }
        let mut insert_id = 0i64;
        let c = self.conn.as_ref().unwrap();
        c.execute(
            r#"
            insert into cbox_task (box_id, obj_id, nonce, origin_path, target_path, task_type, create_at, modify_at, status) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
        "#,
            params![par.box_id, par.obj_id, par.nonce, par.origin_path, par.target_path, par.task_type, par.create_at, par.modify_at, par.status],
        )?;
        insert_id = c.last_insert_rowid();

        match insert_id {
            0 => Err(Error::Other("sqlite error: failed to get insert id".into())),
            _ => Ok(insert_id),
        }
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
        // async_std::task::spawn(async move {
        //     self.trigger_task();
        // });

        Ok(())
    }
}
