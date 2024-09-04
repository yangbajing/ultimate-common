use axum::Router;

use crate::{app::AppState, auth::auth_routes, user::user_routes};

pub fn new_api_router(app_state: AppState) -> Router {
  Router::new()
    .nest("/api/v1", Router::new().nest("/user", user_routes()).nest("/auth", auth_routes()))
    .with_state(app_state)
}
