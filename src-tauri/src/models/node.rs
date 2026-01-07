use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeInfo {
    pub name: String,
    pub protocol: String,
    pub address: String,
    pub port: u16,
    pub file_name: String,
}
