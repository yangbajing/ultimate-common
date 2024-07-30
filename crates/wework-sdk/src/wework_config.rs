use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WeworkConfig {
    corp_id: String,
    agent_id: i32,
    secret: String,
}

impl WeworkConfig {
    pub fn corp_id(&self) -> &str {
        &self.corp_id
    }
    pub fn agent_id(&self) -> i32 {
        self.agent_id
    }
    pub fn secret(&self) -> &str {
        &self.secret
    }
}
