use std::sync::Arc;

use axum::{
  async_trait,
  extract::FromRequestParts,
  http::{request::Parts, HeaderMap, StatusCode},
  Json,
};
use derive_getters::Getters;
use tonic::{metadata::MetadataMap, Status};
use ultimate::{ctx::Ctx, security::SecurityUtils, DataError};
use ultimate_common::time::now_utc;
use ultimate_db::ModelManager;
use ultimate_web::{extract_session, AppError};

use crate::app::{get_app_state, AppState};

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

impl TryFrom<&MetadataMap> for CtxW {
  type Error = Status;
  fn try_from(metadata: &MetadataMap) -> core::result::Result<Self, Status> {
    let app = get_app_state();
    let sc = app.ultimate_config().security();
    let req_time = now_utc();

    let token = extract_token(metadata)?;

    let (payload, _) =
      SecurityUtils::decrypt_jwt(sc.pwd(), &token).map_err(|_e| DataError::unauthorized("Failed decode jwt"))?;

    let req_meta = RequestMetadata::from(metadata);
    let ctx = Ctx::try_from_jwt_payload(&payload, Some(req_time))?;
    Ok(CtxW::new(app, ctx, Arc::new(req_meta)))
  }
}

fn extract_token(metadata: &MetadataMap) -> Result<String, Status> {
  let auth_header =
    metadata.get("authorization").ok_or_else(|| Status::unauthenticated("Missing authorization header"))?;

  let auth_str = auth_header.to_str().map_err(|_| Status::unauthenticated("Invalid authorization header"))?;

  if !auth_str.starts_with("Bearer ") {
    return Err(Status::unauthenticated("Invalid token type"));
  }

  Ok(auth_str[7..].to_string())
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

impl From<&MetadataMap> for RequestMetadata {
  fn from(metadata: &MetadataMap) -> Self {
    let app_ver = metadata.get(X_APP_VERSION).map(|v| v.to_str().unwrap_or("").to_string()).unwrap_or_default();
    let dev_id = metadata.get(X_DEVICE_ID).map(|v| v.to_str().unwrap_or("").to_string()).unwrap_or_default();
    Self { app_ver, dev_id }
  }
}
