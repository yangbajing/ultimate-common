use modql::filter::{FilterNodes, OpValString, OpValsString};
use serde::{Deserialize, Serialize};

use crate::user::UserFilter;

#[derive(FilterNodes)]
pub struct LoginFilter {
  pub email: Option<OpValsString>,
  pub phone: Option<OpValsString>,
}

impl From<&SigninReq> for LoginFilter {
  fn from(req: &SigninReq) -> Self {
    Self {
      email: req.email.as_deref().map(|s| OpValString::Eq(s.to_string()).into()),
      phone: req.phone.as_deref().map(|s| OpValString::Eq(s.to_string()).into()),
    }
  }
}

#[derive(Deserialize)]
pub struct SigninReq {
  pub email: Option<String>,
  pub phone: Option<String>,
  #[serde(skip_serializing)]
  pub password: String,
}

impl From<&SigninReq> for UserFilter {
  fn from(value: &SigninReq) -> Self {
    UserFilter {
      email: value.email.as_deref().map(|s| OpValString::Eq(s.to_string()).into()),
      phone: value.phone.as_deref().map(|s| OpValString::Eq(s.to_string()).into()),
      ..Default::default()
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SigninResp {
  pub token: String,
  pub token_type: TokenType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TokenType {
  Bearer,
}
