<template>
  <div class="settings-container">
    <header class="settings-header">
      <h1>系统监控设置</h1>
      <button class="close-btn" @click="closeWindow">
        <svg width="24" height="24" viewBox="0 0 24 24" fill="none">
          <path d="M18 6L6 18M6 6L18 18" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
        </svg>
      </button>
    </header>

    <main class="settings-content">
      <!-- 通用设置 -->
      <section class="settings-section">
        <h2>通用设置</h2>

        <div class="setting-item">
          <label class="setting-label">开机自启动</label>
          <div class="setting-control">
            <label class="switch">
              <input type="checkbox" v-model="settings.autoStart" @change="toggleAutoStart">
              <span class="slider"></span>
            </label>
          </div>
        </div>

        <div class="setting-item">
          <label class="setting-label">显示在任务栏</label>
          <div class="setting-control">
            <label class="switch">
              <input type="checkbox" v-model="settings.showInTaskbar" @change="saveSettings">
              <span class="slider"></span>
            </label>
          </div>
        </div>

        <div class="setting-item">
          <label class="setting-label">窗口置顶</label>
          <div class="setting-control">
            <label class="switch">
              <input type="checkbox" v-model="settings.alwaysOnTop" @change="saveSettings">
              <span class="slider"></span>
            </label>
          </div>
        </div>
      </section>

      <!-- 监控设置 -->
      <section class="settings-section">
        <h2>监控设置</h2>

        <div class="setting-item">
          <label class="setting-label">刷新频率</label>
          <div class="setting-control">
            <select v-model="settings.refreshInterval" @change="saveSettings" class="select-input">
              <option :value="500">0.5 秒</option>
              <option :value="1000">1 秒</option>
              <option :value="2000">2 秒</option>
              <option :value="5000">5 秒</option>
            </select>
          </div>
        </div>

        <div class="setting-item">
          <label class="setting-label">启用 GPU 监控</label>
          <div class="setting-control">
            <label class="switch">
              <input type="checkbox" v-model="settings.enableGpuMonitor" @change="saveSettings">
              <span class="slider"></span>
            </label>
          </div>
        </div>

        <div class="setting-item">
          <label class="setting-label">启用网络监控</label>
          <div class="setting-control">
            <label class="switch">
              <input type="checkbox" v-model="settings.enableNetworkMonitor" @change="saveSettings">
              <span class="slider"></span>
            </label>
          </div>
        </div>
      </section>

      <!-- 外观设置 -->
      <section class="settings-section">
        <h2>外观设置</h2>

        <div class="setting-item">
          <label class="setting-label">透明度</label>
          <div class="setting-control">
            <input
              type="range"
              min="10"
              max="100"
              v-model="settings.opacity"
              @input="saveSettings"
              class="range-input"
            >
            <span class="range-value">{{ settings.opacity }}%</span>
          </div>
        </div>

        <div class="setting-item">
          <label class="setting-label">主题颜色</label>
          <div class="setting-control">
            <div class="color-picker">
              <button
                v-for="color in themeColors"
                :key="color.name"
                :class="['color-btn', { active: settings.themeColor === color.value }]"
                :style="{ backgroundColor: color.value }"
                @click="changeThemeColor(color.value)"
              ></button>
            </div>
          </div>
        </div>
      </section>

      <!-- 高级设置 -->
      <section class="settings-section">
        <h2>高级设置</h2>

        <div class="setting-item">
          <label class="setting-label">日志级别</label>
          <div class="setting-control">
            <select v-model="settings.logLevel" @change="saveSettings" class="select-input">
              <option value="error">错误</option>
              <option value="warn">警告</option>
              <option value="info">信息</option>
              <option value="debug">调试</option>
            </select>
          </div>
        </div>

        <div class="setting-item">
          <label class="setting-label">数据缓存时间</label>
          <div class="setting-control">
            <select v-model="settings.cacheTime" @change="saveSettings" class="select-input">
              <option value="300">5 分钟</option>
              <option value="600">10 分钟</option>
              <option value="1800">30 分钟</option>
              <option value="3600">1 小时</option>
            </select>
          </div>
        </div>
      </section>
    </main>

    <footer class="settings-footer">
      <div class="footer-info">
        <span>系统监控 v1.0.0</span>
        <span>基于 Tauri + Vue 3 构建</span>
      </div>
      <div class="footer-actions">
        <button class="btn btn-secondary" @click="resetSettings">重置设置</button>
        <button class="btn btn-primary" @click="exportSettings">导出配置</button>
      </div>
    </footer>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { enable, disable, isEnabled } from '@tauri-apps/plugin-autostart'

// 设置数据接口
interface Settings {
  autoStart: boolean
  showInTaskbar: boolean
  alwaysOnTop: boolean
  refreshInterval: number
  enableGpuMonitor: boolean
  enableNetworkMonitor: boolean
  opacity: number
  themeColor: string
  logLevel: string
  cacheTime: number
}

