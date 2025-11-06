mod models;
mod monitor;
mod gpu_monitor;
mod errors;
mod retry;
mod adaptive_refresh;

use models::*;
use monitor::SystemMonitor;
use std::sync::Arc;
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, State,
};
use tokio::sync::RwLock;

/// 应用状态（优化内存使用，异步安全，支持增量更新）
pub struct AppState {
    pub monitor: Arc<RwLock<SystemMonitor>>,
    /// 使用 Arc 共享数据，避免不必要的克隆
    pub current_data: Arc<RwLock<Option<Arc<SystemInfo>>>>,
    /// 上一次的数据，用于计算增量更新
    pub last_data: Arc<RwLock<Option<Arc<SystemInfo>>>>,
}

// 获取系统信息（异步版本，提升性能，优化内存使用，增强错误处理）
#[tauri::command]
async fn get_system_info(state: State<'_, AppState>) -> Result<SystemInfo, String> {
    let system_info = {
        let mut monitor = state.monitor.write().await;
        monitor.refresh().await // 使用异步 refresh 方法
            .map_err(|e| e.to_string())? // 转换错误为String
    };

    // 优化内存使用：使用 Arc 共享数据，减少克隆
    let system_info_arc = Arc::new(system_info.clone());
    {
        let mut current_data = state.current_data.write().await;
        *current_data = Some(system_info_arc);
    }

    Ok(system_info)
}

// 获取GPU信息
#[tauri::command]
async fn get_gpu_info(state: State<'_, AppState>) -> Result<Option<GpuInfo>, String> {
    let monitor = state.monitor.read().await;
    Ok(monitor.get_gpu_info())
}

// 获取GPU监控状态
#[tauri::command]
async fn get_gpu_monitor_status(state: State<'_, AppState>) -> Result<(bool, Option<String>), String> {
    let monitor = state.monitor.read().await;
    Ok(monitor.get_gpu_monitor_status())
}

// 获取所有GPU名称
#[tauri::command]
async fn get_gpu_names(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let monitor = state.monitor.read().await;
    Ok(monitor.get_gpu_names())
}

// 获取详细GPU信息
#[tauri::command]
async fn get_detailed_gpu_info(
    device_index: u32,
    state: State<'_, AppState>
) -> Result<String, String> {
    let monitor = state.monitor.read().await;
    monitor.get_detailed_gpu_info(device_index)
}

// 获取当前系统数据（优化内存使用）
#[tauri::command]
async fn get_current_data(state: State<'_, AppState>) -> Result<Option<SystemInfo>, String> {
    let current_data = state.current_data.read().await;
    // 如果有数据，返回 Arc 内部数据的克隆，这比完全克隆更高效
    Ok(current_data.as_ref().map(|arc| (**arc).clone()))
}

