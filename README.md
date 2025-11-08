# System Monitor 系统监控

<div align="center">

[![Tauri](https://img.shields.io/badge/Tauri-2.2-blue.svg)](https://tauri.app/)
[![Vue](https://img.shields.io/badge/Vue-3.5-green.svg)](https://vuejs.org/)
[![Rust](https://img.shields.io/badge/Rust-1.70-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![GitHub release](https://img.shields.io/github/release/xinggaoya/system-monitor.svg)](https://github.com/xinggaoya/system-monitor/releases)
[![Build Status](https://github.com/xinggaoya/system-monitor/workflows/CI/badge.svg)](https://github.com/xinggaoya/system-monitor/actions)

**语言 / Language:** [中文](README.md) | [English](README.en.md)

一个基于 Tauri 2.2 + Vue 3 构建的跨平台桌面系统监控应用，提供实时的 CPU、内存、GPU 和网络监控功能。

[功能特性](#-特性) • [快速开始](#-快速开始) • [下载安装](#-下载安装) • [使用指南](#-使用指南) • [技术栈](#-技术栈) • [贡献指南](#-贡献指南)

</div>

## ✨ 特性

- 🖥️ **实时系统监控**: CPU、内存、GPU 和网络使用情况实时显示
- 🎯 **轻量级设计**: 最小化资源占用，优雅的悬浮窗显示
- 🔄 **系统托盘集成**: 完整的托盘菜单功能，支持显示/隐藏和退出操作
- 🎨 **现代化界面**: 透明悬浮窗，美观的视觉效果和动画
- ⚡ **高性能**: Rust 后端 + Vue 前端，响应迅速，资源占用低
- 🌐 **跨平台支持**: 支持 Windows、macOS 和 Linux 三大平台
- 📊 **详细网络监控**: 实时网络流量监控，支持上下载速度显示
- 🎮 **GPU 监控**: 支持 NVIDIA GPU 监控（需要 NVML 库）
- 🌡️ **温度分组监控**: 自动区分 CPU 包/核心、内存、GPU 等热点传感器
- 🎯 **帧率捕获（Windows + PresentMon）**: 调用 Intel PresentMon 采集真实帧率，若未安装则优雅降级为提示信息
- 🔧 **高度可配置**: 支持自定义刷新频率、显示选项和外观设置

## 📸 截图预览

### 主界面
```
┌─────────────────────────────────────┐
│ System Monitor                     │
├─────────────────────────────────────┤
│ CPU 45%  内存 62%  GPU 78%  网络    │
│ ↓1.2MB ↑800KB                     │
│ 🌡️ CPU: 45°C  GPU: 72°C            │
└─────────────────────────────────────┘
```

### 系统托盘
- 右键菜单：显示/隐藏主窗口、退出应用
- 左键点击：切换窗口显示/隐藏状态
- 实时显示系统状态图标

## 🚀 快速开始

### 环境要求

- **Node.js**: 18+
- **Rust**: 1.70+
- **包管理器**: pnpm (推荐) 或 npm / yarn

### 安装依赖

```bash
# 克隆仓库
git clone https://github.com/xinggaoya/system-monitor.git
cd system-monitor

# 安装前端依赖
pnpm install

# 安装 Tauri CLI (如果尚未安装)
cargo install tauri-cli --version "^2.0"
```

### 开发模式

```bash
# 启动开发服务器
pnpm tauri dev

# 或者使用 npm
npm run tauri dev
```

### 构建发布版本

```bash
# 构建生产版本
pnpm tauri build

# 构建特定平台
pnpm tauri build --target x86_64-pc-windows-msvc  # Windows
pnpm tauri build --target x86_64-apple-darwin      # macOS
pnpm tauri build --target x86_64-unknown-linux-gnu # Linux
```

构建产物将生成在 `src-tauri/target/release/bundle/` 目录中。

### Windows 帧率采集（可选）

帧率模块依赖 [Intel PresentMon](https://www.intel.com/content/www/us/en/download/705483/presentmon.html) 获取真实 FPS，配置步骤如下：

1. 下载并解压最新 `PresentMon.exe`
2. 将所在目录加入 `PATH`，或设置环境变量 `PRESENTMON_PATH=C:\Tools\PresentMon\PresentMon.exe`
3. 重启 System Monitor，在设置 → “显示模块” 中开启“帧率”，即可看到实时 FPS/帧时间

> 目前仅 Windows 支持帧率采集；macOS / Linux 会显示“未接入”提示，不会展示模拟数据。

## 📥 下载安装

### 预编译版本

从 [GitHub Releases](https://github.com/xinggaoya/system-monitor/releases) 下载最新的预编译版本：

| 平台 | 文件名 | 大小 |
|------|--------|------|
| **Windows** | `system_monitor_1.1.0_x64-setup.exe` | ~8MB |
| **macOS** | `system_monitor_1.1.0_x64.dmg` | ~6MB |
| **Linux** | `system_monitor_1.1.0_amd64.AppImage` | ~7MB |

### 安装说明

#### Windows
1. 下载 `.exe` 安装包
2. 右键选择"以管理员身份运行"
3. 按照安装向导完成安装
4. 从开始菜单启动应用

#### macOS
1. 下载 `.dmg` 文件
2. 双击打开安装包
3. 将应用拖拽到 Applications 文件夹
4. 从 Launchpad 启动应用

#### Linux
1. 下载 `.AppImage` 文件
2. 添加执行权限：`chmod +x system_monitor_*.AppImage`
3. 直接运行：`./system_monitor_*.AppImage`

## 🎯 使用指南

### 基本操作

1. **启动应用**: 安装后从系统菜单启动或直接运行
2. **拖动窗口**: 按住鼠标左键拖动到任意位置
3. **托盘操作**:
   - 左键点击托盘图标：显示/隐藏主窗口
   - 右键点击托盘图标：显示菜单选项

### 监控指标说明

| 指标 | 说明 | 显示格式 |
|------|------|----------|
| **CPU 使用率** | 处理器使用百分比 | `CPU 45%` |
| **内存使用率** | 物理内存使用百分比 | `内存 62%` |
| **GPU 使用率** | 显卡使用百分比 (NVIDIA) | `GPU 78%` |
| **网络速度** | 实时上传/下载速度 | `↓1.2MB ↑800KB` |
| **温度监控** | CPU 和 GPU 温度 | `🌡️ CPU: 45°C` |

### 配置选项

应用支持以下配置（通过配置文件或设置界面）：

- **刷新间隔**: 数据更新频率（默认 1000ms）
- **显示选项**: 选择要监控的系统指标
- **外观设置**: 窗口透明度、位置和主题
- **通知设置**: 异常情况提醒

## 🏗️ 技术栈

### 前端技术
- **Vue 3.5** - 现代化前端框架，Composition API
- **TypeScript** - 类型安全的 JavaScript 超集
- **Pinia** - 轻量级状态管理库
- **Vite** - 快速的前端构建工具
- **CSS3** - 现代样式，支持透明和动画效果

### 后端技术
- **Tauri 2.2** - 跨平台桌面应用框架
- **Rust** - 系统级编程语言，内存安全
- **sysinfo 0.33** - 系统信息获取库
- **nvml-wrapper** - NVIDIA GPU 监控库

### 开发工具
- **ESLint** - 代码质量检查
- **Prettier** - 代码格式化
- **GitHub Actions** - CI/CD 自动化
- **Clippy** - Rust 代码质量检查

## 📁 项目结构

```
system-monitor/
├── README.md                   # 项目文档（中文）
├── README.en.md               # 项目文档（英文）
├── LICENSE                    # MIT 许可证
├── package.json              # 项目依赖配置
├── tsconfig.json             # TypeScript 配置
├── vite.config.ts            # Vite 构建配置
├── src/                      # 前端源码
│   ├── main.ts              # 应用入口
│   ├── App.vue              # 主应用组件
│   ├── components/          # Vue 组件
│   ├── composables/         # 组合式函数
│   ├── stores/              # Pinia 状态管理
│   └── assets/              # 静态资源
├── src-tauri/               # Rust 后端
│   ├── src/                 # Rust 源码
│   │   ├── lib.rs           # 主要应用逻辑
│   │   ├── models.rs        # 数据模型定义
│   │   ├── monitor.rs       # 系统监控实现
│   │   └── gpu_monitor.rs   # GPU 监控实现
│   ├── Cargo.toml           # Rust 依赖配置
│   ├── tauri.conf.json      # Tauri 应用配置
│   └── icons/               # 应用图标资源
└── .github/                 # GitHub 配置
    └── workflows/
        └── ci.yml          # CI/CD 自动化配置
```

## 🎮 GPU 监控说明

### NVIDIA GPU 支持

应用支持 NVIDIA GPU 监控，需要满足以下条件：

1. **安装 NVIDIA 驱动**: 确保系统安装了最新的 NVIDIA 显卡驱动
2. **NVML 库**: 应用使用 nvml-wrapper 进行 GPU 监控
3. **系统权限**: 应用需要适当的系统权限来访问 GPU 信息

### 监控指标

- **GPU 使用率**: 实时 GPU 计算使用百分比
- **显存使用**: GPU 显存使用情况和总量
- **温度监控**: GPU 核心温度（如果硬件支持）
- **时钟频率**: GPU 核心和显存时钟频率

### 故障排除

如果 GPU 监控不可用：
1. 检查 NVIDIA 驱动是否正确安装
2. 确认 NVML 库是否可用
3. 重启应用程序
4. 检查系统权限设置

## 🌐 网络监控

### 监控指标

- **下载速度**: 实时网络下载速度
- **上传速度**: 实时网络上传速度
- **网络接口**: 自动检测活动网络接口
- **流量统计**: 累计网络流量统计

### 显示格式

```
实时速度显示：
↓下载速度 ↑上传速度
示例：↓1.2MB ↑800KB

支持的单位：
B/s, KB/s, MB/s, GB/s
```

## 🎨 界面特性

### 悬浮窗设计

- **透明背景**: 半透明背景，不遮挡桌面内容
- **始终置顶**: 可选择是否始终显示在最前面
- **可拖动**: 支持鼠标拖动到任意位置
- **最小化设计**: 占用最小的屏幕空间
- **响应式**: 自动适应不同屏幕分辨率

### 系统托盘集成

- **托盘图标**: 系统托盘中显示应用图标
- **状态指示**: 图标显示系统运行状态
- **右键菜单**:
  - 显示/隐藏主窗口
  - 退出应用
  - 关于信息
- **左键操作**: 切换窗口显示/隐藏状态

## 🐛 故障排除

### 常见问题及解决方案

#### GPU 监控不可用
**问题**: GPU 数据显示为空或 "--"
**解决方案**:
- 确保安装了 NVIDIA 显卡驱动
- 检查 NVML 库是否正确安装
- 重启应用程序
- 以管理员权限运行（Windows）

#### 托盘图标不显示
**问题**: 系统托盘中看不到应用图标
**解决方案**:
- 检查系统托盘设置
- 重启应用程序
- 确认应用有适当的系统权限
- 在任务管理器中查看后台进程

#### 应用无法启动
**问题**: 双击图标后应用没有响应
**解决方案**:
- 检查系统日志获取详细错误信息
- 确认有足够的系统权限
- 尝试以管理员权限运行（Windows）
- 重新安装应用

#### 构建失败
**问题**: 开发环境下构建报错
**解决方案**:
- 确保所有依赖已正确安装
- 检查 Rust 和 Node.js 版本
- 清理缓存：`pnpm store prune` 和 `cargo clean`
- 更新 Tauri CLI：`cargo install tauri-cli --force`

### 调试模式

启用调试模式获取详细日志：

```bash
# Windows
set RUST_LOG=debug && pnpm tauri dev

# macOS/Linux
RUST_LOG=debug pnpm tauri dev

# 或者创建日志文件
RUST_LOG=debug pnpm tauri dev > system-monitor.log 2>&1
```

### 性能监控

监控应用自身的资源使用：

```bash
# Windows
tasklist | findstr system_monitor

# macOS/Linux
ps aux | grep system_monitor

# 详细资源使用
htop  # Linux
活动监视器 # macOS
任务管理器 # Windows
```

## 🔒 安全性与隐私

### 隐私保护

- **本地处理**: 所有系统监控数据在本地处理，不上传到外部服务器
- **最小权限**: 应用只请求必要的系统权限
- **开源代码**: 所有源代码开源，可审查安全性
- **无网络通信**: 应用不会主动连接外部网络（除了更新检查）

### 系统权限

应用请求以下系统权限：

| 权限类型 | 用途 | 必要性 |
|----------|------|--------|
| 系统信息访问 | 读取 CPU、内存、GPU、网络数据 | 必需 |
| 文件系统访问 | 存储配置文件和日志 | 必需 |
| 网络访问 | 检查应用更新（可选） | 可选 |
| 托盘集成 | 系统托盘功能 | 必需 |

### 安全措施

- **代码审查**: 所有代码经过严格审查
- **依赖检查**: 定期检查第三方依赖的安全性
- **最小攻击面**: 只启用必要的功能模块
- **内存安全**: Rust 语言保证内存安全

## 🚀 性能优化

### 资源占用

| 指标 | 典型值 | 说明 |
|------|--------|------|
| **内存使用** | < 50MB | 包含所有监控数据 |
| **CPU 占用** | < 1% | 空闲时的 CPU 使用率 |
| **启动时间** | < 3 秒 | 从启动到显示界面 |
| **磁盘占用** | < 100MB | 安装后的磁盘空间占用 |

### 优化建议

1. **调整刷新频率**: 根据需要调整数据更新间隔
   - 高频监控：500ms（适合游戏场景）
   - 标准监控：1000ms（推荐设置）
   - 节能模式：2000ms（笔记本电池模式）

2. **选择性监控**: 只启用需要的监控功能
   - 禁用 GPU 监控（如果不使用 NVIDIA 显卡）
   - 关闭温度监控（如果不需要温度数据）

3. **后台运行优化**:
   - 最小化时自动降低更新频率
   - 系统空闲时暂停非必要监控

### 性能监控工具

使用系统工具监控应用性能：

```bash
# 详细性能分析
cargo build --release
perf record ./src-tauri/target/release/system_monitor  # Linux
Instruments - Time Profiler  # macOS
xperf  # Windows
```

## 🤝 贡献指南

我们欢迎所有形式的贡献！请遵循以下步骤：

### 贡献类型

- 🐛 **Bug 报告**: 发现并报告问题
- 💡 **功能建议**: 提出新功能想法
- 📝 **文档改进**: 完善项目文档
- 🔧 **代码贡献**: 提交代码修复或新功能
- 🎨 **设计改进**: UI/UX 设计优化
- 🧪 **测试用例**: 添加或改进测试

### 开发流程

1. **Fork 仓库**:
   ```bash
   # 在 GitHub 上 Fork 项目
   # 然后克隆你的 Fork
   git clone https://github.com/你的用户名/system-monitor.git
   cd system-monitor
   ```

2. **创建功能分支**:
   ```bash
   git checkout -b feature/AmazingFeature
   # 或修复分支
   git checkout -b fix/IssueNumber
   ```

3. **开发和测试**:
   ```bash
   # 安装依赖
   pnpm install

   # 启动开发服务器
   pnpm tauri dev

   # 运行测试
   pnpm test
   cargo test
   ```

4. **代码质量检查**:
   ```bash
   # 检查代码格式
   pnpm lint
   cargo fmt --check
   cargo clippy -- -D warnings

   # 类型检查
   pnpm type-check
   ```

5. **提交更改**:
   ```bash
   git add .
   git commit -m "feat: 添加某个神奇功能

   - 详细描述功能内容
   - 说明实现方式
   - 包含测试用例"
   ```

6. **推送分支**:
   ```bash
   git push origin feature/AmazingFeature
   ```

7. **创建 Pull Request**:
   - 在 GitHub 上创建 PR
   - 填写详细的 PR 描述
   - 等待代码审查

### 代码规范

#### Rust 代码规范
- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量
- 遵循 Rust 官方命名约定
- 为公共 API 添加文档注释

```rust
/// 示例：好的 Rust 代码
/// 监控系统 CPU 使用率
///
/// # Arguments
///
/// * `interval_ms` - 监控间隔（毫秒）
///
/// # Returns
///
/// 返回 CPU 使用率百分比
///
/// # Examples
///
/// ```
/// let cpu_usage = monitor_cpu(1000);
/// println!("CPU 使用率: {}%", cpu_usage);
/// ```
pub fn monitor_cpu(interval_ms: u64) -> f32 {
    // 实现代码
}
```

#### TypeScript/Vue 代码规范
- 遵循 ESLint 和 Prettier 规范
- 使用 TypeScript 严格模式
- 组件命名使用 PascalCase
- 为组件添加 PropType 定义

```typescript
// 示例：好的 Vue 组件代码
<template>
  <div class="system-monitor">
    <!-- 模板内容 -->
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

// 组件逻辑
</script>

<style scoped>
.system-monitor {
  /* 样式定义 */
}
</style>
```

#### 提交信息规范
使用 [Conventional Commits](https://conventionalcommits.org/) 规范：

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

**类型说明**:
- `feat`: 新功能
- `fix`: Bug 修复
- `docs`: 文档更新
- `style`: 代码格式化
- `refactor`: 代码重构
- `test`: 测试相关
- `chore`: 构建/工具相关

**示例**:
```
feat(monitor): 添加 GPU 温度监控

- 实现 NVIDIA GPU 温度读取
- 添加温度过高警告功能
- 更新 UI 显示温度信息

Closes #123
```

### 测试指南

#### 运行测试
```bash
# 前端测试
pnpm test
pnpm test:coverage

# Rust 测试
cargo test
cargo test --release

# 集成测试
pnpm test:e2e
```

#### 编写测试
```rust
// Rust 测试示例
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
// TypeScript 测试示例
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

## 📊 路线图

### v1.0.0 (2025-01-07) ✅

- [x] **核心监控功能**: CPU、内存、GPU、网络实时监控
- [x] **悬浮窗设计**: 透明可置顶的监控窗口
- [x] **系统托盘集成**: 完整的托盘菜单功能
- [x] **跨平台支持**: Windows、macOS、Linux 三平台支持
- [x] **GPU 监控**: NVIDIA GPU 监控支持
- [x] **自适应刷新**: 根据系统负载调整刷新频率
- [x] **窗口设置**: 置顶、任务栏显示等窗口偏好设置

### v1.1.0 (当前版本 - 2025-01-07) ✅

- [x] **历史数据记录**: 记录和显示历史系统数据
- [x] **图表显示**: 添加实时曲线图和历史趋势图
- [x] **自定义监控**: 允许用户选择监控指标
- [x] **主题支持**: 支持亮色/暗色主题切换
- [x] **性能优化**: 进一步优化内存和CPU使用
- [x] **磁盘监控增强**: 更详细的磁盘信息和IO监控
- [x] **温度监控改进**: 更准确的温度传感器数据

### v1.2.0 (开发中)

- [ ] **多显示器支持**: 支持在不同显示器显示监控窗口
- [ ] **远程监控**: 支持网络远程监控功能
- [ ] **移动端应用**: 开发配套的移动端应用
- [ ] **插件系统**: 支持第三方监控插件

### v1.3.0 (计划中)

- [ ] **云端数据同步**: 可选的数据云端同步功能
- [ ] **告警系统**: 自定义阈值和通知系统

### v2.0.0 (长期规划)

- [ ] **AI 预测**: 基于历史数据的系统性能预测
- [ ] **分布式监控**: 支持多台设备集中监控
- [ ] **企业版功能**: 针对企业用户的高级功能
- [ ] **API 接口**: 提供 REST API 和 WebSocket 接口
- [ ] **容器化部署**: 支持 Docker 和 Kubernetes 部署

## 📄 许可证

本项目采用 [MIT 许可证](LICENSE)。

### 许可证摘要

- ✅ **商业使用**: 可以用于商业项目
- ✅ **修改**: 可以修改源代码
- ✅ **分发**: 可以分发和销售
- ✅ **私人使用**: 可以私人使用
- ❗ **责任**: 软件按"原样"提供，无任何担保
- ❗ **版权**: 必须包含原始版权和许可证声明

## 🙏 致谢

感谢以下开源项目和贡献者：

### 核心技术栈
- [Tauri](https://tauri.app/) - 跨平台桌面应用框架
- [Vue.js](https://vuejs.org/) - 渐进式 JavaScript 框架
- [Rust](https://www.rust-lang.org/) - 系统编程语言
- [Vite](https://vitejs.dev/) - 快速的前端构建工具

### 依赖库
- [sysinfo](https://github.com/GuillaumeGomez/sysinfo) - 系统信息库
- [nvml-wrapper](https://github.com/Cldfire/nvml-wrapper) - NVIDIA GPU 监控库
- [Pinia](https://pinia.vuejs.org/) - Vue 状态管理库

### 开发工具
- [ESLint](https://eslint.org/) - JavaScript 代码检查工具
- [Prettier](https://prettier.io/) - 代码格式化工具
- [GitHub Actions](https://github.com/features/actions) - CI/CD 平台

### 社区贡献者
感谢所有为项目贡献代码、报告问题、提出建议的贡献者！

<a href="https://github.com/xinggaoya/system-monitor/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=xinggaoya/system-monitor" />
</a>

## 📞 联系方式

### 获取帮助

- **GitHub Issues**: [提交问题](https://github.com/xinggaoya/system-monitor/issues)
- **GitHub Discussions**: [参与讨论](https://github.com/xinggaoya/system-monitor/discussions)
- **Wiki**: [项目文档](https://github.com/xinggaoya/system-monitor/wiki)

### 联系维护者

- **Email**: [项目维护者邮箱]
- **Twitter**: [@项目Twitter账号]
- **微信群**: [扫码加入微信群]

### 反馈和建议

我们重视每一位用户的反馈和建议：

- 🐛 **Bug 报告**: 请提供详细的复现步骤
- 💡 **功能建议**: 描述使用场景和期望效果
- 📝 **文档问题**: 指出文档中的错误或不清楚的地方
- 🌟 **评价鼓励**: 在 GitHub 上给项目点 Star

---

<div align="center">

**⭐ 如果这个项目对你有帮助，请给个 Star 支持一下！**

Made with ❤️ by [System Monitor Team](https://github.com/xinggaoya/system-monitor)

[![Star History Chart](https://api.star-history.com/svg?repos=xinggaoya/system-monitor&type=Date)](https://star-history.com/#xinggaoya/system-monitor&Date)

</div>
