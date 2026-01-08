use crate::services::monitor::MONITOR;

#[derive(serde::Serialize)]
pub struct TrafficStats {
    pub upload_speed: u64,
    pub download_speed: u64,
    pub upload_total: u64,
    pub download_total: u64,
}

#[derive(serde::Serialize)]
pub struct IpInfo {
    pub external_ip: Option<String>,
    pub internal_ip: Option<String>,
}

#[tauri::command]
pub fn get_traffic_stats() -> TrafficStats {
    let mut monitor = MONITOR.lock().unwrap();
    let (up, down) = monitor.get_speed();
    let (total_up, total_down) = monitor.get_totals();

    TrafficStats {
        upload_speed: up,
        download_speed: down,
        upload_total: total_up,
        download_total: total_down,
    }
}

#[tauri::command]
pub async fn get_ip_info() -> IpInfo {
    // 公网 IP
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(3))
        .build()
        .unwrap_or_default();

    let external_ip = match client.get("https://api.ipify.org").send().await {
        Ok(resp) => resp.text().await.ok(),
        Err(_) => None,
    };

    // 内网 IP
    let internal_ip = local_ip_address::local_ip().ok().map(|ip| ip.to_string());

    IpInfo {
        external_ip,
        internal_ip,
    }
}
