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
    password_verify,
};
use crate::{
    cipher::encrypt_or_decrypt,
    errors::Error,
    mgr::{
        init_task_record, spawn_and_log_error, web3storage_upload, App, ControlEvent, TaskEvent,
    },
};
use async_std::{
    channel::{bounded, unbounded, Receiver, Sender},
    prelude::*,
};
use futures::{select, FutureExt};
use std::io::Read;
use std::{
    collections::hash_map::{Entry, HashMap},
    fs::create_dir_all,
    sync::{Arc, Mutex},
};
use tauri::{Manager, RunEvent};

async fn task_control_loop(cipherbox_app: Arc<Mutex<App>>, mut rx: Receiver<ControlEvent>) {
    let concurrent_num = 1;
    let mut chan_id: i32 = 1;
    let (relaese_chan_tx, mut release_chan_rx) = bounded(1);
    let mut channels: HashMap<i32, Sender<ControlEvent>> = HashMap::new();

    loop {
        select! {
            event = rx.next().fuse() => match event {
                Some(event) => {
                    match event {
                        ControlEvent::LoopStart => {
                            let mut applock = cipherbox_app.lock().unwrap();
                            let appref = &mut *applock;

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
                        }
                        ControlEvent::Pause(task_id) => {
                            println!("pause {}", task_id);
                        }
                        ControlEvent::PauseAll => {
                            println!("pause all tasks");
                        }
                        ControlEvent::Cancel(task_id) => {
                            println!("cancel {}", task_id);
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
    'Outer: loop {
        let mut task_err: Option<(i64, Error)> = None;
        let task = {
            let applock = cipherbox_app.lock().unwrap();
            let appref = &*applock;
            appref.get_pending_task()
        };
        match task {
            Some(task) => match init_task_record(&task) {
                Ok(mut task_record) => {
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
                        // try to receive control event
                        select! {
                            ev = chan.next().fuse() => match ev {
                                Some(ev) => match ev {
                                        ControlEvent::Pause(id) => {
                                            if id == task.id {
                                                break 'Outer;
                                            }
                                        }
                                        ControlEvent::PauseAll => {
                                            break 'Outer;
                                        }
                                        ControlEvent::Cancel(id) => {
                                            if id == task.id {
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
                                        let applock = cipherbox_app.lock().unwrap();
                                        let appref = &*applock;
                                        let key = appref.user_key.as_ref();

                                        if key.is_none() {
                                            eprint!("unexpected user key is none");
                                            task_err = Some((task_record.task_id, Error::Other("unexpected user key is none".into())));
                                            break;
                                        }
                                        let mut d = vec![0u8;n];
                                        encrypt_or_decrypt(&buffer[..n], &mut d, key.unwrap(), &task.nonce);
                                        d
                                    };

                                    match web3storage_upload(encrypted_data).await {
                                        Ok(cid) => {
                                            upload_chore.chunk_uploaded += 1;
                                            upload_chore.chunks.push(cid);
                                        },
                                        Err(err) =>  {
                                            task_err = Some((task_record.task_id, err));
                                            break;
                                        }
                                    };

                                },
                                Err(err) => {
                                    eprint!("{}", err);
                                }
                            }
                        }
                    }
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
                let appref = &*applock;
                if let Err(err) = appref.record_task_err(err.0, err.1) {
                    eprint!("record task err failed: {}", err);
                }
            }
            None => {}
        }
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
