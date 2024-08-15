use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
  #[error("Api error: {errcode} | {errmsg}")]
  WeworkError { errcode: i32, errmsg: String, json: serde_json::Value },

  #[error(transparent)]
  ReqwestError(#[from] reqwest::Error),

  #[error(transparent)]
  JsonError(#[from] serde_json::Error),
}
