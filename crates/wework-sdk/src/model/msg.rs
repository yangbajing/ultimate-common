use std::fmt::Display;

use serde::Serialize;
use strum::AsRefStr;
use typed_builder::TypedBuilder;

/// 发送应用消息
/// https://developer.work.weixin.qq.com/document/path/90236
///
/// `touser`, `toparty`, `totag` 至少设置一个。
#[derive(Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Message {
  #[builder(default, setter(strip_option))]
  pub touser: Option<String>,
  #[builder(default, setter(strip_option))]
  pub toparty: Option<String>,
  #[builder(default, setter(strip_option))]
  pub totag: Option<String>,

  pub msgtype: MsgType,

  pub agentid: i32,

  #[builder(default, setter(strip_option))]
  pub text: Option<MsgText>,

  #[builder(default = 1)]
  pub safe: i32,
  // pub enable_id_trans: i32,
  #[builder(default = 1)]
  pub enable_duplicate_check: i32,

  #[builder(default = 1800)]
  pub duplicate_check_interval: i32,
}

#[derive(Debug, Serialize, AsRefStr)]
#[allow(non_camel_case_types)]
pub enum MsgType {
  text,
  markdown,
  image,
}

impl Display for MsgType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.pad(self.as_ref())
  }
}

#[derive(Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(setter(into, strip_option)))]
pub struct MsgText {
  #[builder(setter(!strip_option))]
  content: String,
  #[builder(default)]
  mentioned_list: Option<Vec<String>>,
  #[builder(default)]
  mentioned_mobile_list: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct MsgMarkdown {
  content: String,
}

impl MsgMarkdown {
  pub fn new(content: impl Into<String>) -> Self {
    Self { content: content.into() }
  }
}

#[derive(Debug, Serialize)]
pub struct MsgImage {
  media_id: String,
}

impl MsgImage {
  pub fn new(media_id: impl Into<String>) -> Self {
    Self { media_id: media_id.into() }
  }
}
