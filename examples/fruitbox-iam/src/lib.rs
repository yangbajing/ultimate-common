pub mod app;
mod auth;
mod ctx;
mod endpoint;
mod permission;
mod proto;
mod role;
mod user;

use endpoint::grpc::grpc_serve;

pub async fn run() -> ultimate::Result<()> {
  grpc_serve()?.await
}
