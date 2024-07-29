use modql::field::{Fields, HasSeaFields};
use sea_query::Iden;
use serde::Deserialize;
use sqlx::{postgres::PgRow, FromRow};
use ultimate_common::time::OffsetDateTime;

#[derive(Deserialize, Default, FromRow, Fields)]
pub struct UserRoleRel {
  pub user_id: i64,
  pub role_id: i64,
  pub cid: Option<i64>,
  pub ctime: Option<OffsetDateTime>,
}

#[allow(dead_code)]
pub trait UserRolePgRow: HasSeaFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl UserRolePgRow for UserRoleRel {}

#[derive(Iden)]
pub enum UserRoleIden {
  RoleId,
  UserId,
}
