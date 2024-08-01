use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct GrpcConfig {
    pub enable: bool,

    pub server_addr: String,

    pub plaintext: bool,

    pub clients: HashMap<String, GrpcClientConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrpcClientConfig {
    pub addr: String,

    pub plaintext: bool,
}
