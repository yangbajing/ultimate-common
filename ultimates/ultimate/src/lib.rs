//! 秦岭 Rust 公共库
//! This is an example of a footnote[^note].
//!
//! 这里是另一个角标[^2]
//!
//! [^2]: 这里是第二个角标内容。
//! 很好，未换行！
//!
//! [^note]: This text is the contents of the footnote, which will be rendered towards the bottom.
pub mod configuration;
pub mod ctx;
pub mod error;
pub mod metas;
pub mod model;
pub mod security;
pub mod starter;
pub mod trace;

pub type Result<T> = core::result::Result<T, error::DataError>;
