use derive_getters::Getters;
use typed_builder::TypedBuilder;
use ultimate::configuration::UltimateConfig;

use ultimate::Result;
use ultimate::{configuration::ConfigState, starter};
use ultimate_db::{DbState, ModelManager};

#[derive(Clone, TypedBuilder, Getters)]
pub struct Application {
  db_state: DbState,
  config_state: ConfigState,
}

impl Application {
  pub fn mm(&self) -> &ModelManager {
    self.db_state().mm()
  }

  pub fn ultimate_config(&self) -> &UltimateConfig {
    self.config_state().ultimate_config()
  }
}

pub async fn new_application() -> Result<Application> {
  let config_state = starter::load_and_init();
  let c = config_state.ultimate_config();

  let db_state = DbState::from_config(c.db()).await?;

  let state = Application::builder().config_state(config_state).db_state(db_state).build();
  Ok(state)
}
