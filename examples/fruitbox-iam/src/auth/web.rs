use axum::{routing::post, Json, Router};
use ultimate_web::{ok, AppResult};

use crate::{
  app::AppState,
  v1::{SigninReplay, SigninRequest},
};

use super::AuthServ;

pub fn auth_routes() -> Router<AppState> {
  Router::new().route("/signin", post(signin))
}

async fn signin(auth_serv: AuthServ, Json(req): Json<SigninRequest>) -> AppResult<SigninReplay> {
  let resp = auth_serv.signin(req).await?;
  ok(resp)
}
