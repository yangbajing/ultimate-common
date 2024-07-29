use ultimate::{
  error::DataError,
  model::auth::{LoginByPasswordReq, LoginResp},
  security::pwd::verify_pwd,
  Result,
};
use josekit::jwt::JwtPayload;

use crate::{
  application::Application,
  iam::user::serv::{find_by_login, find_user_credential_by_id},
};

pub(crate) async fn login_by_password(state: &Application, req: &LoginByPasswordReq) -> Result<LoginResp> {
  let mm = state.db_state().mm();
  let u = find_by_login(mm, req.into()).await?;
  let uc = find_user_credential_by_id(mm, u.id).await?;

  verify_pwd(&req.pwd, &uc.pwd_hash).await?;

  let mut payload = JwtPayload::new();
  payload.set_subject(u.id.to_string());
  let token = state
    .ultimate_config()
    .security()
    .encrypt_jwt(payload)
    .map_err(|_e| DataError::unauthorized("Failed generate token"))?;

  Ok(LoginResp { token })
}

pub(crate) async fn logout(_ctx: &crate::ctx::Ctx) -> Result<()> {
  // TODO 设置 token 为黑名单
  todo!()
}
