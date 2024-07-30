use axum::routing::{get, post};
use axum::{extract::Path, Json, Router};

use ultimate_web::AppResult;

use crate::application::Application;
use crate::ctx::Ctx;

use super::model::*;
use super::serv;

pub fn routes() -> Router<Application> {
    Router::new()
        .route("/:id", get(get_role_by_id).put(update_role).delete(delete_role_by_id))
        .route("/", post(create_role))
        .route("/grant_user", post(grant_user))
        .route("/revoke_user", post(revoke_user))
}

/// 角色: 根据ID查询明细
async fn get_role_by_id(ctx: Ctx, Path(id): Path<i64>) -> AppResult<Option<RoleEntity>> {
    let r = serv::find_role_by_id(&ctx, id).await?;
    Ok(r.into())
}

/// 角色: 创建
async fn create_role(ctx: Ctx, Json(role_c): Json<RoleForCreate>) -> AppResult<i64> {
    let id = serv::create_role(&ctx, role_c).await?;
    Ok(id.into())
}

/// 角色: 更新
async fn update_role(ctx: Ctx, Path(id): Path<i64>, Json(role_u): Json<RoleForUpdate>) -> AppResult<()> {
    serv::update_role(&ctx, id, role_u).await?;
    Ok(().into())
}

/// 角色: 删除
async fn delete_role_by_id(ctx: Ctx, Path(id): Path<i64>) -> AppResult<()> {
    serv::delete_role_by_id(&ctx, id).await?;
    Ok(().into())
}

/// 角色：分配用户以关联角色
async fn grant_user(ctx: Ctx, Json(req): Json<RoleRelUsersReq>) -> AppResult<()> {
    serv::grant_role_to_users(&ctx, req).await?;
    Ok(().into())
}

async fn revoke_user(ctx: Ctx, Json(req): Json<RoleRelUsersReq>) -> AppResult<()> {
    serv::revoke_role_from_users(&ctx, req).await?;
    Ok(().into())
}
