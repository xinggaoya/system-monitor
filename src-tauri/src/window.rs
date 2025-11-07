//! 窗口管理模块
//!
//! 负责管理应用窗口的显示、隐藏、创建和销毁等操作

use tauri::{WebviewWindowBuilder, WebviewUrl, Manager};
use log::{debug, error};

/// 切换主窗口的显示/隐藏状态
///
/// # Arguments
/// * `app_handle` - Tauri 应用句柄
///
/// # Returns
/// * `Result<(), String>` - 操作成功或错误信息
pub fn toggle_window_internal(app_handle: &tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app_handle.get_webview_window("main") {
        match window.is_visible() {
            Ok(visible) => {
                if visible {
                    // 当前窗口可见，执行隐藏操作
                    window.hide().map_err(|e| {
                        error!("隐藏窗口失败: {}", e);
                        e.to_string()
                    })?;
                    debug!("窗口已隐藏");
                } else {
                    // 当前窗口隐藏，执行显示操作
                    window.unminimize().map_err(|e| {
                        error!("取消最小化失败: {}", e);
                        e.to_string()
                    })?;
                    window.show().map_err(|e| {
                        error!("显示窗口失败: {}", e);
                        e.to_string()
                    })?;
                    window.set_focus().map_err(|e| {
                        error!("设置焦点失败: {}", e);
                        e.to_string()
                    })?;
                    debug!("窗口已显示并获得焦点");
                }
            }
            Err(e) => {
                error!("检查窗口可见性失败: {}", e);
                return Err(e.to_string());
            }
        }
    } else {
        error!("找不到主窗口");
        return Err("找不到主窗口".to_string());
    }
    Ok(())
}

/// Tauri 命令：切换主窗口显示状态
///
/// # Arguments
/// * `app_handle` - Tauri 应用句柄
///
/// # Returns
/// * `Result<(), String>` - 操作成功或错误信息
#[tauri::command]
pub async fn toggle_window(app_handle: tauri::AppHandle) -> Result<(), String> {
    toggle_window_internal(&app_handle)
}

/// 显示设置窗口（支持窗口重建）
///
/// # Arguments
/// * `app_handle` - Tauri 应用句柄
///
/// # Returns
/// * `Result<(), String>` - 操作成功或错误信息
#[tauri::command]
pub async fn show_settings_window(app_handle: tauri::AppHandle) -> Result<(), String> {
    // 尝试获取现有设置窗口
    if let Some(window) = app_handle.get_webview_window("settings") {
        match window.is_visible() {
            Ok(visible) => {
                return if visible {
                    // 窗口已可见，直接获得焦点
                    window.set_focus().map_err(|e| {
                        error!("设置窗口聚焦失败: {}", e);
                        e.to_string()
                    })?;
                    debug!("设置窗口已获得焦点");
                    Ok(())
                } else {
                    // 窗口隐藏，取消最小化并显示
                    window.unminimize().map_err(|e| {
                        error!("设置窗口取消最小化失败: {}", e);
                        e.to_string()
                    })?;
                    window.show().map_err(|e| {
                        error!("显示设置窗口失败: {}", e);
                        e.to_string()
                    })?;
                    window.set_focus().map_err(|e| {
                        error!("设置窗口聚焦失败: {}", e);
                        e.to_string()
                    })?;
                    debug!("设置窗口已显示并获得焦点");
                    Ok(())
                }
            }
            Err(e) => {
                error!("检查设置窗口可见性失败: {}", e);
                // 继续创建新窗口
            }
        }
    }

    // 窗口不存在或已被销毁，需要重新创建
    debug!("设置窗口不存在，正在创建新窗口...");

    // 配置设置窗口属性
    let webview_url = WebviewUrl::App("/settings".into());
    let window = WebviewWindowBuilder::new(&app_handle, "settings", webview_url)
        .title("系统监控设置")
        .inner_size(800.0, 600.0)          // 初始大小
        .min_inner_size(600.0, 400.0)      // 最小大小
        .resizable(true)                   // 可调整大小
        .decorations(true)                 // 显示标题栏
        .shadow(true)                      // 显示阴影
        .center()                          // 居中显示
        .build()
        .map_err(|e| {
            error!("创建设置窗口失败: {}", e);
            e.to_string()
        })?;

    // 显示新创建的窗口
    window.show().map_err(|e| {
        error!("显示新创建的设置窗口失败: {}", e);
        e.to_string()
    })?;

    // 设置窗口焦点
    window.set_focus().map_err(|e| {
        error!("设置新创建的设置窗口焦点失败: {}", e);
        e.to_string()
    })?;

    debug!("设置窗口创建成功并已显示");
    Ok(())
}

/// 关闭设置窗口
///
/// # Arguments
/// * `app_handle` - Tauri 应用句柄
///
/// # Returns
/// * `Result<(), String>` - 操作成功或错误信息
#[tauri::command]
pub async fn close_settings_window(app_handle: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app_handle.get_webview_window("settings") {
        window.hide().map_err(|e| {
            error!("隐藏设置窗口失败: {}", e);
            e.to_string()
        })?;
        debug!("设置窗口已隐藏");
    } else {
        error!("找不到设置窗口");
        return Err("找不到设置窗口".to_string());
    }
    Ok(())
}

/// 应用窗口偏好（置顶 / 任务栏可见等）
///
/// # Arguments
/// * `app_handle` - 应用句柄
/// * `always_on_top` - 是否置顶
/// * `show_in_taskbar` - 是否在任务栏显示窗口图标
#[tauri::command]
pub async fn apply_window_preferences(
    app_handle: tauri::AppHandle,
    always_on_top: Option<bool>,
    show_in_taskbar: Option<bool>,
) -> Result<(), String> {
    let Some(window) = app_handle.get_webview_window("main") else {
        error!("应用窗口偏好失败: 找不到主窗口");
        return Err("找不到主窗口".into());
    };

    if let Some(value) = always_on_top {
        window.set_always_on_top(value).map_err(|e| {
            error!("设置置顶状态失败: {}", e);
            e.to_string()
        })?;
        debug!("窗口置顶状态更新为 {}", value);
    }

    if let Some(show) = show_in_taskbar {
        window.set_skip_taskbar(!show).map_err(|e| {
            error!("设置任务栏可见失败: {}", e);
            e.to_string()
        })?;
        debug!("窗口任务栏可见性更新为 {}", show);
    }

    Ok(())
}

/// 退出应用程序
///
/// # Arguments
/// * `app_handle` - Tauri 应用句柄
///
/// # Returns
/// * `Result<(), String>` - 操作成功或错误信息
#[tauri::command]
pub async fn quit_app(app_handle: tauri::AppHandle) -> Result<(), String> {
    app_handle.exit(0);
    Ok(())
}
