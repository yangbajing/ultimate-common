use anyhow::Result;
use huaweicloud_sdk_core::client::ClientConfig;

#[tokio::main]
async fn main() -> Result<()> {
  let file = std::env::var("CARGO_MANIFEST_DIR")? + "/.app.toml";
  let cc = ClientConfig::load_from_file(&file)?;

  let b = cc.create_http_client_builder().unwrap();
  let request_builder =
    b.resource_path(&format!("/v1/{}/vpcs", cc.credentials().project_id().unwrap())).query("limit=1").build(&cc)?;

  let resp = request_builder.send().await?;
  println!("HTTP Response is {resp:?}");
  let text = resp.text().await?;
  println!("Response body is {text}");

  Ok(())
}
