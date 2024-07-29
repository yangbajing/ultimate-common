//! 企业微信 SDK
pub mod client;
mod consts;
mod error;
pub mod metas;
pub mod model;
mod utils;
mod wework_config;

pub use error::{Error, Result};
pub use wework_config::WeworkConfig;
