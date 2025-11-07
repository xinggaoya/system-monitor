//! 系统托盘管理模块
//!
//! 负责创建和管理系统托盘图标、菜单以及相关事件处理

use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle,
};
use log::{info, debug, error, warn};
use crate::window::{toggle_window_internal, show_settings_window};

/// 创建并初始化系统托盘
///
/// # Arguments
/// * `app` - Tauri 应用句柄
///
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - 创建成功或错误信息
pub fn create_tray(app: &tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // 创建托盘菜单项
    let show_item = MenuItemBuilder::with_id("show", "显示/隐藏").build(app)?;
    let settings_item = MenuItemBuilder::with_id("settings", "设置").build(app)?;
    let quit_item = MenuItemBuilder::with_id("quit", "退出").build(app)?;

    // 构建托盘菜单
    let menu = MenuBuilder::new(app)
        .items(&[&show_item, &settings_item, &quit_item])
        .build()?;

    // 创建托盘图标并配置事件处理
    let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .tooltip("系统监控")
        .on_menu_event(move |app, event| {
            handle_tray_menu_event(app, event);
        })
        .on_tray_icon_event(|tray, event| {
            handle_tray_icon_event(tray, event);
        })
        .build(app)?;

    info!("系统托盘创建成功");
    Ok(())
}

/// 处理托盘菜单事件
///
/// # Arguments
/// * `app` - Tauri 应用句柄
/// * `event` - 菜单事件
fn handle_tray_menu_event(app: &AppHandle, event: tauri::menu::MenuEvent) {
    match event.id.as_ref() {
        "show" => {
            // 执行切换窗口显示/隐藏逻辑
            match toggle_window_internal(app) {
                Ok(_) => debug!("托盘菜单切换窗口状态成功"),
                Err(e) => error!("托盘菜单切换窗口状态失败: {}", e),
            }
        }
        "settings" => {
            // 显示设置窗口（支持重建）
            let app_handle = app.clone();
            tauri::async_runtime::spawn(async move {
                match show_settings_window(app_handle).await {
                    Ok(_) => debug!("托盘菜单打开设置窗口成功"),
                    Err(e) => error!("托盘菜单打开设置窗口失败: {}", e),
                }
            });
        }
        "quit" => {
            info!("通过托盘菜单退出应用");
            app.exit(0);
        }
        _ => {}
    }
}

/// 处理托盘图标点击事件
///
/// # Arguments
/// * `tray` - 托盘图标引用
/// * `event` - 托盘事件
fn handle_tray_icon_event(tray: &tauri::tray::TrayIcon, event: TrayIconEvent) {
    if let TrayIconEvent::Click {
        button: MouseButton::Left,
        button_state: MouseButtonState::Up,
        ..
    } = event
    {
        let app = tray.app_handle();
        debug!("托盘图标被点击，切换窗口状态");
        match toggle_window_internal(app) {
            Ok(_) => debug!("托盘图标切换窗口状态成功"),
            Err(e) => error!("托盘图标切换窗口状态失败: {}", e),
        }
    }
}