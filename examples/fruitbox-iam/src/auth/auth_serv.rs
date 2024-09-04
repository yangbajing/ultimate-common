use axum::{
  async_trait,
  extract::FromRequestParts,
  http::{request::Parts, StatusCode},
};
use ultimate::{security::pwd::verify_pwd, Result};
use ultimate_web::AppError;

use crate::{
  app::AppState,
  user::{UserFilter, UserServ},
};

use super::{utils::make_token, SigninReq, SigninResp, TokenType};

pub struct AuthServ {
  app: AppState,
}

impl AuthServ {
  pub fn new(app: AppState) -> Self {
    Self { app }
  }

  pub async fn signin(&self, req: SigninReq) -> Result<SigninResp> {
    let user_serv = UserServ::new(self.app.create_super_admin_ctx());

    let (u, uc) = user_serv.get_fetch_credential(UserFilter::from(&req)).await?;
    verify_pwd(&req.password, &uc.encrypted_pwd).await?;

    let token = make_token(self.app.ultimate_config().security(), u.id)?;
    Ok(SigninResp { token, token_type: TokenType::Bearer })
  }
}

#[async_trait]
impl FromRequestParts<AppState> for AuthServ {
  type Rejection = (StatusCode, AppError);

  async fn from_request_parts(_parts: &mut Parts, state: &AppState) -> core::result::Result<Self, Self::Rejection> {
    Ok(AuthServ::new(state.clone()))
  }
}
