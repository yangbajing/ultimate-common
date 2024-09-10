use tonic::{Request, Response, Status};

use crate::{
  app::get_app_state,
  proto::v1::{
    auth_service_server::{AuthService, AuthServiceServer},
    SigninReplay, SigninRequest,
  },
};

use super::auth_serv;

pub struct AuthServiceImpl;

#[tonic::async_trait]
impl AuthService for AuthServiceImpl {
  async fn signin(&self, request: Request<SigninRequest>) -> Result<Response<SigninReplay>, Status> {
    let app = get_app_state();
    let res = auth_serv::signin(app, request.into_inner()).await?;
    Ok(Response::new(res))
  }
}

pub fn auth_svc() -> AuthServiceServer<AuthServiceImpl> {
  AuthServiceServer::new(AuthServiceImpl)
}
