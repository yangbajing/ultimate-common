use derive_more::derive::Constructor;
use tonic::{service::interceptor::InterceptedService, Request, Response, Status};

use crate::{
  endpoint::grpc::interceptor::auth_interceptor,
  proto::v1::{
    create_user_reply,
    user_service_server::{UserService, UserServiceServer},
    CreateUserReply, CreateUserRequest, DeleteUserReply, FindUserRequest, PageUserReply, PageUserRequest,
    UpdateUserRequest, UserDto, UserReply,
  },
};

use super::user_serv;

#[derive(Constructor)]
pub struct UserServiceImpl;

#[tonic::async_trait]
impl UserService for UserServiceImpl {
  async fn find(&self, request: Request<FindUserRequest>) -> Result<Response<UserReply>, Status> {
    let ctx = request.extensions().try_into()?;
    let user = user_serv::find_option_by_id(ctx, request.get_ref().id).await?.map(UserDto::from);
    Ok(Response::new(UserReply { user }))
  }

  async fn create(&self, request: Request<CreateUserRequest>) -> Result<Response<CreateUserReply>, Status> {
    let (_, exts, req) = request.into_parts();
    let returining_payload = req.returining_payload;

    let ctx = (&exts).try_into()?;
    let id = user_serv::create(ctx, req.try_into()?).await?;

    let reply = if returining_payload {
      let u = user_serv::find_by_id(ctx, id).await?;
      create_user_reply::Reply::User(u.into())
    } else {
      create_user_reply::Reply::Id(id)
    };
    Ok(Response::new(CreateUserReply { reply: Some(reply) }))
  }

  async fn update(&self, request: Request<UpdateUserRequest>) -> Result<Response<UserReply>, Status> {
    let (_, exts, req) = request.into_parts();
    let ctx = (&exts).try_into()?;
    let id = req.id;
    let returning_payload = req.returning_payload;

    user_serv::update_by_id(ctx, id, req.try_into()?).await?;

    let user = if returning_payload {
      let u = user_serv::find_option_by_id(ctx, id).await?;
      u.map(UserDto::from)
    } else {
      None
    };
    Ok(Response::new(UserReply { user }))
  }

  async fn page(&self, request: Request<PageUserRequest>) -> Result<Response<PageUserReply>, Status> {
    let (_, exts, req) = request.into_parts();
    let ctx = (&exts).try_into()?;

    let page = user_serv::page(ctx, req.into()).await?;
    Ok(Response::new(page.into()))
  }

  async fn delete(&self, request: Request<FindUserRequest>) -> Result<Response<DeleteUserReply>, Status> {
    let ctx = request.extensions().try_into()?;
    let id = request.get_ref().id;
    user_serv::delete_by_id(ctx, id).await?;
    Ok(Response::new(DeleteUserReply {}))
  }
}

pub fn user_svc(
) -> InterceptedService<UserServiceServer<UserServiceImpl>, fn(Request<()>) -> Result<Request<()>, Status>> {
  UserServiceServer::with_interceptor(UserServiceImpl::new(), auth_interceptor)
}
