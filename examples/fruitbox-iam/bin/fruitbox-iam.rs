use fruitbox_iam::{app::get_app_state, endpoint::grpc::grpc_serve};
use ultimate::Result;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[tokio::main]
async fn main() -> Result<()> {
  let app = get_app_state();

  // let (ret_grpc, ret_router) =
  //   tokio::join!(tokio::spawn(grpc_serve(app.clone())?), tokio::spawn(start_router(app.clone())));

  // ret_grpc??;
  // ret_router??;

  grpc_serve(app.clone())?.await?;

  Ok(())
}
