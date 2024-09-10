use futures::{Future, TryFutureExt};
use tonic::transport::Server;
use ultimate::DataError;

use crate::{app::get_app_state, auth::auth_svc, permission::permission_svc, role::role_svc, user::grpc::user_svc};

pub fn grpc_serve() -> ultimate::Result<impl Future<Output = std::result::Result<(), DataError>>> {
  let grpc_conf = get_app_state().ultimate_config().grpc();
  let grpc_addr = grpc_conf.server_addr.parse()?;

  let mut b = Server::builder();

  #[cfg(feature = "tonic-web")]
  {
    b = b.accept_http1(true).layer(tonic_web::GrpcWebLayer::new());
  }

  let mut svc = b.add_service(permission_svc()).add_service(role_svc()).add_service(user_svc()).add_service(auth_svc());

  #[cfg(feature = "tonic-reflection")]
  {
    let service = tonic_reflection::server::Builder::configure()
      .register_encoded_file_descriptor_set(crate::pb::fruitbox_iam::v1::FILE_DESCRIPTOR_SET)
      .build_v1()
      .unwrap();
    svc = svc.add_service(service);
  }

  // let s = svc.into_service();

  let serve = svc.serve(grpc_addr).map_err(DataError::from);
  Ok(serve)
}
