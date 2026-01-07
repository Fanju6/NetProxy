mod commands;
mod models;
mod services;
mod utils;

use commands::{config, nodes, proxy};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // 代理控制
            proxy::start_proxy,
            proxy::stop_proxy,
            proxy::get_proxy_status,
            proxy::set_system_proxy,
            proxy::select_node,
            proxy::get_selected_node,
            // 节点管理
            nodes::import_link,
            nodes::import_subscription,
            nodes::list_nodes,
            nodes::delete_node,
            // 配置管理
            config::get_dns_config,
            config::save_dns_config,
            config::get_routing_config,
            config::save_routing_config,
            config::get_xray_log,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
