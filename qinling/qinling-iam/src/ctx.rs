use axum::{async_trait, extract::FromRequestParts, http::request::Parts, Json};
use derive_getters::Getters;
use hyper::StatusCode;
use typed_builder::TypedBuilder;
use ultimate::{configuration::UltimateConfig, ctx::Session, Result};
use ultimate_db::ModelManager;
use ultimate_web::{extract_session, AppError};

use crate::application::{new_application, Application};

#[derive(TypedBuilder, Getters)]
pub struct Ctx {
  session: Session,
  state: Application,
}

impl Ctx {
  pub fn mm(&self) -> &ModelManager {
    self.state().db_state().mm()
  }

  pub fn ultimate_config(&self) -> &UltimateConfig {
    self.state().config_state().ultimate_config()
  }

  #[allow(unused)]
  pub(crate) async fn load_on_test() -> Result<Self> {
    let ctx = Self { session: Session::new_root(), state: new_application().await? };
    Ok(ctx)
  }
}

#[async_trait]
impl FromRequestParts<Application> for Ctx {
  type Rejection = (StatusCode, Json<AppError>);

  async fn from_request_parts(parts: &mut Parts, state: &Application) -> core::result::Result<Self, Self::Rejection> {
    match extract_session(parts, state.ultimate_config().security()) {
      Ok(session) => Ok(Ctx::builder().session(session).state(state.clone()).build()),
      Err(e) => Err((StatusCode::UNAUTHORIZED, Json(e.into()))),
    }
  }
}
