//! 自适应刷新频率管理模块
//! 根据系统状态和用户使用模式智能调整刷新频率

#![allow(dead_code)]

use crate::models::*;
use std::time::{Duration, Instant};

/// 从 RefreshStrategyType 转换为 RefreshStrategy
impl From<RefreshStrategyType> for RefreshStrategy {
    fn from(strategy_type: RefreshStrategyType) -> Self {
        match strategy_type {
            RefreshStrategyType::Fixed { interval_ms } => {
                RefreshStrategy::Fixed(Duration::from_millis(interval_ms))
            }
            RefreshStrategyType::Adaptive {
                min_interval_ms,
                max_interval_ms,
                cpu_threshold,
                memory_threshold,
                change_threshold,
            } => RefreshStrategy::Adaptive {
                min_interval: Duration::from_millis(min_interval_ms),
                max_interval: Duration::from_millis(max_interval_ms),
                cpu_threshold,
                memory_threshold,
                change_threshold,
            },
            RefreshStrategyType::PowerSaving {
                base_interval_ms,
                idle_interval_ms,
                active_interval_ms,
            } => RefreshStrategy::PowerSaving {
                base_interval: Duration::from_millis(base_interval_ms),
                idle_interval: Duration::from_millis(idle_interval_ms),
                active_interval: Duration::from_millis(active_interval_ms),
            },
        }
    }
}

/// 自适应刷新策略
#[derive(Debug, Clone)]
pub enum RefreshStrategy {
    /// 固定间隔
    Fixed(Duration),
    /// 自适应间隔
    Adaptive {
        /// 最小刷新间隔
        min_interval: Duration,
        /// 最大刷新间隔
        max_interval: Duration,
        /// CPU使用率阈值（超过时增加频率）
        cpu_threshold: f32,
        /// 内存使用率阈值（超过时增加频率）
        memory_threshold: f32,
        /// 系统负载变化阈值
        change_threshold: f32,
    },
    /// 智能节能模式
    PowerSaving {
        /// 基础刷新间隔
        base_interval: Duration,
        /// 空闲时刷新间隔
        idle_interval: Duration,
        /// 活跃时刷新间隔
        active_interval: Duration,
    },
}

impl Default for RefreshStrategy {
    fn default() -> Self {
        Self::Adaptive {
            min_interval: Duration::from_millis(500),
            max_interval: Duration::from_millis(5000),
            cpu_threshold: 30.0,
            memory_threshold: 70.0,
            change_threshold: 5.0,
        }
    }
}

/// 刷新历史记录
#[derive(Debug, Clone)]
pub struct RefreshHistory {
    /// 上次刷新时间
    pub last_refresh: Instant,
    /// 上次系统状态
    pub last_system_state: Option<SystemInfo>,
    /// 历史变化记录
    pub change_history: Vec<f32>,
    /// 平均变化率
    pub average_change_rate: f32,
    /// 当前刷新间隔
    pub current_interval: Duration,
    /// 用户活动状态
    pub user_active: bool,
    /// 空闲时间
    pub idle_duration: Duration,
}

impl Default for RefreshHistory {
    fn default() -> Self {
        Self {
            last_refresh: Instant::now(),
            last_system_state: None,
            change_history: Vec::with_capacity(10),
            average_change_rate: 0.0,
            current_interval: Duration::from_millis(1000),
            user_active: true,
            idle_duration: Duration::ZERO,
        }
    }
}

/// 自适应刷新管理器
pub struct AdaptiveRefreshManager {
    strategy: RefreshStrategy,
    history: RefreshHistory,
    /// 系统是否处于高负载状态
    high_load: bool,
    /// 上次检测到高负载的时间
    last_high_load: Option<Instant>,
}

impl AdaptiveRefreshManager {
    /// 创建新的自适应刷新管理器
    pub fn new(strategy: RefreshStrategy) -> Self {
        Self {
            strategy,
            history: RefreshHistory::default(),
            high_load: false,
            last_high_load: None,
        }
    }

    /// 使用默认策略创建管理器
    pub fn default() -> Self {
        Self::new(RefreshStrategy::default())
    }

    /// 更新策略
    pub fn update_strategy(&mut self, strategy: RefreshStrategy) {
        self.strategy = strategy;
    }