// 主题颜色选项
const themeColors = [
  { name: '蓝色', value: '#3b82f6' },
  { name: '绿色', value: '#10b981' },
  { name: '紫色', value: '#8b5cf6' },
  { name: '红色', value: '#ef4444' },
  { name: '橙色', value: '#f97316' },
  { name: '青色', value: '#06b6d4' },
]

// 默认设置
const defaultSettings: Settings = {
  autoStart: false,
  showInTaskbar: false,
  alwaysOnTop: true,
  refreshInterval: 1000,
  enableGpuMonitor: true,
  enableNetworkMonitor: true,
  opacity: 90,
  themeColor: '#3b82f6',
  logLevel: 'info',
  cacheTime: 600,
}

// 响应式设置数据
const settings = ref<Settings>({ ...defaultSettings })

// 加载设置
const loadSettings = async () => {
  try {
    // 从Store加载设置
    const savedSettings = await invoke('get_all_settings')
    if (savedSettings && Object.keys(savedSettings).length > 0) {
      // 将Store中的设置映射到本地设置对象
      settings.value = { ...defaultSettings, ...savedSettings }
    }

    // 检查实际的自动启动状态并同步
    try {
      const autoStartEnabled = await isEnabled()
      settings.value.autoStart = autoStartEnabled
    } catch (autoStartError) {
      console.error('检查自动启动状态失败:', autoStartError)
    }
  } catch (error) {
    console.error('从Store加载设置失败:', error)
    // 如果Store加载失败，尝试从localStorage迁移
    try {
      const localSettings = localStorage.getItem('system-monitor-settings')
      if (localSettings) {
        const parsedSettings = JSON.parse(localSettings)
        settings.value = { ...defaultSettings, ...parsedSettings }
        // 迁移到Store
        await saveSettings()
      }
    } catch (migrateError) {
      console.error('从localStorage迁移设置失败:', migrateError)
    }
  }
}

// 保存设置
const saveSettings = async () => {
  try {
    // 保存到Store
    for (const [key, value] of Object.entries(settings.value)) {
      await invoke('save_settings', { key, value })
    }
    console.log('设置已保存到Store:', settings.value)

    // 同时也保存到localStorage作为备份
    localStorage.setItem('system-monitor-settings', JSON.stringify(settings.value))

    // 调用后端API来应用某些设置
    // 例如：invoke('update_monitor_config', { config: settings.value })
  } catch (error) {
    console.error('保存设置到Store失败:', error)
    // 如果Store保存失败，降级到localStorage
    try {
      localStorage.setItem('system-monitor-settings', JSON.stringify(settings.value))
      console.log('设置已保存到localStorage作为备份')
    } catch (localError) {
      console.error('保存到localStorage也失败:', localError)
    }
  }
}

// 更改主题颜色
const changeThemeColor = (color: string) => {
  settings.value.themeColor = color
  saveSettings()
}

// 切换自动启动
const toggleAutoStart = async () => {
  try {
    if (settings.value.autoStart) {
      await enable()
      console.log('自动启动已启用')
    } else {
      await disable()
      console.log('自动启动已禁用')
    }
    // 保存设置到Store
    await saveSettings()
  } catch (error) {
    console.error('切换自动启动失败:', error)
    // 如果操作失败，恢复UI状态
    settings.value.autoStart = !settings.value.autoStart
  }
}

// 重置设置
const resetSettings = () => {
  if (confirm('确定要重置所有设置吗？')) {
    settings.value = { ...defaultSettings }
    saveSettings()
  }
}

// 导出设置
const exportSettings = () => {
  const dataStr = JSON.stringify(settings.value, null, 2)
  const dataUri = 'data:application/json;charset=utf-8,'+ encodeURIComponent(dataStr)

  const exportFileDefaultName = 'system-monitor-settings.json'

  const linkElement = document.createElement('a')
  linkElement.setAttribute('href', dataUri)
  linkElement.setAttribute('download', exportFileDefaultName)
  linkElement.click()
}

// 关闭窗口
const closeWindow = async () => {
  try {
    await invoke('close_settings_window')
  } catch (error) {
    console.error('关闭设置窗口失败:', error)
    // 如果 Tauri 命令失败，尝试直接关闭窗口
    if (window.close) {
      window.close()
    }
  }
}

// 组件挂载时加载设置
onMounted(() => {
  loadSettings()
})
</script>

<style scoped>
.settings-container {
  min-height: 100vh;
  background: linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%);
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  overflow-y: auto;
  max-height: 100vh;
}

/* 设置窗口专用滚动条样式 */
.settings-container::-webkit-scrollbar {
  width: 8px;
}

.settings-container::-webkit-scrollbar-track {
  background: rgba(255, 255, 255, 0.1);
  border-radius: 4px;
}

.settings-container::-webkit-scrollbar-thumb {
  background: rgba(103, 126, 234, 0.6);
  border-radius: 4px;
  transition: background 0.2s;
}

