#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cipher;
mod commands;
mod errors;
mod mgr;

use crate::commands::{
    app_info, backup, box_create, box_list, box_obj_list, box_set_active, password_set,
    password_verify, recover, task_cancel, task_list, task_pause, task_resume,
};
use crate::{
    cipher::encrypt_or_decrypt,
    errors::Error,
    mgr::{
        init_task_record, spawn_and_log_error, web3storage_download, web3storage_upload, App,
        ChoreProgress, Chunks, ControlEvent, TaskRecord,
    },
};
use async_std::{
    channel::{bounded, Receiver, Sender},
    prelude::*,
};
use futures::{select, FutureExt};
use fvm_ipld_encoding::to_vec;
use mgr::{current, CBoxObj};
use std::io::prelude::*;
use std::{
    collections::hash_map::{Entry, HashMap},
    fs::create_dir_all,
    sync::{Arc, Mutex},
};
use tauri::{Manager, RunEvent};
use tauri_plugin_fs_extra::FsExtra;

pub static PROGRESS_EMIT: &str = "task_update";

async fn task_control_loop(cipherbox_app: Arc<Mutex<App>>, mut rx: Receiver<ControlEvent>) {
    println!("into task control loop");
    let concurrent_num = 2;
    let mut chan_id: i32 = 1;
    let (relaese_chan_tx, mut release_chan_rx) = bounded(1);
    let mut channels: HashMap<i32, Sender<ControlEvent>> = HashMap::new();

    loop {
        select! {
            event = rx.next().fuse() => match event {
                Some(event) => {
                    match event {
                        ControlEvent::LoopStart => {
                            println!("receive loop start event");
                            let mut applock = cipherbox_app.lock().unwrap();
                            let appref = &mut *applock;
                            println!("running task num: {}", appref.running_task_num);
                            if appref.running_task_num < concurrent_num {
                                appref.running_task_num += 1;
                                drop(applock);

                                let (se, re) = bounded(1);

                                match channels.entry(chan_id) {
                                    Entry::Occupied(..) => {
                                        eprint!("unexpected occupied entry: {}", chan_id);
                                    }
                                    Entry::Vacant(entry) => {
                                        entry.insert(se.clone());
                                    }
                                }
                                spawn_and_log_error(task_loop(
                                    cipherbox_app.clone(),
                                    relaese_chan_tx.clone(),
                                    chan_id,
                                    re,
                                ));

                                chan_id += 1;
                            }
                        }
                        ControlEvent::Resume(task_id) => {
                            println!("resume {}", task_id);
                            let applock = cipherbox_app.lock().unwrap();
                            applock.resume_task(task_id);
                        }
                        ControlEvent::Pause(task_id) => {
                            println!("pause {}", task_id);
                            for (_, v) in channels.iter() {
                                v.send(ControlEvent::Pause(task_id)).await.unwrap_or_else(|e| eprint!("{}", e));
                            }
                        }
                        ControlEvent::PauseAll => {
                            println!("pause all tasks");
                            for (_, v) in channels.iter() {
                                v.send(ControlEvent::PauseAll).await.unwrap_or_else(|e| eprint!("{}", e));
                            }
                        }
                        ControlEvent::Cancel(task_id) => {
                            println!("cancel {}", task_id);
                            for (_, v) in channels.iter() {
                                v.send(ControlEvent::Cancel(task_id)).await.unwrap_or_else(|e| eprint!("{}", e));
                            }
                        }
                    }
                }
                None => break
            },
            chan_id = release_chan_rx.next().fuse() => match chan_id {
                Some(chanid) => {
                    if channels.remove(&chanid).is_none() {
                        eprint!("relaese chan_id not exist: {}", chanid);
                    };
                }
                None => break
            }
        }
    }
}

