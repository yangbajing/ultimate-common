use axum::{routing::post, Json, Router};
use ultimate_web::{ok, AppResult};

use crate::state::AppState;

use super::{AuthServ, LoginByPwdReq, LoginResp};

pub fn auth_routes() -> Router<AppState> {
  Router::new().route("/login/pwd", post(login_pwd))
}

async fn login_pwd(auth_serv: AuthServ, Json(req): Json<LoginByPwdReq>) -> AppResult<LoginResp> {
  let resp = auth_serv.login_by_pwd(req).await?;
  ok(resp)
}