    /// 计算下次刷新间隔
    pub fn calculate_next_interval(&mut self, current_info: &SystemInfo) -> Duration {
        let now = Instant::now();

        // 先计算变化率，避免借用冲突
        let change_rate = self.calculate_change_rate(current_info);

        let interval = match &self.strategy {
            RefreshStrategy::Fixed(duration) => *duration,
            RefreshStrategy::Adaptive {
                min_interval,
                max_interval,
                cpu_threshold,
                memory_threshold,
                change_threshold,
            } => {
                // 计算系统负载
                let cpu_load = current_info.cpu_usage;
                let memory_load = current_info.memory.usage_percent;

                // 根据负载调整频率
                let target_interval =
                    if cpu_load > *cpu_threshold || memory_load > *memory_threshold {
                        // 高负载：更频繁刷新
                        *min_interval
                    } else if change_rate < *change_threshold {
                        // 低变化：降低刷新频率
                        *max_interval
                    } else {
                        // 中等负载：自适应调整
                        let load_factor = (cpu_load / 100.0).max(memory_load / 100.0);
                        *min_interval + (*max_interval - *min_interval).mul_f32(1.0 - load_factor)
                    };

                // 平滑调整，避免频繁大幅变化
                let smoothing_factor = 0.3;
                let current_interval = self.history.current_interval;
                let new_interval = current_interval.mul_f32(1.0 - smoothing_factor)
                    + target_interval.mul_f32(smoothing_factor);

                new_interval.clamp(*min_interval, *max_interval)
            }
            RefreshStrategy::PowerSaving {
                base_interval,
                idle_interval,
                active_interval,
            } => {
                // 根据用户活动状态调整
                if self.history.user_active {
                    *active_interval
                } else {
                    // 空闲时间越长，刷新间隔越长
                    let idle_factor = (self.history.idle_duration.as_secs_f32() / 60.0).min(1.0);
                    let interval =
                        *active_interval + (*idle_interval - *active_interval).mul_f32(idle_factor);
                    interval.max(*base_interval)
                }
            }
        };

        // 更新历史记录
        self.update_history(current_info, interval, now);
        interval
    }

    /// 计算系统变化率
    fn calculate_change_rate(&mut self, current_info: &SystemInfo) -> f32 {
        if let Some(ref last_info) = self.history.last_system_state {
            // 计算各组件的变化
            let cpu_change = (current_info.cpu_usage - last_info.cpu_usage).abs();
            let memory_change =
                (current_info.memory.usage_percent - last_info.memory.usage_percent).abs();

            // 计算网络变化（速率变化）
            let network_change = if !current_info.network.interfaces.is_empty()
                && !last_info.network.interfaces.is_empty()
            {
                let current_total_rate: f64 = current_info
                    .network
                    .interfaces
                    .iter()
                    .map(|iface| iface.receive_rate + iface.transmit_rate)
                    .sum();
                let last_total_rate: f64 = last_info
                    .network
                    .interfaces
                    .iter()
                    .map(|iface| iface.receive_rate + iface.transmit_rate)
                    .sum();
                (current_total_rate - last_total_rate).abs() as f32 / 1024.0 // 转换为KB/s
            } else {
                0.0
            };

            // 计算温度变化
            let temp_change =
                if !current_info.temperatures.is_empty() && !last_info.temperatures.is_empty() {
                    current_info
                        .temperatures
                        .iter()
                        .zip(last_info.temperatures.iter())
                        .map(|(curr, last)| (curr.temperature - last.temperature).abs())
                        .sum::<f32>()
                        / current_info.temperatures.len() as f32
                } else {
                    0.0
                };

            // 综合变化率（加权平均）
            let total_change =
                cpu_change * 0.4 + memory_change * 0.3 + network_change * 0.2 + temp_change * 0.1;

            // 更新变化历史
            self.history.change_history.push(total_change);
            if self.history.change_history.len() > 10 {
                self.history.change_history.remove(0);
            }

            // 计算平均变化率
            self.history.average_change_rate = self.history.change_history.iter().sum::<f32>()
                / self.history.change_history.len() as f32;

            total_change
        } else {
            100.0 // 第一次刷新时返回高变化率
        }
    }

    /// 更新历史记录
    fn update_history(&mut self, current_info: &SystemInfo, interval: Duration, now: Instant) {
        // 检测高负载状态
        let is_high_load =
            current_info.cpu_usage > 80.0 || current_info.memory.usage_percent > 85.0;

        if is_high_load && !self.high_load {
            self.high_load = true;
            self.last_high_load = Some(now);
        } else if !is_high_load && self.high_load {
            self.high_load = false;
        }

        // 更新用户活动状态（简化版本，基于系统负载判断）
        self.history.user_active = current_info.cpu_usage > 10.0;

        // 更新空闲时间
        if self.history.user_active {
            self.history.idle_duration = Duration::ZERO;
        } else {
            self.history.idle_duration = now.duration_since(self.history.last_refresh);
        }

        // 更新其他历史记录
        self.history.last_refresh = now;
        self.history.last_system_state = Some(current_info.clone());
        self.history.current_interval = interval;
    }