async fn task_loop(
    cipherbox_app: Arc<Mutex<App>>,
    tt: Sender<i32>,
    chan_id: i32,
    mut chan: Receiver<ControlEvent>,
) -> std::result::Result<(), Error> {
    println!("one task loop ----------");
    'Outer: loop {
        let mut task_err: Option<(i64, Error)> = None;
        let task = {
            let applock = cipherbox_app.lock().unwrap();
            let appref = &*applock;
            appref.get_pending_task()
        };
        match task {
            Some(mut task) => match init_task_record(&mut task, cipherbox_app.clone()) {
                Ok(mut task_record) => {
                    let cbox = {
                        let applock = cipherbox_app.lock().unwrap();
                        let appref = &*applock;
                        match appref.get_cbox_by_id(task.box_id) {
                            Ok(cbox) => cbox,
                            Err(err) => {
                                task_err = Some((
                                    task_record.task_id,
                                    Error::Other(format!(
                                        "failed to get cbox when doing task {}",
                                        err
                                    )),
                                ));
                                break 'Outer;
                            }
                        }
                    };
                    if task_record.backup {
                        for upload_chore in task_record.upload_list.iter_mut() {
                            // try to open file
                            let mut fd = match async_std::fs::File::open(&upload_chore.path).await {
                                Ok(fd) => fd,
                                Err(err) => {
                                    eprint!(
                                        "upload chore, failed to open file: {} {}",
                                        &upload_chore.path, err
                                    );
                                    break;
                                }
                            };
                            let mut buffer = vec![0u8; mgr::CHUNK_SIZE];
                            loop {
                                // try to receive control event
                                select! {
                                    ev = chan.next().fuse() => match ev {
                                        Some(ev) => match ev {
                                                ControlEvent::Pause(id) => {
                                                    if id == task.id {
                                                        let applock = cipherbox_app.lock().unwrap();
                                                        applock.update_task_status(id, 6).unwrap_or_default();
                                                        break 'Outer;
                                                    }
                                                }
                                                ControlEvent::PauseAll => {
                                                    let applock = cipherbox_app.lock().unwrap();
                                                    applock.update_task_status(task.id, 6).unwrap_or_default();
                                                    break 'Outer;
                                                }
                                                ControlEvent::Cancel(id) => {
                                                    if id == task.id {
                                                        let applock = cipherbox_app.lock().unwrap();
                                                        applock.update_task_status(id, 7).unwrap_or_default();
                                                        break 'Outer;
                                                    }
                                                }
                                                _ => {}
                                        },
                                        None => {
                                            break 'Outer;
                                        }
                                    },
                                    n = read_full(&mut fd, &mut buffer).fuse() => match n {
                                        Ok(0) => {
                                            break;
                                        },
                                        Ok(n) => {
                                            let encrypted_data = {
                                                let mut d = vec![0u8;n];
                                                if !cbox.encrypt_data { // not encrypted
                                                    d.copy_from_slice(&buffer[..n]);
                                                    d
                                                } else {
                                                    let applock = cipherbox_app.lock().unwrap();
                                                    let appref = &*applock;
                                                    let key = appref.user_key.as_ref();

                                                    if key.is_none() {
                                                        eprint!("unexpected user key is none");
                                                        task_err = Some((task_record.task_id, Error::Other("unexpected user key is none".into())));
                                                        break;
                                                    }

                                                    encrypt_or_decrypt(&buffer[..n], &mut d, key.unwrap(), &task.nonce);
                                                    d
                                                }
                                            };

                                            match web3storage_upload(encrypted_data, &cbox).await {
                                                Ok(cid) => {
                                                    upload_chore.chunk_uploaded += 1;
                                                    upload_chore.uploaded_size += n as u64;
                                                    upload_chore.chunks.push(cid);
                                                },
                                                Err(err) =>  {
                                                    task_err = Some((task_record.task_id, err));
                                                    break 'Outer;
                                                }
                                            };
                                            task_record.finished_size += n as u64;
                                            {
                                                let applock = cipherbox_app.lock().unwrap();
                                                if let Some(h) = applock.tauri_handle.as_ref() {
                                                    h.emit_all(PROGRESS_EMIT, ChoreProgress{
                                                        box_id: task.box_id,
                                                        task_id: task_record.task_id,
                                                        total: task_record.total,
                                                        total_size: task_record.total_size,
                                                        finished: task_record.finished,
                                                        finished_size: task_record.finished_size,
                                                        backup: task_record.backup,
                                                        recover: task_record.recover,
                                                        err: task_record.err.clone(),
                                                    }).unwrap_or(());
                                                };
                                                applock.update_task_progress(task_record.task_id, task_record.total, task_record.total_size, task_record.finished, task_record.finished_size).unwrap_or_else(|e| eprint!("{}", e));

                                            }
                                        },
                                        Err(err) => {
                                            eprint!("{}", err);
                                        }
                                    }
                                }
                            }
                            let mut chunks_ref = Chunks::default();
                            chunks_ref.chunk_size = mgr::CHUNK_SIZE as u64;
                            chunks_ref.chunk_count = upload_chore.chunk_uploaded;
                            for cid in upload_chore.chunks.iter() {
                                chunks_ref.chunks.push(cid.clone());
                            }
                            let crdata = match to_vec(&chunks_ref) {
                                Ok(d) => d,
                                Err(err) => {
                                    task_err = Some((task_record.task_id, Error::from(err)));
                                    break 'Outer;
                                }
                            };
                            match web3storage_upload(crdata, &cbox).await {
                                Ok(cid) => {
                                    upload_chore.chunks_ref = cid.to_string();
                                }
                                Err(err) => {
                                    task_err = Some((task_record.task_id, err));
                                    break 'Outer;
                                }
                            };
                            task_record.finished += 1;
                            {
                                let applock = cipherbox_app.lock().unwrap();
                                if let Some(h) = applock.tauri_handle.as_ref() {
                                    h.emit_all(
                                        PROGRESS_EMIT,
                                        ChoreProgress {
                                            box_id: task.box_id,
                                            task_id: task_record.task_id,
                                            total: task_record.total,
                                            total_size: task_record.total_size,
                                            finished: task_record.finished,
                                            finished_size: task_record.finished_size,
                                            backup: task_record.backup,
                                            recover: task_record.recover,
                                            err: task_record.err.clone(),
                                        },
                                    )
                                    .unwrap_or(());
                                };
                                applock
                                    .update_task_progress(
                                        task_record.task_id,
                                        task_record.total,
                                        task_record.total_size,
                                        task_record.finished,
                                        task_record.finished_size,
                                    )
                                    .unwrap_or_else(|e| eprint!("{}", e));
                            }
                        }
                    }
                    if task_record.recover {
                        for download_chore in task_record.download_list.iter_mut() {
                            let p =
                                std::path::Path::new(&task.target_path).join(&download_chore.path);
                            println!("recover path");
                            let mut fd = match std::fs::OpenOptions::new()
                                .write(true)
                                .create(true)
                                .open(p)
                            {
                                Ok(fd) => fd,
                                Err(err) => {
                                    eprint!(
                                        "download chore, failed to open file: {} {}",
                                        &download_chore.path, err
                                    );
                                    break;
                                }
                            };
                            for cid in download_chore.chunks.iter() {
                                // try to receive control event
                                select! {
                                    ev = chan.next().fuse() => match ev {
                                        Some(ev) => match ev {
                                                ControlEvent::Pause(id) => {
                                                    if id == task.id {
                                                        let applock = cipherbox_app.lock().unwrap();
                                                        applock.update_task_status(id, 6).unwrap_or_default();
                                                        break 'Outer;
                                                    }
                                                }
                                                ControlEvent::PauseAll => {
                                                    let applock = cipherbox_app.lock().unwrap();
                                                    applock.update_task_status(task.id, 6).unwrap_or_default();
                                                    break 'Outer;
                                                }
                                                ControlEvent::Cancel(id) => {
                                                    if id == task.id {
                                                        let applock = cipherbox_app.lock().unwrap();
                                                        applock.update_task_status(id, 7).unwrap_or_default();
                                                        break 'Outer;
                                                    }
                                                }
                                                _ => {}
                                        },
                                        None => {
                                            break 'Outer;
                                        }
                                    },
                                    n = web3storage_download(cid.to_string()).fuse() => match n {
                                        Ok(d) => {
                                            let dn = d.len();
                                            let decrypted_data = {
                                                if !cbox.encrypt_data { // no need to decrypt data
                                                    d
                                                } else {
                                                    let applock = cipherbox_app.lock().unwrap();
                                                    let appref = &*applock;
                                                    let key = appref.user_key.as_ref();

                                                    if key.is_none() {
                                                        eprint!("unexpected user key is none");
                                                        task_err = Some((task_record.task_id, Error::Other("unexpected user key is none".into())));
                                                        break;
                                                    }
                                                    let mut od = vec![0u8;d.len()];
                                                    encrypt_or_decrypt(&d, &mut od, key.unwrap(), &task.nonce);
                                                    od
                                                }
                                            };
                                            match fd.write(&decrypted_data) {
                                                Ok(n) => {
                                                    format!("len: {} write: {}", dn, n)
                                                }
                                                Err(err) => {
                                                    eprint!("write err: {}", err);
                                                    task_err = Some((task_record.task_id, Error::Other("write err".into())));
                                                    break;
                                                }
                                            };

                                            task_record.finished_size += dn as u64;
                                            {
                                                let applock = cipherbox_app.lock().unwrap();
                                                if let Some(h) = applock.tauri_handle.as_ref() {
                                                    h.emit_all(PROGRESS_EMIT, ChoreProgress{
                                                        box_id: task.box_id,
                                                        task_id: task_record.task_id,
                                                        total: task_record.total,
                                                        total_size: task_record.total_size,
                                                        finished: task_record.finished,
                                                        finished_size: task_record.finished_size,
                                                        backup: task_record.backup,
                                                        recover: task_record.recover,
                                                        err: task_record.err.clone(),
                                                    }).unwrap_or(());
                                                };
                                                applock.update_task_progress(task_record.task_id, task_record.total, task_record.total_size, task_record.finished, task_record.finished_size).unwrap_or_else(|e| eprint!("{}", e));

                                            }
                                        },
                                        Err(err) => {
                                            eprint!("{}", err);
                                        }
                                    }
                                }
                            }
                            task_record.finished += 1;
                            {
                                let applock = cipherbox_app.lock().unwrap();
                                if let Some(h) = applock.tauri_handle.as_ref() {
                                    h.emit_all(
                                        PROGRESS_EMIT,
                                        ChoreProgress {
                                            box_id: task.box_id,
                                            task_id: task_record.task_id,
                                            total: task_record.total,
                                            total_size: task_record.total_size,
                                            finished: task_record.finished,
                                            finished_size: task_record.finished_size,
                                            backup: task_record.backup,
                                            recover: task_record.recover,
                                            err: task_record.err.clone(),
                                        },
                                    )
                                    .unwrap_or(());
                                };
                                applock
                                    .update_task_progress(
                                        task_record.task_id,
                                        task_record.total,
                                        task_record.total_size,
                                        task_record.finished,
                                        task_record.finished_size,
                                    )
                                    .unwrap_or_else(|e| eprint!("{}", e));
                            }
                        }
                    }
                    let applock = cipherbox_app.lock().unwrap();
                    //let appref = &*applock;
                    // save a finished task to db
                    if task.task_type == 0 {
                        // save record for backup task
                        let tpath = std::path::PathBuf::from(&task.origin_path);

                        for chore in task_record.upload_list.iter() {
                            let chore_path = std::path::PathBuf::from(&chore.path);
                            // insert cbox_obj
                            let mut cbo = CBoxObj::default();
                            cbo.box_id = task.box_id;
                            cbo.cid = chore.chunks_ref.clone();
                            cbo.create_at = match current() {
                                Ok(t) => t,
                                Err(err) => {
                                    eprintln!("{}", err);
                                    0
                                }
                            };
                            cbo.modify_at = cbo.create_at;
                            cbo.nonce = task.nonce.clone();
                            cbo.obj_type = 0;
                            cbo.origin_path = chore.path.clone();
                            let filename = match chore_path.file_name() {
                                Some(name) => name.to_str().unwrap_or("").to_string(),
                                None => String::new(),
                            };
                            cbo.name = filename.clone();
                            cbo.path = match chore_path.strip_prefix(&tpath) {
                                Ok(p) => p.to_str().unwrap_or("").into(),
                                Err(_) => String::new(),
                            };
                            if cbo.path == "" {
                                cbo.path = cbo.name.clone();
                            }
                            cbo.cid = chore.chunks_ref.clone();
                            cbo.size = chore.size;
                            cbo.parent_id = match applock.get_parent_id(cbo.box_id, &cbo.path) {
                                Ok(id) => id,
                                Err(_) => 0,
                            };
                            applock.create_cbox_obj(&cbo).unwrap();
                        }
                        println!(
                            "task #{} finished: {}",
                            task_record.task_id, task_record.finished
                        );
                    }
                    applock
                        .update_task_status(task.id, 5)
                        .unwrap_or_else(|e| eprint!("{}", e))
                }
                Err(err) => {
                    task_err = Some((task.id, err));
                }
            },
            None => break 'Outer,
        }
        // handle task error
        match task_err {
            Some(err) => {
                let applock = cipherbox_app.lock().unwrap();
                if let Err(err) = applock.record_task_err(err.0, err.1) {
                    eprint!("record task err failed: {}", err);
                }
            }
            None => {}
        }
    }
    // reduce running task
    {
        let mut applock = cipherbox_app.lock().unwrap();
        let appref = &mut *applock;
        appref.running_task_num -= 1;
    }
    match tt.send(chan_id).await {
        Ok(_) => Ok(()),
        Err(err) => {
            eprint!("{}", err);
            Ok(())
        }
    }
}

