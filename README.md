# System Monitor 系统监控

[English](#english) | [中文](#中文)

---

## 中文

一个基于 Tauri 2.2 + Vue 3 构建的跨平台桌面系统监控应用，提供实时的 CPU、内存、GPU 和网络监控功能。

![System Monitor](https://img.shields.io/badge/Tauri-2.2-blue.svg)
![Vue](https://img.shields.io/badge/Vue-3.5-green.svg)
![Rust](https://img.shields.io/badge/Rust-1.70-orange.svg)
![License](https://img.shields.io/badge/License-MIT-yellow.svg)

### ✨ 特性

- 🖥️ **实时系统监控**: CPU、内存、GPU 和网络使用情况
- 🎯 **轻量级设计**: 最小化资源占用，悬浮窗显示
- 🔄 **系统托盘集成**: 完整的托盘菜单功能
- 🎨 **现代化界面**: 透明悬浮窗，美观的视觉效果
- ⚡ **高性能**: Rust 后端 + Vue 前端，响应迅速
- 🌐 **跨平台**: 支持 Windows、macOS 和 Linux

### 📸 截图

```
CPU 45% | 内存 62% | GPU -- | 网络
↓1.2MB ↑800KB
```

### 🚀 快速开始

#### 环境要求

- Node.js 18+
- Rust 1.70+
- pnpm (推荐) 或 npm

#### 安装依赖

```bash
# 克隆仓库
git clone https://github.com/xinggaoya/system-monitor.git
cd system-monitor

# 安装前端依赖
pnpm install

# 安装 Tauri CLI
cargo install tauri-cli
```

#### 开发模式

```bash
pnpm tauri dev
```

#### 构建发布版本

```bash
pnpm tauri build
```

### 🏗️ 技术栈

**前端:**
- Vue 3.5 - 现代化前端框架
- TypeScript - 类型安全的 JavaScript
- Pinia - 状态管理
- Vite - 快速构建工具

**后端:**
- Tauri 2.2 - 跨平台桌面应用框架
- Rust - 系统级编程语言
- sysinfo 0.33 - 系统信息获取
- nvml-wrapper - GPU 监控 (NVIDIA)

### 📁 项目结构

```
system-monitor/
├── src/                    # 前端源码
│   ├── components/         # Vue 组件
│   ├── composables/        # 组合式函数
│   ├── stores/            # Pinia 状态管理
│   └── assets/            # 静态资源
├── src-tauri/              # Rust 后端
│   ├── src/               # Rust 源码
│   ├── icons/             # 应用图标
│   └── tauri.conf.json    # Tauri 配置
└── .github/               # GitHub Actions
```

### 🔧 配置说明

应用支持以下配置选项：

- **刷新间隔**: 数据更新频率
- **显示选项**: 选择要监控的系统指标
- **外观设置**: 窗口透明度和位置

### 🐛 故障排除

**GPU 监控不可用:**
- 确保安装了 NVIDIA 驱动
- 检查 NVML 库是否正确安装

**托盘图标不显示:**
- 检查系统托盘设置
- 重启应用程序

**构建失败:**
- 确保所有依赖已正确安装
- 检查 Rust 和 Node.js 版本

### 🤝 贡献

欢迎贡献代码！请遵循以下步骤：

1. Fork 本仓库
2. 创建功能分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 创建 Pull Request

### 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

---

## English

A cross-platform desktop system monitoring application built with Tauri 2.2 + Vue 3, providing real-time CPU, memory, GPU, and network monitoring capabilities.

![System Monitor](https://img.shields.io/badge/Tauri-2.2-blue.svg)
![Vue](https://img.shields.io/badge/Vue-3.5-green.svg)
![Rust](https://img.shields.io/badge/Rust-1.70-orange.svg)
![License](https://img.shields.io/badge/License-MIT-yellow.svg)

### ✨ Features

- 🖥️ **Real-time System Monitoring**: CPU, memory, GPU, and network usage
- 🎯 **Lightweight Design**: Minimal resource usage with floating window display
- 🔄 **System Tray Integration**: Complete tray menu functionality
- 🎨 **Modern Interface**: Transparent floating window with beautiful visual effects
- ⚡ **High Performance**: Rust backend + Vue frontend for responsive experience
- 🌐 **Cross-platform**: Support for Windows, macOS, and Linux

### 📸 Screenshot

```
CPU 45% | Memory 62% | GPU -- | Network
↓1.2MB ↑800KB
```

### 🚀 Quick Start

#### Prerequisites

- Node.js 18+
- Rust 1.70+
- pnpm (recommended) or npm

#### Installation

```bash
# Clone repository
git clone https://github.com/xinggaoya/system-monitor.git
cd system-monitor

# Install frontend dependencies
pnpm install

# Install Tauri CLI
cargo install tauri-cli
```

#### Development Mode

```bash
pnpm tauri dev
```

#### Build Release Version

```bash
pnpm tauri build
```

### 🏗️ Tech Stack

**Frontend:**
- Vue 3.5 - Modern frontend framework
- TypeScript - Type-safe JavaScript
- Pinia - State management
- Vite - Fast build tool

**Backend:**
- Tauri 2.2 - Cross-platform desktop application framework
- Rust - System-level programming language
- sysinfo 0.33 - System information retrieval
- nvml-wrapper - GPU monitoring (NVIDIA)

### 📁 Project Structure

```
system-monitor/
├── src/                    # Frontend source code
│   ├── components/         # Vue components
│   ├── composables/        # Composable functions
│   ├── stores/            # Pinia state management
│   └── assets/            # Static assets
├── src-tauri/              # Rust backend
│   ├── src/               # Rust source code
│   ├── icons/             # Application icons
│   └── tauri.conf.json    # Tauri configuration
└── .github/               # GitHub Actions
```

### 🔧 Configuration

The application supports the following configuration options:

- **Refresh Interval**: Data update frequency
- **Display Options**: Choose system metrics to monitor
- **Appearance Settings**: Window transparency and position

### 🐛 Troubleshooting

**GPU Monitoring Unavailable:**
- Ensure NVIDIA drivers are installed
- Check if NVML library is properly installed

**Tray Icon Not Showing:**
- Check system tray settings
- Restart the application

**Build Failures:**
- Ensure all dependencies are correctly installed
- Check Rust and Node.js versions

### 🤝 Contributing

Contributions are welcome! Please follow these steps:

1. Fork this repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

### 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.