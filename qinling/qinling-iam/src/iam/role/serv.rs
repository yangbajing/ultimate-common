use ultimate::Result;

use crate::ctx::Ctx;
use crate::iam::repos::UserRoleBmc;

use super::model::*;
use super::repo::RoleBmc;

pub async fn create_role(ctx: &Ctx, req: RoleForCreate) -> Result<i64> {
    let id = RoleBmc::create(ctx.session(), ctx.mm(), req).await?;
    Ok(id)
}

pub async fn update_role(ctx: &Ctx, id: i64, req: RoleForUpdate) -> Result<()> {
    RoleBmc::update(ctx.session(), ctx.mm(), id, req).await?;
    Ok(())
}

pub async fn find_role_by_id(ctx: &Ctx, id: i64) -> Result<Option<RoleEntity>> {
    let role = RoleBmc::find_role_by_id(ctx.mm(), id).await?;
    Ok(role)
}

pub async fn delete_role_by_id(ctx: &Ctx, id: i64) -> Result<()> {
    RoleBmc::delete_by_id(ctx.session(), ctx.mm(), id).await?;
    Ok(())
}

pub async fn grant_role_to_users(ctx: &Ctx, req: RoleRelUsersReq) -> Result<u64> {
    let len = UserRoleBmc::insert_many(ctx.session(), ctx.mm(), req.into_user_role_entities()).await?;
    Ok(len)
}

pub(crate) async fn revoke_role_from_users(ctx: &Ctx, req: RoleRelUsersReq) -> Result<u64> {
    let n = RoleBmc::delete_rel_users(ctx.mm(), req).await?;
    Ok(n)
}
