use ultimate::ctx::Session;
use ultimate_db::{
    base::{self, DbBmc},
    ModelManager, Result,
};

use super::model::*;

pub struct UserRoleBmc;

impl DbBmc for UserRoleBmc {
    const TABLE: &'static str = "user_role";
    const SCHEMA: &'static str = "iam";

    fn has_modification_timestamps() -> bool {
        false
    }
}

impl UserRoleBmc {
    pub(crate) async fn insert_many(
        ctx: &Session,
        mm: &ModelManager,
        data: impl IntoIterator<Item = UserRoleRel>,
    ) -> Result<u64> {
        let len = base::insert_many::<Self, _>(ctx, mm, data).await?;
        Ok(len)
    }
}
