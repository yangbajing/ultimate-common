use anyhow::Result;
use config::Config;
use tracing::info;
use tracing_subscriber::{fmt, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};
use wework_sdk::{
  client::{WeworkClient, WeworkContacts, WeworkMessages},
  model::msg::{Message, MsgText, MsgType},
  WeworkConfig,
};

#[tokio::main]
async fn main() -> Result<()> {
  std::env::set_var("RUST_LOG", "info,wework_sdk=debug");
  tracing_subscriber::registry().with(fmt::layer()).init();

  let c: WeworkConfig = Config::builder()
    .add_source(config::File::with_name("crates/wework-sdk/.app-local.toml"))
    .build()?
    .get("wework")?;

  let client = WeworkClient::create(c).await?;

  let mut req_msg = Message::builder()
    .touser("yangjing")
    .agentid(client.config().agent_id())
    .msgtype(MsgType::text)
    .text(MsgText::builder().content("我再试一下").build())
    .build();
  let ret = client.send_app_msg(&req_msg).await?;
  info!("Send message: {}", ret);

  let userid = "yangjing";
  let user = client.get_user(userid).await?;
  info!("Get user is {:?}", user);

  tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

  req_msg.text = Some(MsgText::builder().content("我还要再试一下下").build());
  let ret = client.send_app_msg(&req_msg).await?;
  info!("Send message: {}", ret);
  Ok(())
}
