#![allow(dead_code)]
use tauri::{AppHandle, State};
use crate::{
    cipher::{set_password, verify_password},
    models::{CBox},
    mgr::{
      App,
      CIPHER_MESSAGE_NAME,
      AppInfo,
      CreateCboxParams,
      CommonRes,
    },
    errors::Error,
};


pub struct BoxParams {
    name: String, 
    encrypt_data: bool,
}
/*
 * api - backup directory
 */

/*
 * api - backup files
 */

/*
 * api - create a box for data backup
 */
#[tauri::command]
pub async fn box_create(par: CreateCboxParams, app: State<'_, App>) -> Result<CommonRes<CBox>, Error> {
    match app.create_cbox(par) {
      Ok(cb) => {
        Ok(CommonRes::ok(cb))
      },
      Err(e) => {
        Ok(CommonRes::error(e))
      }
    }
}
/*
 * api - box objects list
 */

/*
 * api - box list
 */
#[tauri::command]
pub async fn box_list(app: State<'_, App>) -> Result<Vec<CBox>, Error> {
    app.list_cbox()
}
/*
 * api - create password for cipherbox 
 */
#[tauri::command]
pub async fn password_set(password: String, app: AppHandle, capp: State<'_, App>) -> Result<CommonRes<()>, Error> {
    let mut path_to_save = match app.path_resolver().app_dir() {
        Some(p) => p,
        None => {
          return Ok(CommonRes::error(Error::TauriApi("failed to get app dir".into())))
        }
    };
    path_to_save.push(CIPHER_MESSAGE_NAME);
    let key = match set_password(password, path_to_save){
      Ok(key) => key,
      Err(e) => return Ok(CommonRes::error(e))
    };
    
    *capp.user_key.lock().unwrap() = Some(key);
    Ok(CommonRes::ok(()))
}

/*
 * api - verify password 
 *      will unlock cipherbox for user if pass the verification
 */
#[tauri::command]
pub async fn password_verify(password: String, app: AppHandle, capp: State<'_, App>) -> Result<CommonRes<bool>, Error> {
    let mut path_to_save = match app.path_resolver().app_dir() {
        Some(p) => p,
        None => {
          return Ok(CommonRes::error(Error::TauriApi("failed to get app dir".into())))
        }
    };
    path_to_save.push(CIPHER_MESSAGE_NAME);
    let key = match verify_password(password, path_to_save) {
        Ok(key) => key,
        Err(e) => {
          return Ok(CommonRes::error(e))
        }
    };
    *capp.user_key.lock().unwrap() = Some(key);
    Ok(CommonRes::ok(true))
}

/*
 * api - get app info
 *      
 */
#[tauri::command]
pub async fn app_info(capp: State<'_, App>) -> Result<CommonRes<AppInfo>, Error> {
    Ok(CommonRes::ok(capp.app_info()))
}



// #[tauri::command]
// pub async fn backup(path: String, app: AppHandle) -> anyhow::Result<(), String> {
//   let fp = PathBuf::from(path);
//   let mut target_path = app.path_resolver().app_dir().ok_or("failed to get app dir")?;
//   target_path.push(String::from("encrypted"));
//   let file_name = fp.file_name().unwrap();
//   target_path.push(file_name);
//   fs::create_dir_all(target_path.parent().ok_or("failed to get parent dir")?)
//     .map_err(|_| "failed to create dir")?;
//   fs::copy(fp, target_path)
//     .map_err(|_| "failed to do backup")?;
//   Ok(())
// }

// #[tauri::command]
// pub async fn encrypt_file(path: String, app: AppHandle) -> Result<(), String>  {
//   let fp = PathBuf::from(path);
//   let mut target_path = app.path_resolver().app_dir().ok_or("failed to get app dir")?;
//   target_path.push(String::from("encrypted"));
//   let file_name = fp.file_name().unwrap();
//   target_path.push(file_name);
//   encrypt_or_decrypt_file(fp, target_path)
//     .map_err(|err| format!("failed to encrypt file, error: {:?}", err))?;

//     Ok(())
// }

// #[tauri::command]
// pub async fn decrypt_file(path: String) -> Result<(), String>  {
//   let fp = PathBuf::from(path);
//   let parent_dir = fp.parent().ok_or("failed to get parent dir")?;
//   let file_name = fp.file_name().ok_or("failed to get file name")?;
//   let mut decrypted_file_name = OsString::new();
//   decrypted_file_name.push(OsString::from("x01."));
//   decrypted_file_name.push(file_name);
//   let target_path = PathBuf::new()
//     .join(parent_dir)
//     .join(decrypted_file_name);
//   encrypt_or_decrypt_file(fp, target_path)
//     .map_err(|err| format!("failed to decrypt file, error: {:?}", err))?;

//     Ok(())
// }
