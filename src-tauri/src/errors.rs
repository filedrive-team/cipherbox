use serde::{Serialize,Serializer};


#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("io error: {0}")]
  Io(#[from] std::io::Error),
  #[error("Sqlite error: {0}")]
  Sqlite(#[from] rusqlite::Error),
  #[error("Tauri api error: {0}")]
  TauriApi(String),
  #[error("password not match")]
  BadPassword,
  #[error("* : {0}")]
  Other(String)
}

impl Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}


impl From<cid::Error> for Error {
    fn from(err: cid::Error) -> Error {
        Error::Other(err.to_string())
    }
}

