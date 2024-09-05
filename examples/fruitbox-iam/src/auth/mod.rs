mod auth_serv;
mod grpc;
mod model;
pub mod utils;
mod web;

use auth_serv::AuthServ;
pub use grpc::auth_grpc_server;
use model::*;
pub use web::auth_routes;
