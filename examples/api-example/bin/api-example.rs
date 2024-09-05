use api_example::{app::new_app_state, router::start_router};

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[tokio::main]
async fn main() -> ultimate::Result<()> {
  let app = new_app_state()?;

  start_router(app).await?;
  Ok(())
}
