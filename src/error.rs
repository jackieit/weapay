use aes_gcm::Error as AesError;
use openssl::error::ErrorStack as OpensslError;
use reqwest::header::ToStrError as HeaderError;
use reqwest::Error as ReqwestError;
use serde_json::Error as JsonError;

use std::fmt;
use std::io::Error as IoError;
use std::string::FromUtf8Error as Utf8Error;
use std::time::SystemTimeError as TimeError;

#[derive(Debug)]
pub struct WeaError(String, String);

impl WeaError {
    pub fn new(kind: impl Into<String>, message: String) -> WeaError {
        WeaError(kind.into(), message)
    }
}
impl fmt::Display for WeaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WeaError: From {}, {}", self.0, self.1)
    }
}
impl std::error::Error for WeaError {}
impl From<IoError> for WeaError {
    fn from(err: IoError) -> Self {
        WeaError::new("StdIo", err.to_string())
    }
}
impl From<Utf8Error> for WeaError {
    fn from(err: Utf8Error) -> Self {
        WeaError::new("FromUtf8", err.to_string())
    }
}
impl From<TimeError> for WeaError {
    fn from(err: TimeError) -> Self {
        WeaError::new("SystemTime", err.to_string())
    }
}
impl From<JsonError> for WeaError {
    fn from(err: JsonError) -> Self {
        WeaError::new("JsonConvert", err.to_string())
    }
}
impl From<HeaderError> for WeaError {
    fn from(err: HeaderError) -> Self {
        WeaError::new("ReqwestHeader", err.to_string())
    }
}
impl From<ReqwestError> for WeaError {
    fn from(err: ReqwestError) -> Self {
        WeaError::new("Reqwest", err.to_string())
    }
}
impl From<OpensslError> for WeaError {
    fn from(err: OpensslError) -> Self {
        WeaError::new("Openssl", err.to_string())
    }
}
impl From<AesError> for WeaError {
    fn from(err: AesError) -> Self {
        WeaError::new("Aes", err.to_string())
    }
}
