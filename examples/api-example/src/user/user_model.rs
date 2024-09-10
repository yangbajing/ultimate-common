use enum_iterator::Sequence;
use modql::{
  field::Fields,
  filter::{FilterNodes, OpValsInt32, OpValsInt64, OpValsString, OpValsValue},
};
use sea_query::enum_def;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use sqlx::prelude::FromRow;
use ultimate::{DataError, Result};
use ultimate_api::v1::{Page, PagePayload, Pagination};
use ultimate_common::{regex, time::UtcDateTime};
use ultimate_db::{to_sea_chrono_utc, DbRowType};

#[derive(Debug, Serialize, FromRow, Fields)]
#[enum_def]
pub struct User {
  pub id: i64,
  pub email: Option<String>,
  pub phone: Option<String>,
  pub name: String,
  pub status: UserStatus,
  pub cid: i64,
  pub ctime: UtcDateTime,
  pub mid: Option<i64>,
  pub mtime: Option<UtcDateTime>,
}
impl DbRowType for User {}

#[derive(Debug, Default, PartialEq, Eq, Serialize_repr, Deserialize_repr, Sequence, sqlx::Type)]
#[repr(i32)]
pub enum UserStatus {
  #[default]
  Normal = 10,
  Disable = 99,
  Enable = 100,
}

impl From<UserStatus> for sea_query::Value {
  fn from(value: UserStatus) -> Self {
    sea_query::Value::Int(Some(value as i32))
  }
}

impl sea_query::Nullable for UserStatus {
  fn null() -> sea_query::Value {
    sea_query::Value::Int(None)
  }
}

#[derive(Debug, Deserialize, Fields)]
pub struct UserForCreate {
  pub email: Option<String>,
  pub phone: Option<String>,
  pub name: Option<String>,
  pub status: Option<UserStatus>,
}

impl UserForCreate {
  /// 校验数据并进行初始化。`email` 或 `phone` 至少有一个，若两个值都设置，则只有 `email` 有效。
  ///
  /// 当 `name` 未设置时，将从 `email` 或 `phone` 中取值。
  pub fn validate_and_init(mut self) -> Result<Self> {
    if let Some(email) = self.email.as_deref() {
      if !regex::is_email(email) {
        return Err(DataError::bad_request("The 'email' field is invalid"));
      }
    } else if let Some(phone) = self.phone.as_deref() {
      if !regex::is_phone(phone) {
        return Err(DataError::bad_request("The 'phone' field is invalid"));
      }
    } else {
      return Err(DataError::bad_request("At least one 'email' or 'phone' is required"));
    };

    let has_name = self.name.as_deref().is_some_and(|n| !n.is_empty());
    if !has_name {
      self.name = match self.email.as_deref() {
        Some(email) => email.split('@').next().map(ToString::to_string),
        None => self.phone.clone(),
      };
    }

    Ok(self)
  }
}

#[derive(Debug, Deserialize, Fields)]
pub struct UserForUpdate {
  pub name: Option<String>,
  pub status: Option<UserStatus>,
}

#[derive(Debug, Default, Deserialize)]
pub struct UserForPage {
  pub page: Option<Pagination>,
  pub filter: Option<UserFilter>,
}

#[derive(Debug, Default, Deserialize, FilterNodes)]
pub struct UserFilter {
  pub email: Option<OpValsString>,

  pub phone: Option<OpValsString>,

  pub name: Option<OpValsString>,

  pub status: Option<OpValsInt32>,

  pub cid: Option<OpValsInt64>,

  #[modql(to_sea_value_fn = "to_sea_chrono_utc")]
  pub ctime: Option<OpValsValue>,

  pub mid: Option<OpValsInt64>,

  #[modql(to_sea_value_fn = "to_sea_chrono_utc")]
  pub mtime: Option<OpValsValue>,
}

#[derive(Debug, Serialize)]
pub struct UserPage {
  pub page: Page,
  pub items: Vec<User>,
}

impl From<PagePayload<User>> for UserPage {
  fn from(value: PagePayload<User>) -> Self {
    Self { page: value.page, items: value.items }
  }
}
