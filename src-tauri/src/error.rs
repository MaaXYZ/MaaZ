#![allow(clippy::absolute_paths)]
use serde::Serialize;

pub type MaaResult<T> = Result<T, MaaError>;

#[derive(Serialize, Debug)]
pub enum MaaError {
    Utf8Error(String),
    MaaHandleInitError,
    DeviceConnectionError,
    IOError(String),
    JsonDeError(String),
    UnknowTaskError(String),
    ResourceInitError,
    ResourceBindError,
    FindDeviceError,
    MaaToolkitInitError,
    PluginNotExist(String),
    PluginNoResource,
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

impl From<serde_json::error::Error> for MaaError {
    fn from(e: serde_json::error::Error) -> Self {
        MaaError::JsonDeError(e.to_string())
    }
}
