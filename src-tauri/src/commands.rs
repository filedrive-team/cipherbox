#![allow(dead_code)]
use crate::{
    cipher::{set_password, verify_password},
    errors::Error,
    mgr::{
        App, AppInfo, CBox, CBoxObj, CBoxTask, CommonRes, ControlEvent, CreateCboxParams,
        CIPHER_MESSAGE_NAME,
    },
};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, State};

/*
 * api - task pause
 */
#[tauri::command]
pub async fn task_pause(id: u64, app: State<'_, Arc<Mutex<App>>>) -> Result<CommonRes<()>, Error> {
    let app = app.lock().unwrap();

    if app.task_trigger.is_none() {
        return Ok(CommonRes::error(Error::Other(
            "task trigger is none".to_string(),
        )));
    }
    let s = app.task_trigger.as_ref().unwrap();

    let event = match id {
        0 => ControlEvent::PauseAll,
        _ => ControlEvent::Pause(id as i64),
    };
    match async_std::task::block_on(s.send(event)) {
        Ok(_) => Ok(CommonRes::ok(())),
        Err(e) => {
            eprintln!("failed to send pause message: {}", e);
            Ok(CommonRes::error(Error::Other(format!("{}", e))))
        }
    }
}

/*
 * api - task resume
 */
#[tauri::command]
pub async fn task_resume(id: u64, app: State<'_, Arc<Mutex<App>>>) -> Result<CommonRes<()>, Error> {
    if id == 0 {
        return Ok(CommonRes::error(Error::Other("id is 0".to_string())));
    }
    let app = app.lock().unwrap();

    if app.task_trigger.is_none() {
        return Ok(CommonRes::error(Error::Other(
            "task trigger is none".to_string(),
        )));
    }
    let s = app.task_trigger.as_ref().unwrap();

    match async_std::task::block_on(s.send(ControlEvent::Resume(id as i64))) {
        Ok(_) => Ok(CommonRes::ok(())),
        Err(e) => {
            eprintln!("failed to send resume message: {}", e);
            Ok(CommonRes::error(Error::Other(format!("{}", e))))
        }
    }
}

/*
 * api - task cancel
 */
#[tauri::command]
pub async fn task_cancel(id: i64, app: State<'_, Arc<Mutex<App>>>) -> Result<CommonRes<()>, Error> {
    if id == 0 {
        return Ok(CommonRes::error(Error::Other("id is 0".to_string())));
    }
    let app = app.lock().unwrap();

    if app.task_trigger.is_none() {
        return Ok(CommonRes::error(Error::Other(
            "task trigger is none".to_string(),
        )));
    }
    let s = app.task_trigger.as_ref().unwrap();

    match async_std::task::block_on(s.send(ControlEvent::Cancel(id as i64))) {
        Ok(_) => Ok(CommonRes::ok(())),
        Err(e) => {
            eprintln!("failed to send resume message: {}", e);
            Ok(CommonRes::error(Error::Other(format!("{}", e))))
        }
    }
}

/*
 * api - task list
 */
#[tauri::command]
pub async fn task_list(
    status: Vec<i32>,
    app: State<'_, Arc<Mutex<App>>>,
) -> Result<CommonRes<Vec<CBoxTask>>, Error> {
    let app = app.lock().unwrap();
    match app.list_task(status) {
        Ok(list) => Ok(CommonRes::ok(list)),
        Err(e) => Ok(CommonRes::error(e)),
    }
}

/*
 * api - box objects list
 */
#[tauri::command]
pub async fn box_obj_list(
    box_id: i64,
    last_id: i32,
    app: State<'_, Arc<Mutex<App>>>,
) -> Result<CommonRes<Vec<CBoxObj>>, Error> {
    let app = app.lock().unwrap();
    match app.list_cbox_obj() {
        Ok(list) => Ok(CommonRes::ok(list)),
        Err(e) => Ok(CommonRes::error(e)),
    }
}

/*
 * api - backup files
 */

