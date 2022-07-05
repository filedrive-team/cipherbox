#![allow(dead_code)]
use std::fs::{read, write, self};
use std::path::{PathBuf};
use crypto::symmetriccipher::SynchronousStreamCipher;
use tauri::{AppHandle, api};
use api::{path::{home_dir}};
use crypto::{chacha20};

const TEST_KEY:[u8;32] = [
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
    0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
    0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
];
const TEST_NONCE:[u8;12] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];


#[tauri::command]
pub async fn backup(path: String) -> anyhow::Result<(), &'static str> {
  let fp = PathBuf::from(path);
  let target = home_dir()
  .ok_or("failed to get home dir")?
  .join("Downloads/1233")
  .join(fp.file_name().ok_or("")?);
  fs::create_dir_all(target.parent().ok_or("failed to get parent dir")?).expect("failed to create dir");
  fs::copy(fp, target).expect("failed to do backup");
  Ok(())
}

// #[tauri::command]
// pub async fn encrypt_file(path: String, app: tauri::AppHandle)  {
//   let fp = PathBuf::from(path);
//   let target = home_dir().unwrap().join("Downloads/encrypted").join(fp.file_name().unwrap());
//   fs::create_dir_all(target.parent().unwrap());
//   fs::copy(fp, target);
  
// }

#[cfg(test)]
mod test {
    use crypto::symmetriccipher::{Decryptor, Encryptor};
    use crypto::buffer::{RefReadBuffer, RefWriteBuffer};
    use super::*;
    #[test]
    fn test_encrypt_decrypt() {
        let msg: &[u8] = b"Every thing is work";
        print!("{:?}", &msg);
        let msgLen = msg.len();
        let mut encoded = b"Every thing is work".to_owned();
        let mut cipher = chacha20::ChaCha20::new(&TEST_KEY[..], &TEST_NONCE[..]);
        cipher.encrypt(&mut RefReadBuffer::new(msg), &mut RefWriteBuffer::new(&mut encoded), true);
        //cipher.process(msg, &mut encoded[..]);
        print!("{:?}", encoded);
        let mut decrypted = b" is workEvery thing".to_owned();
        cipher.decrypt(&mut RefReadBuffer::new(&encoded), &mut RefWriteBuffer::new(&mut decrypted), true);
        print!("{:?}", &decrypted);

        assert_eq!(msg, &decrypted[..]);
    }
}