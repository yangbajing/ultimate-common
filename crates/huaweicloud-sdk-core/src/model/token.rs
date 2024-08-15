use super::{Auth, TokenScope, UserDomain};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct TokenResp {
  token: Token,
  #[serde(default)]
  subject_token: String,
}

#[derive(Debug, Deserialize)]
pub struct Token {
  pub expires_at: String,
  pub methods: Vec<String>,
  pub domain: UserDomain,
  pub issued_at: String,
  pub user: UserPasswordForResp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPasswordForResp {
  pub domain: UserDomain,
  pub id: String,
  pub name: String,
  pub password_expires_at: String,
}

impl TokenResp {
  pub fn subject_token(&self) -> &str {
    &self.subject_token
  }
  pub fn with_subject_token(mut self, subject_token: impl Into<String>) -> Self {
    self.subject_token = subject_token.into();
    self
  }
  pub fn token(&self) -> &Token {
    &self.token
  }
}

#[derive(Serialize, Deserialize)]
pub struct TokenReq {
  auth: Auth,
  scope: TokenScope,
}
impl TokenReq {
  pub fn new(auth: Auth, scope: TokenScope) -> Self {
    Self { auth, scope }
  }

  pub fn auth(&self) -> &Auth {
    &self.auth
  }
}
