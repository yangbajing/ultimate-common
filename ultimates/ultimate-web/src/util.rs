use axum::extract::Query;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::Json;
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::{Authorization, HeaderMapExt};
use serde::de::DeserializeOwned;
use ultimate::configuration::model::SecruityConfig;
use ultimate::ctx::Session;
use ultimate::error::DataError;
use ultimate::security::{AccessToken, SecurityUtils};
use ultimate_common::time;

use crate::error::AppError;
use crate::AppResult;

pub fn unauthorized_app_error(msg: impl Into<String>) -> (StatusCode, Json<AppError>) {
  (StatusCode::UNAUTHORIZED, Json(AppError::new(msg).with_err_code(401)))
}

/// 从 Http Request Parts 中获取 [SessionCtx]
pub fn extract_session(parts: &Parts, sc: &SecruityConfig) -> Result<Session, DataError> {
  let req_time = time::now();

  let token = if let Some(Authorization(bearer)) = parts.headers.typed_get::<Authorization<Bearer>>() {
    bearer.token().to_string()
  } else if let Ok(at) = Query::<AccessToken>::try_from_uri(&parts.uri) {
    at.0.access_token
  } else {
    return Err(DataError::unauthorized("Missing token"));
  };

  let (payload, _) =
    SecurityUtils::decrypt_jwt(sc.pwd(), &token).map_err(|_e| DataError::unauthorized("Failed decode jwt"))?;

  Session::try_from_jwt_payload(&payload, Some(req_time))
}

pub fn opt_to_app_result<T>(opt: Option<T>) -> AppResult<T>
where
  T: DeserializeOwned,
{
  if let Some(v) = opt {
    Ok(Json(v))
  } else {
    Err(AppError::new_with_code(404, "Not found."))
  }
}
