use chrono::{DateTime, Utc};
use modql::{
  field::Fields,
  filter::{FilterNodes, OpValsInt32, OpValsInt64},
};
use sea_query::enum_def;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use ultimate_db::DbRowType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow, Fields)]
#[enum_def]
#[sqlx(type_name = "role")]
pub struct RoleEntity {
  pub user_id: i64,
  pub role_id: i64,
  pub status: i32,
  pub cid: i64,
  pub ctime: DateTime<Utc>,
}
impl DbRowType for RoleEntity {}

#[derive(Fields)]
pub struct RoleForCreate {
  pub user_id: i64,
  pub role_id: i64,
  pub status: Option<i32>,
}

#[derive(Fields)]
pub struct RoleForUpdate {
  pub user_id: Option<i64>,
  pub role_id: Option<i64>,
  pub status: Option<i32>,
}

#[derive(Default, FilterNodes)]
pub struct RoleFilter {
  pub user_id: Option<OpValsInt64>,
  pub role_id: Option<OpValsInt64>,
  pub status: Option<OpValsInt32>,
}
