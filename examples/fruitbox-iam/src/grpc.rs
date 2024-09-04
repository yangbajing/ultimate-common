use auth::{auth_service_server::AuthService, SigninReplay, SigninRequest};
use tonic::{Request, Response, Status};

use crate::app::AppState;

pub mod auth {
  tonic::include_proto!("fruitbox.auth");
}

pub struct AuthServiceImpl {
  app: AppState,
}

impl AuthServiceImpl {
  pub fn new(app: AppState) -> Self {
    Self { app }
  }
}

#[tonic::async_trait]
impl AuthService for AuthServiceImpl {
  async fn signin(&self, request: Request<SigninRequest>) -> Result<Response<SigninReplay>, Status> {
    println!("Incoming request is: {:?}", request);

    Ok(Response::new(SigninReplay { token: "token".to_string(), token_type: "Bearer".to_string() }))
  }
}
