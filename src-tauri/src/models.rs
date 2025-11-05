use serde::{Deserialize, Serialize};
use std::time::Duration;

/// 系统监控数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    /// CPU使用率（百分比）
    pub cpu_usage: f32,
    /// 内存使用情况
    pub memory: MemoryInfo,
    /// 网络使用情况
    pub network: NetworkInfo,
    /// 磁盘使用情况
    pub disk: DiskInfo,
    /// 系统信息
    pub system: SystemDetails,
    /// 组件温度（如果可用）
    pub temperatures: Vec<TemperatureInfo>,
}

/// 内存信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInfo {
    /// 总内存（字节）
    pub total: u64,
    /// 已使用内存（字节）
    pub used: u64,
    /// 可用内存（字节）
    pub available: u64,
    /// 使用率（百分比）
    pub usage_percent: f32,
    /// 交换区总大小（字节）
    pub swap_total: u64,
    /// 交换区已使用（字节）
    pub swap_used: u64,
}

/// 网络信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInfo {
    /// 网络接口列表
    pub interfaces: Vec<NetworkInterface>,
    /// 总下载量（字节）
    pub total_received: u64,
    /// 总上传量（字节）
    pub total_transmitted: u64,
}

/// 网络接口
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterface {
    /// 接口名称
    pub name: String,
    /// 接收量（字节）
    pub received: u64,
    /// 传输量（字节）
    pub transmitted: u64,
    /// 接收速率（字节/秒）
    pub receive_rate: f64,
    /// 传输速率（字节/秒）
    pub transmit_rate: f64,
}

/// 磁盘信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskInfo {
    /// 磁盘列表
    pub disks: Vec<Disk>,
}

/// 磁盘
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Disk {
    /// 磁盘名称
    pub name: String,
    /// 挂载点
    pub mount_point: String,
    /// 文件系统类型
    pub file_system: String,
    /// 总空间（字节）
    pub total_space: u64,
    /// 可用空间（字节）
    pub available_space: u64,
    /// 使用空间（字节）
    pub used_space: u64,
    /// 使用率（百分比）
    pub usage_percent: f32,
}

/// 系统详情
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemDetails {
    /// 系统名称
    pub name: Option<String>,
    /// 内核版本
    pub kernel_version: Option<String>,
    /// 操作系统版本
    pub os_version: Option<String>,
    /// 主机名
    pub host_name: Option<String>,
    /// CPU核心数
    pub cpu_count: usize,
    /// CPU品牌
    pub cpu_brand: Option<String>,
    /// CPU频率（MHz）
    pub cpu_frequency: Option<u64>,
}

/// 温度信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemperatureInfo {
    /// 组件名称
    pub label: String,
    /// 当前温度（摄氏度）
    pub temperature: f32,
    /// 最高温度（摄氏度）
    pub max: Option<f32>,
    /// 临界温度（摄氏度）
    pub critical: Option<f32>,
}

/// GPU信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuInfo {
    /// GPU名称
    pub name: String,
    /// GPU使用率（百分比）
    pub usage_percent: f32,
    /// GPU内存使用情况
    pub memory: GpuMemoryInfo,
    /// GPU温度（摄氏度）
    pub temperature: Option<f32>,
    /// GPU频率（MHz）
    pub frequency: Option<u64>,
}

/// GPU内存信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuMemoryInfo {
    /// 总内存（字节）
    pub total: u64,
    /// 已使用内存（字节）
    pub used: u64,
    /// 使用率（百分比）
    pub usage_percent: f32,
}

/// 监控配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorConfig {
    /// 刷新间隔（毫秒）
    pub refresh_interval: u64,
    /// 是否启用CPU监控
    pub enable_cpu: bool,
    /// 是否启用内存监控
    pub enable_memory: bool,
    /// 是否启用网络监控
    pub enable_network: bool,
    /// 是否启用磁盘监控
    pub enable_disk: bool,
    /// 是否启用温度监控
    pub enable_temperature: bool,
    /// 是否启用GPU监控
    pub enable_gpu: bool,
}

impl Default for MonitorConfig {
    fn default() -> Self {
        Self {
            refresh_interval: 1000, // 1秒
            enable_cpu: true,
            enable_memory: true,
            enable_network: true,
            enable_disk: true,
            enable_temperature: true,
            enable_gpu: true,
        }
    }
}

impl From<Duration> for MonitorConfig {
    fn from(duration: Duration) -> Self {
        Self {
            refresh_interval: duration.as_millis() as u64,
            ..Default::default()
        }
    }
}