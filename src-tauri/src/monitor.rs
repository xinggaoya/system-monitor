use crate::adaptive_refresh::{AdaptiveRefreshManager, RefreshStatistics};
use crate::errors::MonitorError;
use crate::frame_monitor::FrameMonitor;
use crate::gpu_monitor::GpuMonitor;
use crate::models::*;
use crate::retry::{RetryConfig, RetryManager};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use sysinfo::{Components, Disks, Networks, ProcessesToUpdate, System};
use tokio::sync::RwLock;
use tokio::time::sleep;

// 简化复杂类型的定义
type NetworkDataMap = HashMap<String, (u64, u64, Instant)>;

/// 系统监控器（优化为异步安全，支持智能重试和自适应刷新）
pub struct SystemMonitor {
    system: Arc<RwLock<System>>,
    last_network_data: Arc<Mutex<NetworkDataMap>>,
    config: MonitorConfig,
    gpu_monitor: GpuMonitor,
    frame_monitor: FrameMonitor,
    #[allow(dead_code)]
    retry_manager: RetryManager,
    adaptive_refresh: AdaptiveRefreshManager,
}

impl SystemMonitor {
    /// 创建新的系统监控器
    pub fn new(config: MonitorConfig) -> Self {
        let mut system = System::new();
        system.refresh_all();

        // 初始化GPU监控器
        let gpu_monitor = match GpuMonitor::new() {
            Ok(monitor) => {
                println!("GPU监控器初始化成功");
                monitor
            }
            Err(e) => {
                eprintln!("GPU监控器初始化失败: {}", e);
                GpuMonitor::default()
            }
        };

        // 创建自适应刷新管理器
        let adaptive_refresh = AdaptiveRefreshManager::new(config.refresh_strategy.clone().into());

        Self {
            system: Arc::new(RwLock::new(system)),
            last_network_data: Arc::new(Mutex::new(HashMap::new())),
            config,
            gpu_monitor,
            frame_monitor: FrameMonitor::new(),
            retry_manager: RetryManager::new(RetryConfig::default()),
            adaptive_refresh,
        }
    }

    /// 刷新系统信息（异步版本，提升性能，异步安全，支持智能重试）
    pub async fn refresh(&mut self) -> Result<SystemInfo, String> {
        self.refresh_internal().await.map_err(|e| e.to_string())
    }

    /// 带智能重试的刷新系统信息
    pub async fn refresh_with_retry(&mut self) -> Result<SystemInfo, String> {
        const MAX_RETRIES: u32 = 2;
        const RETRY_DELAY_MS: u64 = 300;

        let mut attempt = 0;
        loop {
            match self.refresh_internal().await {
                Ok(info) => return Ok(info),
                Err(error) => {
                    attempt += 1;
                    if attempt > MAX_RETRIES || !error.is_retryable() {
                        eprintln!("系统信息刷新失败 (尝试 {}/{}): {}", attempt, MAX_RETRIES + 1, error);
                        return Err(error.to_string());
                    }
                    
                    eprintln!("系统信息刷新遇到临时错误，准备重试 ({}): {}", attempt, error);
                    sleep(Duration::from_millis(RETRY_DELAY_MS * attempt as u64)).await;
                }
            }
        }
    }

    /// 内部刷新实现
    async fn refresh_internal(&mut self) -> Result<SystemInfo, MonitorError> {
        // 1. 刷新系统组件数据
        self.refresh_components().await;

        // 2. 获取读取锁并提取数据
        let system = self.system.read().await;
        
        // 3. 独立获取其他动态数据
        // 注意：这里创建新的实例来获取最新状态，sysinfo 的设计模式
        let disks = Disks::new_with_refreshed_list();
        let networks = Networks::new_with_refreshed_list();
        let components = Components::new_with_refreshed_list();

        // 4. 组装数据
        let cpu_usage = self.get_cpu_usage(&system);
        let memory = self.get_memory_info(&system);
        let network = self.get_network_info(&networks);
        let disk = self.get_disk_info(&disks);
        let system_details = self.get_system_details(&system);

        // 获取温度信息
        let temperatures = if self.config.enable_temperature {
            self.get_temperature_info(&components)
        } else {
            Vec::new()
        };

        Ok(SystemInfo {
            cpu_usage,
            memory,
            network,
            disk,
            system: system_details,
            temperatures,
        })
    }