    /// 检查是否应该跳过本次刷新
    pub fn should_skip_refresh(&self) -> bool {
        match &self.strategy {
            RefreshStrategy::Adaptive {
                change_threshold, ..
            } => {
                // 如果系统非常稳定且长时间低负载，可以跳过刷新
                self.history.average_change_rate < change_threshold / 2.0
                    && !self.high_load
                    && self.history.idle_duration > Duration::from_secs(30)
            }
            RefreshStrategy::PowerSaving { idle_interval, .. } => {
                // 空闲模式下，超过最大间隔时才刷新
                self.history.idle_duration > *idle_interval
            }
            _ => false,
        }
    }

    /// 获取当前刷新间隔
    pub fn current_interval(&self) -> Duration {
        self.history.current_interval
    }

    /// 获取系统状态统计
    pub fn get_statistics(&self) -> RefreshStatistics {
        RefreshStatisticsInternal {
            current_interval: self.history.current_interval,
            average_change_rate: self.history.average_change_rate,
            high_load: self.high_load,
            user_active: self.history.user_active,
            idle_duration: self.history.idle_duration,
            last_high_load: self.last_high_load,
        }
        .into()
    }

    /// 重置历史记录
    pub fn reset_history(&mut self) {
        self.history = RefreshHistory::default();
        self.high_load = false;
        self.last_high_load = None;
    }
}

/// 刷新统计信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RefreshStatistics {
    /// 当前刷新间隔（毫秒）
    pub current_interval_ms: u64,
    /// 平均变化率
    pub average_change_rate: f32,
    /// 是否高负载
    pub high_load: bool,
    /// 用户是否活跃
    pub user_active: bool,
    /// 空闲持续时间（毫秒）
    pub idle_duration_ms: u64,
    /// 上次高负载时间（时间戳，毫秒）
    pub last_high_load_timestamp_ms: Option<u64>,
}

impl From<RefreshStatisticsInternal> for RefreshStatistics {
    fn from(internal: RefreshStatisticsInternal) -> Self {
        Self {
            current_interval_ms: internal.current_interval.as_millis() as u64,
            average_change_rate: internal.average_change_rate,
            high_load: internal.high_load,
            user_active: internal.user_active,
            idle_duration_ms: internal.idle_duration.as_millis() as u64,
            last_high_load_timestamp_ms: internal.last_high_load.map(|time| {
                let duration = time.duration_since(
                    std::time::Instant::now() - std::time::Duration::from_secs(86400),
                );
                let system_time = std::time::SystemTime::now() - duration;
                system_time
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis() as u64
            }),
        }
    }
}

/// 内部刷新统计信息（包含Instant类型，不用于序列化）
#[derive(Debug, Clone)]
pub struct RefreshStatisticsInternal {
    /// 当前刷新间隔
    pub current_interval: Duration,
    /// 平均变化率
    pub average_change_rate: f32,
    /// 是否高负载
    pub high_load: bool,
    /// 用户是否活跃
    pub user_active: bool,
    /// 空闲持续时间
    pub idle_duration: Duration,
    /// 上次高负载时间
    pub last_high_load: Option<Instant>,
}

/// 便利函数：创建常用策略
pub mod strategies {
    use super::*;

    /// 高性能策略（快速刷新）
    pub fn high_performance() -> RefreshStrategy {
        RefreshStrategy::Adaptive {
            min_interval: Duration::from_millis(200),
            max_interval: Duration::from_millis(2000),
            cpu_threshold: 20.0,
            memory_threshold: 60.0,
            change_threshold: 3.0,
        }
    }

    /// 平衡策略（默认）
    pub fn balanced() -> RefreshStrategy {
        RefreshStrategy::default()
    }

    /// 节能策略（慢速刷新）
    pub fn power_saving() -> RefreshStrategy {
        RefreshStrategy::PowerSaving {
            base_interval: Duration::from_millis(2000),
            idle_interval: Duration::from_millis(10000),
            active_interval: Duration::from_millis(1000),
        }
    }

    /// 固定频率策略
    pub fn fixed(interval: Duration) -> RefreshStrategy {
        RefreshStrategy::Fixed(interval)
    }
}

// Duration 乘法扩展
trait DurationExt {
    fn mul_f32(self, factor: f32) -> Duration;
}

impl DurationExt for Duration {
    fn mul_f32(self, factor: f32) -> Duration {
        Duration::from_millis((self.as_millis() as f32 * factor) as u64)
    }
}
