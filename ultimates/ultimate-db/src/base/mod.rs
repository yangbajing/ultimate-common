use derive_more::Display;
use sea_query::{Iden, SimpleExpr};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod crud_fns;
mod db_bmc;
mod macro_utils;
mod utils;

pub use crud_fns::*;
pub use db_bmc::*;
pub use utils::*;

const LIST_LIMIT_DEFAULT: i64 = 1000;
const LIST_LIMIT_MAX: i64 = 5000;

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
