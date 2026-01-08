use crate::models::ProxyStatus;
use crate::services::system_proxy; // 引入 system_proxy
use crate::utils::paths;
use std::os::windows::process::CommandExt;
use std::process::{Child, Command};
use std::sync::Mutex; // 引入 Windows 进程扩展

const CREATE_NO_WINDOW: u32 = 0x08000000;

static XRAY_PROCESS: Mutex<Option<Child>> = Mutex::new(None);
static CURRENT_NODE: Mutex<Option<String>> = Mutex::new(None);

pub fn start(node_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 先停止已有进程
    stop()?;

    let xray_path = paths::get_xray_path();
    let confdir = paths::get_confdir();
    let data_dir = paths::get_data_dir();
    let node_path = paths::get_outbounds_dir().join(node_file);

    // 确保 logs 目录存在
    let logs_dir = data_dir.join("logs");
    if !logs_dir.exists() {
        std::fs::create_dir_all(&logs_dir)?;
    }

    let child = Command::new(&xray_path)
        .arg("-confdir")
        .arg(&confdir)
        .arg("-c")
        .arg(&node_path)
        .current_dir(&data_dir) // 设置工作目录
        .creation_flags(CREATE_NO_WINDOW) // 设置无窗口启动
        .spawn()?;

    *XRAY_PROCESS.lock().unwrap() = Some(child);
    *CURRENT_NODE.lock().unwrap() = Some(node_file.to_string());

    // 启动成功，设置系统代理
    system_proxy::set_proxy(true, 10808)?;

    Ok(())
}

pub fn stop() -> Result<(), Box<dyn std::error::Error>> {
    let mut process = XRAY_PROCESS.lock().unwrap();
    if let Some(ref mut child) = *process {
        let _ = child.kill();
        let _ = child.wait();
    }
    *process = None;
    *CURRENT_NODE.lock().unwrap() = None;

    // 关闭系统代理
    system_proxy::set_proxy(false, 10808)?;

    Ok(())
}

pub fn get_status() -> ProxyStatus {
    let process = XRAY_PROCESS.lock().unwrap();
    let running = process.is_some();
    let current_node = CURRENT_NODE.lock().unwrap().clone();

    // 获取系统代理真实状态
    let system_proxy_enabled = system_proxy::is_enabled();

    ProxyStatus {
        running,
        system_proxy: system_proxy_enabled,
        current_node,
        port: 10808,
    }
}
