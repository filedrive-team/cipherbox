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
use crate::mgr::App;
use async_std::{channel::bounded, prelude::*};
use std::{
    fs::create_dir_all,
    sync::{Arc, Mutex},
};
use tauri::{Manager, RunEvent};

#[async_std::main]
async fn main() -> () {
    // init cipherbox app instance
    let mut cipherbox_app = App::default();
    let (tx, mut rx) = bounded(2);
    cipherbox_app.task_trigger = Some(tx);
    // wrap app into Arc/Mutex for multipule thread sharing
    let cipherbox_app = Arc::new(Mutex::new(cipherbox_app));
    // clone an app instance for tauri setup callback
    let cipherbox_app_clone = cipherbox_app.clone();
    // spawn a thread
    // loop for trigger or pause async task
    let hd = async_std::task::spawn(async move {
        while let Some(_x) = rx.next().await {
            let mut applock = cipherbox_app.lock().unwrap();
            let appref = &mut *applock;

            match appref.processing {
                false => {
                    appref.processing = true;
                    drop(applock);
                    let cloned_app = cipherbox_app.clone();
                    async_std::task::spawn(async move {
                        for i in 0..2 {
                            let mut applock = cloned_app.lock().unwrap();
                            let appref = &mut *applock;

                            dbg!(i, appref.processing);
                        }
                    });
                }
                _ => {}
            }
        }
    });
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
