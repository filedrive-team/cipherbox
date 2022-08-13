use serde::{Serialize, Serializer};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("convert error: {0}")]
    Convert(#[from] std::str::Utf8Error),
    #[error("Tauri api error: {0}")]
    TauriApi(String),
    #[error("no db connection yet")]
    NoDBConnection,
    #[error("password not match")]
    BadPassword,
    #[error("session expired")]
    SessionExpired,
    #[error("Reqwest error: {0}")]
    Request(#[from] reqwest::Error),
    #[error("* : {0}")]
    Other(String),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl From<fvm_ipld_encoding::Error> for Error {
    fn from(err: fvm_ipld_encoding::Error) -> Error {
        Error::Other(err.to_string())
    }
}

impl From<cid::Error> for Error {
    fn from(err: cid::Error) -> Error {
        Error::Other(err.to_string())
    }
}

impl From<toml::ser::Error> for Error {
    fn from(err: toml::ser::Error) -> Error {
        Error::Other(err.to_string())
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Error {
        Error::Other(err.to_string())
    }
}

impl From<std::time::SystemTimeError> for Error {
    fn from(err: std::time::SystemTimeError) -> Error {
        Error::Other(err.to_string())
    }
}
