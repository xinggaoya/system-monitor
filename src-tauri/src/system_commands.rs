//! 系统信息相关命令模块
//!
//! 负责处理系统监控、GPU信息和智能刷新等系统相关命令

use crate::models::*;
use tauri::State;
use std::sync::Arc;
use crate::adaptive_refresh;
use log::{info, debug, error, warn};

/// 获取系统信息（异步版本，提升性能，优化内存使用，增强错误处理）
///
/// # Arguments
/// * `state` - 应用状态
///
/// # Returns
/// * `Result<SystemInfo, String>` - 系统信息或错误信息
#[tauri::command]
pub async fn get_system_info(state: State<'_, crate::AppState>) -> Result<SystemInfo, String> {
    // 异步刷新系统信息
    let system_info = {
        let mut monitor = state.monitor.write().await;
        monitor.refresh().await // 使用异步 refresh 方法
            .map_err(|e| {
                error!("刷新系统信息失败: {}", e);
                e.to_string()
            })? // 转换错误为String
    };

    // 优化内存使用：使用 Arc 共享数据，减少克隆
    let system_info_arc = Arc::new(system_info.clone());
    {
        let mut current_data = state.current_data.write().await;
        *current_data = Some(system_info_arc);
    }

    debug!("系统信息获取成功");
    Ok(system_info)
}

/// 获取GPU信息
///
/// # Arguments
/// * `state` - 应用状态
///
/// # Returns
/// * `Result<Option<GpuInfo>, String>` - GPU信息或错误信息
#[tauri::command]
pub async fn get_gpu_info(state: State<'_, crate::AppState>) -> Result<Option<GpuInfo>, String> {
    let monitor = state.monitor.read().await;
    let gpu_info = monitor.get_gpu_info();
    debug!("GPU信息获取: {:?}", gpu_info.is_some());
    Ok(gpu_info)
}

/// 获取GPU监控状态
///
/// # Arguments
/// * `state` - 应用状态
///
/// # Returns
/// * `Result<(bool, Option<String>), String>` - GPU监控状态和相关信息
#[tauri::command]
pub async fn get_gpu_monitor_status(state: State<'_, crate::AppState>) -> Result<(bool, Option<String>), String> {
    let monitor = state.monitor.read().await;
    let (enabled, error) = monitor.get_gpu_monitor_status();
    debug!("GPU监控状态: 启用={}, 错误={:?}", enabled, error);
    Ok((enabled, error))
}

/// 获取所有GPU名称
///
/// # Arguments
/// * `state` - 应用状态
///
/// # Returns
/// * `Result<Vec<String>, String>` - GPU名称列表或错误信息
#[tauri::command]
pub async fn get_gpu_names(state: State<'_, crate::AppState>) -> Result<Vec<String>, String> {
    let monitor = state.monitor.read().await;
    let gpu_names = monitor.get_gpu_names();
    debug!("获取到 {} 个GPU名称", gpu_names.len());
    Ok(gpu_names)
}

/// 获取详细GPU信息
///
/// # Arguments
/// * `device_index` - GPU设备索引
/// * `state` - 应用状态
///
/// # Returns
/// * `Result<String, String>` - 详细GPU信息或错误信息
#[tauri::command]
pub async fn get_detailed_gpu_info(
    device_index: u32,
    state: State<'_, crate::AppState>
) -> Result<String, String> {
    let monitor = state.monitor.read().await;
    match monitor.get_detailed_gpu_info(device_index) {
        Ok(info) => {
            debug!("获取GPU {} 详细信息成功", device_index);
            Ok(info)
        }
        Err(e) => {
            error!("获取GPU {} 详细信息失败: {}", device_index, e);
            Err(e)
        }
    }
}

/// 获取当前系统数据（优化内存使用）
///
/// # Arguments
/// * `state` - 应用状态
///
/// # Returns
/// * `Result<Option<SystemInfo>, String>` - 当前系统数据或错误信息
#[tauri::command]
pub async fn get_current_data(state: State<'_, crate::AppState>) -> Result<Option<SystemInfo>, String> {
    let current_data = state.current_data.read().await;
    // 如果有数据，返回 Arc 内部数据的克隆，这比完全克隆更高效
    let result = current_data.as_ref().map(|arc| (**arc).clone());
    debug!("获取当前数据: {}", result.is_some());
    Ok(result)
}

