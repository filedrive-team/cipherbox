use crate::errors::Error;
use crypto::{
    chacha20, hmac::Hmac, pbkdf2::pbkdf2, sha2::Sha256, symmetriccipher::SynchronousStreamCipher,
};
use rand::{thread_rng, Rng};
use std::fs::{read, write};
use std::iter::repeat;
use std::path::PathBuf;
use std::sync::Mutex;
// use rustc_serialize::base64;
// use rustc_serialize::base64::{FromBase64, ToBase64};

static MSG: &str = "cipherbox::password::authentication";

const PBKDF2IC: u32 = 1024;

#[derive(Default)]
pub struct DerivedKey(pub Mutex<Option<[u8; 32]>>);

pub(crate) fn set_password(password: String, path_to_save: PathBuf) -> Result<[u8; 32], Error> {
    // generate a nonce Vec<u8>
    let mut nonce = gen_nonce(16);
    // using pbkdf2 derive a key from password
    let derived = pbkdf2_with_nonce(&password, &nonce, PBKDF2IC);
    // using derived key to encryted a message
    let msg = MSG.as_bytes();
    let mut output: Vec<u8> = repeat(0u8).take(msg.len()).collect();
    encrypt_or_decrypt(msg, &mut output, &derived, &nonce[..12]);

    nonce.append(&mut output);
    // save encryted nonce and message to disk
    write(path_to_save, nonce)?;
    Ok(derived)
}

pub(crate) fn verify_password(password: String, path_to_save: PathBuf) -> Result<[u8; 32], Error> {
    let encrypted_msg = read(path_to_save)?;
    let nonce = &encrypted_msg[..16];
    let nonce_20 = &encrypted_msg[..12];
    let enmsg = &encrypted_msg[16..];

    let derived = pbkdf2_with_nonce(&password, &nonce, PBKDF2IC);

    let mut output: Vec<u8> = repeat(0u8).take(enmsg.len()).collect();

    encrypt_or_decrypt(enmsg, &mut output, &derived, nonce_20);

    if output != MSG.as_bytes() {
        return Err(Error::BadPassword);
    }
    Ok(derived)
}

// pub(crate) fn encrypt_or_decrypt_file(
//     source: PathBuf,
//     target: PathBuf,
// ) -> anyhow::Result<(), Error> {
//     let sd = read(source)?;
//     let mut cipher = chacha20::ChaCha20::new(&TEST_KEY[..], &TEST_NONCE[..]);
//     let mut encoded: Vec<u8> = repeat(0u8).take(sd.len()).collect();
//     cipher.process(&sd, &mut encoded);
//     write(target, encoded)?;
//     Ok(())
// }

pub fn encrypt_or_decrypt(source: &[u8], output: &mut [u8], key: &[u8], nonce: &[u8]) {
    let mut cipher = chacha20::ChaCha20::new(key, nonce);
    cipher.process(source, output);
}
pub fn gen_nonce(len: usize) -> Vec<u8> {
    let mut rng = thread_rng();
    repeat(0).take(len).map(|_| rng.gen()).collect()
}

pub fn pbkdf2_with_nonce(password: &str, nonce: &[u8], c: u32) -> [u8; 32] {
    let mut dk = [0u8; 32];

    let mut mac = Hmac::new(Sha256::new(), password.as_bytes());

    pbkdf2(&mut mac, nonce, c, &mut dk);
    dk
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_KEY: [u8; 32] = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
        0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d,
        0x1e, 0x1f,
    ];
    const TEST_NONCE: [u8; 12] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
    ];
    #[test]
    fn test_password_verification() {
        let password = "cipherbox$0awesome0$".to_owned();
        let mut path = std::env::temp_dir();
        path.push("message2verify");
        let k1 = set_password(password.clone(), path.clone()).unwrap();

        let k2 = verify_password(password.clone(), path).unwrap();
        assert_eq!(k1, k2);
    }

    #[test]
    fn test_password_derive() {
        let password = "cipherbox$0awesome0$";
        let nonce = gen_nonce(16);
        let derived01 = pbkdf2_with_nonce(password, &nonce, PBKDF2IC);
        let derived02 = pbkdf2_with_nonce(password, &nonce, PBKDF2IC);
        assert_eq!(derived01, derived02);
    }

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
