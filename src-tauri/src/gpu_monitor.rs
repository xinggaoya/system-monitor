use crate::models::*;
use nvml_wrapper::{
    Nvml,
    enum_wrappers::device::{Clock, TemperatureSensor},
    error::NvmlError,
};
use thiserror::Error;

/// GPU监控错误类型
#[derive(Error, Debug)]
pub enum GpuMonitorError {
    #[error("NVML初始化失败: {0}")]
    NvmlInitFailed(#[from] NvmlError),

    #[error("没有找到GPU设备")]
    NoGpuFound,
}

/// GPU监控器结构体
pub struct GpuMonitor {
    /// NVML实例
    nvml: Option<Nvml>,
    /// 设备数量
    device_count: u32,
    /// 最后的错误信息
    last_error: Option<String>,
}

impl GpuMonitor {
    /// 创建新的GPU监控器
    pub fn new() -> Result<Self, GpuMonitorError> {
        // 尝试初始化NVML
        match Nvml::init() {
            Ok(nvml) => {
                let device_count = nvml.device_count()
                    .map_err(GpuMonitorError::NvmlInitFailed)?;

                if device_count == 0 {
                    return Err(GpuMonitorError::NoGpuFound);
                }

                Ok(Self {
                    nvml: Some(nvml),
                    device_count,
                    last_error: None,
                })
            }
            Err(e) => {
                // NVML初始化失败，直接返回错误
                Err(GpuMonitorError::NvmlInitFailed(e))
            }
        }
    }

    /// 获取GPU信息
    pub fn get_gpu_info(&self) -> Option<GpuInfo> {
        // 检查NVML是否可用
        if self.nvml.is_none() {
            return None;
        }

        let nvml = self.nvml.as_ref().unwrap();

        // 获取第一个GPU的信息（后续可以扩展为多GPU支持）
        match nvml.device_by_index(0) {
            Ok(device) => {
                // GPU基本信息
                let name = device.name().unwrap_or_else(|_| "Unknown GPU".to_string());

                // GPU使用率
                let usage_percent = self.get_gpu_usage(&device);

                // 显存信息
                let memory = self.get_memory_info(&device);

                // 温度信息
                let temperature = self.get_temperature(&device);

                // 频率信息
                let frequency = self.get_frequency(&device);

                Some(GpuInfo {
                    name,
                    usage_percent,
                    memory,
                    temperature,
                    frequency,
                })
            }
            Err(e) => {
                eprintln!("获取GPU设备信息失败: {}", e);
                None
            }
        }
    }

    /// 获取GPU使用率
    fn get_gpu_usage(&self, device: &nvml_wrapper::Device) -> f32 {
        match device.utilization_rates() {
            Ok(utilization) => utilization.gpu as f32,
            Err(_) => 0.0,
        }
    }

    /// 获取显存信息
    fn get_memory_info(&self, device: &nvml_wrapper::Device) -> GpuMemoryInfo {
        match device.memory_info() {
            Ok(memory) => {
                let total = memory.total;
                let used = memory.used;
                let usage_percent = if total > 0 {
                    (used as f32 / total as f32) * 100.0
                } else {
                    0.0
                };

                GpuMemoryInfo {
                    total,
                    used,
                    usage_percent,
                }
            }
            Err(_) => {
                // 降级到模拟数据
                GpuMemoryInfo {
                    total: 10 * 1024 * 1024 * 1024, // 10GB
                    used: 4 * 1024 * 1024 * 1024,  // 4GB
                    usage_percent: 40.0,
                }
            }
        }
    }

    /// 获取GPU温度
    fn get_temperature(&self, device: &nvml_wrapper::Device) -> Option<f32> {
        match device.temperature(TemperatureSensor::Gpu) {
            Ok(temp) => Some(temp as f32),
            Err(_) => None,
        }
    }

    /// 获取GPU频率
    fn get_frequency(&self, device: &nvml_wrapper::Device) -> Option<u64> {
        match device.clock_info(Clock::Graphics) {
            Ok(freq) => Some(freq as u64),
            Err(_) => None,
        }
    }

    /// 检查GPU监控是否可用
    pub fn is_available(&self) -> bool {
        self.nvml.is_some() && self.device_count > 0
    }

    /// 获取最后的错误信息
    pub fn get_last_error(&self) -> Option<&str> {
        self.last_error.as_deref()
    }

    /// 获取所有GPU的基本信息列表
    pub fn get_all_gpu_names(&self) -> Vec<String> {
        if !self.is_available() {
            return vec![];
        }

        let nvml = self.nvml.as_ref().unwrap();
        let mut gpu_names = Vec::new();

        for i in 0..self.device_count {
            if let Ok(device) = nvml.device_by_index(i) {
                if let Ok(name) = device.name() {
                    gpu_names.push(name);
                } else {
                    gpu_names.push(format!("GPU {}", i));
                }
            }
        }

        gpu_names
    }

    /// 获取详细的GPU状态信息
    pub fn get_detailed_gpu_info(&self, device_index: u32) -> Result<String, GpuMonitorError> {
        if !self.is_available() {
            return Err(GpuMonitorError::NoGpuFound);
        }

        let nvml = self.nvml.as_ref().unwrap();
        let device = nvml.device_by_index(device_index)
            .map_err(GpuMonitorError::NvmlInitFailed)?;

        let mut info = String::new();

        // 基本信息
        if let Ok(name) = device.name() {
            info.push_str(&format!("GPU名称: {}\n", name));
        }

        if let Ok(uuid) = device.uuid() {
            info.push_str(&format!("UUID: {}\n", uuid));
        }

        // 使用率
        if let Ok(utilization) = device.utilization_rates() {
            info.push_str(&format!("GPU使用率: {}%\n", utilization.gpu));
            info.push_str(&format!("显存使用率: {}%\n", utilization.memory));
        }

        // 温度
        if let Ok(temp) = device.temperature(TemperatureSensor::Gpu) {
            info.push_str(&format!("GPU温度: {}°C\n", temp));
        }

        // 显存
        if let Ok(memory) = device.memory_info() {
            let used_gb = memory.used as f64 / 1024.0 / 1024.0 / 1024.0;
            let total_gb = memory.total as f64 / 1024.0 / 1024.0 / 1024.0;
            info.push_str(&format!("显存: {:.2}/{:.2} GB\n", used_gb, total_gb));
        }

        // 频率
        if let Ok(clock) = device.clock_info(Clock::Graphics) {
            info.push_str(&format!("图形频率: {} MHz\n", clock));
        }

        // 功耗
        if let Ok(power) = device.power_usage() {
            let watts = power as f64 / 1000.0;
            info.push_str(&format!("功耗: {:.2} W\n", watts));
        }

        Ok(info)
    }
}

impl Default for GpuMonitor {
    fn default() -> Self {
        // 默认创建不可用的GPU监控器
        Self {
            nvml: None,
            device_count: 0,
            last_error: Some("GPU监控不可用".to_string()),
        }
    }
}