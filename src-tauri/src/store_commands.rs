//! 存储相关命令模块
//!
//! 负责处理应用设置和数据的持久化存储操作

use tauri::AppHandle;
use tauri_plugin_store::StoreExt;
use serde_json::Value;
use std::collections::HashMap;
use log::{info, debug, error, warn};

/// 保存设置到Store
///
/// # Arguments
/// * `app_handle` - Tauri 应用句柄
/// * `key` - 设置项的键名
/// * `value` - 要保存的设置值
///
/// # Returns
/// * `Result<(), String>` - 保存成功或错误信息
#[tauri::command]
pub async fn save_settings(
    app_handle: AppHandle,
    key: String,
    value: Value,
) -> Result<(), String> {
    // 获取或创建存储实例
    let store = app_handle.store("settings.json")
        .map_err(|e| {
            error!("获取存储实例失败: {}", e);
            e.to_string()
        })?;

    // 设置键值对
    store.set(&key, value);

    // 保存到磁盘
    store.save()
        .map_err(|e| {
            error!("保存设置失败: {}", e);
            e.to_string()
        })?;

    debug!("设置 '{}' 已成功保存", key);
    Ok(())
}

/// 从Store获取单个设置项
///
/// # Arguments
/// * `app_handle` - Tauri 应用句柄
/// * `key` - 要获取的设置项键名
///
/// # Returns
/// * `Result<Option<Value>, String>` - 设置值或错误信息
#[tauri::command]
pub async fn get_settings(
    app_handle: AppHandle,
    key: String,
) -> Result<Option<Value>, String> {
    // 获取存储实例
    let store = app_handle.store("settings.json")
        .map_err(|e| {
            error!("获取存储实例失败: {}", e);
            e.to_string()
        })?;

    // 获取指定键的值
    let value = store.get(&key);
    debug!("获取设置 '{}': {:?}", key, value);

    Ok(value)
}

/// 获取所有设置项
///
/// # Arguments
/// * `app_handle` - Tauri 应用句柄
///
/// # Returns
/// * `Result<HashMap<String, Value>, String>` - 所有设置项或错误信息
#[tauri::command]
pub async fn get_all_settings(
    app_handle: AppHandle,
) -> Result<HashMap<String, Value>, String> {
    // 获取存储实例
    let store = app_handle.store("settings.json")
        .map_err(|e| {
            error!("获取存储实例失败: {}", e);
            e.to_string()
        })?;

    // 获取所有键值对
    let store_data = store.entries();
    let mut settings = HashMap::new();

    // 转换为 HashMap 格式
    for (key, value) in store_data {
        settings.insert(key.clone(), value);
    }

    info!("获取所有设置，共 {} 项", settings.len());
    Ok(settings)
}

/// 删除指定设置项
///
/// # Arguments
/// * `app_handle` - Tauri 应用句柄
/// * `key` - 要删除的设置项键名
///
/// # Returns
/// * `Result<(), String>` - 删除成功或错误信息
#[tauri::command]
pub async fn delete_settings(
    app_handle: AppHandle,
    key: String,
) -> Result<(), String> {
    // 获取存储实例
    let store = app_handle.store("settings.json")
        .map_err(|e| {
            error!("获取存储实例失败: {}", e);
            e.to_string()
        })?;

    // 检查键是否存在
    if store.get(&key).is_some() {
        // 删除指定键
        store.delete(&key);

        // 保存更改到磁盘
        store.save()
            .map_err(|e| {
                error!("保存更改失败: {}", e);
                e.to_string()
            })?;

        debug!("设置 '{}' 已成功删除", key);
    } else {
        debug!("设置 '{}' 不存在，跳过删除", key);
    }

    Ok(())
}

/// 批量更新设置项
///
/// # Arguments
/// * `app_handle` - Tauri 应用句柄
/// * `settings` - 要更新的设置项键值对
///
/// # Returns
/// * `Result<(), String>` - 更新成功或错误信息
#[tauri::command]
pub async fn update_multiple_settings(
    app_handle: AppHandle,
    settings: HashMap<String, Value>,
) -> Result<(), String> {
    // 获取存储实例
    let store = app_handle.store("settings.json")
        .map_err(|e| {
            error!("获取存储实例失败: {}", e);
            e.to_string()
        })?;

    // 批量更新设置项
    for (key, value) in settings {
        store.set(&key, value);
    }

    // 保存更改到磁盘
    store.save()
        .map_err(|e| {
            error!("保存批量设置失败: {}", e);
            e.to_string()
        })?;

    info!("批量设置更新成功，共 {} 项", store.entries().len());
    Ok(())
}

/// 清空所有设置
///
/// # Arguments
/// * `app_handle` - Tauri 应用句柄
///
/// # Returns
/// * `Result<(), String>` - 清空成功或错误信息
#[tauri::command]
pub async fn clear_all_settings(app_handle: AppHandle) -> Result<(), String> {
    // 获取存储实例
    let store = app_handle.store("settings.json")
        .map_err(|e| {
            error!("获取存储实例失败: {}", e);
            e.to_string()
        })?;

    // 获取所有键并清空
    let entries = store.entries();
    for (key, _) in entries {
        store.delete(&key);
    }

    // 保存更改到磁盘
    store.save()
        .map_err(|e| {
            error!("清空设置保存失败: {}", e);
            e.to_string()
        })?;

    info!("所有设置已清空");
    Ok(())
}