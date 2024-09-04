use fruitbox_iam::{app::new_app_state, router::new_api_router};
use ultimate::Result;
use ultimate_web::server::init_server;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[tokio::main]
async fn main() -> Result<()> {
  let state = new_app_state().await?;
  let router = new_api_router(state.clone());

  init_server(state.ultimate_config(), router).await?;
  Ok(())
}
