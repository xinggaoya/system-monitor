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
    /// 刷新间隔（毫秒）- 仅在固定频率模式下使用
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
    /// 刷新策略类型
    pub refresh_strategy: RefreshStrategyType,
}

/// 刷新策略类型（用于序列化）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RefreshStrategyType {
    /// 固定频率
    Fixed { interval_ms: u64 },
    /// 自适应频率
    Adaptive {
        min_interval_ms: u64,
        max_interval_ms: u64,
        cpu_threshold: f32,
        memory_threshold: f32,
        change_threshold: f32,
    },
    /// 节能模式
    PowerSaving {
        base_interval_ms: u64,
        idle_interval_ms: u64,
        active_interval_ms: u64,
    },
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
            refresh_strategy: RefreshStrategyType::Adaptive {
                min_interval_ms: 500,
                max_interval_ms: 5000,
                cpu_threshold: 30.0,
                memory_threshold: 70.0,
                change_threshold: 5.0,
            },
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

/// 增量更新数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfoDelta {
    /// 更新的时间戳
    pub timestamp: u64,
    /// CPU使用率（如果有变化）
    pub cpu_usage: Option<f32>,
    /// 内存信息（如果有变化）
    pub memory: Option<MemoryInfo>,
    /// 网络信息（总是更新，因为包含速率）
    pub network: Option<NetworkInfo>,
    /// 磁盘信息（如果有变化）
    pub disk: Option<DiskInfo>,
    /// 系统信息（很少变化）
    pub system: Option<SystemDetails>,
    /// 温度信息（如果有变化）
    pub temperatures: Option<Vec<TemperatureInfo>>,
    /// 完整数据（用于初始化或重大变化时）
    pub full_data: Option<SystemInfo>,
}

impl SystemInfoDelta {
    /// 创建完整的增量更新（用于初始化）
    pub fn full(system_info: SystemInfo) -> Self {
        let full_data = system_info.clone();
        Self {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            cpu_usage: Some(system_info.cpu_usage),
            memory: Some(system_info.memory),
            network: Some(system_info.network),
            disk: Some(system_info.disk),
            system: Some(system_info.system),
            temperatures: Some(system_info.temperatures),
            full_data: Some(full_data),
        }
    }

    /// 创建增量更新（比较新旧数据）
    pub fn from_diff(old: &SystemInfo, new: &SystemInfo) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        let cpu_usage = if (old.cpu_usage - new.cpu_usage).abs() > 0.5 {
            Some(new.cpu_usage)
        } else {
            None
        };

        let memory = if (old.memory.usage_percent - new.memory.usage_percent).abs() > 0.5 {
            Some(new.memory.clone())
        } else {
            None
        };

        let disk = if !old.disk.disks.is_empty() && !new.disk.disks.is_empty() {
            // 简单比较磁盘数量和使用率变化
            if old.disk.disks.len() != new.disk.disks.len() ||
               old.disk.disks.iter().zip(new.disk.disks.iter())
                   .any(|(old_disk, new_disk)| (old_disk.usage_percent - new_disk.usage_percent).abs() > 1.0) {
                Some(new.disk.clone())
            } else {
                None
            }
        } else {
            None
        };

        let system = if old.system.cpu_count != new.system.cpu_count ||
                       old.system.cpu_brand != new.system.cpu_brand ||
                       old.system.cpu_frequency != new.system.cpu_frequency {
            Some(new.system.clone())
        } else {
            None
        };

        let temperatures = if old.temperatures.len() != new.temperatures.len() ||
                           old.temperatures.iter().zip(new.temperatures.iter())
                               .any(|(old_temp, new_temp)| (old_temp.temperature - new_temp.temperature).abs() > 1.0) {
            Some(new.temperatures.clone())
        } else {
            None
        };

        Self {
            timestamp,
            cpu_usage,
            memory,
            network: Some(new.network.clone()), // 网络信息总是更新（包含速率）
            disk,
            system,
            temperatures,
            full_data: None,
        }
    }

    /// 检查是否为空更新（没有实际变化）
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.cpu_usage.is_none() &&
        self.memory.is_none() &&
        self.network.is_none() &&
        self.disk.is_none() &&
        self.system.is_none() &&
        self.temperatures.is_none() &&
        self.full_data.is_none()
    }
}