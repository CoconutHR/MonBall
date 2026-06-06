use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct SystemStats {
    pub cpu_usage: f32,          // 百分比 (0.0 - 100.0)
    pub mem_usage: f32,          // 百分比 (0.0 - 100.0)
    pub disk_usage: f32,         // 百分比 (0.0 - 100.0)
    pub net_rx_rate: u64,        // 接收速率 (Bytes/sec)
    pub net_tx_rate: u64,        // 发送速率 (Bytes/sec)
    pub cpu_temp: Option<f32>,   // CPU Package 温度 (摄氏度)
    pub acpi_temp: Option<f32>,  // ACPI 主板温度 (摄氏度)
    pub wifi_temp: Option<f32>,  // WiFi 模块温度 (摄氏度)
    pub timestamp: u64,          // Unix 时间戳 (秒)
}
