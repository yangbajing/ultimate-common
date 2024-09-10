use futures::{Future, TryFutureExt};
use tonic::transport::Server;
use ultimate::DataError;

use crate::{app::get_app_state, auth::auth_svc, permission::permission_svc, role::role_svc, user::grpc::user_svc};

pub fn grpc_serve() -> ultimate::Result<impl Future<Output = std::result::Result<(), DataError>>> {
  let grpc_conf = get_app_state().ultimate_config().grpc();
  let grpc_addr = grpc_conf.server_addr.parse()?;

  let serve = Server::builder()
    .add_service(permission_svc())
    .add_service(role_svc())
    .add_service(user_svc())
    .add_service(auth_svc())
    .serve(grpc_addr)
    .map_err(DataError::from);
  Ok(serve)
}
