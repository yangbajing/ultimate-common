use ultimate::{error::DataError, Result};
use ultimate_common::time::OffsetDateTime;
use modql::field::{Fields, HasSeaFields};
use sea_query::Iden;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, FromRow};

use crate::iam::repos::model::UserRoleRel;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow, Fields)]
pub struct UserEntity {
  pub id: i64,

  pub username: Option<String>,

  pub phone: Option<String>,

  pub name: Option<String>,

  pub status: i16,
  pub cid: i64,
  pub ctime: OffsetDateTime,
  pub mid: Option<i64>,
  pub mtime: Option<OffsetDateTime>,
}
pub trait UserPgRow: HasSeaFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}
impl UserPgRow for UserEntity {}

// Note: Since the entity properties Iden will be given by modql
//       UserIden does not have to be exhaustive, but just have the columns
//       we use in our specific code.
#[derive(Iden)]
pub enum UserIden {
  Id,
  Username,
  Phone,
  PwdHash,
}

#[derive(Default, FromRow, Fields)]
pub struct UserCredentialEntity {
  pub id: i64,
  pub pwd_hash: String,
  pub cid: i64,
  pub ctime: OffsetDateTime,
  pub mid: Option<i64>,
  pub mtime: Option<OffsetDateTime>,
}

#[derive(Clone, Deserialize, Fields)]
pub struct UserForCreate {
  pub username: Option<String>,
  pub phone: Option<String>,
  pub password: Option<String>,
  pub name: Option<String>,
  #[serde(default = "default_status")]
  pub status: i16,
}
impl Default for UserForCreate {
  fn default() -> Self {
    Self {
      username: Default::default(),
      phone: Default::default(),
      password: Default::default(),
      name: Default::default(),
      status: default_status(),
    }
  }
}
fn default_status() -> i16 {
  1
}
impl UserForCreate {
  pub fn validate(mut self) -> Result<Self> {
    if self.username.is_none() && self.phone.is_none() {
      return Err(DataError::bad_request("'username' and 'password' must be set at least one"));
    }

    match self.phone.as_ref() {
      Some(phone) if self.username.is_none() => {
        self.username = Some(phone.clone());
      }
      _ => { // do nothing
      }
    }

    if self.status < 1 {
      return Err(DataError::bad_request(format!("'status' must be > 0, but it is: {}", self.status)));
    }

    Ok(self)
  }
}

#[derive(Deserialize, Fields)]
pub struct UserForUpdate {
  pub username: Option<String>,
  pub name: Option<String>,
  pub phone: Option<String>,
  pub status: Option<i16>,
}

#[derive(Deserialize)]
pub struct PwdForUpdate {
  pub id: i64,
  pub old_password: Option<String>,
  pub new_password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserRelRolesReq {
  pub user_id: i64,
  pub role_ids: Vec<i64>,
}
impl UserRelRolesReq {
  pub(crate) fn into_user_role_entities(self) -> Vec<UserRoleRel> {
    let user_id = self.user_id;
    self.role_ids.into_iter().map(|rid| UserRoleRel { user_id, role_id: rid, ..Default::default() }).collect()
  }
}
