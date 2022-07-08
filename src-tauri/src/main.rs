#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cipher;
mod commands;
use crate::commands::{backup, encrypt_file, decrypt_file};

fn main() {
  let context = tauri::generate_context!();
  tauri::Builder::default()
    .menu(if cfg!(target_os = "macos") {
      tauri::Menu::os_default(&context.package_info().name)
    } else {
      tauri::Menu::default()
    })
    .invoke_handler(tauri::generate_handler![backup, encrypt_file, decrypt_file])
    .run(context)
    .expect("error while running tauri application");
}
