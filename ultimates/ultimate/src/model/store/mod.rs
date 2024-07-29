// region:    --- Modules

use std::borrow::Cow;
use std::time::Duration;

use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use ultimate_common::model::sensitive::UriString;

pub use crate::model::store::dbx::{Error, Result};

pub(in crate::model) mod dbx;

// endregion: --- Modules

#[derive(Debug, Clone, Serialize, Deserialize, Getters)]
pub struct DbConfig {
  enable: bool,
  /// The URI of the database
  // #[serde(serialize_with = "ser_str_secret")]
  url: UriString,
  /// Maximum number of connections for a pool
  max_connections: Option<u32>,
  /// Minimum number of connections for a pool
  min_connections: Option<u32>,
  /// Maximum idle time for a particular connection to prevent
  /// network resource exhaustion
  idle_timeout: Option<Duration>,
  /// Set the maximum amount of time to spend waiting for acquiring a connection
  acquire_timeout: Option<Duration>,
  /// Set the maximum lifetime of individual connections
  max_lifetime: Option<Duration>,
  /// Enable SQLx statement logging
  sqlx_logging: Option<bool>,
  /// SQLx statement logging level (ignored if `sqlx_logging` is false)
  sqlx_logging_level: Option<String>,
  /// set sqlcipher key
  sqlcipher_key: Option<Cow<'static, str>>,
  /// Schema search path (PostgreSQL only)
  schema_search_path: Option<String>,
}
