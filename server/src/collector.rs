use crate::types::SystemStats;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use sysinfo::{Disks, Networks, System};
use tokio::time::sleep;

pub struct StatsCollector {
    stats: Arc<Mutex<SystemStats>>,
    last_net_rx: Arc<Mutex<u64>>,
    last_net_tx: Arc<Mutex<u64>>,
    last_time: Arc<Mutex<Duration>>,
}

impl StatsCollector {
    pub fn new() -> Self {
        let initial_stats = SystemStats {
            cpu_usage: 0.0,
            mem_usage: 0.0,
            disk_usage: 0.0,
            net_rx_rate: 0,
            net_tx_rate: 0,
            timestamp: 0,
        };
        Self {
            stats: Arc::new(Mutex::new(initial_stats)),
            last_net_rx: Arc::new(Mutex::new(0)),
            last_net_tx: Arc::new(Mutex::new(0)),
            last_time: Arc::new(Mutex::new(Duration::ZERO)),
        }
    }

    pub fn get_stats(&self) -> SystemStats {
        self.stats.lock().unwrap().clone()
    }

    pub async fn start_background_task(self: Arc<Self>) {
        let mut sys = System::new_all();
        let mut networks = Networks::new_with_refreshed_list();

        // 首次采样建立基线
        sleep(Duration::from_millis(500)).await;
        sys.refresh_all();
        networks.refresh();

        let total_rx: u64 = networks.iter().map(|(_, d)| d.received()).sum();
        let total_tx: u64 = networks.iter().map(|(_, d)| d.transmitted()).sum();
        *self.last_net_rx.lock().unwrap() = total_rx;
        *self.last_net_tx.lock().unwrap() = total_tx;
        *self.last_time.lock().unwrap() = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::ZERO);

        loop {
            // 1. CPU & Memory - 使用 refresh_all 刷新所有指标
            sys.refresh_all();

            // CPU 使用率：各 CPU 核心平均值
            let cpu = sys.global_cpu_usage();
            let mem = if sys.total_memory() > 0 {
                (sys.used_memory() as f32 / sys.total_memory() as f32) * 100.0
            } else {
                0.0
            };

            // 2. Disk (仅根目录)
            let disks = Disks::new_with_refreshed_list();
            let disk = disks
                .iter()
                .find(|d| d.mount_point().to_str() == Some("/"))
                .map(|d| {
                    if d.total_space() > 0 {
                        ((d.total_space() - d.available_space()) as f32 / d.total_space() as f32)
                            * 100.0
                    } else {
                        0.0
                    }
                })
                .unwrap_or(0.0);

            // 3. Network 速率（精确差分）
            networks.refresh();
            let current_rx: u64 = networks.iter().map(|(_, d)| d.received()).sum();
            let current_tx: u64 = networks.iter().map(|(_, d)| d.transmitted()).sum();
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or(Duration::ZERO);

            let (rx_rate, tx_rate) = {
                let last_rx = *self.last_net_rx.lock().unwrap();
                let last_tx = *self.last_net_tx.lock().unwrap();
                let last_time = *self.last_time.lock().unwrap();
                let elapsed = now.as_secs_f64() - last_time.as_secs_f64();
                if elapsed > 0.0 {
                    (
                        ((current_rx.saturating_sub(last_rx)) as f64 / elapsed) as u64,
                        ((current_tx.saturating_sub(last_tx)) as f64 / elapsed) as u64,
                    )
                } else {
                    (0, 0)
                }
            };

            // 更新历史值
            *self.last_net_rx.lock().unwrap() = current_rx;
            *self.last_net_tx.lock().unwrap() = current_tx;
            *self.last_time.lock().unwrap() = now;

            let timestamp = now.as_secs();

            // 更新共享状态（防御性处理，避免 panic）
            if let Ok(mut guard) = self.stats.lock() {
                guard.cpu_usage = cpu;
                guard.mem_usage = mem;
                guard.disk_usage = disk;
                guard.net_rx_rate = rx_rate;
                guard.net_tx_rate = tx_rate;
                guard.timestamp = timestamp;
            }

            sleep(Duration::from_millis(1000)).await;
        }
    }
}
