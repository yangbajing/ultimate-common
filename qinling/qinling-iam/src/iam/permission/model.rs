use modql::field::{Fields, HasSeaFields};
use sea_query::enum_def;
use serde::Deserialize;
use sqlx::{postgres::PgRow, prelude::FromRow};
use ultimate_common::time::UtcDateTime;

#[derive(Default, Deserialize, FromRow, Fields)]
#[enum_def]
pub struct PermissionRoleRel {
    pub perm_id: i32,
    pub role_id: i64,
    pub cid: Option<i64>,
    pub ctime: Option<UtcDateTime>,
}
pub trait PermissionRoleRelPgRow: HasSeaFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}
impl PermissionRoleRelPgRow for PermissionRoleRel {}

#[derive(Default, Deserialize, FromRow, Fields)]
#[enum_def]
pub struct PermissionUserRel {
    pub perm_id: i32,
    pub user_id: i64,
    pub cid: Option<i64>,
    pub ctime: Option<UtcDateTime>,
}
pub trait PermissionUserRelPgRow: HasSeaFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}
impl PermissionUserRelPgRow for PermissionUserRel {}