    /// 刷新核心系统组件
    async fn refresh_components(&self) {
        // 获取写锁进行更新
        let mut system = self.system.write().await;

        // 刷新 CPU 和 内存
        system.refresh_cpu_usage();
        system.refresh_memory();
        
        // 刷新进程（如果需要）
        // system.refresh_processes(ProcessesToUpdate::All, false);

        // 释放锁
        drop(system);

        // 等待 CPU 数据收集（sysinfo 要求两次刷新之间有间隔才能计算 CPU 使用率）
        // 注意：这个 sleep 会导致整个刷新过程变慢，需要权衡
        // 如果调用频率本身就低于 MINIMUM_CPU_UPDATE_INTERVAL，这里其实可以优化
        sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL).await;
    }

    /// 智能刷新系统信息（包含自适应频率管理）
    pub async fn smart_refresh(&mut self) -> Result<SystemInfo, String> {
        // 检查是否应该跳过刷新
        if self.adaptive_refresh.should_skip_refresh() {
            // 返回上次的缓存数据或特定的状态码可能更好，但这里保持接口一致
            // 暂时返回错误作为"跳过"的信号，前端应处理这种情况
            return Err("SKIPPED_STABLE".to_string());
        }

        // 执行刷新
        let system_info = self.refresh_internal().await.map_err(|e| e.to_string())?;

        // 计算下次刷新间隔
        self.adaptive_refresh.calculate_next_interval(&system_info);

        Ok(system_info)
    }

    /// 获取建议的刷新间隔
    pub fn suggested_refresh_interval(&self) -> std::time::Duration {
        self.adaptive_refresh.current_interval()
    }

    /// 获取刷新统计信息
    pub fn get_refresh_statistics(&self) -> RefreshStatistics {
        self.adaptive_refresh.get_statistics()
    }

    /// 更新配置（包括刷新策略）
    pub fn update_config(&mut self, config: MonitorConfig) {
        self.config = config.clone();
        // 更新自适应刷新策略
        self.adaptive_refresh
            .update_strategy(config.refresh_strategy.into());
    }

    /// 获取CPU使用率
    fn get_cpu_usage(&self, system: &System) -> f32 {
        if self.config.enable_cpu {
            system.global_cpu_usage()
        } else {
            0.0
        }
    }

    /// 获取内存信息
    fn get_memory_info(&self, system: &System) -> MemoryInfo {
        let total = system.total_memory();
        let used = system.used_memory();
        let available = system.available_memory();
        let usage_percent = if total > 0 {
            (used as f32 / total as f32) * 100.0
        } else {
            0.0
        };

        let swap_total = system.total_swap();
        let swap_used = system.used_swap();

        MemoryInfo {
            total,
            used,
            available,
            usage_percent,
            swap_total,
            swap_used,
        }
    }

    /// 获取网络信息
    fn get_network_info(&self, networks: &Networks) -> NetworkInfo {
        let mut last_data = self.last_network_data.lock().unwrap();
        let current_time = Instant::now();

        let mut interfaces = Vec::new();
        let mut total_received = 0u64;
        let mut total_transmitted = 0u64;

        for (interface_name, data) in networks {
            let received = data.total_received();
            let transmitted = data.total_transmitted();

            total_received += received;
            total_transmitted += transmitted;

            // 计算速率
            let (receive_rate, transmit_rate) =
                if let Some((last_received, last_transmitted, last_time)) =
                    last_data.get(interface_name)
                {
                    let duration = current_time.duration_since(*last_time);
                    if duration.as_secs_f64() > 0.0 {
                        let receive_rate = (received.saturating_sub(*last_received) as f64)
                            / duration.as_secs_f64();
                        let transmit_rate = (transmitted.saturating_sub(*last_transmitted) as f64)
                            / duration.as_secs_f64();
                        (receive_rate, transmit_rate)
                    } else {
                        (0.0, 0.0)
                    }
                } else {
                    (0.0, 0.0)
                };

            interfaces.push(NetworkInterface {
                name: interface_name.clone(),
                received,
                transmitted,
                receive_rate,
                transmit_rate,
            });

            // 更新最后记录的数据
            last_data.insert(
                interface_name.clone(),
                (received, transmitted, current_time),
            );
        }

        NetworkInfo {
            interfaces,
            total_received,
            total_transmitted,
        }
    }

    /// 获取磁盘信息
    fn get_disk_info(&self, disks: &Disks) -> DiskInfo {
        let mut disk_list = Vec::new();

        for disk in disks {
            let total_space = disk.total_space();
            let available_space = disk.available_space();
            let used_space = total_space.saturating_sub(available_space);
            let usage_percent = if total_space > 0 {
                (used_space as f32 / total_space as f32) * 100.0
            } else {
                0.0
            };

            disk_list.push(Disk {
                name: disk.name().to_string_lossy().to_string(),
                mount_point: disk.mount_point().to_string_lossy().to_string(),
                file_system: disk.file_system().to_string_lossy().to_string(),
                total_space,
                available_space,
                used_space,
                usage_percent,
            });
        }

        DiskInfo { disks: disk_list }
    }

    /// 获取系统详情
    fn get_system_details(&self, system: &System) -> SystemDetails {
        let cpu_count = system.cpus().len();
        let cpu_brand = system.cpus().first().map(|cpu| cpu.brand().to_string());
        let cpu_frequency = system.cpus().first().map(|cpu| cpu.frequency());

        SystemDetails {
            name: System::name(),
            kernel_version: System::kernel_version(),
            os_version: System::os_version(),
            host_name: System::host_name(),
            cpu_count,
            cpu_brand,
            cpu_frequency,
        }
    }

    /// 获取温度信息
    fn get_temperature_info(&self, components: &Components) -> Vec<TemperatureInfo> {
        let mut temperatures = Vec::new();

        for component in components {
            if let Some(temp) = component.temperature() {
                temperatures.push(TemperatureInfo {
                    label: component.label().to_string(),
                    temperature: temp,
                    max: component.max(),
                    critical: component.critical(),
                    category: Some(Self::classify_temperature_category(component.label())),
                });
            }
        }

        temperatures
    }

    fn classify_temperature_category(label: &str) -> String {
        let normalized = label.to_lowercase();
        let contains = |keywords: &[&str]| keywords.iter().any(|k| normalized.contains(k));

        if normalized.contains("cpu") && contains(&["package", "tdie", "tctl", "socket", "die"]) {
            return "cpu-package".to_string();
        }
        if normalized.contains("cpu") && contains(&["core", "#", "ccd", "ccx", "thread", "l3"]) {
            return "cpu-core".to_string();
        }
        if contains(&["dimm", "memory", "ram"]) {
            return "memory".to_string();
        }
        if contains(&["gpu", "graphics", "video"]) {
            return "gpu".to_string();
        }
        if contains(&["vrm", "vcore", "soc"]) {
            return "vrm".to_string();
        }
        if contains(&["pch", "chipset", "motherboard", "board"]) {
            return "motherboard".to_string();
        }
        if contains(&["nvme", "ssd", "hdd", "m.2", "m2", "drive", "storage"]) {
            return "storage".to_string();
        }

        "other".to_string()
    }

    /// 获取GPU信息
    pub fn get_gpu_info(&self) -> Option<GpuInfo> {
        if self.config.enable_gpu {
            self.gpu_monitor.get_gpu_info()
        } else {
            None
        }
    }

    /// 获取帧率信息
    pub async fn capture_frame_stats(
        &self,
        duration: Duration,
    ) -> Result<FrameStats, MonitorError> {
        self.frame_monitor.capture_frame_stats(duration).await
    }

    /// 获取GPU监控器状态信息
    pub fn get_gpu_monitor_status(&self) -> (bool, Option<String>) {
        let is_available = self.gpu_monitor.is_available();
        let error = self.gpu_monitor.get_last_error().map(|s| s.to_string());
        (is_available, error)
    }

    /// 获取所有GPU名称
    pub fn get_gpu_names(&self) -> Vec<String> {
        self.gpu_monitor.get_all_gpu_names()
    }

    /// 获取详细GPU信息
    pub fn get_detailed_gpu_info(&self, device_index: u32) -> Result<String, String> {
        self.gpu_monitor
            .get_detailed_gpu_info(device_index)
            .map_err(|e| e.to_string())
    }

    /// 获取当前配置
    pub fn get_config(&self) -> &MonitorConfig {
        &self.config
    }
}
