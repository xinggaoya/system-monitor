//! 智能重试机制模块
//! 提供基于错误类型的自动重试功能

#![allow(dead_code)]

use std::time::Duration;
use tokio::time::sleep;
use crate::errors::MonitorError;

/// 重试配置
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// 最大重试次数
    pub max_retries: usize,
    /// 初始重试延迟（毫秒）
    pub initial_delay_ms: u64,
    /// 最大重试延迟（毫秒）
    pub max_delay_ms: u64,
    /// 退避倍数
    pub backoff_multiplier: f64,
    /// 抖动因子（用于避免雷群效应）
    pub jitter_factor: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay_ms: 500,
            max_delay_ms: 5000,
            backoff_multiplier: 2.0,
            jitter_factor: 0.1,
        }
    }
}

/// 重试策略
#[derive(Debug, Clone)]
pub enum RetryStrategy {
    /// 固定延迟
    Fixed(Duration),
    /// 指数退避
    ExponentialBackoff,
    /// 线性退避
    LinearBackoff,
}

/// 重试结果
#[derive(Debug, Clone)]
pub enum RetryResult<T> {
    /// 成功
    Success(T),
    /// 失败（已达到最大重试次数）
    Failed(MonitorError),
    /// 跳过重试（不可重试的错误）
    Skipped(MonitorError),
}

impl RetryConfig {
    /// 创建新的重试配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置最大重试次数
    pub fn max_retries(mut self, retries: usize) -> Self {
        self.max_retries = retries;
        self
    }

    /// 设置初始延迟
    pub fn initial_delay(mut self, delay: Duration) -> Self {
        self.initial_delay_ms = delay.as_millis() as u64;
        self
    }

    /// 设置最大延迟
    pub fn max_delay(mut self, delay: Duration) -> Self {
        self.max_delay_ms = delay.as_millis() as u64;
        self
    }

    /// 设置退避倍数
    pub fn backoff_multiplier(mut self, multiplier: f64) -> Self {
        self.backoff_multiplier = multiplier;
        self
    }

    /// 计算重试延迟
    pub fn calculate_delay(&self, attempt: usize) -> Duration {
        if self.max_retries == 0 {
            return Duration::from_millis(0);
        }

        let base_delay = self.initial_delay_ms as f64;
        let delay = match attempt {
            0 => base_delay,
            _ => {
                let exponential_delay = base_delay * self.backoff_multiplier.powi(attempt as i32);
                exponential_delay.min(self.max_delay_ms as f64)
            }
        };

        // 添加抖动以避免雷群效应
        let jitter = delay * self.jitter_factor * (rand::random::<f64>() - 0.5) * 2.0;
        let final_delay = (delay + jitter).max(0.0) as u64;

        Duration::from_millis(final_delay)
    }
}

/// 智能重试器
pub struct RetryManager {
    config: RetryConfig,
}

impl RetryManager {
    /// 创建新的重试管理器
    pub fn new(config: RetryConfig) -> Self {
        Self { config }
    }

    /// 使用默认配置创建重试管理器
    pub fn default() -> Self {
        Self::new(RetryConfig::default())
    }

    /// 执行带重试的操作
    pub async fn execute_with_retry<F, T, Fut>(&self, mut operation: F) -> RetryResult<T>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<T, MonitorError>>,
    {
        let mut last_error = None;

        for attempt in 0..=self.config.max_retries {
            match operation().await {
                Ok(result) => return RetryResult::Success(result),
                Err(error) => {
                    // 检查是否可以重试
                    if !error.is_retryable() {
                        return RetryResult::Skipped(error);
                    }

                    last_error = Some(error.clone());

                    // 如果不是最后一次尝试，等待后重试
                    if attempt < self.config.max_retries {
                        let delay = self.config.calculate_delay(attempt);
                        sleep(delay).await;
                    }
                }
            }
        }

        // 所有重试都失败了
        RetryResult::Failed(last_error.unwrap_or_else(|| MonitorError::GenericError("重试失败".to_string())))
    }

    /// 执行带自定义重试策略的操作
    pub async fn execute_with_custom_retry<F, T, Fut>(
        &self,
        mut operation: F,
        should_retry: impl Fn(&MonitorError, usize) -> bool,
    ) -> RetryResult<T>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<T, MonitorError>>,
    {
        let mut last_error = None;

        for attempt in 0..=self.config.max_retries {
            match operation().await {
                Ok(result) => return RetryResult::Success(result),
                Err(error) => {
                    last_error = Some(error.clone());

                    // 检查是否应该重试
                    if !should_retry(&error, attempt) || attempt == self.config.max_retries {
                        break;
                    }

                    let delay = self.config.calculate_delay(attempt);
                    sleep(delay).await;
                }
            }
        }

        RetryResult::Failed(last_error.unwrap_or_else(|| MonitorError::GenericError("重试失败".to_string())))
    }
}

/// 为常见操作提供的便利函数
pub mod convenience {
    use super::*;

    /// 重试获取系统信息
    pub async fn retry_system_info<F, T, Fut>(
        operation: F,
        config: Option<RetryConfig>,
    ) -> RetryResult<T>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<T, MonitorError>>,
    {
        let retry_manager = RetryManager::new(config.unwrap_or_default());
        retry_manager.execute_with_retry(operation).await
    }

    /// 快速重试（使用默认配置）
    pub async fn quick_retry<F, T, Fut>(operation: F) -> RetryResult<T>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<T, MonitorError>>,
    {
        retry_system_info(operation, None).await
    }

    /// 带条件的重试
    pub async fn conditional_retry<F, T, Fut>(
        operation: F,
        should_retry: impl Fn(&MonitorError, usize) -> bool,
        config: Option<RetryConfig>,
    ) -> RetryResult<T>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<T, MonitorError>>,
    {
        let retry_manager = RetryManager::new(config.unwrap_or_default());
        retry_manager.execute_with_custom_retry(operation, should_retry).await
    }
}

// 添加 rand 依赖的简单实现
mod rand {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{SystemTime, UNIX_EPOCH};

    pub fn random<T>() -> T
    where
        T: From<f64>,
    {
        let mut hasher = DefaultHasher::new();
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .hash(&mut hasher);
        let hash = hasher.finish();

        // 简单的哈希到浮点数转换
        (hash as f64 / u64::MAX as f64).into()
    }
}