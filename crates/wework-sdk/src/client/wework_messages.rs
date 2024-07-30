use async_trait::async_trait;

use crate::{api_url, model::msg::Message, Result};

use super::WeworkClient;

#[async_trait]
pub trait WeworkMessages {
    /// 发送应用消息
    async fn send_app_msg(&self, req: &Message) -> Result<serde_json::Value>;
}

static MESSAGE_SEND: &str = api_url!("/message/send");

#[async_trait]
impl WeworkMessages for WeworkClient {
    async fn send_app_msg(&self, req: &Message) -> Result<serde_json::Value> {
        let rb = self.client.post(MESSAGE_SEND).json(req);
        let ret = self.send(rb).await?;
        Ok(ret)
    }
}
