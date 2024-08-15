use huaweicloud_sdk_core::client::ClientConfig;
use huaweicloud_sdk_core::Result;
use reqwest::header::{HeaderMap, HeaderValue};
use std::sync::Arc;

pub struct ObsClient {
  client_config: Arc<ClientConfig>,
}

impl ObsClient {
  pub fn new(client_config: Arc<ClientConfig>) -> Self {
    Self { client_config }
  }

  pub fn client_config(&self) -> &ClientConfig {
    &self.client_config
  }

  pub async fn put_object(&self, _bucket: &str, _key: &str, object: &[u8]) -> Result<()> {
    let mut with_headers = HeaderMap::new();
    with_headers.insert("Content-Length", HeaderValue::from_str(format!("{}", object.len()).as_str())?);

    let _cb = self.client_config().create_http_client_builder()?;

    Ok(())
  }
}