// 获取系统信息增量更新（减少网络传输）
#[tauri::command]
async fn get_system_info_delta(state: State<'_, AppState>) -> Result<SystemInfoDelta, String> {
    let system_info = {
        let mut monitor = state.monitor.write().await;
        monitor.refresh().await
            .map_err(|e| e.to_string())?
    };

    // 优化内存使用：使用 Arc 共享数据
    let system_info_arc = Arc::new(system_info.clone());

    // 计算增量更新
    let delta = {
        let last_data = state.last_data.read().await;

        if let Some(ref last) = *last_data {
            // 有历史数据，计算增量
            SystemInfoDelta::from_diff(&**last, &system_info)
        } else {
            // 第一次获取，返回完整数据
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

// 更新监控配置
#[tauri::command]
async fn update_monitor_config(
    config: MonitorConfig,
    state: State<'_, AppState>
) -> Result<(), String> {
    let mut monitor = state.monitor.write().await;
    monitor.update_config(config);
    Ok(())
}

// 智能刷新系统信息（包含自适应频率管理）
#[tauri::command]
async fn smart_refresh_system_info(state: State<'_, AppState>) -> Result<SystemInfo, String> {
    let mut monitor = state.monitor.write().await;
    monitor.smart_refresh().await
}

// 获取建议的刷新间隔
#[tauri::command]
async fn get_suggested_refresh_interval(state: State<'_, AppState>) -> Result<u64, String> {
    let monitor = state.monitor.read().await;
    Ok(monitor.suggested_refresh_interval().as_millis() as u64)
}

// 获取刷新统计信息
#[tauri::command]
async fn get_refresh_statistics(state: State<'_, AppState>) -> Result<adaptive_refresh::RefreshStatistics, String> {
    let monitor = state.monitor.read().await;
    Ok(monitor.get_refresh_statistics())
}

// 内部切换窗口显示/隐藏状态函数
fn toggle_window_internal(app_handle: &tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app_handle.get_webview_window("main") {
        match window.is_visible() {
            Ok(visible) => {
                if visible {
                    window.hide().map_err(|e| {
                        eprintln!("隐藏窗口失败: {}", e);
                        e.to_string()
                    })?;
                    println!("窗口已隐藏");
                } else {
                    window.unminimize().map_err(|e| {
                        eprintln!("取消最小化失败: {}", e);
                        e.to_string()
                    })?;
                    window.show().map_err(|e| {
                        eprintln!("显示窗口失败: {}", e);
                        e.to_string()
                    })?;
                    window.set_focus().map_err(|e| {
                        eprintln!("设置焦点失败: {}", e);
                        e.to_string()
                    })?;
                    println!("窗口已显示并获得焦点");
                }
            }
            Err(e) => {
                eprintln!("检查窗口可见性失败: {}", e);
                return Err(e.to_string());
            }
        }
    } else {
        eprintln!("找不到主窗口");
        return Err("找不到主窗口".to_string());
    }
    Ok(())
}

// 显示/隐藏主窗口
#[tauri::command]
async fn toggle_window(app_handle: tauri::AppHandle) -> Result<(), String> {
    toggle_window_internal(&app_handle)
}

// 显示设置窗口（支持窗口重建）
#[tauri::command]
async fn show_settings_window(app_handle: tauri::AppHandle) -> Result<(), String> {
    // 尝试获取现有窗口
    if let Some(window) = app_handle.get_webview_window("settings") {
        match window.is_visible() {
            Ok(visible) => {
                if visible {
                    window.set_focus().map_err(|e| {
                        eprintln!("设置窗口聚焦失败: {}", e);
                        e.to_string()
                    })?;
                    println!("设置窗口已获得焦点");
                } else {
                    window.unminimize().map_err(|e| {
                        eprintln!("设置窗口取消最小化失败: {}", e);
                        e.to_string()
                    })?;
                    window.show().map_err(|e| {
                        eprintln!("显示设置窗口失败: {}", e);
                        e.to_string()
                    })?;
                    window.set_focus().map_err(|e| {
                        eprintln!("设置窗口聚焦失败: {}", e);
                        e.to_string()
                    })?;
                    println!("设置窗口已显示并获得焦点");
                }
                return Ok(());
            }
            Err(e) => {
                eprintln!("检查设置窗口可见性失败: {}", e);
                // 尝试重建窗口
            }
        }
    }

    // 窗口不存在或已被销毁，需要重新创建
    println!("设置窗口不存在，正在创建新窗口...");

    use tauri::{WebviewWindowBuilder, WebviewUrl};

    let webview_url = WebviewUrl::App("/settings".into());
    let window = WebviewWindowBuilder::new(&app_handle, "settings", webview_url)
        .title("系统监控设置")
        .inner_size(800.0, 600.0)
        .min_inner_size(600.0, 400.0)
        .resizable(true)
        .decorations(true)
        .shadow(true)
        .center()
        .build()
        .map_err(|e| {
            eprintln!("创建设置窗口失败: {}", e);
            e.to_string()
        })?;

    window.show().map_err(|e| {
        eprintln!("显示新创建的设置窗口失败: {}", e);
        e.to_string()
    })?;

    window.set_focus().map_err(|e| {
        eprintln!("设置新创建的设置窗口焦点失败: {}", e);
        e.to_string()
    })?;

    println!("设置窗口创建成功并已显示");
    Ok(())
}

// 关闭设置窗口
#[tauri::command]
async fn close_settings_window(app_handle: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app_handle.get_webview_window("settings") {
        window.hide().map_err(|e| {
            eprintln!("隐藏设置窗口失败: {}", e);
            e.to_string()
        })?;
        println!("设置窗口已隐藏");
    } else {
        eprintln!("找不到设置窗口");
        return Err("找不到设置窗口".to_string());
    }
    Ok(())
}

// 退出应用
#[tauri::command]
async fn quit_app(app_handle: tauri::AppHandle) -> Result<(), String> {
    app_handle.exit(0);
    Ok(())
}

// Store 相关命令
use tauri_plugin_store::StoreExt;

// 保存设置到Store
#[tauri::command]
async fn save_settings(app_handle: tauri::AppHandle, key: String, value: serde_json::Value) -> Result<(), String> {
    let store = app_handle.store("settings.json").map_err(|e| e.to_string())?;
    store.set(&key, value);
    store.save().map_err(|e| e.to_string())?;
    Ok(())
}

// 从Store获取设置
#[tauri::command]
async fn get_settings(app_handle: tauri::AppHandle, key: String) -> Result<Option<serde_json::Value>, String> {
    let store = app_handle.store("settings.json").map_err(|e| e.to_string())?;
    Ok(store.get(&key))
}

// 获取所有设置
#[tauri::command]
async fn get_all_settings(app_handle: tauri::AppHandle) -> Result<std::collections::HashMap<String, serde_json::Value>, String> {
    let store = app_handle.store("settings.json").map_err(|e| e.to_string())?;
    let mut settings = std::collections::HashMap::new();

    // 获取Store中所有的键值对
    let store_data = store.entries();
    for (key, value) in store_data {
        settings.insert(key.clone(), value);
    }

    Ok(settings)
}

// 删除设置
#[tauri::command]
async fn delete_settings(app_handle: tauri::AppHandle, key: String) -> Result<(), String> {
    let store = app_handle.store("settings.json").map_err(|e| e.to_string())?;
    store.delete(&key);
    store.save().map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();

    // 基础插件
    builder = builder.plugin(tauri_plugin_opener::init());

    // 集成单例插件，防止多开应用
    // #[cfg(desktop)]
    // {
    //     builder = builder.plugin(tauri_plugin_single_instance::init(|app, args, cwd| {
    //         let _ = app.get_webview_window("main")
    //             .expect("no main window")
    //             .set_focus();
    //     }));
    // }

    // 集成窗口状态插件，实现窗口状态持久化
    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_window_state::Builder::default().build());
    }

    // 集成Store插件，用于数据持久化存储
    builder = builder.plugin(tauri_plugin_store::Builder::new().build());

    // 集成自动启动插件
    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_autostart::Builder::new().build());
    }

    builder
        .setup(|app| {
            // 创建系统监控器
            let monitor = SystemMonitor::new(MonitorConfig::default());

            // 创建应用状态（优化内存使用，异步安全，支持增量更新）
            let app_state = AppState {
                monitor: Arc::new(RwLock::new(monitor)),
                current_data: Arc::new(RwLock::new(None)),
                last_data: Arc::new(RwLock::new(None)),
            };

            // 创建托盘菜单
            let show_item = MenuItemBuilder::with_id("show", "显示/隐藏").build(app)?;
            let settings_item = MenuItemBuilder::with_id("settings", "设置").build(app)?;
            let quit_item = MenuItemBuilder::with_id("quit", "退出").build(app)?;
            let menu = MenuBuilder::new(app)
                .items(&[&show_item, &settings_item, &quit_item])
                .build()?;

            // 创建托盘图标
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .tooltip("系统监控")
                .on_menu_event(move |app, event| match event.id.as_ref() {
                    "show" => {
                        // 执行切换窗口显示/隐藏逻辑
                        match toggle_window_internal(app) {
                            Ok(_) => println!("托盘菜单切换窗口状态成功"),
                            Err(e) => eprintln!("托盘菜单切换窗口状态失败: {}", e),
                        }
                    }
                    "settings" => {
                        // 显示设置窗口（支持重建）
                        let app_handle = app.clone();
                        tauri::async_runtime::spawn(async move {
                            match show_settings_window(app_handle).await {
                                Ok(_) => println!("托盘菜单打开设置窗口成功"),
                                Err(e) => eprintln!("托盘菜单打开设置窗口失败: {}", e),
                            }
                        });
                    }
                    "quit" => {
                        println!("通过托盘菜单退出应用");
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        println!("托盘图标被点击，切换窗口状态");
                        match toggle_window_internal(app) {
                            Ok(_) => println!("托盘图标切换窗口状态成功"),
                            Err(e) => eprintln!("托盘图标切换窗口状态失败: {}", e),
                        }
                    }
                })
                .build(app)?;

            // 管理应用状态
            app.manage(app_state);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_system_info,
            get_gpu_info,
            get_gpu_monitor_status,
            get_gpu_names,
            get_detailed_gpu_info,
            get_current_data,
            get_system_info_delta,
            update_monitor_config,
            smart_refresh_system_info,
            get_suggested_refresh_interval,
            get_refresh_statistics,
            toggle_window,
            show_settings_window,
            close_settings_window,
            quit_app,
            // Store 相关命令
            save_settings,
            get_settings,
            get_all_settings,
            delete_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
