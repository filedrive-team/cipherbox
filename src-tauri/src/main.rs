#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cipher;
mod commands;
mod errors;
mod mgr;

use tauri::Manager;

use crate::cipher::DerivedKey;
use crate::commands::{
    app_info, backup, box_create, box_list, box_obj_list, box_set_active, password_set,
    password_verify,
};
use crate::mgr::App;

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .setup(|app| {
            let app_dir = app
                .path_resolver()
                .app_dir()
                .ok_or("failed to get app dir during setup")?;
            let app_dir = app_dir.as_os_str().to_owned();
            let mut cipherboxapp = App::new(app_dir);
            cipherboxapp.init_db().expect("failed to open sqlite");
            if let Err(e) = cipherboxapp.read_cache() {
                dbg!(e);
            }
            app.manage(cipherboxapp);
            Ok(())
        })
        .menu(if cfg!(target_os = "macos") {
            tauri::Menu::os_default(&context.package_info().name)
        } else {
            tauri::Menu::default()
        })
        .manage(DerivedKey::default())
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
        .run(context)
        .expect("error while running tauri application");
}
