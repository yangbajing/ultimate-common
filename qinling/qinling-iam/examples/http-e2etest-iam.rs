use anyhow::Result;
use asserhttp::{AsserhttpBody, AsserhttpStatus};
use qinling_iam::iam::{
  role::model::{RoleEntity, RoleForCreate},
  user::model::{UserEntity, UserRelRolesReq},
};
use reqwest::{Client, ClientBuilder};
use serde_json::json;
use ultimate_db::auth::LoginResp;

static URL_BASE: &str = "http://localhost:9500";
static USERNAME: &str = "qinling";
static PWD: &str = "Qinling.2024";

#[tokio::main]
async fn main() -> Result<()> {
  let client = ClientBuilder::new().build()?;

  // Login
  let token = login(&client).await?;

  // Verify get user
  let user_id = 1;
  get_user_by_id(&client, &token, user_id).await?;

  // Create Role
  let role_id: i64 = create_role(&client, &token).await?;

  // Get role by id
  get_role_by_id(&client, &token, role_id).await?;

  // grant role to user
  let req = UserRelRolesReq { user_id, role_ids: vec![role_id] };
  grant_role_to_user(&client, &token, req).await?;

  Ok(())
}

async fn grant_role_to_user(client: &Client, token: &str, req: UserRelRolesReq) -> Result<()> {
  let resp = client.post(format!("{URL_BASE}/api/user/grant_role")).bearer_auth(token).json(&req).send().await?;

  let status = resp.status();

  let ret = resp.json::<serde_json::Value>().await?;
  println!("Return is {}", ret);

  assert!(status.is_success());
  Ok(())
}

async fn get_role_by_id(client: &Client, token: &str, role_id: i64) -> Result<()> {
  client
    .get(format!("{URL_BASE}/api/role/{role_id}"))
    .bearer_auth(token)
    .send()
    .await
    .expect_status_ok()
    .expect_body_json(|role: RoleEntity| {
      println!("Resp is: {role:?}");
      assert_eq!(role.id, role_id);
    });

  Ok(())
}

async fn create_role(client: &Client, token: &str) -> Result<i64> {
  let req = RoleForCreate { name: "测试角色（管理员）".to_string(), status: None };
  let resp = client.post(format!("{URL_BASE}/api/role")).bearer_auth(token).json(&req).send().await?;
  let json: serde_json::Value = resp.json().await?;
  println!("json: {json:?}");
  let role_id = serde_json::from_value(json)?;

  Ok(role_id)
}

async fn get_user_by_id(client: &Client, token: &str, user_id: i64) -> Result<()> {
  client
    .get(format!("{URL_BASE}/api/user/{user_id}"))
    .bearer_auth(token)
    .send()
    .await
    .expect_status_ok()
    .expect_body_json(|user: UserEntity| {
      println!("Resp is: {user:?}");
      assert_eq!(user.id, 1);
      assert_eq!(user.username.as_deref(), Some(USERNAME));
    });
  Ok(())
}

async fn login(client: &Client) -> Result<String> {
  let resp = client
    .post(format!("{URL_BASE}/auth/login"))
    .json(&json!({
        "username": USERNAME,
        "pwd": PWD
    }))
    .send()
    .await?;
  let status = resp.status();

  let token = resp.json::<LoginResp>().await?.token;
  println!("Token is: {token}");
  assert!(!token.is_empty());

  assert!(status.is_success());
  Ok(token)
}
