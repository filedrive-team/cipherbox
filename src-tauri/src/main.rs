#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cipher;
mod commands;
mod db;
mod models;
mod mgr;

use tauri::Manager;

use crate::commands::{
  backup, encrypt_file, decrypt_file,
  box_create,
};
use crate::mgr::{App};
use crate::cipher::{DerivedKey};

fn main() {
  let context = tauri::generate_context!();
  tauri::Builder::default()
    .setup(|app| {
      let app_dir = app.path_resolver().app_dir().ok_or("failed to get app dir during setup")?;
      let app_dir = app_dir.as_os_str().to_owned();
      let mut cipherboxapp = App::new(app_dir);
      cipherboxapp.connect_db().expect("failed to open sqlite");
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
        backup, encrypt_file, decrypt_file, box_create
    ])
    .run(context)
    .expect("error while running tauri application");
}


#[cfg(test)]
mod test {
  #[test]
  fn test_a () {

  }
}