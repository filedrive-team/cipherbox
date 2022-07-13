#![allow(dead_code)]
use std::fs;
use std::path::{PathBuf};
use tauri::{AppHandle};
use crate::cipher::{encrypt_or_decrypt_file, set_password, verify_password};
use std::ffi::{OsString};
use std::error::Error;
static CIPHER_MESSAGE_NAME: &str = "cipher_message";

/*
 * api - backup directory
 */

/*
 * api - backup files
 */

/*
 * api - create a box for data backup
 */

/*
 * api - box objects list
 */

/*
 * api - box list
 */

/*
 * api - create password for cipherbox 
 */
#[tauri::command]
pub async fn password_set(password: String, app: AppHandle) -> Result<(), Box<dyn Error>> {
    let mut path_to_save = app.path_resolver().app_dir().ok_or("failed to get app dir")?;
    path_to_save.push(CIPHER_MESSAGE_NAME);
    set_password(password, path_to_save)?;
    Ok(())
}

/*
 * api - verify password 
 *      will unlock cipherbox for user if pass the verification
 */
#[tauri::command]
pub async fn password_verify(password: String, app: AppHandle) -> Result<bool, String> {
    let mut path_to_save = app.path_resolver().app_dir().ok_or("failed to get app dir")?;
    path_to_save.push(CIPHER_MESSAGE_NAME);
    verify_password(password, path_to_save)
}


#[tauri::command]
pub async fn backup(path: String, app: AppHandle) -> anyhow::Result<(), String> {
  let fp = PathBuf::from(path);
  let mut target_path = app.path_resolver().app_dir().ok_or("failed to get app dir")?;
  target_path.push(String::from("encrypted"));
  let file_name = fp.file_name().unwrap();
  target_path.push(file_name);
  fs::create_dir_all(target_path.parent().ok_or("failed to get parent dir")?)
    .map_err(|_| "failed to create dir")?;
  fs::copy(fp, target_path)
    .map_err(|_| "failed to do backup")?;
  Ok(())
}

#[tauri::command]
pub async fn encrypt_file(path: String, app: AppHandle) -> Result<(), String>  {
  let fp = PathBuf::from(path);
  let mut target_path = app.path_resolver().app_dir().ok_or("failed to get app dir")?;
  target_path.push(String::from("encrypted"));
  let file_name = fp.file_name().unwrap();
  target_path.push(file_name);
  encrypt_or_decrypt_file(fp, target_path)
    .map_err(|err| format!("failed to encrypt file, error: {:?}", err))?;

    Ok(())
}

#[tauri::command]
pub async fn decrypt_file(path: String) -> Result<(), String>  {
  let fp = PathBuf::from(path);
  let parent_dir = fp.parent().ok_or("failed to get parent dir")?;
  let file_name = fp.file_name().ok_or("failed to get file name")?;
  let mut decrypted_file_name = OsString::new();
  decrypted_file_name.push(OsString::from("x01."));
  decrypted_file_name.push(file_name);
  let target_path = PathBuf::new()
    .join(parent_dir)
    .join(decrypted_file_name);
  encrypt_or_decrypt_file(fp, target_path)
    .map_err(|err| format!("failed to decrypt file, error: {:?}", err))?;

    Ok(())
}
