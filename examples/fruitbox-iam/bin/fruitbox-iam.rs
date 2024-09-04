use fruitbox_iam::{
  app::new_app_state,
  grpc::{auth::auth_service_server::AuthServiceServer, AuthServiceImpl},
  router::new_api_router,
};
use tonic::transport::Server;
use ultimate::Result;
use ultimate_web::server::init_server;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[tokio::main]
async fn main() -> Result<()> {
  let app = new_app_state().await?;
  let router = new_api_router(app.clone());

  let grpc_addr = "0.0.0.0:8889".parse()?;
  let app2 = app.clone();
  tokio::spawn(async move {
    Server::builder().add_service(AuthServiceServer::new(AuthServiceImpl::new(app2))).serve(grpc_addr).await.unwrap();
  });

  init_server(app.ultimate_config(), router).await?;
  Ok(())
}
