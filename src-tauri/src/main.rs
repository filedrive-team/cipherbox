#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cipher;
mod commands;
mod db;

use tauri::Manager;

use crate::commands::{backup, encrypt_file, decrypt_file};
use crate::db::{db_init};

fn main() {
  let context = tauri::generate_context!();
  tauri::Builder::default()
    .setup(|app| {
      let mut app_dir = app.path_resolver().app_dir().ok_or("failed to get app dir during setup")?;
      match db_init(&mut app_dir) {
        Ok(db) => {
          app.manage(db);
          Ok(())
        },
        Err(err) => return Err(err)
      }
    })
    .menu(if cfg!(target_os = "macos") {
      tauri::Menu::os_default(&context.package_info().name)
    } else {
      tauri::Menu::default()
    })
    .invoke_handler(tauri::generate_handler![backup, encrypt_file, decrypt_file])
    .run(context)
    .expect("error while running tauri application");
}
