mod crud_fns;
mod macro_utils;
mod utils;

// region:    --- Modules

use derive_more::{Display, From};
use modql::SIden;
use sea_query::{Iden, IntoIden, SimpleExpr, TableRef};

// -- Flatten hierarchy for user code.
pub use crud_fns::*;
use serde::{Deserialize, Serialize};
pub use utils::*;
use uuid::Uuid;

const LIST_LIMIT_DEFAULT: i64 = 1000;
const LIST_LIMIT_MAX: i64 = 5000;

// endregion: --- Consts

// region:    --- SeaQuery Idens

#[derive(Iden)]
pub enum CommonIden {
  Id,
  OwnerId,
  LogiscalDeletion,
  OptimisticLock,
}

#[derive(Iden)]
pub enum TimestampIden {
  Cid,
  Ctime,
  Mid,
  Mtime,
}

// endregion: --- SeaQuery Idens

/// The DbBmc trait must be implemented for the Bmc struct of an entity.
/// It specifies meta information such as the table name,
/// whether the table has timestamp columns (cid, ctime, mid, mtime), and more as the
/// code evolves.
///
/// Note: This trait should not be confused with the BaseCrudBmc trait, which provides
///       common default CRUD BMC functions for a given Bmc/Entity.
pub trait DbBmc {
  const TABLE: &'static str;
  const SCHEMA: &'static str = "public";

  fn table_ref() -> TableRef {
    TableRef::SchemaTable(SIden(Self::SCHEMA).into_iden(), SIden(Self::TABLE).into_iden())
  }

  fn qualified_table() -> (&'static str, &'static str) {
    (Self::SCHEMA, Self::TABLE)
  }

  /// Specifies that the table for this Bmc has timestamps (cid, ctime, mid, mtime) columns.
  /// This will allow the code to update those as needed.
  ///
  /// default: true
  fn has_creation_timestamps() -> bool {
    true
  }

  /// default: true
  fn has_modification_timestamps() -> bool {
    true
  }

  /// 是否使用逻辑删除
  ///
  /// default: false
  fn use_logical_deletion() -> bool {
    false
  }

  /// Specifies if the entity table managed by this BMC
  /// has an `owner_id` column that needs to be set on create (by default ctx.user_id).
  ///
  /// default: false
  fn has_owner_id() -> bool {
    false
  }

  /// 乐观锁
  /// default: false
  fn has_optimistic_lock() -> bool {
    false
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Display)]
#[serde(untagged)]
pub enum Id {
  I32(i32),
  I64(i64),
  String(String),
  Uuid(Uuid),
}

impl From<Id> for SimpleExpr {
  fn from(value: Id) -> Self {
    match value {
      Id::I32(id) => SimpleExpr::Value(id.into()),
      Id::I64(id) => SimpleExpr::Value(id.into()),
      Id::String(id) => SimpleExpr::Value(id.into()),
      Id::Uuid(id) => SimpleExpr::Value(id.into()),
    }
  }
}

impl From<i32> for Id {
  fn from(value: i32) -> Self {
    Id::I32(value)
  }
}

impl From<i64> for Id {
  fn from(value: i64) -> Self {
    Id::I64(value)
  }
}

impl From<String> for Id {
  fn from(value: String) -> Self {
    Id::String(value)
  }
}

impl From<&str> for Id {
  fn from(value: &str) -> Self {
    Id::String(value.to_string())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[derive(PartialEq, Serialize, Deserialize)]
  struct TestModel {
    pub role_id: i32,
    pub user_id: i64,
    pub order_id: Uuid,
    pub dict_id: String,
  }

  #[test]
  fn test_id() -> anyhow::Result<()> {
    let id = Id::I32(32);
    println!("id is {id}");
    let order_id = Id::Uuid(Uuid::now_v7());
    println!("order id is {order_id}");
    assert_eq!("32", serde_json::to_string(&id)?);
    assert_eq!(serde_json::to_string(&Id::String("abcdefg".into()))?, r#""abcdefg""#);

    let tm = TestModel {
      role_id: 53,
      user_id: 2309457238947,
      order_id: Uuid::now_v7(),
      dict_id: "system.run.mode".to_string(),
    };

    let v = serde_json::to_value(tm)?;
    let role_id: i32 = serde_json::from_value(v.get("role_id").unwrap().clone())?;
    assert_eq!(role_id, 53);

    Ok(())
  }
}
