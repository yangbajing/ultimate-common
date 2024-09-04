use axum::{routing::post, Json, Router};
use ultimate_web::{ok, AppResult};

use crate::app::AppState;

use super::{AuthServ, SigninReq, SigninResp};

pub fn auth_routes() -> Router<AppState> {
  Router::new().route("/signin", post(signin))
}

async fn signin(auth_serv: AuthServ, Json(req): Json<SigninReq>) -> AppResult<SigninResp> {
  let resp = auth_serv.signin(req).await?;
  ok(resp)
}
