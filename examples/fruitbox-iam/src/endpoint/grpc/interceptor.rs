use tonic::{Request, Status};

use crate::ctx::CtxW;

pub fn auth_interceptor(mut request: Request<()>) -> Result<Request<()>, Status> {
  let ctx: CtxW = request.metadata().try_into()?;

  // 将 Ctx 对象存储在 request 的 extensions 中
  request.extensions_mut().insert(ctx);

  Ok(request)
}
