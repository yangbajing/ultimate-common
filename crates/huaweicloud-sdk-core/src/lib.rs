pub mod client;
pub mod digest;
mod error;
mod iam_client;
pub mod metas;
pub mod model;

pub use error::{Result, SdkError};
pub use iam_client::IamClient;

pub static ALGORITHM: &str = "SDK-HMAC-SHA256";
pub static HEADER_CONTENT_SHA_256: &str = "x-sdk-content-sha256";
pub static HEADER_X_SDK_DATE: &str = "X-Sdk-Date";

/// "%Y%m%dT%H%M%SZ"
pub const SDK_DATE_FORMAT: &str = "%Y%m%dT%H%M%SZ";
