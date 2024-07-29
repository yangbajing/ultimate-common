use serde::{Deserialize, Serialize};
use ultimate_common::{
  string::{deser_str_to_vecu8, ser_vecu8_to_str},
  time::{self, Duration, OffsetDateTime},
};

#[derive(Clone, Deserialize, Serialize)]
pub struct SecruityConfig {
  pwd: PwdConf,
  token: TokenConf,
}

impl SecruityConfig {
  pub fn pwd(&self) -> &PwdConf {
    &self.pwd
  }

  pub fn token(&self) -> &TokenConf {
    &self.token
  }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct PwdConf {
  /// 密码过期秒数
  pwd_expires_in: i64,

  #[serde(deserialize_with = "deser_str_to_vecu8", serialize_with = "ser_vecu8_to_str")]
  pwd_key: Vec<u8>,

  /// 创建新用户时的默认密码（未未指定）
  default_pwd: String,
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
