//! 系统监控应用主入口模块
//!
//! 这是应用程序的主入口点，负责模块的声明和应用的启动。

// 声明所有模块
mod models;
mod monitor;
mod gpu_monitor;
mod errors;
mod retry;
mod adaptive_refresh;
mod tray;
mod window;
mod store_commands;
mod system_commands;
mod app;

// 重新导出应用状态，供其他模块使用
pub use app::AppState;

// Tauri 应用入口点
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 调用应用模块的 run 函数启动应用
    app::run();
}
