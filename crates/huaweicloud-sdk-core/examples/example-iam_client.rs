use anyhow::Result;
use huaweicloud_sdk_core::{
  client::ClientConfig,
  model::{
    auth::AuthReq, auth::AuthResp, Auth, Effect, Identity, IdentityMethod, IdentityPolicy, IdentityToken,
    PolicyStatement, PolicyVersion,
  },
  IamClient,
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
  let file = std::env::var("CARGO_MANIFEST_DIR")? + "/.app.toml";
  let mut cc = ClientConfig::load_from_file(&file)?;

  cc.credentials_mut().endpoint = Some("https://iam.cn-southwest-2.myhuaweicloud.com".into());
  let cc = Arc::new(cc);
  let iam_client = IamClient::new(cc.clone());

  let auth_req = AuthReq::new(Auth {
    identity: Identity {
      methods: vec![IdentityMethod::TOKEN],
      token: Some(IdentityToken::duration_seconds(24 * 60 * 60)),
      policy: Some(IdentityPolicy {
        version: PolicyVersion::V1_1,
        statement: vec![PolicyStatement::builder()
          .effect(Effect::Allow)
          .append_action("obs:object:*")
          .append_resource("obs:*:*:object:*")
          .build()],
      }),
      password: None,
    },
  });
  let result: AuthResp = iam_client.securitytokens(&auth_req).await?;
  println!("The result is {result:?}");

  Ok(())
}
