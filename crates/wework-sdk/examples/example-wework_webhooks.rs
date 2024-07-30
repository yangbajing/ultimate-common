use anyhow::Result;
use tracing_subscriber::{fmt, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};
use wework_sdk::{
    client::{WeworkWebhookClient, WeworkWebhooks},
    model::{
        msg::{MsgMarkdown, MsgType},
        webhook::WebhookMsg,
    },
};

/// https://qyapi.weixin.qq.com/cgi-bin/webhook/send?key=1ba8ea09-d9cd-4bda-a964-1fb8bfa74aaf
#[tokio::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "info,wework_sdk=debug");
    tracing_subscriber::registry().with(fmt::layer()).init();

    let client = WeworkWebhookClient::new(reqwest::Client::new());
    let key = "1ba8ea09-d9cd-4bda-a964-1fb8bfa74aaf";
    let markdown = MsgMarkdown::new(
        r#"# 测试消息

    大家不要紧张，这是测试数据。我接下来要**@几个人** <@jiangqiang> <@wuchuanjiang>

    1. 第一行 <@yangjing>
    2. 第二行 <@008139>
    3. 第三行"#,
    );
    let msg = WebhookMsg::builder().msgtype(MsgType::markdown).markdown(markdown).build();

    client.webhook_send(key, &msg).await?;

    Ok(())
}
