use async_trait::async_trait;
use reqwest::{
  multipart::{self, Part},
  Client, RequestBuilder,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::{
  api_url,
  model::webhook::{WebhookMsg, WebhookUploadType, WebhookUploaded},
  utils, Result,
};

use super::WeworkClient;

#[async_trait]
pub trait WeworkWebhooks {
  /// 发送消息
  /// 消息类型及数据格式
  async fn webhook_send(&self, key: &str, msg: &WebhookMsg) -> Result<()>;

  /// 文件上传接口
  /// https://developer.work.weixin.qq.com/document/path/99110#文件上传接口
  async fn webhook_upload(&self, key: &str, typ: WebhookUploadType, part: Part) -> Result<WebhookUploaded>;

  // async fn _send_webhook<T: DeserializeOwned>(&self, rb: RequestBuilder) -> Result<T>;
}

static SEND: &str = api_url!("/webhook/send");
static UPLOAD: &str = api_url!("/webhook/upload");

#[async_trait]
impl WeworkWebhooks for WeworkClient {
  async fn webhook_send(&self, key: &str, msg: &WebhookMsg) -> Result<()> {
    let rb = self.client.post(SEND).query(&[("key", key)]).json(msg);
    self._send_direct::<Value>(rb).await?;
    Ok(())
  }

  async fn webhook_upload(&self, key: &str, typ: WebhookUploadType, part: Part) -> Result<WebhookUploaded> {
    let form = multipart::Form::new().part("media", part);
    let rb = self.client.post(UPLOAD).query(&[("key", key), ("type", typ.as_ref())]).multipart(form);
    self._send_direct(rb).await
  }
}

pub struct WeworkWebhookClient {
  client: Client,
}

impl WeworkWebhookClient {
  pub fn new(client: Client) -> Self {
    Self { client }
  }

  async fn _send_webhook<T: DeserializeOwned>(&self, rb: RequestBuilder) -> Result<T> {
    let request = rb.build()?;
    let json = self.client.execute(request).await?.json().await?;
    let ret = utils::extract(json)?;
    Ok(ret)
  }
}

#[async_trait]
impl WeworkWebhooks for WeworkWebhookClient {
  async fn webhook_send(&self, key: &str, msg: &WebhookMsg) -> Result<()> {
    let rb = self.client.post(SEND).query(&[("key", key)]).json(msg);
    self._send_webhook(rb).await?;
    Ok(())
  }

  async fn webhook_upload(&self, key: &str, typ: WebhookUploadType, part: Part) -> Result<WebhookUploaded> {
    let form = multipart::Form::new().part("media", part);
    let rb = self.client.post(UPLOAD).query(&[("key", key), ("type", typ.as_ref())]).multipart(form);
    self._send_webhook(rb).await
  }
}
