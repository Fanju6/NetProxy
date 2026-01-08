use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::time::Instant;
use sysinfo::Networks;

// 全局监控状态
pub struct MonitorState {
    networks: Networks,
    last_update: Instant,
    last_rx: u64,
    last_tx: u64,
}

impl MonitorState {
    pub fn new() -> Self {
        // 初始化网络列表 (需刷新)
        let networks = Networks::new_with_refreshed_list();

        let (rx, tx) = networks.iter().fold((0, 0), |(acc_rx, acc_tx), (_, data)| {
            (
                acc_rx + data.total_received(),
                acc_tx + data.total_transmitted(),
            )
        });

        Self {
            networks,
            last_update: Instant::now(),
            last_rx: rx,
            last_tx: tx,
        }
    }

    pub fn get_speed(&mut self) -> (u64, u64) {
        // 刷新网络数据
        self.networks.refresh();

        let now = Instant::now();
        let elapsed = now.duration_since(self.last_update).as_secs_f64();

        // 避免除以零或间隔过短
        if elapsed < 0.5 {
            return (0, 0);
        }

        let (current_rx, current_tx) =
            self.networks
                .iter()
                .fold((0, 0), |(acc_rx, acc_tx), (_, data)| {
                    (
                        acc_rx + data.total_received(),
                        acc_tx + data.total_transmitted(),
                    )
                });

        // 计算差值
        let rx_delta = current_rx.saturating_sub(self.last_rx);
        let tx_delta = current_tx.saturating_sub(self.last_tx);

        // 更新状态
        self.last_rx = current_rx;
        self.last_tx = current_tx;
        self.last_update = now;

        // 计算网速 (字节/秒)
        let down_speed = (rx_delta as f64 / elapsed) as u64;
        let up_speed = (tx_delta as f64 / elapsed) as u64;

        (up_speed, down_speed)
    }

    pub fn get_totals(&self) -> (u64, u64) {
        self.networks
            .iter()
            .fold((0, 0), |(acc_rx, acc_tx), (_, data)| {
                (
                    acc_rx + data.total_received(),
                    acc_tx + data.total_transmitted(),
                )
            })
    }
}

// 全局实例
pub static MONITOR: Lazy<Mutex<MonitorState>> = Lazy::new(|| Mutex::new(MonitorState::new()));
