use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProxyStatus {
    pub running: bool,
    pub system_proxy: bool,
    pub current_node: Option<String>,
    pub port: u16,
}

impl Default for ProxyStatus {
    fn default() -> Self {
        Self {
            running: false,
            system_proxy: false,
            current_node: None,
            port: 10808,
        }
    }
}
