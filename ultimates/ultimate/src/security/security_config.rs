use derive_getters::Getters;
use ultimate_common::{
  string::{deser_str_to_vecu8, ser_vecu8_to_str},
  time::{self, Duration, OffsetDateTime},
};
use josekit::{jwe::JweHeader, jwt::JwtPayload, JoseError};
use serde::{Deserialize, Serialize};

use super::jose::{decrypt_jwe_dir, encrypt_jwe_dir};

#[derive(Clone, Deserialize, Serialize, Getters)]
pub struct SecruityConfig {
  pub(crate) pwd: PwdConf,
  pub(crate) token: TokenConf,
}

impl SecruityConfig {
  pub fn encrypt_jwt(&self, mut payload: JwtPayload) -> Result<String, JoseError> {
    if payload.expires_at().is_none() {
      let expires_at = self.token().token_expires_at().into();
      payload.set_expires_at(&expires_at);
    }
    encrypt_jwe_dir(self.token().secret_key(), &payload)
  }

  pub fn decrypt_jwt(&self, token: &str) -> Result<(JwtPayload, JweHeader), JoseError> {
    decrypt_jwe_dir(self.token().secret_key(), token)
  }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct PwdConf {
  pub(crate) pwd_expires_in: i64,
  #[serde(deserialize_with = "deser_str_to_vecu8", serialize_with = "ser_vecu8_to_str")]
  pub(crate) pwd_key: Vec<u8>,
  pub(crate) default_pwd: String,
}

impl PwdConf {
  pub fn pwd_expires_in(&self) -> i64 {
    self.pwd_expires_in
  }

  pub fn pwd_expires_at(&self) -> OffsetDateTime {
    time::now_utc() + Duration::seconds(self.pwd_expires_in())
  }

  pub fn pwd_key(&self) -> &[u8] {
    &self.pwd_key
  }

  pub fn default_pwd(&self) -> &str {
    &self.default_pwd
  }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct TokenConf {
  #[serde(deserialize_with = "deser_str_to_vecu8", serialize_with = "ser_vecu8_to_str")]
  pub(crate) secret_key: Vec<u8>,
  pub(crate) token_expires_in: i64,
  #[serde(deserialize_with = "deser_str_to_vecu8", serialize_with = "ser_vecu8_to_str")]
  public_key: Vec<u8>,
  #[serde(deserialize_with = "deser_str_to_vecu8", serialize_with = "ser_vecu8_to_str")]
  private_key: Vec<u8>,
}

impl TokenConf {
  pub fn secret_key(&self) -> &[u8] {
    &self.secret_key
  }

  pub fn token_expires_in(&self) -> i64 {
    self.token_expires_in
  }
  pub fn token_expires_at(&self) -> OffsetDateTime {
    time::now_utc() + Duration::seconds(self.token_expires_in())
  }

  pub fn public_key(&self) -> &[u8] {
    &self.public_key
  }

  pub fn private_key(&self) -> &[u8] {
    &self.private_key
  }
}