.settings-container::-webkit-scrollbar-thumb:hover {
  background: rgba(103, 126, 234, 0.8);
}

.settings-header {
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(10px);
  border-bottom: 1px solid rgba(0, 0, 0, 0.1);
  padding: 1.5rem 2rem;
  display: flex;
  justify-content: space-between;
  align-items: center;
  box-shadow: 0 2px 20px rgba(0, 0, 0, 0.1);
}

.settings-header h1 {
  margin: 0;
  font-size: 1.5rem;
  font-weight: 600;
  color: #1f2937;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.close-btn {
  background: none;
  border: none;
  padding: 0.5rem;
  border-radius: 0.5rem;
  cursor: pointer;
  color: #6b7280;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
}

.close-btn:hover {
  background: rgba(239, 68, 68, 0.1);
  color: #ef4444;
}

.settings-content {
  padding: 2rem;
  max-width: 800px;
  margin: 0 auto;
}

.settings-section {
  background: rgba(255, 255, 255, 0.9);
  backdrop-filter: blur(10px);
  border-radius: 1rem;
  padding: 1.5rem;
  margin-bottom: 1.5rem;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  border: 1px solid rgba(255, 255, 255, 0.2);
}

.settings-section h2 {
  margin: 0 0 1.5rem 0;
  font-size: 1.25rem;
  font-weight: 600;
  color: #1f2937;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.settings-section h2::before {
  content: '';
  width: 4px;
  height: 1.25rem;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border-radius: 2px;
}

.setting-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 0;
  border-bottom: 1px solid rgba(0, 0, 0, 0.05);
}

.setting-item:last-child {
  border-bottom: none;
}

.setting-label {
  font-weight: 500;
  color: #374151;
  font-size: 0.95rem;
}

.setting-control {
  display: flex;
  align-items: center;
  gap: 1rem;
}

/* 开关样式 */
.switch {
  position: relative;
  display: inline-block;
  width: 50px;
  height: 24px;
}

.switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: #ccc;
  transition: 0.3s;
  border-radius: 24px;
}

.slider:before {
  position: absolute;
  content: "";
  height: 18px;
  width: 18px;
  left: 3px;
  bottom: 3px;
  background-color: white;
  transition: 0.3s;
  border-radius: 50%;
}

input:checked + .slider {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

input:checked + .slider:before {
  transform: translateX(26px);
}

/* 选择器样式 */
.select-input {
  padding: 0.5rem 1rem;
  border: 1px solid #d1d5db;
  border-radius: 0.5rem;
  background: white;
  color: #374151;
  font-size: 0.9rem;
  cursor: pointer;
  transition: border-color 0.2s;
}

.select-input:focus {
  outline: none;
  border-color: #667eea;
  box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
}

/* 滑块样式 */
.range-input {
  width: 150px;
  height: 6px;
  border-radius: 3px;
  background: #d1d5db;
  outline: none;
  -webkit-appearance: none;
}

.range-input::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  cursor: pointer;
}

.range-input::-moz-range-thumb {
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  cursor: pointer;
  border: none;
}

.range-value {
  font-weight: 500;
  color: #667eea;
  min-width: 40px;
  text-align: right;
}

/* 颜色选择器 */
.color-picker {
  display: flex;
  gap: 0.5rem;
}

.color-btn {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  border: 2px solid transparent;
  cursor: pointer;
  transition: all 0.2s;
  position: relative;
}

.color-btn:hover {
  transform: scale(1.1);
}

.color-btn.active {
  border-color: #667eea;
  box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.2);
}

.color-btn.active::after {
  content: '✓';
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  color: white;
  font-weight: bold;
  font-size: 14px;
}

/* 页脚 */
.settings-footer {
  background: rgba(255, 255, 255, 0.9);
  backdrop-filter: blur(10px);
  border-top: 1px solid rgba(0, 0, 0, 0.1);
  padding: 1.5rem 2rem;
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: 2rem;
}

.footer-info {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.footer-info span {
  font-size: 0.85rem;
  color: #6b7280;
}

.footer-actions {
  display: flex;
  gap: 1rem;
}

.btn {
  padding: 0.75rem 1.5rem;
  border: none;
  border-radius: 0.5rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  font-size: 0.9rem;
}

.btn-primary {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
}

.btn-primary:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
}

.btn-secondary {
  background: #f3f4f6;
  color: #374151;
  border: 1px solid #d1d5db;
}

.btn-secondary:hover {
  background: #e5e7eb;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .settings-content {
    padding: 1rem;
  }

  .settings-section {
    padding: 1rem;
  }

  .setting-item {
    flex-direction: column;
    align-items: flex-start;
    gap: 1rem;
  }

  .settings-footer {
    flex-direction: column;
    gap: 1rem;
    text-align: center;
  }

  .footer-actions {
    width: 100%;
    justify-content: center;
  }
}
</style>