#[tauri::command]
pub async fn backup(
    box_id: i64,
    targets: Vec<String>,
    app: State<'_, Arc<Mutex<App>>>,
) -> Result<CommonRes<()>, Error> {
    let app = app.lock().unwrap();
    match app.add_backup_tasks(box_id, targets) {
        Ok(_) => Ok(CommonRes::ok(())),
        Err(e) => Ok(CommonRes::error(e)),
    }
}

/*
 * api - recover files
 */

#[tauri::command]
pub async fn recover(
    box_id: i64,
    target_dir: String,
    obj_ids: Vec<i64>,
    app: State<'_, Arc<Mutex<App>>>,
) -> Result<CommonRes<()>, Error> {
    let app = app.lock().unwrap();
    match app.add_recover_tasks(box_id, target_dir, obj_ids) {
        Ok(_) => Ok(CommonRes::ok(())),
        Err(e) => Ok(CommonRes::error(e)),
    }
}

/*
 * api - create a box for data backup
 */
#[tauri::command]
pub async fn box_create(
    par: CreateCboxParams,
    app: State<'_, Arc<Mutex<App>>>,
) -> Result<CommonRes<CBox>, Error> {
    let app = &mut app.lock().unwrap();
    match app.create_cbox(par) {
        Ok(cb) => Ok(CommonRes::ok(cb)),
        Err(e) => Ok(CommonRes::error(e)),
    }
}

/*
 * api - set active box
 */
#[tauri::command]
pub async fn box_set_active(
    id: i64,
    app: State<'_, Arc<Mutex<App>>>,
) -> Result<CommonRes<CBox>, Error> {
    let app = &mut app.lock().unwrap();
    match app.set_active_box(id) {
        Ok(cb) => Ok(CommonRes::ok(cb)),
        Err(e) => Ok(CommonRes::error(e)),
    }
}

/*
 * api - box list
 */
#[tauri::command]
pub async fn box_list(app: State<'_, Arc<Mutex<App>>>) -> Result<CommonRes<Vec<CBox>>, Error> {
    let app = app.lock().unwrap();
    match app.list_cbox() {
        Ok(list) => Ok(CommonRes::ok(list)),
        Err(e) => Ok(CommonRes::error(e)),
    }
}
/*
 * api - create password for cipherbox
 */
#[tauri::command]
pub async fn password_set(
    password: String,
    app: AppHandle,
    capp: State<'_, Arc<Mutex<App>>>,
) -> Result<CommonRes<()>, Error> {
    let mut path_to_save = match app.path_resolver().app_dir() {
        Some(p) => p,
        None => {
            return Ok(CommonRes::error(Error::TauriApi(
                "failed to get app dir".into(),
            )))
        }
    };
    path_to_save.push(CIPHER_MESSAGE_NAME);
    let key = match set_password(password, path_to_save) {
        Ok(key) => key,
        Err(e) => return Ok(CommonRes::error(e)),
    };
    let capp = &mut capp.lock().unwrap();
    capp.user_key = Some(key);
    Ok(CommonRes::ok(()))
}

/*
 * api - verify password
 *      will unlock cipherbox for user if pass the verification
 */
#[tauri::command]
pub async fn password_verify(
    password: String,
    app: AppHandle,
    capp: State<'_, Arc<Mutex<App>>>,
) -> Result<CommonRes<bool>, Error> {
    let mut path_to_save = match app.path_resolver().app_dir() {
        Some(p) => p,
        None => {
            return Ok(CommonRes::error(Error::TauriApi(
                "failed to get app dir".into(),
            )))
        }
    };
    path_to_save.push(CIPHER_MESSAGE_NAME);
    let key = match verify_password(password, path_to_save) {
        Ok(key) => key,
        Err(e) => return Ok(CommonRes::error(e)),
    };
    let capp = &mut capp.lock().unwrap();
    capp.user_key = Some(key);
    Ok(CommonRes::ok(true))
}

/*
 * api - get app info
 *
 */
#[tauri::command]
pub async fn app_info(capp: State<'_, Arc<Mutex<App>>>) -> Result<CommonRes<AppInfo>, Error> {
    Ok(CommonRes::ok(capp.lock().unwrap().app_info()))
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
