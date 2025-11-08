//! 应用初始化和配置模块
//!
//! 负责应用的初始化、插件配置和应用状态管理

use crate::models::*;
use crate::monitor::SystemMonitor;
use crate::store_commands;
use crate::system_commands;
use crate::tray;
use crate::window;
use log::info;
use std::sync::Arc;
use tauri::Manager;
use tokio::sync::RwLock;

/// 应用状态（优化内存使用，异步安全，支持增量更新）
pub struct AppState {
    pub monitor: Arc<RwLock<SystemMonitor>>,
    /// 使用 Arc 共享数据，避免不必要的克隆
    pub current_data: Arc<RwLock<Option<Arc<SystemInfo>>>>,
    /// 上一次的数据，用于计算增量更新
    pub last_data: Arc<RwLock<Option<Arc<SystemInfo>>>>,
}

impl AppState {
    /// 创建新的应用状态实例
    ///
    /// # Arguments
    /// * `config` - 监控配置
    ///
    /// # Returns
    /// * `AppState` - 新的应用状态实例
    pub fn new(config: MonitorConfig) -> Self {
        let monitor = SystemMonitor::new(config);
        Self {
            monitor: Arc::new(RwLock::new(monitor)),
            current_data: Arc::new(RwLock::new(None)),
            last_data: Arc::new(RwLock::new(None)),
        }
    }
}

/// 配置和初始化应用插件
///
/// # Arguments
/// * `builder` - Tauri 应用构建器
///
/// # Returns
/// * `tauri::Builder<tauri::Wry>` - 配置完成的应用构建器
pub fn configure_plugins(mut builder: tauri::Builder<tauri::Wry>) -> tauri::Builder<tauri::Wry> {
    // 基础插件
    builder = builder.plugin(tauri_plugin_opener::init());

    // 配置日志插件
    builder = builder.plugin(
        tauri_plugin_log::Builder::new()
            .level(log::LevelFilter::Info) // 设置日志级别
            .level_for("system_monitor_tray", log::LevelFilter::Debug) // 托盘模块使用详细日志
            .level_for("system_monitor_window", log::LevelFilter::Debug) // 窗口模块使用详细日志
            .level_for("system_monitor_system", log::LevelFilter::Info) // 系统模块使用信息级别
            .level_for("system_monitor_store", log::LevelFilter::Debug) // 存储模块使用详细日志
            .target(tauri_plugin_log::Target::new(
                tauri_plugin_log::TargetKind::LogDir {
                    file_name: Some("system_monitor".to_string()),
                },
            )) // 输出到日志目录
            .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal) // 使用本地时区
            .format(|out, message, record| {
                // 自定义日志格式：[时间] [级别] [模块] 消息
                out.finish(format_args!(
                    "[{} {} {}] {}",
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                    record.level(),
                    record.target(),
                    message
                ))
            })
            .build(),
    );
    info!("日志插件已配置");

    // 集成窗口状态插件，实现窗口状态持久化
    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_window_state::Builder::default().build());
        info!("窗口状态插件已初始化");
    }

    // 集成Store插件，用于数据持久化存储
    builder = builder.plugin(tauri_plugin_store::Builder::new().build());
    info!("Store插件已初始化");

    // 集成自动启动插件
    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_autostart::Builder::new().build());
        info!("自动启动插件已初始化");
    }

    // 注：单例插件暂时注释，如需要可以取消注释
    // #[cfg(desktop)]
    // {
    //     builder = builder.plugin(tauri_plugin_single_instance::init(|app, args, cwd| {
    //         let _ = app.get_webview_window("main")
    //             .expect("no main window")
    //             .set_focus();
    //     }));
    //     println!("单例插件已初始化");
    // }

    builder
}

/// 初始化应用
///
/// # Arguments
/// * `app` - Tauri 应用句柄
///
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - 初始化成功或错误信息
pub fn initialize_app(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    info!("开始初始化应用...");

    // 创建应用状态
    let app_state = AppState::new(MonitorConfig::default());
    info!("应用状态创建成功");

    // 创建系统托盘
    tray::create_tray(app.handle())?;
    info!("系统托盘初始化成功");

    // 管理应用状态
    app.manage(app_state);
    info!("应用状态管理成功");

    info!("应用初始化完成");
    Ok(())
}

/// 运行应用程序
///
/// 这是应用的主入口点，负责创建和运行整个应用程序
pub fn run() {
    info!("启动系统监控应用...");

    // 配置插件
    let builder = configure_plugins(tauri::Builder::default());

    builder
        .invoke_handler(tauri::generate_handler![
            // 系统信息相关命令
            system_commands::get_system_info,
            system_commands::get_gpu_info,
            system_commands::get_gpu_monitor_status,
            system_commands::get_gpu_names,
            system_commands::get_detailed_gpu_info,
            system_commands::get_frame_stats,
            system_commands::get_current_data,
            system_commands::get_system_info_delta,
            system_commands::update_monitor_config,
            system_commands::smart_refresh_system_info,
            system_commands::get_suggested_refresh_interval,
            system_commands::get_refresh_statistics,
            system_commands::reset_refresh_statistics,
            // 窗口管理相关命令
            window::toggle_window,
            window::show_settings_window,
            window::close_settings_window,
            window::apply_window_preferences,
            window::quit_app,
            // 存储相关命令
            store_commands::save_settings,
            store_commands::get_settings,
            store_commands::get_all_settings,
            store_commands::delete_settings,
            store_commands::update_multiple_settings,
            store_commands::clear_all_settings,
        ])
        .setup(|app| initialize_app(app))
        .run(tauri::generate_context!())
        .expect("运行 Tauri 应用时发生错误");
}
