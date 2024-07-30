use sea_query::{all, Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use ultimate::ctx::Session;
use ultimate_db::{
    base::{self, check_number_of_affected, DbBmc},
    ModelManager, Result,
};

use crate::iam::repos::model::UserRoleIden;

use super::model::*;

pub struct RoleBmc;
impl DbBmc for RoleBmc {
    const TABLE: &'static str = "role";
    const SCHEMA: &'static str = "iam";
}

impl RoleBmc {
    pub(crate) async fn find_role_by_id(mm: &ModelManager, id: i64) -> Result<Option<RoleEntity>> {
        let opt = base::get::<Self, _>(mm, id.into()).await?;
        Ok(opt)
    }

    pub(crate) async fn create(session: &Session, mm: &ModelManager, role_c: RoleForCreate) -> Result<i64> {
        let id = base::create::<Self, _>(session, mm, role_c).await?;
        Ok(id)
    }

    pub(crate) async fn update(session: &Session, mm: &ModelManager, id: i64, role_u: RoleForUpdate) -> Result<()> {
        base::update::<Self, _>(session, mm, id.into(), role_u).await?;
        Ok(())
    }

    pub(crate) async fn delete_by_id(session: &Session, mm: &ModelManager, id: i64) -> Result<()> {
        base::delete::<Self>(session, mm, id.into()).await?;
        Ok(())
    }

    pub(crate) async fn delete_rel_users(mm: &ModelManager, req: RoleRelUsersReq) -> Result<u64> {
        let user_ids_len = req.user_ids.len();
        let query = Query::delete()
            .from_table(Self::table_ref())
            .cond_where(all![
                Expr::col(UserRoleIden::RoleId).is(req.role_id),
                Expr::col(UserRoleIden::UserId).is_in(req.user_ids)
            ])
            .to_owned();
        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

        let mm = mm.new_with_txn()?;
        mm.dbx().begin_txn().await?;

        let n = mm.dbx().execute(sqlx::query_with(&sql, values)).await?;
        let r = check_number_of_affected::<Self>(user_ids_len, n)?;

        mm.dbx().commit_txn().await?;

        Ok(r)
    }
}
