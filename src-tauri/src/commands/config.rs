use crate::models::{DnsConfig, RoutingConfig};
use crate::utils::paths;
use std::fs;
use std::io::{Read, Seek, SeekFrom};

#[tauri::command]
pub fn get_dns_config() -> Result<DnsConfig, String> {
    let config_path = paths::get_confdir().join("02_dns.json");
    let content = fs::read_to_string(&config_path).map_err(|e| e.to_string())?;

    let wrapper: serde_json::Value = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    let dns = wrapper.get("dns").ok_or("Missing dns field")?;

    serde_json::from_value(dns.clone()).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_dns_config(config: DnsConfig) -> Result<(), String> {
    let config_path = paths::get_confdir().join("02_dns.json");

    let wrapper = serde_json::json!({ "dns": config });
    let content = serde_json::to_string_pretty(&wrapper).map_err(|e| e.to_string())?;

    fs::write(&config_path, content).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_routing_config() -> Result<RoutingConfig, String> {
    let config_path = paths::get_confdir().join("03_routing.json");
    let content = fs::read_to_string(&config_path).map_err(|e| e.to_string())?;

    let wrapper: serde_json::Value = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    let routing = wrapper.get("routing").ok_or("Missing routing field")?;

    serde_json::from_value(routing.clone()).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_routing_config(config: RoutingConfig) -> Result<(), String> {
    let config_path = paths::get_confdir().join("03_routing.json");

    let wrapper = serde_json::json!({ "routing": config });
    let content = serde_json::to_string_pretty(&wrapper).map_err(|e| e.to_string())?;

    fs::write(&config_path, content).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_xray_log(log_type: String) -> Result<String, String> {
    let filename = if log_type == "access" {
        "access.log"
    } else {
        "error.log"
    };

    let log_path = paths::get_data_dir().join("logs").join(filename);
    if !log_path.exists() {
        return Ok(String::new());
    }

    let mut file = fs::File::open(log_path).map_err(|e| e.to_string())?;
    let metadata = file.metadata().map_err(|e| e.to_string())?;
    let size = metadata.len();

    // 只读取最后 50KB
    let max_bytes = 50 * 1024;
    if size > max_bytes {
        file.seek(SeekFrom::End(-(max_bytes as i64)))
            .map_err(|e| e.to_string())?;
    }

    let mut content = String::new();
    file.read_to_string(&mut content)
        .map_err(|e| e.to_string())?;

    Ok(content)
}
