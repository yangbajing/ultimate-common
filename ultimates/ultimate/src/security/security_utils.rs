use josekit::{jwe::JweHeader, jwt::JwtPayload, JoseError};

use crate::configuration::model::{PwdConf, TokenConf};

use super::jose::{decrypt_jwe_dir, encrypt_jwe_dir};

pub struct SecurityUtils;

impl SecurityUtils {
  pub fn encrypt_jwt(token_conf: &TokenConf, mut payload: JwtPayload) -> Result<String, JoseError> {
    if payload.expires_at().is_none() {
      let expires_at = token_conf.token_expires_at().into();
      payload.set_expires_at(&expires_at);
    }
    encrypt_jwe_dir(token_conf.secret_key(), &payload)
  }

  pub fn decrypt_jwt(token_conf: &TokenConf, token: &str) -> Result<(JwtPayload, JweHeader), JoseError> {
    decrypt_jwe_dir(token_conf.secret_key(), token)
  }

  pub fn encrypt_pwd_jwt(pwd_conf: &PwdConf, mut payload: JwtPayload) -> Result<String, JoseError> {
    if payload.expires_at().is_none() {
      let expires_at = pwd_conf.pwd_expires_at().into();
      payload.set_expires_at(&expires_at);
    }
    encrypt_jwe_dir(pwd_conf.pwd_key(), &payload)
  }

  pub fn decrypt_pwd_jwt(pwd_conf: &PwdConf, token: &str) -> Result<(JwtPayload, JweHeader), JoseError> {
    decrypt_jwe_dir(pwd_conf.pwd_key(), token)
  }
}
