use derive_more::derive::Constructor;
use tonic::{Request, Response, Status};

use crate::{
  app::AppState,
  v1::{
    auth_service_server::{AuthService, AuthServiceServer},
    SigninReplay, SigninRequest,
  },
};

use super::auth_serv::AuthServ;

#[derive(Constructor)]
pub struct AuthServiceImpl {
  auth_serv: AuthServ,
}

#[tonic::async_trait]
impl AuthService for AuthServiceImpl {
  async fn signin(&self, request: Request<SigninRequest>) -> Result<Response<SigninReplay>, Status> {
    let res = self.auth_serv.signin(request.into_inner()).await?;
    Ok(Response::new(res))
  }
}

pub fn auth_grpc_server(app: AppState) -> AuthServiceServer<AuthServiceImpl> {
  AuthServiceServer::new(AuthServiceImpl::new(AuthServ::new(app)))
}
