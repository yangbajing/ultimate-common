use derive_more::derive::Constructor;
use tonic::{service::interceptor::InterceptedService, Response, Status};
use tracing::debug;
use ultimate_api::v1::Page;

use crate::{
  app::AppState,
  ctx::CtxW,
  endpoint::grpc::interceptor::auth_interceptor,
  v1::user_service_server::{UserService, UserServiceServer},
};

use super::UserServ;

#[derive(Constructor)]
pub struct UserServiceImpl;

#[tonic::async_trait]
impl UserService for UserServiceImpl {
  async fn find(
    &self,
    request: tonic::Request<crate::v1::FindUserRequest>,
  ) -> Result<tonic::Response<crate::v1::UserDto>, tonic::Status> {
    let ctx = request.extensions().get::<CtxW>().ok_or_else(|| Status::unauthenticated("Missing token"))?;
    let u = UserServ::find_by_id(ctx, request.get_ref().id).await?;
    Ok(Response::new(u.into()))
  }

  async fn create(
    &self,
    request: tonic::Request<crate::v1::CreateUserRequest>,
  ) -> Result<tonic::Response<crate::v1::UserDto>, tonic::Status> {
    // 实现 create 方法
    todo!()
  }

  async fn update(
    &self,
    request: tonic::Request<crate::v1::UpdateUserRequest>,
  ) -> Result<tonic::Response<crate::v1::UserDto>, tonic::Status> {
    // 实现 update 方法
    todo!()
  }

  async fn page(
    &self,
    request: tonic::Request<crate::v1::PageUserRequest>,
  ) -> Result<tonic::Response<crate::v1::PageUserReply>, tonic::Status> {
    debug!("请求是: {:?}", request);

    let pagination = request.get_ref().pagination.clone().unwrap_or_default();
    let total_size = 0; // 这里需要实际计算总数
    Ok(Response::new(crate::v1::PageUserReply { page: Some(Page::new(&pagination, total_size)), records: vec![] }))
  }
}
