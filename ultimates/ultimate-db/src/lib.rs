use ultimate::configuration::model::DbConfig;

pub mod acs;
pub mod auth;
pub mod base;
mod error;
mod model_manager;
pub mod modql_utils;
pub mod store;
mod types;

pub use error::{Error, Result};
pub use model_manager::*;
pub use types::*;

#[derive(Clone)]
pub struct DbState {
    mm: ModelManager,
}

impl DbState {
    pub async fn from_config(db: &DbConfig) -> Result<Self> {
        let mm = ModelManager::new(db).await?;
        Ok(DbState { mm })
    }

    pub fn mm(&self) -> &ModelManager {
        &self.mm
    }
}