async fn read_full(
    f: &mut async_std::fs::File,
    mut bs: &mut [u8],
) -> Result<usize, std::io::Error> {
    let mut readed = 0usize;
    while !bs.is_empty() {
        match f.read(bs).await {
            Ok(0) => break,
            Ok(n) => {
                let tmp = bs;
                bs = &mut tmp[n..];
                readed += n;
            }
            Err(e) => return Err(e),
        }
    }

    Ok(readed)
}

#[async_std::main]
async fn main() -> () {
    // init cipherbox app instance
    let mut cipherbox_app = App::default();
    let (tx, rx) = bounded(10);
    cipherbox_app.task_trigger = Some(tx);
    // wrap app into Arc/Mutex for multipule thread sharing
    let cipherbox_app = Arc::new(Mutex::new(cipherbox_app));
    // clone an app instance for tauri setup callback
    let cipherbox_app_clone = cipherbox_app.clone();
    // spawn a thread
    // loop for trigger or pause async task
    let hd = async_std::task::spawn(task_control_loop(cipherbox_app, rx));

    let context = tauri::generate_context!();
    let tauri_app = tauri::Builder::default()
        .plugin(FsExtra::default())
        .setup(move |app| {
            let app_dir = app.path_resolver().app_dir().unwrap();

            if !&app_dir.exists() {
                _ = create_dir_all(&app_dir).unwrap();
            }

            let app_dir = app_dir.as_os_str().to_owned();
            {
                let cipherboxapp = &mut *cipherbox_app_clone.lock().unwrap();
                cipherboxapp.setup(app_dir);
                cipherboxapp.init_db().expect("failed to open sqlite");
                cipherboxapp.resume_tasks();
                if let Err(e) = cipherboxapp.read_cache() {
                    eprint!("{}", e);
                }
                cipherboxapp.tauri_handle = Some(app.handle());
            }

            app.manage(cipherbox_app_clone);

            Ok(())
        })
        .menu(if cfg!(target_os = "macos") {
            tauri::Menu::os_default(&context.package_info().name)
        } else {
            tauri::Menu::default()
        })
        .invoke_handler(tauri::generate_handler![
            app_info,
            password_set,
            password_verify,
            box_create,
            box_list,
            box_set_active,
            backup,
            box_obj_list,
            task_list,
            task_cancel,
            task_resume,
            task_pause,
            recover,
        ])
        .build(context)
        .expect("error while running tauri application");

    tauri_app.run(|_app_handle, e| {
        match e {
            // Keep the event loop running even if all windows are closed
            // This allow us to catch system tray events when there is no window
            RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        }
    });
    hd.await;
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_user_key() -> [u8; 32] {
        let mut uk = [3u8; 32];
        // let rng = cipher::gen_nonce(32);
        // for (i, d) in uk.iter_mut().enumerate() {
        //     *d = rng[i]
        // }
        uk
    }

    #[async_std::test]
    async fn test_main() {
        //let temp_dir = std::env::temp_dir();
        let temp_dir = std::path::PathBuf::from("/Users/lifeng/cipherbox-test");
        // init a App
        let mut app = App::default();
        let (tx, rx) = bounded(10);
        app.task_trigger = Some(tx);
        app.setup(temp_dir.as_os_str().to_owned());
        // init db
        app.init_db().expect("failed to init sqlite");
        app.set_user_key(test_user_key());

        // // create a Cbox
        // let cbpa01: mgr::CreateCboxParams = serde_json::from_str(
        //     r#"
        //     {
        //         "name": "cbox_x_00001",
        //         "encryptData": true,
        //         "provider": 1,
        //         "accessToken": "000"
        //     }
        // "#,
        // )
        // .expect("failed tp do json deserialize");
        // let new_box01 = app.create_cbox(cbpa01).expect("failed to create cbox");
        // wrap app into Arc/Mutex for multipule thread sharing
        let cipherbox_app = Arc::new(Mutex::new(app));

        // spawn a thread
        // loop for trigger or pause async task
        let hd = async_std::task::spawn(task_control_loop(cipherbox_app.clone(), rx));
        async_std::task::spawn(async move {
            let applock = cipherbox_app.lock().unwrap();
            // applock
            //     .add_backup_tasks(
            //         new_box01.id,
            //         vec![String::from("/Users/lifeng/nc62/t223.txt")],
            //     )
            //     .unwrap();
            applock
                .add_recover_tasks(2, "/Users/lifeng/dmaker/store".to_owned(), vec![2])
                .unwrap();

            // std::thread::sleep_ms(2000);
            // async_std::task::block_on(
            //     applock
            //         .task_trigger
            //         .as_ref()
            //         .unwrap()
            //         .send(ControlEvent::Pause(1)),
            // )
            // .unwrap();
        });

        hd.await;
    }
}
