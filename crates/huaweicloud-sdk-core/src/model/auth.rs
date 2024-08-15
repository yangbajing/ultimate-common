use super::{Auth, Credential};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct AuthResp {
  credential: Credential,
}

impl AuthResp {
  pub fn new(credential: Credential) -> Self {
    Self { credential }
  }

  pub fn credential(&self) -> &Credential {
    &self.credential
  }
}

#[derive(Serialize, Deserialize)]
pub struct AuthReq {
  auth: Auth,
}
impl AuthReq {
  pub fn new(auth: Auth) -> Self {
    Self { auth }
  }

  pub fn auth(&self) -> &Auth {
    &self.auth
  }
}
