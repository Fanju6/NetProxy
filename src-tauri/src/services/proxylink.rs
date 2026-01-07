use crate::models::NodeInfo;
use crate::utils::paths;
use std::fs;
use std::process::Command;

pub fn import_link(link: &str) -> Result<NodeInfo, Box<dyn std::error::Error>> {
    let proxylink_path = paths::get_proxylink_path();
    let outbounds_dir = paths::get_outbounds_dir();

    // 确保输出目录存在
    fs::create_dir_all(&outbounds_dir)?;

    let output = Command::new(&proxylink_path)
        .arg("-parse")
        .arg(link)
        .arg("-format")
        .arg("xray")
        .arg("-auto")
        .arg("-dir")
        .arg(&outbounds_dir)
        .current_dir(&outbounds_dir) // 设置工作目录
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("proxylink failed: {}", stderr).into());
    }

    // 解析输出获取节点信息
    let _stdout = String::from_utf8_lossy(&output.stdout);

    Ok(NodeInfo {
        name: "Imported".to_string(),
        protocol: "unknown".to_string(),
        address: "".to_string(),
        port: 0,
        file_name: "imported.json".to_string(),
    })
}

pub fn import_subscription(
    url: &str,
    sub_name: &str,
) -> Result<Vec<NodeInfo>, Box<dyn std::error::Error>> {
    let proxylink_path = paths::get_proxylink_path();
    let outbounds_dir = paths::get_outbounds_dir();

    // 创建订阅专用目录: outbounds/sub_订阅名称
    let sub_dir = outbounds_dir.join(format!("sub_{}", sub_name));
    fs::create_dir_all(&sub_dir)?;

    let output = Command::new(&proxylink_path)
        .arg("-sub")
        .arg(url)
        .arg("-format")
        .arg("xray")
        .arg("-dir")
        .arg(&sub_dir)
        .current_dir(&sub_dir) // 设置工作目录
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("proxylink subscription failed: {}", stderr).into());
    }

    // 返回空列表，实际节点会被 list_nodes 读取
    Ok(Vec::new())
}
