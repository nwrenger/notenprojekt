use std::fmt;

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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::PoisonedLock => write!(f, "a mutex lock was poisoned"),
            Error::Tauri(msg) => write!(f, "tauri error: {}", msg),
            Error::Database(msg) => write!(f, "database error: {}", msg),
            Error::FileSystem(msg) => write!(f, "filesystem error: {}", msg),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

impl std::error::Error for Error {}

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

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Error::Database(err.to_string())
    }
}

impl From<tauri::Error> for Error {
    fn from(err: tauri::Error) -> Self {
        Error::Tauri(err.to_string())
    }
}
