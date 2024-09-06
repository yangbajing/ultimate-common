use derive_more::derive::Constructor;
use tonic::{Request, Response, Status};

use crate::{
  app::AppState,
  v1::{
    auth_service_server::{AuthService, AuthServiceServer},
    SigninReplay, SigninRequest,
  },
};

use super::auth_serv;

#[derive(Constructor)]
pub struct AuthServiceImpl {
  app: AppState,
}

#[tonic::async_trait]
impl AuthService for AuthServiceImpl {
  async fn signin(&self, request: Request<SigninRequest>) -> Result<Response<SigninReplay>, Status> {
    let res = auth_serv::signin(&self.app, request.into_inner()).await?;
    Ok(Response::new(res))
  }
}

pub fn auth_grpc_server(app: AppState) -> AuthServiceServer<AuthServiceImpl> {
  AuthServiceServer::new(AuthServiceImpl::new(app))
}
