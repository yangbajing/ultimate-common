pub mod interceptor;

use futures::{Future, TryFutureExt};
use tonic::transport::Server;
use ultimate::DataError;

use crate::{app::AppState, auth, user::grpc::UserServiceImpl, v1::user_service_server::UserServiceServer};

use self::interceptor::auth_interceptor;

pub fn grpc_serve(app: AppState) -> ultimate::Result<impl Future<Output = std::result::Result<(), DataError>>> {
  let grpc_addr = app.ultimate_config().grpc().server_addr.parse()?;
  let serve = Server::builder()
    .add_service(UserServiceServer::with_interceptor(UserServiceImpl::new(), auth_interceptor))
    .add_service(auth::auth_grpc_server(app.clone()))
    .serve(grpc_addr)
    .map_err(DataError::from);
  Ok(serve)
}
