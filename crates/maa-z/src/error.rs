#![allow(clippy::absolute_paths)]
use serde::Serialize;

pub type MaaResult<T> = Result<T, MaaError>;

#[derive(Serialize, Debug)]
pub enum MaaError {
    Utf8Error(String),
    MaaHandleInitError,
    DeviceConnectionError,
    IOError(String),
    TOMLDeError(String),
    TOMLSerError(String),
    UnknowTaskError(String),
    ResourceInitError,
    ResourceBindError,
    FindDeviceError,
    MaaToolkitInitError,
    InvalidCallbackEvent(String)
}

impl From<std::str::Utf8Error> for MaaError {
    fn from(e: std::str::Utf8Error) -> Self {
        MaaError::Utf8Error(e.to_string())
    }
}

impl From<std::io::Error> for MaaError {
    fn from(e: std::io::Error) -> Self {
        MaaError::IOError(e.to_string())
    }
}

impl From<toml::de::Error> for MaaError {
    fn from(e: toml::de::Error) -> Self {
        MaaError::TOMLDeError(e.to_string())
    }
}

impl From<toml::ser::Error> for MaaError {
    fn from(e: toml::ser::Error) -> Self {
        MaaError::TOMLSerError(e.to_string())
    }
}
