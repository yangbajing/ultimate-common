use axum::{
  async_trait,
  extract::FromRequestParts,
  http::{request::Parts, StatusCode},
};
use derive_new::new;
use ultimate::{security::pwd::verify_pwd, Result};
use ultimate_web::AppError;

use crate::{
  app::AppState,
  user::{UserFilter, UserServ},
  util::make_token,
};

use super::{LoginByPwdReq, LoginResp, TokenType};

#[derive(new)]
pub struct AuthServ {
  app: AppState,
}

impl AuthServ {
  pub async fn login_by_pwd(&self, req: LoginByPwdReq) -> Result<LoginResp> {
    let user_serv = UserServ::new(self.app.create_super_admin_ctx());

    let (u, uc) = user_serv.get_fetch_credential(UserFilter::from(&req)).await?;
    verify_pwd(&req.pwd, &uc.encrypted_pwd).await?;

    let token = make_token(self.app.ultimate_config().security(), u.id)?;
    Ok(LoginResp { token, token_type: TokenType::Bearer })
  }
}

#[async_trait]
impl FromRequestParts<AppState> for AuthServ {
  type Rejection = (StatusCode, AppError);

  async fn from_request_parts(_parts: &mut Parts, state: &AppState) -> core::result::Result<Self, Self::Rejection> {
    Ok(AuthServ::new(state.clone()))
  }
}
