use modql::field::{Fields, HasSeaFields};
use sea_query::{all, Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use ultimate::ctx::Session;
use ultimate_db::{
  auth::LoginBy,
  base::{self, check_number_of_affected, DbBmc},
  Error, ModelManager, Result,
};

use crate::iam::repos::model::UserRoleIden;

use super::model::{UserCredentialEntity, UserEntity, UserForCreate, UserForUpdate, UserIden, UserRelRolesReq};

pub struct UserBmc;

impl DbBmc for UserBmc {
  const TABLE: &'static str = "user";
  const SCHEMA: &'static str = "iam";
}

impl UserBmc {
  pub async fn find_by_id(mm: &ModelManager, id: i64) -> Result<Option<UserEntity>> {
    let u = base::get::<Self, _>(mm, id.into()).await?;
    Ok(u)
  }

  pub async fn find_by_login(mm: &ModelManager, req: LoginBy) -> Result<UserEntity> {
    // -- Build query
    let mut query = Query::select();
    let select = query.from(Self::table_ref()).columns(UserEntity::sea_idens());
    if let Some(username) = req.username {
      select.and_where(Expr::col(UserIden::Username).eq(username));
    }

    // -- Execute query
    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let sqlx_query = sqlx::query_as_with::<_, UserEntity, _>(&sql, values);
    let u = mm.dbx().fetch_optional(sqlx_query).await?.ok_or_else(|| Error::NotFound {
      schema: Self::SCHEMA,
      table: Self::TABLE,
      sql,
    })?;

    Ok(u)
  }

  pub async fn create(ctx: &Session, mm: &ModelManager, user_c: UserForCreate) -> Result<i64> {
    let username = user_c.username.clone().unwrap_or_default();
    let phone = user_c.phone.clone().unwrap_or_default();

    let user_id = base::create::<Self, _>(ctx, mm, user_c).await.map_err(|model_error| {
      Error::resolve_unique_violation(
        model_error,
        Some(|table: &str, constraint: &str| {
          if table == UserBmc::TABLE {
            if constraint.contains("username") {
              Some(Error::UserAlreadyExists { key: "username", value: username })
            } else if constraint.contains("phone") {
              Some(Error::UserAlreadyExists { key: "phone", value: phone })
            } else {
              None
            }
          } else {
            None // Error::UniqueViolation will be created by resolve_unique_violation
          }
        }),
      )
    })?;

    Ok(user_id)
  }

  pub async fn update(session: &Session, mm: &ModelManager, id: i64, user_u: UserForUpdate) -> Result<()> {
    base::update::<Self, _>(session, mm, id.into(), user_u).await
  }

  pub async fn delete(session: &Session, mm: &ModelManager, id: i64) -> Result<()> {
    base::delete::<Self>(session, mm, id.into()).await
  }

  pub(crate) async fn delete_rel_roles(mm: &ModelManager, req: UserRelRolesReq) -> Result<u64> {
    let role_ids_len = req.role_ids.len();
    let query = Query::delete()
      .from_table(Self::table_ref())
      .cond_where(all![
        Expr::col(UserRoleIden::UserId).is(req.user_id),
        Expr::col(UserRoleIden::RoleId).is_in(req.role_ids)
      ])
      .to_owned();
    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let mm = mm.new_with_txn()?;
    mm.dbx().begin_txn().await?;

    let n = mm.dbx().execute(sqlx::query_with(&sql, values)).await?;
    let r = check_number_of_affected::<Self>(role_ids_len, n)?;

    mm.dbx().commit_txn().await?;

    Ok(r)
  }
}

pub struct UserCredentialBmc;

impl DbBmc for UserCredentialBmc {
  const TABLE: &'static str = "user_credential";
  const SCHEMA: &'static str = "iam";
}

impl UserCredentialBmc {
  pub async fn insert(ctx: &Session, mm: &ModelManager, uc: UserCredentialEntity) -> Result<i64> {
    base::create::<Self, _>(ctx, mm, uc).await
  }

  pub async fn find_by_id(mm: &ModelManager, id: i64) -> Result<Option<UserCredentialEntity>> {
    base::get::<Self, _>(mm, id.into()).await
  }

  pub(crate) async fn update_password(session: &Session, mm: &ModelManager, uc: UserCredentialEntity) -> Result<()> {
    base::update::<Self, _>(session, mm, uc.id.into(), UpdatePassword { pwd_hash: uc.pwd_hash }).await
  }
}

#[derive(Fields)]
struct UpdatePassword {
  pub pwd_hash: String,
}
