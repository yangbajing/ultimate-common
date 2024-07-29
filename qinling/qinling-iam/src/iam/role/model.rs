use modql::field::{Fields, HasSeaFields};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, FromRow};
use ultimate_common::time::OffsetDateTime;

use crate::iam::repos::model::UserRoleRel;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, FromRow, Fields)]
pub struct RoleEntity {
  pub id: i64,
  pub name: String,
  pub status: i16,
  pub cid: i64,
  pub ctime: OffsetDateTime,
  pub mid: Option<i64>,
  pub mtime: Option<OffsetDateTime>,
}
pub trait RolePgRow: HasSeaFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}
impl RolePgRow for RoleEntity {}

#[derive(Serialize, Deserialize, Fields)]
pub struct RoleForCreate {
  pub name: String,
  pub status: Option<i16>,
}

#[derive(Deserialize, Fields)]
pub struct RoleForUpdate {
  pub name: Option<String>,
  pub status: Option<i16>,
}

#[derive(Deserialize)]
pub struct RoleRelUsersReq {
  pub role_id: i64,
  pub user_ids: Vec<i64>,
}
impl RoleRelUsersReq {
  pub(crate) fn into_user_role_entities(self) -> Vec<UserRoleRel> {
    let role_id = self.role_id;
    self.user_ids.into_iter().map(|uid| UserRoleRel { user_id: uid, role_id, ..Default::default() }).collect()
  }
}
