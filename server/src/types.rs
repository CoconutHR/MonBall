use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct SystemStats {
    pub cpu_usage: f32,   // 百分比 (0.0 - 100.0)
    pub mem_usage: f32,   // 百分比 (0.0 - 100.0)
    pub disk_usage: f32,  // 百分比 (0.0 - 100.0)
    pub net_rx_rate: u64, // 接收速率 (Bytes/sec)
    pub net_tx_rate: u64, // 发送速率 (Bytes/sec)
    pub timestamp: u64,   // Unix 时间戳 (秒)
}
