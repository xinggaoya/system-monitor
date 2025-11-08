//! 错误类型定义模块
//! 提供具体的错误类型，提升错误处理的质量和可维护性

#![allow(dead_code)]

use thiserror::Error;

/// 系统监控器的主要错误类型
#[derive(Error, Debug, Clone)]
pub enum MonitorError {
    /// 系统信息获取失败
    #[error("系统信息获取失败: {0}")]
    SystemInfoError(String),

    /// CPU信息获取失败
    #[error("CPU信息获取失败: {0}")]
    CpuInfoError(String),

    /// 内存信息获取失败
    #[error("内存信息获取失败: {0}")]
    MemoryInfoError(String),

    /// 磁盘信息获取失败
    #[error("磁盘信息获取失败: {0}")]
    DiskInfoError(String),

    /// 网络信息获取失败
    #[error("网络信息获取失败: {0}")]
    NetworkInfoError(String),

    /// 温度信息获取失败
    #[error("温度信息获取失败: {0}")]
    TemperatureInfoError(String),

    /// GPU监控不可用
    #[error("GPU监控不可用: {0}")]
    GpuUnavailable(String),

    /// GPU信息获取失败
    #[error("GPU信息获取失败: {0}")]
    GpuInfoError(String),

    /// 互斥锁 poisoning 错误
    #[error("数据访问冲突: {0}")]
    MutexError(String),

    /// 数据序列化/反序列化错误
    #[error("数据处理错误: {0}")]
    SerializationError(String),

    /// 配置错误
    #[error("配置错误: {0}")]
    ConfigError(String),

    /// IO错误
    #[error("IO操作错误: {0}")]
    IoError(String),

    /// 通用错误
    #[error("未知错误: {0}")]
    GenericError(String),
}

/// GPU监控器专用错误类型
#[derive(Error, Debug)]
pub enum GpuMonitorError {
    /// NVML库初始化失败
    #[error("NVML库初始化失败: {0}")]
    NvmlInitError(String),

    /// GPU设备未找到
    #[error("GPU设备未找到: 设备索引 {device_index}")]
    DeviceNotFound { device_index: u32 },

    /// GPU数据获取失败
    #[error("GPU数据获取失败: {0}")]
    DataFetchError(String),

    /// GPU权限不足
    #[error("GPU访问权限不足")]
    PermissionDenied,

    /// GPU不支持（如AMD/Intel GPU）
    #[error("GPU不支持或驱动不兼容")]
    UnsupportedGpu,

    /// NVML库未安装
    #[error("NVML库未安装或不可用")]
    NvmlNotAvailable,
}

/// 结果类型别名，使用我们的错误类型
pub type MonitorResult<T> = Result<T, MonitorError>;
pub type GpuResult<T> = Result<T, GpuMonitorError>;

impl MonitorError {
    /// 创建系统信息错误
    pub fn system_info<S: Into<String>>(msg: S) -> Self {
        Self::SystemInfoError(msg.into())
    }

    /// 创建CPU信息错误
    pub fn cpu_info<S: Into<String>>(msg: S) -> Self {
        Self::CpuInfoError(msg.into())
    }

    /// 创建内存信息错误
    pub fn memory_info<S: Into<String>>(msg: S) -> Self {
        Self::MemoryInfoError(msg.into())
    }

    /// 创建GPU不可用错误
    pub fn gpu_unavailable<S: Into<String>>(msg: S) -> Self {
        Self::GpuUnavailable(msg.into())
    }

    /// 创建互斥锁错误
    pub fn mutex_error<S: Into<String>>(msg: S) -> Self {
        Self::MutexError(msg.into())
    }

    /// 判断是否为可重试的错误
    pub fn is_retryable(&self) -> bool {
        match self {
            MonitorError::SystemInfoError(_)
            | MonitorError::CpuInfoError(_)
            | MonitorError::MemoryInfoError(_)
            | MonitorError::DiskInfoError(_)
            | MonitorError::NetworkInfoError(_)
            | MonitorError::TemperatureInfoError(_)
            | MonitorError::GpuInfoError(_) => true,
            MonitorError::GpuUnavailable(_)
            | MonitorError::MutexError(_)
            | MonitorError::SerializationError(_)
            | MonitorError::ConfigError(_)
            | MonitorError::IoError(_) => false,
            MonitorError::GenericError(_) => false,
        }
    }

    /// 获取错误的重试建议延迟（毫秒）
    pub fn retry_delay_ms(&self) -> u64 {
        match self {
            MonitorError::SystemInfoError(_) | MonitorError::CpuInfoError(_) => 500,
            MonitorError::MemoryInfoError(_) | MonitorError::NetworkInfoError(_) => 1000,
            MonitorError::DiskInfoError(_) | MonitorError::TemperatureInfoError(_) => 2000,
            MonitorError::GpuInfoError(_) => 3000,
            _ => 0, // 不可重试的错误返回0
        }
    }
}

// 自动转换常见错误类型（暂时移除sysinfo转换，后续添加）

// 注意：复杂的类型转换实现会在后续添加，目前简化错误处理

impl From<GpuMonitorError> for MonitorError {
    fn from(err: GpuMonitorError) -> Self {
        MonitorError::GpuUnavailable(err.to_string())
    }
}
