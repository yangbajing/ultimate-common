use axum::{
  async_trait,
  extract::FromRequestParts,
  http::{request::Parts, StatusCode},
  Json,
};
use derive_new::new;
use ultimate::{DataError, Result};
use ultimate_web::AppError;

use crate::{app::AppState, ctx::CtxW};

use super::{
  User, UserBmc, UserCredential, UserCredentialBmc, UserFilter, UserForCreate, UserForPage, UserForUpdate, UserPage,
};

#[derive(new)]
pub struct UserServ {
  ctx: CtxW,
}

impl UserServ {
  pub async fn create(&self, req: UserForCreate) -> Result<i64> {
    let id = UserBmc::create(self.ctx.mm(), req.validate_and_init()?).await?;
    Ok(id)
  }

  pub async fn page(&self, req: UserForPage) -> Result<UserPage> {
    let page =
      UserBmc::page(self.ctx.mm(), req.filter.into_iter().collect::<Vec<_>>(), req.page.unwrap_or_default()).await?;
    Ok(page.into())
  }

  pub async fn find_by_id(&self, id: i64) -> Result<User> {
    let u = UserBmc::find_by_id(self.ctx.mm(), id).await?;
    Ok(u)
  }

  pub async fn update_by_id(&self, id: i64, req: UserForUpdate) -> Result<()> {
    UserBmc::update_by_id(self.ctx.mm(), id, req).await?;
    Ok(())
  }

  pub async fn delete_by_id(&self, id: i64) -> Result<()> {
    UserBmc::delete_by_id(self.ctx.mm(), id).await?;
    Ok(())
  }

  pub(crate) async fn get_fetch_credential(&self, req: UserFilter) -> Result<(User, UserCredential)> {
    let u =
      UserBmc::find_unique(self.ctx.mm(), vec![req]).await?.ok_or_else(|| DataError::not_found("User not exists."))?;
    let uc = UserCredentialBmc::find_by_id(self.ctx.mm(), u.id).await?;
    Ok((u, uc))
  }
}

#[async_trait]
impl FromRequestParts<AppState> for UserServ {
  type Rejection = (StatusCode, Json<AppError>);

  async fn from_request_parts(parts: &mut Parts, state: &AppState) -> core::result::Result<Self, Self::Rejection> {
    let ctx = CtxW::from_request_parts(parts, state).await?;
    Ok(UserServ::new(ctx))
  }
}
