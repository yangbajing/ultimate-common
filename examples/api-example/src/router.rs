use std::future::Future;

use axum::Router;
use ultimate_web::server::init_server;

use crate::{app::AppState, auth::auth_routes, user::user_routes};

fn new_api_router(app_state: AppState) -> Router {
  Router::new().nest("/v1/user", user_routes()).nest("/auth", auth_routes()).with_state(app_state)
}

pub fn start_router(app: AppState) -> impl Future<Output = ultimate::Result<()>> {
  let conf = app.config_state.ultimate_config_clone();
  let router = new_api_router(app);
  init_server(conf, router)
}
