use crate::models::ProxyStatus;
use crate::services::{system_proxy, xray};
use crate::utils::paths;
use once_cell::sync::Lazy;
use std::fs;
use std::sync::Mutex;

// 全局存储选中的节点
static SELECTED_NODE: Lazy<Mutex<Option<String>>> = Lazy::new(|| Mutex::new(None));

#[tauri::command]
pub async fn start_proxy(node_file: String) -> Result<(), String> {
    xray::start(&node_file).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn stop_proxy() -> Result<(), String> {
    xray::stop().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_proxy_status() -> ProxyStatus {
    let mut status = xray::get_status();
    // 获取当前选中的节点
    if let Ok(node) = SELECTED_NODE.lock() {
        status.current_node = node.clone();
    }
    status
}

#[tauri::command]
pub async fn set_system_proxy(enable: bool, port: u16) -> Result<(), String> {
    system_proxy::set_proxy(enable, port).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn select_node(node_file: String) -> Result<(), String> {
    let mut selected = SELECTED_NODE.lock().map_err(|e| e.to_string())?;
    *selected = Some(node_file.clone());

    // 保存到配置文件
    let config_path = paths::get_data_dir().join("selected_node.txt");
    fs::write(&config_path, &node_file).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn get_selected_node() -> Option<String> {
    // 先尝试从内存读取
    if let Ok(node) = SELECTED_NODE.lock() {
        if node.is_some() {
            return node.clone();
        }
    }

    // 从文件读取
    let config_path = paths::get_data_dir().join("selected_node.txt");
    if let Ok(content) = fs::read_to_string(&config_path) {
        let node = content.trim().to_string();
        if !node.is_empty() {
            // 更新内存
            if let Ok(mut selected) = SELECTED_NODE.lock() {
                *selected = Some(node.clone());
            }
            return Some(node);
        }
    }

    None
}
