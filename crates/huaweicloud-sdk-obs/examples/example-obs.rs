use std::sync::Arc;

use anyhow::Result;
use huaweicloud_sdk_core::client::ClientConfig;
use huaweicloud_sdk_obs::ObsClient;

#[tokio::main]
async fn main() -> Result<()> {
  let file = std::env::var("CARGO_MANIFEST_DIR")? + "/.app.toml";
  let cc = ClientConfig::load_from_file(&file)?;
  let cc = Arc::new(cc);

  let _obs = ObsClient::new(cc.clone());

  Ok(())
}
