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

                    // 解析详细信息
                    let (protocol, address, port) = parse_node_details(&path);

                    nodes.push(NodeInfo {
                        name: name.clone(),
                        protocol,
                        address,
                        port,
                        file_name: relative_path,
                    });
                }
            }
        }
    }
}

fn parse_node_details(path: &Path) -> (String, String, u16) {
    if let Ok(content) = fs::read_to_string(path) {
        if let Ok(mut json) = serde_json::from_str::<serde_json::Value>(&content) {
            // 处理包装结构: { "outbounds": [ { ... } ] }
            // 仅当根目录有 "outbounds" 字段且为数组时处理
            if let Some(outbounds) = json.get("outbounds").and_then(|v| v.as_array()) {
                if let Some(first) = outbounds.first() {
                    json = first.clone();
                }
            }

            let protocol = json
                .get("protocol")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_string();

            // 处理 freedom (直连)
            if protocol == "freedom" {
                return (protocol, "Direct/Local".to_string(), 0);
            }

            let settings = json.get("settings");
            let mut address = String::new();
            let mut port = 0;

            if let Some(s) = settings {
                // vnext (vmess, vless, trojan)
                if let Some(vnext) = s
                    .get("vnext")
                    .and_then(|v| v.as_array())
                    .and_then(|a| a.first())
                {
                    address = vnext
                        .get("address")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    port = vnext.get("port").and_then(|v| v.as_u64()).unwrap_or(0) as u16;
                }
                // servers (shadowsocks)
                else if let Some(servers) = s
                    .get("servers")
                    .and_then(|v| v.as_array())
                    .and_then(|a| a.first())
                {
                    address = servers
                        .get("address")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    port = servers.get("port").and_then(|v| v.as_u64()).unwrap_or(0) as u16;
                }
            }

            return (protocol, address, port);
        }
    }
    ("unknown".to_string(), "".to_string(), 0)
}

#[tauri::command]
pub fn delete_node(file_path: String) -> Result<(), String> {
    let outbounds_dir = paths::get_outbounds_dir();
    let full_path = outbounds_dir.join(&file_path);

    fs::remove_file(&full_path).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn ping_node(address: String, port: u16) -> Result<i64, String> {
    use std::net::TcpStream;
    use std::net::ToSocketAddrs;
    use std::time::{Duration, Instant};

    // 在 blocking 线程中执行 DNS 解析和连接，避免阻塞 async runtime
    tauri::async_runtime::spawn_blocking(move || {
        let start = Instant::now();
        let target = format!("{}:{}", address, port);

        // 解析地址
        let socket_addrs = target.to_socket_addrs().map_err(|e| e.to_string())?;
        let mut last_err = None;

        for addr in socket_addrs {
            // 设置 3 秒超时
            match TcpStream::connect_timeout(&addr, Duration::from_secs(3)) {
                Ok(_) => {
                    let duration = start.elapsed().as_millis() as i64;
                    return Ok(duration);
                }
                Err(e) => last_err = Some(e),
            }
        }

        Err(last_err
            .map(|e| e.to_string())
            .unwrap_or_else(|| "无法解析地址".to_string()))
    })
    .await
    .map_err(|e| e.to_string())?
}
