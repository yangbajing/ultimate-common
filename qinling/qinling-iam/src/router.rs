use axum::Router;

use crate::{application::Application, iam};

pub fn router(app_state: Application) -> Router {
  iam::routes().with_state(app_state)
}