/// 获取系统信息增量更新（减少网络传输）
///
/// # Arguments
/// * `state` - 应用状态
///
/// # Returns
/// * `Result<SystemInfoDelta, String>` - 增量更新数据或错误信息
#[tauri::command]
pub async fn get_system_info_delta(state: State<'_, crate::AppState>) -> Result<SystemInfoDelta, String> {
    // 异步刷新系统信息
    let system_info = {
        let mut monitor = state.monitor.write().await;
        monitor.refresh().await
            .map_err(|e| {
                error!("刷新系统信息失败: {}", e);
                e.to_string()
            })?
    };

    // 优化内存使用：使用 Arc 共享数据
    let system_info_arc = Arc::new(system_info.clone());

    // 计算增量更新
    let delta = {
        let last_data = state.last_data.read().await;

        if let Some(ref last) = *last_data {
            // 有历史数据，计算增量
            let delta = SystemInfoDelta::from_diff(last, &system_info);
            debug!("计算增量更新成功");
            delta
        } else {
            // 第一次获取，返回完整数据
            info!("首次获取，返回完整数据");
            SystemInfoDelta::full(system_info.clone())
        }
    };

    // 更新状态
    {
        let mut current_data = state.current_data.write().await;
        let mut last_data = state.last_data.write().await;

        // 保存当前数据
        *current_data = Some(system_info_arc.clone());

        // 更新上一次的数据（用于下次计算增量）
        *last_data = Some(system_info_arc);
    }

    Ok(delta)
}

/// 更新监控配置
///
/// # Arguments
/// * `config` - 新的监控配置
/// * `state` - 应用状态
///
/// # Returns
/// * `Result<(), String>` - 更新成功或错误信息
#[tauri::command]
pub async fn update_monitor_config(
    config: MonitorConfig,
    state: State<'_, crate::AppState>
) -> Result<(), String> {
    let mut monitor = state.monitor.write().await;
    monitor.update_config(config);
    info!("监控配置更新成功");
    Ok(())
}

/// 智能刷新系统信息（包含自适应频率管理）
///
/// # Arguments
/// * `state` - 应用状态
///
/// # Returns
/// * `Result<SystemInfo, String>` - 刷新后的系统信息或错误信息
#[tauri::command]
pub async fn smart_refresh_system_info(state: State<'_, crate::AppState>) -> Result<SystemInfo, String> {
    let mut monitor = state.monitor.write().await;
    match monitor.smart_refresh().await {
        Ok(info) => {
            debug!("智能刷新系统信息成功");
            Ok(info)
        }
        Err(e) => {
            error!("智能刷新系统信息失败: {}", e);
            Err(e)
        }
    }
}

/// 获取建议的刷新间隔
///
/// # Arguments
/// * `state` - 应用状态
///
/// # Returns
/// * `Result<u64, String>` - 建议的刷新间隔（毫秒）或错误信息
#[tauri::command]
pub async fn get_suggested_refresh_interval(state: State<'_, crate::AppState>) -> Result<u64, String> {
    let monitor = state.monitor.read().await;
    let interval = monitor.suggested_refresh_interval().as_millis() as u64;
    debug!("建议刷新间隔: {}ms", interval);
    Ok(interval)
}

/// 获取刷新统计信息
///
/// # Arguments
/// * `state` - 应用状态
///
/// # Returns
/// * `Result<adaptive_refresh::RefreshStatistics, String>` - 刷新统计信息或错误信息
#[tauri::command]
pub async fn get_refresh_statistics(state: State<'_, crate::AppState>) -> Result<adaptive_refresh::RefreshStatistics, String> {
    let monitor = state.monitor.read().await;
    let stats = monitor.get_refresh_statistics();
    debug!("获取刷新统计信息成功");
    Ok(stats)
}

/// 重置刷新统计信息
///
/// # Arguments
/// * `state` - 应用状态
///
/// # Returns
/// * `Result<(), String>` - 重置成功或错误信息
#[tauri::command]
pub async fn reset_refresh_statistics(state: State<'_, crate::AppState>) -> Result<(), String> {
    let _monitor = state.monitor.write().await;
    warn!("刷新统计信息重置功能暂未实现");
    Ok(())
}