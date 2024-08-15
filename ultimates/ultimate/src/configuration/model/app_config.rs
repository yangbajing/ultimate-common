use serde::{Deserialize, Serialize};

use crate::RunMode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
  run_mode: RunMode,
  name: String,
}

impl AppConfig {
  pub fn run_mode(&self) -> &RunMode {
    &self.run_mode
  }

  pub fn name(&self) -> &str {
    &self.name
  }
}
