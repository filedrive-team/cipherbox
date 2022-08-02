use super::*;

impl App {
    pub(crate) fn set_user_key(&mut self, key: [u8; 32]) {
        self.user_key = Some(key);
    }
    pub fn release_user_key(&mut self) {
        self.user_key = None;
    }
    pub fn is_user_key_set(&self) -> bool {
        if let None = self.user_key {
            return false;
        }
        true
    }
    pub fn has_set_password(&self) -> bool {
        // check if the encryted message and nonce exist
        match std::fs::metadata(std::path::PathBuf::from(&self.app_dir).join(CIPHER_MESSAGE_NAME)) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
