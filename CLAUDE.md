# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 常用命令

### 开发模式
```bash
# 启动开发服务器（前端 + 后端）
pnpm tauri dev

# 仅启动前端开发服务器
pnpm dev
```

### 构建
```bash
# 构建生产版本
pnpm tauri build

# 仅构建前端
pnpm build
```

### 测试
```bash
# 应用内测试：点击应用界面的"🧪 测试"按钮
# 或在开发者控制台中调用：
# systemStore.testDataRefresh()

# 详细测试：打开 test_data_refresh.html 文件进行浏览器端测试
```

## 技术栈架构

这是一个基于 Tauri 2.2 + Vue 3 + Rust 的系统监控应用：

### 前端架构 (Vue 3)
- **框架**: Vue 3.5.13 + TypeScript 5.7
- **状态管理**: Pinia 2.3.0 (使用 stores/system.ts)
- **组合式函数**: composables/useSystemMonitor.ts - 提供响应式系统监控功能
- **构建工具**: Vite 6.0.3
- **UI设计**: 极简悬浮窗，圆角白色设计，可置顶显示

### 后端架构 (Rust)
- **框架**: Tauri 2.2 + tokio 异步运行时
- **系统监控**: sysinfo 0.33 库
- **模块结构**:
  - `src-tauri/src/lib.rs` - 主要入口，定义 Tauri 命令
  - `src-tauri/src/models.rs` - 数据结构定义
  - `src-tauri/src/monitor.rs` - 系统监控逻辑
  - `src-tauri/src/test_commands.rs` - 测试命令

### 核心功能模块

#### 数据流向
1. **Rust 后端**: 使用 sysinfo 库获取系统信息
2. **Tauri Bridge**: 通过 invoke 调用传递数据
3. **Pinia Store**: 管理前端状态和业务逻辑
4. **Vue 组合式函数**: 提供响应式数据轮询

#### 关键组件
- **悬浮监控窗口**: 显示 CPU、内存、GPU、网络使用率
- **系统托盘**: 支持显示/隐藏、退出操作
- **自动轮询**: 每秒更新系统数据
- **测试功能**: 内置数据刷新验证机制

#### 监控指标
- CPU: 使用率百分比、核心数、型号频率
- 内存: 总容量、使用量、使用率、交换分区
- 网络: 接口列表、上传/下载速率、总流量
- 磁盘: 分区信息、容量使用情况、文件系统
- GPU: 使用率、显存、温度、频率（模拟数据）
- 温度: 各组件温度传感器读数

### 重要配置
- **开发服务器端口**: 9000 (Vite)
- **HMR 端口**: 9001
- **刷新间隔**: 默认 1000ms (可配置)
- **路径别名**: @ 指向 src 目录

### 数据更新机制
- 前端使用 `setInterval` 每秒调用后端 API
- GPU 信息每 5 次轮询更新一次（减少查询频率）
- 支持手动刷新和配置更新
- 错误处理和重试机制