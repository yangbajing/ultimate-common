use axum::{
  async_trait,
  extract::FromRequestParts,
  http::{request::Parts, StatusCode},
};
use derive_more::derive::Constructor;
use ultimate::{security::pwd::verify_pwd, Result};
use ultimate_web::AppError;

use crate::{
  app::AppState,
  user::{UserFilter, UserServ},
  v1::{SigninReplay, SigninRequest, TokenType},
};

use super::utils::make_token;

#[derive(Constructor)]
pub struct AuthServ {
  app: AppState,
}

impl AuthServ {
  pub async fn signin(&self, req: SigninRequest) -> Result<SigninReplay> {
    let ctx = self.app.create_super_admin_ctx();

    let (u, uc) = UserServ::get_fetch_credential(&ctx, UserFilter::from(&req)).await?;
    verify_pwd(&req.password, &uc.encrypted_pwd).await?;

    let token = make_token(self.app.ultimate_config().security(), u.id)?;
    Ok(SigninReplay { token, token_type: TokenType::Bearer as i32 })
  }
}

#[async_trait]
impl FromRequestParts<AppState> for AuthServ {
  type Rejection = (StatusCode, AppError);

  async fn from_request_parts(_parts: &mut Parts, state: &AppState) -> core::result::Result<Self, Self::Rejection> {
    Ok(AuthServ::new(state.clone()))
  }
}
