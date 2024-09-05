use axum::{
  extract::Path,
  routing::{get, post},
  Json, Router,
};
use ultimate::IdI64Result;
use ultimate_web::{ok, AppResult};

use crate::{app::AppState, ctx::CtxW};

use super::{User, UserForCreate, UserForPage, UserForUpdate, UserPage, UserServ};

pub fn user_routes() -> Router<AppState> {
  Router::new()
    .route("/", post(create_user))
    .route("/page", post(page_user))
    .route("/:id", get(get_user).put(update_user).delete(delete_user))
}

async fn create_user(ctx: CtxW, Json(req): Json<UserForCreate>) -> AppResult<IdI64Result> {
  let id = UserServ::create(&ctx, req).await?;
  ok(IdI64Result::new(id))
}

async fn page_user(ctx: CtxW, Json(req): Json<UserForPage>) -> AppResult<UserPage> {
  let page = UserServ::page(&ctx, req).await?;
  ok(page)
}

async fn get_user(ctx: CtxW, Path(id): Path<i64>) -> AppResult<User> {
  let u = UserServ::find_by_id(&ctx, id).await?;
  ok(u)
}

async fn update_user(ctx: CtxW, Path(id): Path<i64>, Json(req): Json<UserForUpdate>) -> AppResult<()> {
  UserServ::update_by_id(&ctx, id, req).await?;
  ok(())
}

async fn delete_user(ctx: CtxW, Path(id): Path<i64>) -> AppResult<()> {
  UserServ::delete_by_id(&ctx, id).await?;
  ok(())
}
