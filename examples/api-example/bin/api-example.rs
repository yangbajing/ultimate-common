use api_example::{router::new_api_router, state::new_app_state};
use ultimate_web::server::init_server;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[tokio::main]
async fn main() -> ultimate::Result<()> {
  let state = new_app_state().await?;
  let conf = state.ultimate_config();
  let router = new_api_router(state.clone());

  init_server(conf, router).await?;
  Ok(())
}
