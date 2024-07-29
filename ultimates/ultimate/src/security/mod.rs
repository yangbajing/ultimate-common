mod error;
pub mod jose;
pub mod pwd;
mod security_config;

use serde::Deserialize;

pub use error::{Error, Result};
pub use security_config::*;

#[derive(Deserialize)]
pub struct AccessToken {
  pub access_token: String,
}
