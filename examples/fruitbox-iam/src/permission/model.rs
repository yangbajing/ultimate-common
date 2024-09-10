use modql::{
  field::Fields,
  filter::{FilterNodes, OpValString, OpValsInt64, OpValsString},
};
use o2o::o2o;
use sea_query::enum_def;
use sqlx::prelude::FromRow;
use ultimate_api::v1::PagePayload;
use ultimate_db::DbRowType;

use crate::proto::v1::{CreatePermission, FilterPermission, PagePermissionReply, PermissionDto, UpdatePermission};

#[derive(Debug, FromRow, Fields, o2o)]
#[enum_def]
#[owned_into(PermissionDto)]
pub struct Permission {
  pub id: i64,
  pub name: String,
  pub description: String,
  pub resource: String,
  pub action: String,

  pub cid: i64,
  pub ctime: i64,
  pub mid: Option<i64>,
  pub mtime: Option<i64>,
}
impl DbRowType for Permission {}

#[derive(Debug, Fields, o2o)]
#[from_owned(CreatePermission)]
pub struct PermissionForCreate {
  pub name: String,
  pub description: Option<String>,
  pub resource: String,
  pub action: String,
}

#[derive(Debug, Fields, o2o)]
#[from_owned(UpdatePermission)]
pub struct PermissionForUpdate {
  pub name: Option<String>,
  pub description: Option<String>,
  pub resource: Option<String>,
  pub action: Option<String>,
}

#[derive(Debug, FilterNodes, Default)]
pub struct PermissionFilter {
  pub id: Option<OpValsInt64>,
  pub name: Option<OpValsString>,
  pub description: Option<OpValsString>,
  pub resource: Option<OpValsString>,
  pub action: Option<OpValsString>,
}

impl From<FilterPermission> for PermissionFilter {
  fn from(value: FilterPermission) -> Self {
    Self {
      description: value.description.map(|v| OpValString::Eq(v).into()),
      resource: value.resource.map(|v| OpValString::Eq(v).into()),
      action: value.action.map(|v| OpValString::Eq(v).into()),
      name: value.name.map(|v| OpValString::Eq(v).into()),
      ..Default::default()
    }
  }
}

impl From<PagePayload<Permission>> for PagePermissionReply {
  fn from(value: PagePayload<Permission>) -> Self {
    let records = value.records.into_iter().map(|v| v.into()).collect();
    Self { page: Some(value.page), records }
  }
}
