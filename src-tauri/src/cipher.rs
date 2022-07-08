use tauri::api::{path::{home_dir, resolve_path}};
use crypto::{chacha20, symmetriccipher::{SynchronousStreamCipher}};
use std::fs::{read, write, self};
use std::path::{PathBuf};
use std::io;
use std::iter::{repeat};

const TEST_KEY:[u8;32] = [
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
    0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
    0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
];
const TEST_NONCE:[u8;12] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];


pub(crate) fn encrypt_or_decrypt_file(source: PathBuf, target: PathBuf) -> anyhow::Result<(), io::Error> {
    let sd = read(source)?;
    let mut cipher = chacha20::ChaCha20::new(&TEST_KEY[..], &TEST_NONCE[..]);
    let mut encoded: Vec<u8> = repeat(0u8).take(sd.len()).collect();
    cipher.process(&sd, &mut encoded);
    write(target, encoded)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_encrypt_decrypt() {
        let msg: &[u8] = b"Every thing is work";
        print!("{:?}", &msg);
        let mut encoded: Vec<u8> = repeat(0u8).take(msg.len()).collect();
        let mut cipher = chacha20::ChaCha20::new(&TEST_KEY[..], &TEST_NONCE[..]);
        let mut cipher2 = chacha20::ChaCha20::new(&TEST_KEY[..], &TEST_NONCE[..]);
        // encrypt
        cipher.process(msg, &mut encoded);
        print!("{:?}", encoded);
        let mut decrypted: Vec<u8> = repeat(0u8).take(msg.len()).collect();
        // decrypt
        cipher2.process(&encoded, &mut decrypted);
        print!("{:?}", &decrypted);

        assert_eq!(msg, &decrypted[..]);
    }
}