use axum::Router;

use crate::{auth::auth_routes, state::AppState, user::user_routes};

pub fn new_api_router(app_state: AppState) -> Router {
  Router::new().nest("/v1/user", user_routes()).nest("/auth", auth_routes()).with_state(app_state)
}
