use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Error, Serialize)]
pub enum Error {
  #[error("Txn can't commit, no open txn")]
  TxnCantCommitNoOpenTxn,
  #[error("Cannot begin txn with txn false")]
  CannotBeginTxnWithTxnFalse,
  #[error("Cannot commit txn with txn false")]
  CannotCommitTxnWithTxnFalse,
  #[error("No txn")]
  NoTxn,

  #[error("Config invalid, error is: {0}")]
  ConfigInvalid(&'static str),

  // -- Externals
  #[error(transparent)]
  Sqlx(
    #[from]
    #[serde_as(as = "DisplayFromStr")]
    sqlx::Error,
  ),
}
