use axum::Router;

use crate::application::Application;

pub mod auth;
pub mod permission;
pub(in crate::iam) mod repos;
pub mod role;
pub mod user;

pub fn routes() -> Router<Application> {
    Router::new()
        .nest("/auth", auth::web::routes())
        .nest("/api/user", user::web::routes())
        .nest("/api/role", role::web::routes())
}
