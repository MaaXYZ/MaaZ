#![allow(clippy::absolute_paths)]
use serde::Serialize;

pub type MaaResult<T> = Result<T, MaaError>;

#[derive(Serialize)]
pub enum MaaError {
    Utf8Error(String),
    MaaHandleInitError,
    DeviceConnectionError
}

impl From<std::str::Utf8Error> for MaaError {
    fn from(e: std::str::Utf8Error) -> Self {
        MaaError::Utf8Error(e.to_string())
    }
}
