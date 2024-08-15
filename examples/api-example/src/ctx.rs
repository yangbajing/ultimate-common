use std::sync::Arc;

use axum::{
  async_trait,
  extract::FromRequestParts,
  http::{request::Parts, HeaderMap, StatusCode},
  Json,
};
use derive_getters::Getters;
use ultimate::ctx::Ctx;
use ultimate_db::ModelManager;
use ultimate_web::{extract_session, AppError};

use crate::state::AppState;

static X_APP_VERSION: &str = "X-APP-VARSION";
static X_DEVICE_ID: &str = "X-DEVICE-ID";

#[derive(Clone, Getters)]
pub struct CtxW {
  ctx: Ctx,
  mm: ModelManager,
  req_meta: Arc<RequestMetadata>,
}
impl CtxW {
  pub fn new(state: &AppState, ctx: Ctx, req_meta: Arc<RequestMetadata>) -> Self {
    let mm = state.mm().clone().with_ctx(ctx.clone());
    Self { ctx, mm, req_meta }
  }
}

#[async_trait]
impl FromRequestParts<AppState> for CtxW {
  type Rejection = (StatusCode, Json<AppError>);

  async fn from_request_parts(parts: &mut Parts, state: &AppState) -> core::result::Result<Self, Self::Rejection> {
    match extract_session(parts, state.ultimate_config().security()) {
      Ok(ctx) => Ok(CtxW::new(state, ctx, Arc::new(RequestMetadata::from(&parts.headers)))),
      Err(e) => Err((StatusCode::UNAUTHORIZED, Json(e.into()))),
    }
  }
}

#[derive(Clone, Default)]
pub struct RequestMetadata {
  app_ver: String,
  dev_id: String,
}

impl RequestMetadata {
  pub fn app_ver(&self) -> &str {
    self.app_ver.as_str()
  }

  pub fn dev_id(&self) -> &str {
    self.dev_id.as_str()
  }
}

impl From<&HeaderMap> for RequestMetadata {
  fn from(headers: &HeaderMap) -> Self {
    let app_ver = headers.get(X_APP_VERSION).map(|v| v.to_str().unwrap_or("").to_string()).unwrap_or_default();
    let dev_id = headers.get(X_DEVICE_ID).map(|v| v.to_str().unwrap_or("").to_string()).unwrap_or_default();
    Self { app_ver, dev_id }
  }
}
