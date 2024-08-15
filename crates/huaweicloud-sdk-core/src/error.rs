use crate::digest::InvalidLength;
use reqwest::header::{InvalidHeaderName, InvalidHeaderValue, ToStrError};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, SdkError>;

#[derive(Error, Debug)]
pub enum SdkError {
  #[error("api[{0}] error: {1}")]
  ApiError(String, String),

  // 能用错误
  #[error("invalid header value")]
  InvalidHeaderValue(#[from] InvalidHeaderValue),

  #[error("invalid header name")]
  InvalidHeaderName(#[from] InvalidHeaderName),

  #[error("header to str error")]
  ToStrError(#[from] ToStrError),

  #[error("invalid length")]
  InvalidLength(#[from] InvalidLength),

  #[error("unknown data store error")]
  Unknown,

  #[error("parse error")]
  ParseError(#[from] url::ParseError),

  #[error("data store disconnected")]
  Disconnect(#[from] std::io::Error),

  #[error("the data for key `{0}` is not available")]
  Redaction(String),

  #[error(transparent)]
  JsonError(#[from] serde_json::Error),

  #[error(transparent)]
  HttpError(#[from] reqwest::Error),

  #[error(transparent)]
  ConfigError(#[from] config::ConfigError),
}

impl SdkError {
  pub fn api_error(api_path: impl Into<String>, err_msg: impl Into<String>) -> SdkError {
    SdkError::ApiError(api_path.into(), err_msg.into())
  }
}
