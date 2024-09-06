use modql::filter::OpValInt64;
use ultimate::{DataError, Result};

use crate::ctx::CtxW;

use super::{
  User, UserBmc, UserCredential, UserCredentialBmc, UserFilter, UserForCreate, UserForPage, UserForUpdate, UserPage,
};

pub async fn create(ctx: &CtxW, req: UserForCreate) -> Result<i64> {
  let id = UserBmc::create(ctx.mm(), req.validate_and_init()?).await?;
  Ok(id)
}

pub async fn page(ctx: &CtxW, req: UserForPage) -> Result<UserPage> {
  let page = UserBmc::page(ctx.mm(), req.filter, req.page).await?;
  Ok(page.into())
}

pub async fn find_option_by_id(ctx: &CtxW, id: i64) -> Result<Option<User>> {
  let f = UserFilter { id: Some(OpValInt64::Eq(id).into()), ..Default::default() };
  let u = UserBmc::find_unique(ctx.mm(), vec![f]).await?;
  Ok(u)
}

pub async fn find_by_id(ctx: &CtxW, id: i64) -> Result<User> {
  let u = UserBmc::find_by_id(ctx.mm(), id).await?;
  Ok(u)
}

pub async fn update_by_id(ctx: &CtxW, id: i64, req: UserForUpdate) -> Result<()> {
  UserBmc::update_by_id(ctx.mm(), id, req).await?;
  Ok(())
}

pub async fn delete_by_id(ctx: &CtxW, id: i64) -> Result<()> {
  UserBmc::delete_by_id(ctx.mm(), id).await?;
  Ok(())
}

pub(crate) async fn get_fetch_credential(ctx: &CtxW, req: UserFilter) -> Result<(User, UserCredential)> {
  let u = UserBmc::find_unique(ctx.mm(), vec![req]).await?.ok_or_else(|| DataError::not_found("User not exists."))?;
  let uc = UserCredentialBmc::find_by_id(ctx.mm(), u.id).await?;
  Ok((u, uc))
}
