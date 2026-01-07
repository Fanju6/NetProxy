use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DnsConfig {
    #[serde(default)]
    pub tag: String,
    #[serde(default)]
    pub query_strategy: String,
    #[serde(default)]
    pub disable_fallback_if_match: bool,
    #[serde(default)]
    pub hosts: HashMap<String, String>,
    #[serde(default)]
    pub servers: Vec<DnsServer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DnsServer {
    Simple(String),
    Complex {
        address: String,
        #[serde(default)]
        domains: Vec<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoutingConfig {
    #[serde(default)]
    pub domain_strategy: String,
    #[serde(default)]
    pub rules: Vec<RoutingRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoutingRule {
    #[serde(rename = "type")]
    pub rule_type: String,
    #[serde(default)]
    pub inbound_tag: Vec<String>,
    pub outbound_tag: String,
    #[serde(default)]
    pub domain: Vec<String>,
    #[serde(default)]
    pub ip: Vec<String>,
    #[serde(default)]
    pub port: Option<String>,
}
