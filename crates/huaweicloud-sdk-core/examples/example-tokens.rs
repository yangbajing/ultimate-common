use std::sync::Arc;

use anyhow::Result;
use huaweicloud_sdk_core::{
  client::ClientConfig,
  model::{token::TokenReq, Auth, Identity, IdentityMethod, IdentityPassword, TokenScope},
  IamClient,
};

#[tokio::main]
async fn main() -> Result<()> {
  let file = std::env::var("CARGO_MANIFEST_DIR")? + "/.app.toml";
  let mut cc = ClientConfig::load_from_file(&file)?;
  cc.credentials_mut().endpoint = Some("https://iam.cn-southwest-2.myhuaweicloud.com".into());
  let cc = Arc::new(cc);
  let iam_client = IamClient::new(cc.clone());

  let token_req = TokenReq::new(
    Auth {
      identity: Identity::builder()
        .methods(vec![IdentityMethod::PASSWORD])
        .password(Some(IdentityPassword { user: cc.credentials().user_password().unwrap().clone() }))
        .build(),
    },
    TokenScope::from_domain_id(cc.credentials().user_password().unwrap().domain.id.as_ref().unwrap().to_string()),
  );

  // println!("token req is {}", serde_json::to_string(&token_req)?);

  let token = iam_client.tokens(&token_req).await?;

  println!("The token is {token:?}");

  Ok(())
}
