use std::collections::HashMap;

use config::Config;
use serde::de::{Unexpected, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use strum_macros::AsRefStr;

use crate::model::store::DbConfig;
use crate::security::SecruityConfig;

use super::TraceConfig;

#[derive(Clone, Serialize, Deserialize)]
pub struct UltimateConfig {
  pub config_file: String,

  app: AppConfig,

  security: SecruityConfig,

  db: DbConfig,

  trace: TraceConfig,

  web: WebConfig,

  grpc: GrpcConfig,
}

pub trait KeyTrait {
  fn token_key(&self) -> &[u8];
  fn token_duration_sec(&self) -> i64;
}

impl TryFrom<&Config> for UltimateConfig {
  type Error = super::Error;

  fn try_from(c: &Config) -> std::result::Result<Self, Self::Error> {
    let qc = c.get::<UltimateConfig>("ultimate")?;
    Ok(qc)
  }
}

impl UltimateConfig {
  pub fn app(&self) -> &AppConfig {
    &self.app
  }

  pub fn web(&self) -> &WebConfig {
    &self.web
  }

  pub fn security(&self) -> &SecruityConfig {
    &self.security
  }

  pub fn db(&self) -> &DbConfig {
    &self.db
  }

  pub fn trace(&self) -> &TraceConfig {
    &self.trace
  }

  pub fn grpc(&self) -> &GrpcConfig {
    &self.grpc
  }
}

impl KeyTrait for UltimateConfig {
  fn token_key(&self) -> &[u8] {
    self.security.token().secret_key()
  }

  fn token_duration_sec(&self) -> i64 {
    self.security.token().token_expires_in()
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
  pub run_mode: RunMode,
  pub name: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, AsRefStr)]
pub enum RunMode {
  DEV,
  TEST,
  DEMO,
  PROD,
}

impl RunMode {
  pub fn is_dev(&self) -> bool {
    self == &RunMode::DEV
  }
  pub fn is_test(&self) -> bool {
    self == &RunMode::TEST
  }
  pub fn is_stage(&self) -> bool {
    self == &RunMode::DEMO
  }
  pub fn is_prod(&self) -> bool {
    self == &RunMode::PROD
  }
}

impl<'de> Deserialize<'de> for RunMode {
  fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    static MSG: &str = "expect in ('dev', 'test', 'demo', 'prod').";
    struct StrToRunMode;
    impl<'d> Visitor<'d> for StrToRunMode {
      type Value = RunMode;

      fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(MSG)
      }

      fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        if v.eq_ignore_ascii_case(RunMode::DEV.as_ref()) {
          Ok(RunMode::DEV)
        } else if v.eq_ignore_ascii_case(RunMode::TEST.as_ref()) {
          Ok(RunMode::TEST)
        } else if v.eq_ignore_ascii_case(RunMode::DEMO.as_ref()) {
          Ok(RunMode::DEMO)
        } else if v.eq_ignore_ascii_case(RunMode::PROD.as_ref()) {
          Ok(RunMode::PROD)
        } else {
          Err(serde::de::Error::invalid_value(Unexpected::Str(v), &MSG))
        }
      }
    }
    deserializer.deserialize_str(StrToRunMode)
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebConfig {
  enable: bool,
  server_addr: String,
}
impl WebConfig {
  pub fn enable(&self) -> bool {
    self.enable
  }

  pub fn server_addr(&self) -> &str {
    &self.server_addr
  }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GrpcConfig {
  pub enable: bool,
  pub server_addr: String,
  pub plaintext: bool,
  pub clients: HashMap<String, GrpcClientConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrpcClientConfig {
  pub addr: String,

  pub plaintext: bool,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum ApiValidEffect {
  Allow,
  Deny,
}
impl ApiValidEffect {
  pub fn is_deny(&self) -> bool {
    match self {
      ApiValidEffect::Allow => false,
      ApiValidEffect::Deny => true,
    }
  }

  pub fn is_allow(&self) -> bool {
    match self {
      ApiValidEffect::Allow => true,
      ApiValidEffect::Deny => false,
    }
  }
}

impl<'de> Deserialize<'de> for ApiValidEffect {
  fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    deserializer.deserialize_str(StrToApiValidEffect)
  }
}

struct StrToApiValidEffect;
impl<'d> Visitor<'d> for StrToApiValidEffect {
  type Value = ApiValidEffect;

  fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
    formatter.write_str("expect 'allow' or 'deny'.")
  }

  fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    if v.eq_ignore_ascii_case("allow") {
      Ok(ApiValidEffect::Allow)
    } else if v.eq_ignore_ascii_case("deny") {
      Ok(ApiValidEffect::Deny)
    } else {
      Err(serde::de::Error::invalid_value(Unexpected::Str(v), &"expect 'allow' or 'deny'."))
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::configuration::util::load_config;

  use super::*;

  #[test]
  fn test_config_load() {
    // 两个下划线作为层级分隔符
    std::env::set_var("FUSION__WEB__SERVER_ADDR", "0.0.0.0:8000");

    std::env::set_var("FUSION__TOKEN_KEY", "8462b1ec9af827ebed13926f8f1e5409774fa1a21a1c8f726a4a34cf7dcabaf2");
    std::env::set_var("FUSION__PWD_KEY", "80c9a35c0f231219ca14c44fe10c728d");
    std::env::set_var("FUSION__APP__NAME", "ultimate");
    let c = load_config().unwrap();
    let qc = UltimateConfig::try_from(&c).unwrap();

    assert_eq!(qc.token_key(), b"80c9a35c0f231219ca14c44fe10c728d");
    assert_eq!(qc.token_key(), b"8462b1ec9af827ebed13926f8f1e5409774fa1a21a1c8f726a4a34cf7dcabaf2");

    // 由默认配置文件提供
    assert_eq!(&qc.web().server_addr, "0.0.0.0:8000");
    assert_eq!(&qc.app().name, "ultimate");
  }
}
