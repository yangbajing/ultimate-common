use axum::routing::{get, post, put};
use axum::{extract::Path, Json, Router};

use ultimate_web::AppResult;

use crate::application::Application;
use crate::ctx::Ctx;

use super::model::*;
use super::serv;

pub fn routes() -> Router<Application> {
  Router::new()
    .route("/:id", get(get_user_by_id).put(update_user).delete(delete_user))
    .route("/", post(create_user))
    .route("/password", put(update_password))
    .route("/grant_role", post(grant_role))
    .route("/revoke_role", post(revoke_role))
}

/// 用户: 根据ID查询明细 , Path(id): Path<i64>
async fn get_user_by_id(ctx: Ctx, Path(id): Path<i64>) -> AppResult<UserEntity> {
  let user = serv::find_user_by_id(&ctx, id).await?;
  Ok(user.into())
}

/// 用户: 创建
async fn create_user(ctx: Ctx, Json(user_c): Json<UserForCreate>) -> AppResult<i64> {
  let id = serv::create_user(&ctx, user_c).await?;
  Ok(id.into())
}

/// 用户: 更新
async fn update_user(ctx: Ctx, Path(id): Path<i64>, Json(user_u): Json<UserForUpdate>) -> AppResult<()> {
  serv::update_user(&ctx, id, user_u).await?;
  Ok(().into())
}

/// 用户: 删除
/// 注意：当前为物理删除
async fn delete_user(ctx: Ctx, Path(id): Path<i64>) -> AppResult<()> {
  serv::delete_user(&ctx, id).await?;
  Ok(().into())
}

/// 用户: 修改密码
async fn update_password(ctx: Ctx, Json(password_u): Json<PwdForUpdate>) -> AppResult<()> {
  serv::update_user_password(&ctx, password_u).await?;
  Ok(().into())
}

/// 用户: 授权角色
async fn grant_role(ctx: Ctx, Json(req): Json<UserRelRolesReq>) -> AppResult<()> {
  serv::grant_role_to_users(&ctx, req).await?;
  Ok(().into())
}

/// 用户: 回收角色
async fn revoke_role(ctx: Ctx, Json(req): Json<UserRelRolesReq>) -> AppResult<()> {
  serv::revoke_user_from_roles(&ctx, req).await?;
  Ok(().into())
}
