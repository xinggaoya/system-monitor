# System Monitor

<div align="center">

[![Tauri](https://img.shields.io/badge/Tauri-2.2-blue.svg)](https://tauri.app/)
[![Vue](https://img.shields.io/badge/Vue-3.5-green.svg)](https://vuejs.org/)
[![Rust](https://img.shields.io/badge/Rust-1.70-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![GitHub release](https://img.shields.io/github/release/xinggaoya/system-monitor.svg)](https://github.com/xinggaoya/system-monitor/releases)
[![Build Status](https://github.com/xinggaoya/system-monitor/workflows/CI/badge.svg)](https://github.com/xinggaoya/system-monitor/actions)

**Language / 语言:** [English](README.en.md) | [中文](README.md)

A cross-platform desktop system monitoring application built with Tauri 2.2 + Vue 3, providing real-time CPU, memory, GPU, and network monitoring.

[Features](#-features) • [Quick Start](#-quick-start) • [Download & Install](#-download--install) • [User Guide](#-user-guide) • [Tech Stack](#-tech-stack) • [Contributing](#-contributing)

</div>

## ✨ Features

- 🖥️ **Real-time System Monitoring**: Live display of CPU, memory, GPU, and network usage
- 🎯 **Lightweight Design**: Minimal resource usage with elegant floating window display
- 🔄 **System Tray Integration**: Complete tray menu functionality with show/hide and exit options
- 🎨 **Modern Interface**: Transparent floating window with beautiful visual effects and animations
- ⚡ **High Performance**: Rust backend + Vue frontend for responsive performance with low resource usage
- 🌐 **Cross-platform Support**: Support for Windows, macOS, and Linux
- 📊 **Detailed Network Monitoring**: Real-time network traffic monitoring with upload/download speed display
- 🎮 **GPU Monitoring**: NVIDIA GPU monitoring support (requires NVML library)
- 🌡️ **Temperature Breakdown**: Group CPU package/core, memory, and GPU sensors for instant hotspot insights
- 🎯 **Frame Rate Overlay (Planned)**: Module reserved for backend sampling to avoid displaying inaccurate mock data
- 🔧 **Highly Configurable**: Customizable refresh intervals, display options, and appearance settings

## 📸 Screenshots

### Main Interface
```
┌─────────────────────────────────────┐
│ System Monitor                     │
├─────────────────────────────────────┤
│ CPU 45%  Memory 62%  GPU 78%  Net   │
│ ↓1.2MB ↑800KB                     │
│ 🌡️ CPU: 45°C  GPU: 72°C            │
└─────────────────────────────────────┘
```

### System Tray
- Right-click menu: Show/hide main window, exit application
- Left-click: Toggle window visibility
- Real-time system status indicator

## 🚀 Quick Start

### Prerequisites

- **Node.js**: 18+
- **Rust**: 1.70+
- **Package Manager**: pnpm (recommended) or npm / yarn

### Installation

```bash
# Clone the repository
git clone https://github.com/xinggaoya/system-monitor.git
cd system-monitor

# Install frontend dependencies
pnpm install

# Install Tauri CLI (if not already installed)
cargo install tauri-cli --version "^2.0"
```

### Development Mode

```bash
# Start development server
pnpm tauri dev

# Or using npm
npm run tauri dev
```

### Build Release Version

```bash
# Build production version
pnpm tauri build

# Build for specific platforms
pnpm tauri build --target x86_64-pc-windows-msvc  # Windows
pnpm tauri build --target x86_64-apple-darwin      # macOS
pnpm tauri build --target x86_64-unknown-linux-gnu # Linux
```

Build artifacts will be generated in the `src-tauri/target/release/bundle/` directory.

### Windows Frame-Rate Capture (Optional)

The frame overlay relies on [Intel PresentMon](https://www.intel.com/content/www/us/en/download/705483/presentmon.html) for accurate FPS sampling:

1. Download and extract the latest `PresentMon.exe`
2. Add its folder to `PATH`, or set `PRESENTMON_PATH=C:\Tools\PresentMon\PresentMon.exe`
3. Restart System Monitor and enable the “Frame Rate” module under Settings → Display Modules

> Currently only Windows supports frame capture; macOS/Linux will show an “Unavailable” hint instead of mock data.

## 📥 Download & Install

### Pre-built Versions

Download the latest pre-built versions from [GitHub Releases](https://github.com/xinggaoya/system-monitor/releases):

| Platform | Filename | Size |
|----------|----------|------|
| **Windows** | `system_monitor_1.0.2_x64-setup.exe` | ~8MB |
| **macOS** | `system_monitor_1.0.2_x64.dmg` | ~6MB |
| **Linux** | `system_monitor_1.0.2_amd64.AppImage` | ~7MB |

### Installation Instructions

#### Windows
1. Download the `.exe` installer
2. Right-click and "Run as administrator"
3. Follow the installation wizard
4. Launch from Start Menu

#### macOS
1. Download the `.dmg` file
2. Double-click to open the installer
3. Drag the app to Applications folder
4. Launch from Launchpad

#### Linux
1. Download the `.AppImage` file
2. Make it executable: `chmod +x system_monitor_*.AppImage`
3. Run directly: `./system_monitor_*.AppImage`

## 🎯 User Guide

### Basic Operations

1. **Launch Application**: Start from system menu or run directly after installation
2. **Drag Window**: Hold left mouse button to drag to any position
3. **Tray Operations**:
   - Left-click tray icon: Show/hide main window
   - Right-click tray icon: Show menu options

### Monitoring Metrics

| Metric | Description | Display Format |
|--------|-------------|----------------|
| **CPU Usage** | Processor usage percentage | `CPU 45%` |
| **Memory Usage** | Physical memory usage percentage | `Memory 62%` |
| **GPU Usage** | Graphics card usage percentage (NVIDIA) | `GPU 78%` |
| **Network Speed** | Real-time upload/download speed | `↓1.2MB ↑800KB` |
| **Temperature** | CPU and GPU temperature | `🌡️ CPU: 45°C` |

### Configuration Options

The application supports the following configurations (via config file or settings interface):

- **Refresh Interval**: Data update frequency (default 1000ms)
- **Display Options**: Select system metrics to monitor
- **Appearance Settings**: Window transparency, position, and themes
- **Notification Settings**: Alert for abnormal conditions

## 🏗️ Tech Stack

### Frontend Technologies
- **Vue 3.5** - Modern frontend framework with Composition API
- **TypeScript** - Type-safe JavaScript superset
- **Pinia** - Lightweight state management library
- **Vite** - Fast frontend build tool
- **CSS3** - Modern styles with transparency and animation support

### Backend Technologies
- **Tauri 2.2** - Cross-platform desktop application framework
- **Rust** - System-level programming language with memory safety
- **sysinfo 0.33** - System information retrieval library
- **nvml-wrapper** - NVIDIA GPU monitoring library

### Development Tools
- **ESLint** - Code quality checking
- **Prettier** - Code formatting
- **GitHub Actions** - CI/CD automation
- **Clippy** - Rust code quality checking

## 📁 Project Structure

```
system-monitor/
├── README.md                   # Project documentation (Chinese)
├── README.en.md               # Project documentation (English)
├── LICENSE                    # MIT License
├── package.json              # Project dependencies configuration
├── tsconfig.json             # TypeScript configuration
├── vite.config.ts            # Vite build configuration
├── src/                      # Frontend source code
│   ├── main.ts              # Application entry point
│   ├── App.vue              # Main application component
│   ├── components/          # Vue components
│   ├── composables/         # Composable functions
│   ├── stores/              # Pinia state management
│   └── assets/              # Static assets
├── src-tauri/               # Rust backend
│   ├── src/                 # Rust source code
│   │   ├── lib.rs           # Main application logic
│   │   ├── models.rs        # Data model definitions
│   │   ├── monitor.rs       # System monitoring implementation
│   │   └── gpu_monitor.rs   # GPU monitoring implementation
│   ├── Cargo.toml           # Rust dependencies configuration
│   ├── tauri.conf.json      # Tauri application configuration
│   └── icons/               # Application icon resources
└── .github/                 # GitHub configuration
    └── workflows/
        └── ci.yml          # CI/CD automation configuration
```

## 🎮 GPU Monitoring

### NVIDIA GPU Support

The application supports NVIDIA GPU monitoring, requiring the following conditions:

1. **Install NVIDIA Driver**: Ensure the system has the latest NVIDIA graphics driver installed
2. **NVML Library**: The application uses nvml-wrapper for GPU monitoring
3. **System Permissions**: The application needs appropriate system permissions to access GPU information

### Monitoring Metrics

- **GPU Usage**: Real-time GPU compute usage percentage
- **VRAM Usage**: GPU memory usage and total capacity
- **Temperature Monitoring**: GPU core temperature (if hardware supports)
- **Clock Frequency**: GPU core and memory clock frequencies

### Troubleshooting

If GPU monitoring is unavailable:
1. Check if NVIDIA driver is properly installed
2. Confirm NVML library availability
3. Restart the application
4. Check system permission settings

## 🌐 Network Monitoring

### Monitoring Metrics

- **Download Speed**: Real-time network download speed
- **Upload Speed**: Real-time network upload speed
- **Network Interface**: Automatically detects active network interfaces
- **Traffic Statistics**: Cumulative network traffic statistics

### Display Format

```
Real-time speed display:
↓Download Speed ↑Upload Speed
Example: ↓1.2MB ↑800KB

Supported units:
B/s, KB/s, MB/s, GB/s
```

## 🎨 Interface Features

### Floating Window Design

- **Transparent Background**: Semi-transparent background that doesn't obstruct desktop content
- **Always on Top**: Option to always display in foreground
- **Draggable**: Support for mouse dragging to any position
- **Minimal Design**: Occupies minimal screen space
- **Responsive**: Automatically adapts to different screen resolutions

### System Tray Integration

- **Tray Icon**: Application icon displayed in system tray
- **Status Indicator**: Icon shows system running status
- **Right-click Menu**:
  - Show/hide main window
  - Exit application
  - About information
- **Left-click Operation**: Toggle window visibility

## 🐛 Troubleshooting

### Common Issues & Solutions

#### GPU Monitoring Unavailable
**Problem**: GPU data shows empty or "--"
**Solution**:
- Ensure NVIDIA graphics driver is installed
- Check if NVML library is properly installed
- Restart the application
- Run with administrator privileges (Windows)

#### Tray Icon Not Showing
**Problem**: Application icon not visible in system tray
**Solution**:
- Check system tray settings
- Restart the application
- Confirm application has appropriate system permissions
- Check background processes in Task Manager

#### Application Won't Start
**Problem**: No response when double-clicking the icon
**Solution**:
- Check system logs for detailed error information
- Ensure sufficient system permissions
- Try running with administrator privileges (Windows)
- Reinstall the application

#### Build Failures
**Problem**: Build errors in development environment
**Solution**:
- Ensure all dependencies are properly installed
- Check Rust and Node.js versions
- Clear cache: `pnpm store prune` and `cargo clean`
- Update Tauri CLI: `cargo install tauri-cli --force`

### Debug Mode

Enable debug mode for detailed logging:

```bash
# Windows
set RUST_LOG=debug && pnpm tauri dev

# macOS/Linux
RUST_LOG=debug pnpm tauri dev

# Or create log file
RUST_LOG=debug pnpm tauri dev > system-monitor.log 2>&1
```

### Performance Monitoring

Monitor the application's own resource usage:

```bash
# Windows
tasklist | findstr system_monitor

# macOS/Linux
ps aux | grep system_monitor

# Detailed resource usage
htop  # Linux
Activity Monitor # macOS
Task Manager # Windows
```

## 🔒 Security & Privacy

### Privacy Protection

- **Local Processing**: All system monitoring data is processed locally, not uploaded to external servers
- **Minimal Permissions**: Application only requests necessary system permissions
- **Open Source**: All source code is open source and security-auditable
- **No Network Communication**: Application does not actively connect to external networks (except for update checks)

### System Permissions

The application requests the following system permissions:

| Permission Type | Purpose | Necessity |
|-----------------|---------|-----------|
| System Information Access | Read CPU, memory, GPU, network data | Required |
| File System Access | Store configuration files and logs | Required |
| Network Access | Check for application updates (optional) | Optional |
| Tray Integration | System tray functionality | Required |

### Security Measures

- **Code Review**: All code undergoes strict review
- **Dependency Checking**: Regular security checks on third-party dependencies
- **Minimal Attack Surface**: Only enable necessary functional modules
- **Memory Safety**: Rust language ensures memory safety

## 🚀 Performance Optimization

### Resource Usage

| Metric | Typical Value | Description |
|--------|---------------|-------------|
| **Memory Usage** | < 50MB | Includes all monitoring data |
| **CPU Usage** | < 1% | CPU usage when idle |
| **Startup Time** | < 3 seconds | From startup to interface display |
| **Disk Usage** | < 100MB | Disk space usage after installation |

### Optimization Tips

1. **Adjust Refresh Frequency**: Customize data update intervals based on needs
   - High-frequency monitoring: 500ms (suitable for gaming scenarios)
   - Standard monitoring: 1000ms (recommended setting)
   - Power-saving mode: 2000ms (laptop battery mode)

2. **Selective Monitoring**: Only enable necessary monitoring features
   - Disable GPU monitoring (if not using NVIDIA graphics)
   - Turn off temperature monitoring (if temperature data is not needed)

3. **Background Running Optimization**:
   - Automatically reduce update frequency when minimized
   - Pause non-essential monitoring when system is idle

### Performance Monitoring Tools

Use system tools to monitor application performance:

```bash
# Detailed performance analysis
cargo build --release
perf record ./src-tauri/target/release/system_monitor  # Linux
Instruments - Time Profiler  # macOS
xperf  # Windows
```

## 🤝 Contributing

We welcome all forms of contributions! Please follow these steps:

### Contribution Types

- 🐛 **Bug Reports**: Discover and report issues
- 💡 **Feature Suggestions**: Propose new feature ideas
- 📝 **Documentation Improvements**: Enhance project documentation
- 🔧 **Code Contributions**: Submit code fixes or new features
- 🎨 **Design Improvements**: UI/UX design optimizations
- 🧪 **Test Cases**: Add or improve tests

### Development Workflow

1. **Fork Repository**:
   ```bash
   # Fork the project on GitHub
   # Then clone your fork
   git clone https://github.com/yourusername/system-monitor.git
   cd system-monitor
   ```

2. **Create Feature Branch**:
   ```bash
   git checkout -b feature/AmazingFeature
   # Or fix branch
   git checkout -b fix/IssueNumber
   ```

3. **Development and Testing**:
   ```bash
   # Install dependencies
   pnpm install

   # Start development server
   pnpm tauri dev

   # Run tests
   pnpm test
   cargo test
   ```

4. **Code Quality Check**:
   ```bash
   # Check code formatting
   pnpm lint
   cargo fmt --check
   cargo clippy -- -D warnings

   # Type checking
   pnpm type-check
   ```

5. **Submit Changes**:
   ```bash
   git add .
   git commit -m "feat: add amazing feature

   - Detailed description of feature content
   - Explanation of implementation approach
   - Include test cases"
   ```

6. **Push Branch**:
   ```bash
   git push origin feature/AmazingFeature
   ```

7. **Create Pull Request**:
   - Create PR on GitHub
   - Fill in detailed PR description
   - Wait for code review

### Code Standards

#### Rust Code Standards
- Use `cargo fmt` for code formatting
- Use `cargo clippy` for code quality checking
- Follow Rust official naming conventions
- Add documentation comments for public APIs

```rust
/// Example: Good Rust code
/// Monitor system CPU usage
///
/// # Arguments
///
/// * `interval_ms` - Monitoring interval in milliseconds
///
/// # Returns
///
/// Returns CPU usage percentage
///
/// # Examples
///
/// ```
/// let cpu_usage = monitor_cpu(1000);
/// println!("CPU usage: {}%", cpu_usage);
/// ```
pub fn monitor_cpu(interval_ms: u64) -> f32 {
    // Implementation code
}
```

#### TypeScript/Vue Code Standards
- Follow ESLint and Prettier standards
- Use TypeScript strict mode
- Use PascalCase for component naming
- Add PropType definitions for components

```typescript
// Example: Good Vue component code
<template>
  <div class="system-monitor">
    <!-- Template content -->
  </div>
</template>

<script setup lang="ts">
interface Props {
  refreshInterval?: number
  showGpu?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  refreshInterval: 1000,
  showGpu: true
})

// Component logic
</script>

<style scoped>
.system-monitor {
  /* Style definitions */
}
</style>
```

#### Commit Message Standards
Use [Conventional Commits](https://conventionalcommits.org/) specification:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

**Type Descriptions**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation update
- `style`: Code formatting
- `refactor`: Code refactoring
- `test`: Test related
- `chore`: Build/tool related

**Example**:
```
feat(monitor): add GPU temperature monitoring

- Implement NVIDIA GPU temperature reading
- Add high temperature warning functionality
- Update UI to display temperature information

Closes #123
```

### Testing Guidelines

#### Running Tests
```bash
# Frontend tests
pnpm test
pnpm test:coverage

# Rust tests
cargo test
cargo test --release

# Integration tests
pnpm test:e2e
```

#### Writing Tests
```rust
// Rust test example
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_monitoring() {
        let monitor = SystemMonitor::new_default();
        let info = monitor.refresh();
        assert!(info.cpu_usage >= 0.0);
        assert!(info.cpu_usage <= 100.0);
    }
}
```

```typescript
// TypeScript test example
import { describe, it, expect } from 'vitest'
import { mount } from '@vue/test-utils'
import SystemMonitor from '@/components/SystemMonitor.vue'

describe('SystemMonitor', () => {
  it('renders system information correctly', () => {
    const wrapper = mount(SystemMonitor)
    expect(wrapper.find('.cpu-usage').exists()).toBe(true)
  })
})
```

## 📊 Roadmap

### v1.1.0 (In Development)

- [ ] **Historical Data Recording**: Record and display historical system data
- [ ] **Chart Display**: Add real-time curves and historical trend charts
- [ ] **Custom Monitoring**: Allow users to select monitoring metrics
- [ ] **Theme Support**: Support light/dark theme switching
- [ ] **Plugin System**: Support third-party monitoring plugins

### v1.2.0 (Planned)

- [ ] **Multi-monitor Support**: Support display on different monitors
- [ ] **Remote Monitoring**: Support network remote monitoring functionality
- [ ] **Mobile Application**: Develop companion mobile app
- [ ] **Cloud Data Sync**: Optional cloud data synchronization
- [ ] **Alert System**: Custom thresholds and notification system

### v2.0.0 (Long-term Planning)

- [ ] **AI Prediction**: System performance prediction based on historical data
- [ ] **Distributed Monitoring**: Support centralized monitoring of multiple devices
- [ ] **Enterprise Features**: Advanced features for enterprise users
- [ ] **API Interfaces**: Provide REST API and WebSocket interfaces
- [ ] **Container Deployment**: Support Docker and Kubernetes deployment

## 📄 License

This project is licensed under the [MIT License](LICENSE).

### License Summary

- ✅ **Commercial Use**: Can be used for commercial projects
- ✅ **Modification**: Source code can be modified
- ✅ **Distribution**: Can be distributed and sold
- ✅ **Private Use**: Can be used privately
- ❗ **Liability**: Software is provided "as is" without any warranty
- ❗ **Copyright**: Must include original copyright and license notices

## 🙏 Acknowledgments

Thanks to the following open source projects and contributors:

### Core Tech Stack
- [Tauri](https://tauri.app/) - Cross-platform desktop application framework
- [Vue.js](https://vuejs.org/) - Progressive JavaScript framework
- [Rust](https://www.rust-lang.org/) - System programming language
- [Vite](https://vitejs.dev/) - Fast frontend build tool

### Dependency Libraries
- [sysinfo](https://github.com/GuillaumeGomez/sysinfo) - System information library
- [nvml-wrapper](https://github.com/Cldfire/nvml-wrapper) - NVIDIA GPU monitoring library
- [Pinia](https://pinia.vuejs.org/) - Vue state management library

### Development Tools
- [ESLint](https://eslint.org/) - JavaScript code checking tool
- [Prettier](https://prettier.io/) - Code formatting tool
- [GitHub Actions](https://github.com/features/actions) - CI/CD platform

### Community Contributors
Thanks to all contributors who have contributed code, reported issues, and provided suggestions!

<a href="https://github.com/xinggaoya/system-monitor/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=xinggaoya/system-monitor" />
</a>

## 📞 Contact

### Getting Help

- **GitHub Issues**: [Submit Issues](https://github.com/xinggaoya/system-monitor/issues)
- **GitHub Discussions**: [Join Discussions](https://github.com/xinggaoya/system-monitor/discussions)
- **Wiki**: [Project Documentation](https://github.com/xinggaoya/system-monitor/wiki)

### Contact Maintainers

- **Email**: [Project maintainer email]
- **Twitter**: [@ProjectTwitterAccount]
- **WeChat Group**: [Scan QR code to join WeChat group]

### Feedback and Suggestions

We value feedback and suggestions from every user:

- 🐛 **Bug Reports**: Please provide detailed reproduction steps
- 💡 **Feature Suggestions**: Describe use cases and expected effects
- 📝 **Documentation Issues**: Point out errors or unclear parts in documentation
- 🌟 **Rating and Encouragement**: Give the project a Star on GitHub

---

<div align="center">

**⭐ If this project helps you, please give it a Star to show your support!**

Made with ❤️ by [System Monitor Team](https://github.com/xinggaoya/system-monitor)

[![Star History Chart](https://api.star-history.com/svg?repos=xinggaoya/system-monitor&type=Date)](https://star-history.com/#xinggaoya/system-monitor&Date)

</div>
