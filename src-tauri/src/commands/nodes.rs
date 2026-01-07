use crate::models::NodeInfo;
use crate::services::proxylink;
use crate::utils::paths;
use std::fs;
use std::path::Path;

#[tauri::command]
pub async fn import_link(link: String) -> Result<NodeInfo, String> {
    proxylink::import_link(&link).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn import_subscription(url: String, sub_name: String) -> Result<Vec<NodeInfo>, String> {
    proxylink::import_subscription(&url, &sub_name).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_nodes() -> Vec<NodeInfo> {
    let outbounds_dir = paths::get_outbounds_dir();
    let mut nodes = Vec::new();

    // 递归读取节点文件（包括订阅子目录）
    collect_nodes(&outbounds_dir, &outbounds_dir, &mut nodes);

    nodes
}

fn collect_nodes(base_dir: &Path, current_dir: &Path, nodes: &mut Vec<NodeInfo>) {
    if let Ok(entries) = fs::read_dir(current_dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_dir() {
                // 递归处理子目录（订阅目录）
                collect_nodes(base_dir, &path, nodes);
            } else if path.extension().map_or(false, |ext| ext == "json") {
                if let Some(file_name) = path.file_name() {
                    let name = file_name.to_string_lossy().replace(".json", "");

                    // 计算相对路径作为文件名（用于区分不同订阅的节点）
                    let relative_path = path
                        .strip_prefix(base_dir)
                        .unwrap_or(&path)
                        .to_string_lossy()
                        .to_string();

                    nodes.push(NodeInfo {
                        name: name.clone(),
                        protocol: "unknown".to_string(),
                        address: "".to_string(),
                        port: 0,
                        file_name: relative_path,
                    });
                }
            }
        }
    }
}

#[tauri::command]
pub fn delete_node(file_path: String) -> Result<(), String> {
    let outbounds_dir = paths::get_outbounds_dir();
    let full_path = outbounds_dir.join(&file_path);

    fs::remove_file(&full_path).map_err(|e| e.to_string())
}
