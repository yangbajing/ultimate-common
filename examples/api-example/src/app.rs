use std::sync::Arc;

use derive_getters::Getters;
use typed_builder::TypedBuilder;
use ultimate::{
  configuration::{ConfigState, Configuration},
  ctx::Ctx,
  starter,
};
use ultimate_db::{DbState, ModelManager};

use crate::ctx::{CtxW, RequestMetadata};

#[derive(Clone, TypedBuilder, Getters)]
pub struct AppState {
  pub config_state: ConfigState,
  pub db_state: DbState,
}

impl AppState {
  pub fn configuration(&self) -> &Configuration {
    self.config_state().configuration()
  }

  pub fn mm(&self) -> &ModelManager {
    self.db_state().mm()
  }

  pub fn create_root_ctx(&self) -> crate::ctx::CtxW {
    CtxW::new(self, Ctx::new_root(), Arc::new(RequestMetadata::default()))
  }

  pub fn create_super_admin_ctx(&self) -> crate::ctx::CtxW {
    CtxW::new(self, Ctx::new_super_admin(), Arc::new(RequestMetadata::default()))
  }
}

pub async fn new_app_state() -> ultimate::Result<AppState> {
  let config = starter::load_and_init();
  let db = DbState::from_config(config.configuration().db()).await?;
  let app = AppState::builder().config_state(config).db_state(db).build();
  Ok(app)
}
