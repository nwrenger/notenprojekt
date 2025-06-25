use serde::Serialize;

/// Custom error type for the application, serializable for frontend compatibility.
#[derive(Debug, Serialize)]
#[serde(tag = "kind", content = "value")]
pub enum Error {
    /// Mutex lock is poisoned
    PoisonedLock,
    /// General Tauri Error
    Tauri(String),
    /// General Database Error
    Database(String),
    /// File System Error
    FileSystem(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::FileSystem(err.to_string())
    }
}

impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(_: std::sync::PoisonError<T>) -> Self {
        Error::PoisonedLock
    }
}

impl From<rusqlite::Error> for Error {
    fn from(err: rusqlite::Error) -> Self {
        Error::Database(err.to_string())
    }
}

impl From<tauri::Error> for Error {
    fn from(err: tauri::Error) -> Self {
        Error::Tauri(err.to_string())
    }
}
