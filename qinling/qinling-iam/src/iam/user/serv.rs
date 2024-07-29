use ultimate::{
  error::DataError,
  model::{auth::LoginBy, ModelManager},
  security::pwd,
  Result,
};

use crate::{application::Application, ctx::Ctx, iam::repos::UserRoleBmc};

use super::{
  model::*,
  repo::{UserBmc, UserCredentialBmc},
};

pub async fn create_user(ctx: &Ctx, user_c: UserForCreate) -> Result<i64> {
  let mut user_c = user_c.validate()?;

  let password = user_c.password.as_deref().unwrap_or(ctx.ultimate_config().security().pwd().default_pwd());
  let pwd_hash = pwd::generate_pwd(password).await?;
  user_c.password = None;

  // Start the transaction
  let mm = ctx.mm().new_with_txn()?;
  mm.dbx().begin_txn().await?;

  let id = UserBmc::create(ctx.session(), &mm, user_c).await?;
  let uc = UserCredentialEntity {
    id,
    pwd_hash,
    cid: ctx.session().uid(),
    ctime: *ctx.session().req_time(),
    ..Default::default()
  };
  UserCredentialBmc::insert(ctx.session(), &mm, uc).await?;

  // Commit the transaction
  mm.dbx().commit_txn().await?;

  Ok(id)
}

pub async fn find_user_by_id(ctx: &Ctx, id: i64) -> Result<UserEntity> {
  let u = UserBmc::find_by_id(ctx.mm(), id).await?;
  Ok(u)
}

pub async fn update_user(ctx: &Ctx, id: i64, user_u: UserForUpdate) -> Result<()> {
  UserBmc::update(ctx.session(), ctx.mm(), id, user_u).await?;
  Ok(())
}

pub async fn delete_user(ctx: &Ctx, id: i64) -> Result<()> {
  UserBmc::delete(ctx.session(), ctx.mm(), id).await?;
  Ok(())
}

pub async fn update_user_password(ctx: &Ctx, pwd_u: PwdForUpdate) -> Result<()> {
  let mut uc = UserCredentialBmc::find_by_id(ctx.mm(), pwd_u.id).await?;

  // 管理员权限可以不用判断 old_password
  if !is_admin(ctx.state(), ctx.session().uid()) {
    let plain = pwd_u.old_password.ok_or_else(|| DataError::bad_request("普通用户需要传历史密码"))?;
    pwd::verify_pwd(&plain, &uc.pwd_hash).await?;
  }

  let hash = pwd::generate_pwd(&pwd_u.new_password).await?;
  uc.pwd_hash = hash;
  UserCredentialBmc::update_password(ctx.session(), ctx.mm(), uc).await?;

  Ok(())
}

fn is_admin(_state: &Application, _uid: i64) -> bool {
  // TODO 判断用户是否具备 role 权限
  false
}

pub async fn find_by_login(mm: &ModelManager, req: LoginBy) -> Result<UserEntity> {
  let u = UserBmc::find_by_login(mm, req).await?;
  Ok(u)
}

pub async fn find_user_credential_by_id(mm: &ModelManager, id: i64) -> Result<UserCredentialEntity> {
  let uc = UserCredentialBmc::find_by_id(mm, id).await?;
  Ok(uc)
}

pub(crate) async fn revoke_user_from_roles(ctx: &Ctx, req: UserRelRolesReq) -> Result<u64> {
  let n = UserBmc::delete_rel_roles(ctx.mm(), req).await?;
  Ok(n)
}

pub(crate) async fn grant_role_to_users(ctx: &Ctx, req: UserRelRolesReq) -> Result<u64> {
  let n = UserRoleBmc::insert_many(ctx.session(), ctx.mm(), req.into_user_role_entities()).await?;
  Ok(n)
}

#[cfg(test)]
mod tests {
  use super::*;
  use ultimate::Result;

  #[tokio::test]
  async fn test_create_user() -> Result<()> {
    let ctx = Ctx::load_on_test().await?;
    let user_c = UserForCreate {
      phone: Some("18580231385".to_string()),
      password: Some("Ultimate.2024".to_string()),
      name: Some("羊八井2".to_string()),
      status: 2,
      ..Default::default()
    };
    let uid = create_user(&ctx, user_c.clone()).await?;
    assert!(uid > 0);

    let user = find_user_by_id(&ctx, uid).await?;
    println!("user is: {}", serde_json::to_string(&user)?);
    assert_eq!(user.status, user_c.status);
    assert_eq!(user.username, user_c.phone);

    Ok(())
  }

  #[tokio::test]
  async fn test_find_by_id() -> Result<()> {
    let ctx = Ctx::load_on_test().await?;
    let id = 2;
    let u = find_user_by_id(&ctx, id).await?;
    println!("user is: {}", serde_json::to_string_pretty(&u)?);

    assert_eq!(u.id, id);
    assert_eq!(u.mtime, None);

    Ok(())
  }
}
