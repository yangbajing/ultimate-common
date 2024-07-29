use josekit::jwt::JwtPayload;

use ultimate_common::time::{self, Duration, OffsetDateTime, UtcDateTime};

use crate::error::DataError;

/// 会话上下文。
/// 此处 clone 的成本很低，若后续数据多的话可以使用 Arc 加 Wrapper 模式来降低数据复制的成本
#[derive(Clone, Debug)]
pub struct Session {
  /// 会话用户 ID
  user_id: i64,
  /// 请求时时间
  req_time: OffsetDateTime,
  /// 会话过期时间
  expires_at: OffsetDateTime,
}

impl Session {
  pub fn new(user_id: i64, req_time: OffsetDateTime, expires_at: OffsetDateTime) -> Self {
    Self { user_id, req_time, expires_at }
  }

  pub fn new_root() -> Self {
    let req_time = time::now();
    let expires_at = req_time + Duration::minutes(30);
    Self::new(0, req_time, expires_at)
  }

  pub fn user_id(&self) -> i64 {
    self.user_id
  }

  pub fn req_time(&self) -> &OffsetDateTime {
    &self.req_time
  }

  pub fn with_expires_at(mut self, expires_at: OffsetDateTime) -> Self {
    self.expires_at = expires_at;
    self
  }

  pub fn try_from_jwt_payload(payload: &JwtPayload, req_time: Option<OffsetDateTime>) -> Result<Self, DataError> {
    let req_time = req_time.unwrap_or_else(time::now);

    let sub = payload.subject().ok_or_else(|| DataError::unauthorized("'sub' of jwt missing"))?;

    let user_id: i64 = sub.parse().map_err(|_| DataError::unauthorized(format!("<sub:{sub}> invalid")))?;

    let expires_at: UtcDateTime = if let Some(st) = payload.expires_at() {
      let expires_at = st.into();
      if expires_at < time::now_utc() {
        return Err(DataError::unauthorized("The token expired"));
      }
      expires_at
    } else {
      OffsetDateTime::MAX_UTC
    };

    Ok(Session::new(user_id, req_time, expires_at.fixed_offset()))
  }
}

impl TryFrom<JwtPayload> for Session {
  type Error = DataError;

  fn try_from(payload: JwtPayload) -> std::result::Result<Self, Self::Error> {
    Session::try_from_jwt_payload(&payload, Some(time::now()))
  }
}
