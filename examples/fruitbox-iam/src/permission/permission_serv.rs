use ultimate::Result;
use ultimate_api::v1::{PagePayload, Pagination};

use crate::ctx::CtxW;

use super::{permission_bmc::PermissionBmc, Permission, PermissionFilter, PermissionForCreate, PermissionForUpdate};

pub async fn create(ctx: &CtxW, req: PermissionForCreate) -> Result<i64> {
  let id = PermissionBmc::create(ctx.mm(), req.into()).await?;
  Ok(id)
}

pub async fn find_by_id(ctx: &CtxW, id: i64) -> Result<Permission> {
  let res = PermissionBmc::find_by_id(ctx.mm(), id).await?;
  Ok(res)
}

pub async fn update_by_id(ctx: &CtxW, id: i64, req: PermissionForUpdate) -> Result<()> {
  PermissionBmc::update_by_id(ctx.mm(), id, req).await?;
  Ok(())
}

pub async fn delete_by_id(ctx: &CtxW, id: i64) -> Result<()> {
  PermissionBmc::delete_by_id(ctx.mm(), id).await?;
  Ok(())
}

pub async fn page(
  ctx: &CtxW,
  filter: Vec<PermissionFilter>,
  pagination: Pagination,
) -> Result<PagePayload<Permission>> {
  let page = PermissionBmc::page(ctx.mm(), filter, pagination).await?;
  Ok(page)
}
