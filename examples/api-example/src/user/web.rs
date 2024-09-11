use axum::{
  extract::Path,
  routing::{get, post},
  Json, Router,
};
use ultimate::IdI64Result;
use ultimate_web::{ok, AppResult};

use crate::app::AppState;

use super::{User, UserForCreate, UserForPage, UserForUpdate, UserPage, UserServ};

pub fn user_routes() -> Router<AppState> {
  Router::new()
    .route("/", post(create_user))
    .route("/page", post(page_user))
    .route("/:id", get(get_user).put(update_user).delete(delete_user))
}

async fn create_user(user_serv: UserServ, Json(req): Json<UserForCreate>) -> AppResult<IdI64Result> {
  let id = user_serv.create(req).await?;
  ok(IdI64Result::new(id))
}

async fn page_user(user_serv: UserServ, Json(req): Json<UserForPage>) -> AppResult<UserPage> {
  let page = user_serv.page(req).await?;
  ok(page)
}

async fn get_user(user_serv: UserServ, Path(id): Path<i64>) -> AppResult<User> {
  let u = user_serv.find_by_id(id).await?;
  ok(u)
}

async fn update_user(user_serv: UserServ, Path(id): Path<i64>, Json(req): Json<UserForUpdate>) -> AppResult<()> {
  user_serv.update_by_id(id, req).await?;
  ok(())
}

async fn delete_user(user_serv: UserServ, Path(id): Path<i64>) -> AppResult<()> {
  user_serv.delete_by_id(id).await?;
  ok(())
}
