use modql::filter::{FilterNodes, OpValString, OpValsString};
use serde::{Deserialize, Serialize};

use crate::user::UserFilter;

#[derive(FilterNodes)]
pub struct LoginFilter {
  pub email: Option<OpValsString>,
  pub phone: Option<OpValsString>,
}

impl From<&LoginByPwdReq> for LoginFilter {
  fn from(req: &LoginByPwdReq) -> Self {
    Self {
      email: req.email.as_deref().map(|s| OpValString::Eq(s.to_string()).into()),
      phone: req.phone.as_deref().map(|s| OpValString::Eq(s.to_string()).into()),
    }
  }
}

#[derive(Deserialize)]
pub struct LoginByPwdReq {
  pub email: Option<String>,
  pub phone: Option<String>,
  #[serde(skip_serializing)]
  pub pwd: String,
}

impl From<&LoginByPwdReq> for UserFilter {
  fn from(value: &LoginByPwdReq) -> Self {
    UserFilter {
      email: value.email.as_deref().map(|s| OpValString::Eq(s.to_string()).into()),
      phone: value.phone.as_deref().map(|s| OpValString::Eq(s.to_string()).into()),
      ..Default::default()
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResp {
  pub token: String,
  pub token_type: TokenType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TokenType {
  Bearer,
}
