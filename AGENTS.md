# Repository Guidelines

## 项目结构与模块组织
- `src/` 包含 Vue 3 组件、`views/` 页面与 `stores/` Pinia 状态，按功能模块拆分（监控仪表、设置等）。
- `src-tauri/src/` 存放 Rust 侧系统监控逻辑，核心模块包括 `monitor.rs`（采样调度）、`gpu_monitor.rs`（GPU 指标）与 `adaptive_refresh.rs`（刷新节流）；公共类型放在 `models.rs`。
- 静态资源位于 `public/` 与 `src/assets/`，构建产物输出到 `dist/`；若需共享类型，可通过 `src-tauri/gen/` 中的自动生成绑定。

## 构建、测试与开发命令
- `npm run dev`：启动 Vite + Tauri 联调，便于前后端同步调试。
- `npm run build`：执行 TypeScript 检查并产出前端静态文件；完成后可用 `npm run tauri build` 打包桌面端。
- `npm run tauri dev`：快速验证桌面端特性（托盘、自启动等）；Rust 侧热重载依赖 `cargo`。
- `cargo test --manifest-path src-tauri/Cargo.toml`：在后端目录运行所有 Rust 测试，推荐配合 `RUST_LOG=debug` 追踪边界情况。

## 代码风格与命名
- 前端使用 2 空格缩进、`<script setup lang="ts">`，组件文件遵循 PascalCase（例如 `SystemChart.vue`）。
- Rust 模块遵循 snake_case，并通过 `cargo fmt` 与 `cargo clippy --all-targets --all-features` 保持一致风格。
- 状态与路由命名采用功能前缀：如 `useCpuStore`、`/settings/perf`，便于跨文件检索。

## 测试指引
- Rust 端使用 `tokio::test` 与断言宏覆盖关键调度与错误路径；新增异步逻辑时至少提供 happy path + 降级用例。
- 前端暂缺自动化测试，建议为复杂计算封装到 `src/utils/` 并以 Vitest/Newman 等轻量工具补充。
- 命名遵循 `module_feature_expectation`，示例：`monitor_refresh_adapts_interval()`。

## 提交与合并
- Git 历史遵循 Conventional Commits（`feat(monitor): ...`），描述聚焦可观测行为与作用域。
- PR 描述应包含变更摘要、影响面、测试结果以及相关 issue/需求编号；涉及 UI 的请附截图或短视频。
- 在通过 CI 之前请自检 `npm run build` 与 `cargo test`，并确认 `tauri.conf.json` 中的权限未意外放宽。

## 安全与配置
- `.env` 保持本地，不要提交；敏感键值通过操作系统密钥链或 Tauri `plugin-store` 管理。
- 若接入新的系统指标，确保在 `capabilities/` 中声明最小权限，并更新 README 的桌面权限说明。
