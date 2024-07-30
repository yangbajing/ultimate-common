use axum::{
    extract::State,
    routing::{delete, post},
    Json, Router,
};
use ultimate_db::auth::{LoginByPasswordReq, LoginResp};
use ultimate_web::AppResult;

use crate::{application::Application, ctx::Ctx, iam::auth::serv};

pub fn routes() -> Router<Application> {
    Router::new().route("/login", post(login)).route("/logout", delete(logout))
}

async fn login(State(state): State<Application>, Json(req): Json<LoginByPasswordReq>) -> AppResult<LoginResp> {
    let resp = serv::login_by_password(&state, &req).await?;
    Ok(resp.into())
}

async fn logout(ctx: Ctx) -> AppResult<()> {
    serv::logout(&ctx).await?;
    Ok(().into())
}
