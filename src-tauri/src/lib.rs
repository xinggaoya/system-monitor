mod models;
mod monitor;
mod gpu_monitor;

use models::*;
use monitor::SystemMonitor;
use std::sync::{Arc, Mutex};
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, State,
};
use tokio::sync::RwLock;

/// 应用状态
pub struct AppState {
    pub monitor: Arc<Mutex<SystemMonitor>>,
    pub current_data: Arc<RwLock<Option<SystemInfo>>>,
}

// 获取系统信息
#[tauri::command]
async fn get_system_info(state: State<'_, AppState>) -> Result<SystemInfo, String> {
    let system_info = {
        let mut monitor = state.monitor.lock().map_err(|e| e.to_string())?;
        monitor.refresh()
    };

    // 更新当前数据
    let mut current_data = state.current_data.write().await;
    *current_data = Some(system_info.clone());

    Ok(system_info)
}

// 获取GPU信息
#[tauri::command]
async fn get_gpu_info(state: State<'_, AppState>) -> Result<Option<GpuInfo>, String> {
    let monitor = state.monitor.lock().map_err(|e| e.to_string())?;
    Ok(monitor.get_gpu_info())
}

// 获取GPU监控状态
#[tauri::command]
async fn get_gpu_monitor_status(state: State<'_, AppState>) -> Result<(bool, Option<String>), String> {
    let monitor = state.monitor.lock().map_err(|e| e.to_string())?;
    Ok(monitor.get_gpu_monitor_status())
}

// 获取所有GPU名称
#[tauri::command]
async fn get_gpu_names(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let monitor = state.monitor.lock().map_err(|e| e.to_string())?;
    Ok(monitor.get_gpu_names())
}

// 获取详细GPU信息
#[tauri::command]
async fn get_detailed_gpu_info(
    device_index: u32,
    state: State<'_, AppState>
) -> Result<String, String> {
    let monitor = state.monitor.lock().map_err(|e| e.to_string())?;
    monitor.get_detailed_gpu_info(device_index)
}

// 获取当前系统数据
#[tauri::command]
async fn get_current_data(state: State<'_, AppState>) -> Result<Option<SystemInfo>, String> {
    let current_data = state.current_data.read().await;
    Ok(current_data.clone())
}

// 更新监控配置
#[tauri::command]
async fn update_monitor_config(
    config: MonitorConfig,
    state: State<'_, AppState>
) -> Result<(), String> {
    let mut monitor = state.monitor.lock().map_err(|e| e.to_string())?;
    monitor.update_config(config);
    Ok(())
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

// 退出应用
#[tauri::command]
async fn quit_app(app_handle: tauri::AppHandle) -> Result<(), String> {
    app_handle.exit(0);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // 创建系统监控器
            let monitor = SystemMonitor::new(MonitorConfig::default());
            let monitor_arc = Arc::new(Mutex::new(monitor));

            // 创建应用状态
            let app_state = AppState {
                monitor: monitor_arc.clone(),
                current_data: Arc::new(RwLock::new(None)),
            };

            // 创建托盘菜单
            let show_item = MenuItemBuilder::with_id("show", "显示/隐藏").build(app)?;
            let quit_item = MenuItemBuilder::with_id("quit", "退出").build(app)?;
            let menu = MenuBuilder::new(app)
                .items(&[&show_item, &quit_item])
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
            update_monitor_config,
            toggle_window,
            quit_app,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
