#[derive(thiserror::Error, Debug)]
pub enum WeaError {
    #[error("{0}")]
    PayError(#[from] PayError),
    #[error("Decrypt error: {0}")]
    DecryptError(#[from] aes_gcm::Error),
    #[error("io error : {0}")]
    IoError(#[from] std::io::Error),
    #[error("Utf8 convert error: {0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error("System time error: {0}")]
    SystemTimeError(#[from] std::time::SystemTimeError),
    #[error("Json convert error: {0}")]
    JsonConvertError(#[from] serde_json::Error),
    #[error("To String error: {0}")]
    ToStringError(#[from] reqwest::header::ToStrError),
    #[error("request api error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("{0}")]
    OpensslError(#[from] openssl::error::ErrorStack),
}

#[derive(thiserror::Error, Debug)]
#[error("{message}")]
pub struct PayError {
    message: String,
}
impl PayError {
    pub fn new(message: &str) -> Self {
        PayError {
            message: format!("{}", message),
        }
    }
}
