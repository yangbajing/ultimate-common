use anyhow::Result;
use huaweicloud_sdk_core::{
  client::ClientConfig,
  model::{
    auth::AuthReq, Auth, Effect, Identity, IdentityMethod, IdentityPolicy, IdentityToken, PolicyStatement,
    PolicyVersion,
  },
};
use reqwest::{Method, StatusCode};

#[tokio::main]
async fn main() -> Result<()> {
  let file = std::env::var("CARGO_MANIFEST_DIR")? + "/.app.toml";
  let mut cc = ClientConfig::load_from_file(&file)?;
  cc.credentials_mut().endpoint = Some("https://iam.cn-southwest-2.myhuaweicloud.com".into());

  let auth_req = AuthReq::new(Auth {
    identity: Identity::builder()
      .methods(vec![IdentityMethod::TOKEN])
      .token(Some(IdentityToken::duration_seconds(24 * 60 * 60)))
      .policy(Some(IdentityPolicy {
        version: PolicyVersion::V1_1,
        statement: vec![PolicyStatement::builder()
          .effect(Effect::Allow)
          .append_action("obs:object:*")
          .append_resource("obs:*:*:object:*")
          .build()],
      }))
      .build(),
  });
  let request_builder = cc
    .create_http_client_builder()?
    .method(Method::POST)
    .resource_path("/v3.0/OS-CREDENTIAL/securitytokens")
    .json(serde_json::to_value(auth_req)?)
    .build(&cc)?;

  println!("request builder is {:?}", request_builder);

  let resp = request_builder.send().await?;
  println!("HTTP Response is {resp:?}");
  let status_code = resp.status();

  let text = resp.text().await?;
  println!("Response body is {text}");

  assert_eq!(status_code, StatusCode::CREATED);
  Ok(())
}
