use qinling_iam::{application::new_application, router::router};
use tracing::info;
use ultimate_web::server::init_server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let state = new_application().await?;
  let conf = state.config_state().ultimate_config();

  info!("Starting Qinling IAM Service");

  init_server(conf, router(state.clone())).await?;
  // let (web_ret, grpc_ret) = tokio::join!(
  // init_server(conf.web(), router(state.clone())),
  // start_rpc_server(state.clone())
  // );
  // web_ret?;
  // grpc_ret?;
  Ok(())
}
