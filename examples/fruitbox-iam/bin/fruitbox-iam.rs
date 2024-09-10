use fruitbox_iam::app::get_app_state;
use tracing::info;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

fn main() -> ultimate::Result<()> {
  let app = get_app_state();
  let ret = app.runtime().block_on(fruitbox_iam::run());
  info!("Application run finished: {:?}", ret);
  Ok(())
}
