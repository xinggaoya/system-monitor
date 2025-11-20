<template>
  <div class="settings-shell">
    <div class="settings-container">
      <n-page-header
          class="settings-header"
          title="系统监控设置"
          subtitle="自定义刷新策略和外观，让状态一目了然"
      >
        <template #extra>
          <n-button quaternary circle size="large" @click="closeWindow">
            <template #icon>
              <span class="close-icon">×</span>
            </template>
          </n-button>
        </template>
      </n-page-header>

      <n-form ref="formRef" :model="settings" :rules="rules" :label-width="120" class="settings-form">
        <n-grid cols="1 800:2" x-gap="24" y-gap="24">
          <!-- 左侧列 -->
          <n-gi>
            <n-space vertical size="large">
              <n-card title="通用设置" class="settings-card glass-card" size="small">
                <n-form-item label="开机自启动" path="autoStart">
                  <n-switch
                      v-model:value="settings.autoStart"
                      @update:value="handleAutoStartUpdate"
                  >
                    <template #checked>开启</template>
                    <template #unchecked>关闭</template>
                  </n-switch>
                </n-form-item>
                <n-form-item label="显示在任务栏" path="showInTaskbar">
                  <n-switch
                      v-model:value="settings.showInTaskbar"
                      @update:value="handleSettingsChange"
                  />
                </n-form-item>
                <n-form-item label="窗口置顶" path="alwaysOnTop">
                  <n-switch
                      v-model:value="settings.alwaysOnTop"
                      @update:value="handleSettingsChange"
                  />
                </n-form-item>
              </n-card>

              <n-card title="监控策略" class="settings-card glass-card" size="small">
                <n-form-item label="刷新频率" path="refreshInterval">
                  <n-select
                      class="setting-select"
                      v-model:value="settings.refreshInterval"
                      :options="refreshIntervalOptions"
                      size="medium"
                      @update:value="handleSettingsChange"
                  />
                </n-form-item>
                <div class="hint-text">
                  较高的刷新频率会增加 CPU 占用，建议根据实际需求调整。
                </div>
              </n-card>

              <n-card title="外观定制" class="settings-card glass-card" size="small">
                <n-form-item label="背景风格" path="backgroundStyle">
                  <n-select
                      class="setting-select"
                      v-model:value="settings.backgroundStyle"
                      :options="backgroundStyleOptions"
                      size="medium"
                      @update:value="handleSettingsChange"
                  />
                </n-form-item>
                <n-form-item label="透明度" path="opacity">
                  <div class="slider-wrapper">
                    <n-slider
                        v-model:value="settings.opacity"
                        :min="10"
                        :max="100"
                        :step="1"
                        :tooltip="false"
                        @update:value="handleSettingsChange"
                    />
                    <span class="value-tag">{{ settings.opacity }}%</span>
                  </div>
                </n-form-item>
                <n-form-item label="主色调" path="backgroundAccent">
                  <div class="color-picker-wrapper">
                    <n-color-picker
                        v-model:value="settings.backgroundAccent"
                        :modes="['hex']"
                        :show-alpha="false"
                        size="small"
                        class="mini-color-picker"
                        @update:value="changeBackgroundAccent"
                    />
                    <div class="quick-colors">
                      <div
                          v-for="color in accentColors"
                          :key="color.name"
                          class="color-dot"
                          :style="{ backgroundColor: color.value }"
                          :class="{ active: settings.backgroundAccent === color.value }"
                          @click="changeBackgroundAccent(color.value)"
                          :title="color.name"
                      ></div>
                    </div>
                  </div>
                </n-form-item>
                <n-form-item label="字体颜色" path="foregroundColor">
                  <div class="color-picker-wrapper">
                    <n-color-picker
                        v-model:value="settings.foregroundColor"
                        :modes="['hex']"
                        :show-alpha="false"
                        size="small"
                        class="mini-color-picker"
                        @update:value="changeForegroundColor"
                    />
                    <div class="quick-colors">
                      <div
                          v-for="color in foregroundColors"
                          :key="color.name"
                          class="color-dot"
                          :style="{ backgroundColor: color.value }"
                          :class="{ active: settings.foregroundColor === color.value }"
                          @click="changeForegroundColor(color.value)"
                          :title="color.name"
                      ></div>
                    </div>
                  </div>
                </n-form-item>
                <n-form-item label="字体样式" path="fontFamily">
                  <n-select
                      class="setting-select"
                      v-model:value="settings.fontFamily"
                      :options="fontFamilyOptions"
                      size="medium"
                      @update:value="handleSettingsChange"
                  />
                </n-form-item>
              </n-card>
            </n-space>
          </n-gi>

          <!-- 右侧列 -->
          <n-gi>
            <n-space vertical size="large">
              <n-card title="显示模块" class="settings-card glass-card" size="small">
                <template #header-extra>
                  <span class="card-subtitle">拖拽排序暂不可用，请使用箭头</span>
                </template>
                <div class="module-list">
                  <div
                      v-for="(module, index) in orderedMonitorModules"
                      :key="module.key"
                      class="module-item"
                      :class="{ disabled: !module.enabled }"
                  >
                    <div class="module-info">
                      <div class="module-name">{{ module.label }}</div>
                      <div class="module-desc">{{ module.description || monitorModuleDescriptions[module.key] }}</div>
                    </div>
                    <div class="module-actions">
                      <n-button-group size="tiny">
                        <n-button
                            quaternary
                            :disabled="index === 0"
                            @click="moveMonitorModule(module.key, 'up')"
                        >↑</n-button>
                        <n-button
                            quaternary
                            :disabled="index === orderedMonitorModules.length - 1"
                            @click="moveMonitorModule(module.key, 'down')"
                        >↓</n-button>
                      </n-button-group>
                      <n-switch
                          size="small"
                          :value="module.enabled"
                          @update:value="(value: boolean) => toggleMonitorModule(module.key, value)"
                      />
                    </div>
                  </div>
                </div>
              </n-card>

              <n-card v-if="settings.enableTemperatureMonitor" title="温度监控详情" class="settings-card glass-card" size="small">
                <div class="module-list">
                  <div
                      v-for="(panel, index) in orderedTemperaturePanels"
                      :key="panel.key"
                      class="module-item"
                      :class="{ disabled: !panel.enabled }"
                  >
                    <div class="module-info">
                      <div class="module-name">{{ panel.label }}</div>
                      <div class="module-desc">{{ panel.description || temperaturePanelDescriptions[panel.key] }}</div>
                    </div>
                    <div class="module-actions">
                      <n-button-group size="tiny">
                        <n-button
                            quaternary
                            :disabled="index === 0"
                            @click="moveTemperaturePanel(panel.key, 'up')"
                        >↑</n-button>
                        <n-button
                            quaternary
                            :disabled="index === orderedTemperaturePanels.length - 1"
                            @click="moveTemperaturePanel(panel.key, 'down')"
                        >↓</n-button>
                      </n-button-group>
                      <n-switch
                          size="small"
                          :value="panel.enabled"
                          @update:value="(value: boolean) => toggleTemperaturePanel(panel.key, value)"
                      />
                    </div>
                  </div>
                </div>
              </n-card>

              <n-card title="性能微调" class="settings-card glass-card" size="small">
                <n-form-item label="CPU 平滑度" path="cpuSmoothing">
                  <div class="slider-wrapper">
                    <n-slider
                        v-model:value="settings.cpuSmoothing"
                        :min="10"
                        :max="90"
                        :step="5"
                        :tooltip="false"
                        @update:value="handleSettingsChange"
                    />
                    <span class="value-tag">{{ settings.cpuSmoothing }}%</span>
                  </div>
                </n-form-item>
                <n-form-item label="内存平滑度" path="memorySmoothing">
                  <div class="slider-wrapper">
                    <n-slider
                        v-model:value="settings.memorySmoothing"
                        :min="10"
                        :max="90"
                        :step="5"
                        :tooltip="false"
                        @update:value="handleSettingsChange"
                    />
                    <span class="value-tag">{{ settings.memorySmoothing }}%</span>
                  </div>
                </n-form-item>
              </n-card>

              <n-card title="高级选项" class="settings-card glass-card" size="small">
                <n-grid cols="2" x-gap="12">
                  <n-gi>
                    <n-form-item label="日志级别" path="logLevel">
                      <n-select
                          class="setting-select"
                          v-model:value="settings.logLevel"
                          :options="logLevelOptions"
                          size="small"
                          @update:value="handleSettingsChange"
                      />
                    </n-form-item>
                  </n-gi>
                  <n-gi>
                    <n-form-item label="缓存时间" path="cacheTime">
                      <n-select
                          class="setting-select"
                          v-model:value="settings.cacheTime"
                          :options="cacheTimeOptions"
                          size="small"
                          @update:value="handleSettingsChange"
                      />
                    </n-form-item>
                  </n-gi>
                </n-grid>
              </n-card>
            </n-space>
          </n-gi>
        </n-grid>
      </n-form>

      <div class="settings-footer">
        <div class="footer-info">
          <div class="app-name">System Monitor</div>
          <div class="app-version">v1.0.0 · Tauri & Vue 3</div>
        </div>
        <n-space>
          <n-button secondary type="error" size="medium" @click="resetSettings">重置默认</n-button>
          <n-button type="primary" size="medium" @click="exportSettings">导出配置</n-button>
        </n-space>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { storeToRefs } from 'pinia'
import { invoke } from '@tauri-apps/api/core'
import { useSettingsStore, type SettingsState, type TemperaturePanelPreference, type TemperaturePanelKey, type MonitorModulePreference, type MonitorModuleKey } from '@/stores/settings'
import type { FormInst, FormRules } from 'naive-ui'

const settingsStore = useSettingsStore()
const {settings} = storeToRefs(settingsStore)
const formRef = ref<FormInst | null>(null)

const rules: FormRules = {}

const refreshIntervalOptions = [
  {label: '极速 (0.5s)', value: 500},
  {label: '标准 (1s)', value: 1000},
  {label: '节能 (2s)', value: 2000},
  {label: '懒惰 (5s)', value: 5000}
]

const logLevelOptions = [
  {label: '错误', value: 'error'},
  {label: '警告', value: 'warn'},
  {label: '信息', value: 'info'},
  {label: '调试', value: 'debug'}
]

const cacheTimeOptions = [
  {label: '5 分钟', value: 300},
  {label: '10 分钟', value: 600},
  {label: '30 分钟', value: 1800},
  {label: '1 小时', value: 3600}
]

const backgroundStyleOptions = [
  {label: '磨砂玻璃', value: 'glass'},
  {label: '极光渐变', value: 'aurora'},
  {label: '深邃午夜', value: 'midnight'},
  {label: '完全透明', value: 'transparent'}
]

const accentColors = [
  {name: '曜石黑', value: '#0f172a'},
  {name: '深海蓝', value: '#1d4ed8'},
  {name: '薄荷绿', value: '#14b8a6'},
  {name: '极光紫', value: '#8b5cf6'},
  {name: '熔岩红', value: '#ef4444'},
  {name: '活力橙', value: '#f97316'},
  {name: '清澈蓝', value: '#06b6d4'},
  {name: '星空靛', value: '#6366f1'}
]

const foregroundColors = [
  {name: '纯白', value: '#ffffff', text: '#111827'},
  {name: '冷灰', value: '#f3f4f6', text: '#1f2937'},
  {name: '蓝灰', value: '#cbd5f5', text: '#0f172a'},
  {name: '暖金', value: '#facc15', text: '#1f2937'},
  {name: '荧光绿', value: '#a3e635', text: '#0f172a'},
  {name: '珊瑚粉', value: '#fb7185', text: '#0f172a'}
]

const orderedTemperaturePanels = computed<TemperaturePanelPreference[]>(() => {
  const panels = settings.value.temperaturePanels ?? []
  return [...panels].sort((a, b) => (a.order ?? 0) - (b.order ?? 0))
})

const temperaturePanelDescriptions: Record<TemperaturePanelKey, string> = {
  cpu: 'CPU 核心与封装温度',
  memory: '内存模块温度',
  gpu: '显卡核心与显存温度',
  vrm: '主板供电模块温度',
  motherboard: '主板芯片组温度',
  storage: '硬盘/固态硬盘温度'
}

const orderedMonitorModules = computed<MonitorModulePreference[]>(() => {
  const modules = settings.value.monitorModules ?? []
  return [...modules].sort((a, b) => (a.order ?? 0) - (b.order ?? 0))
})

const monitorModuleDescriptions: Record<MonitorModuleKey, string> = {
  cpu: 'CPU 使用率',
  memory: '内存使用情况',
  gpu: '显卡负载',
  disk: '磁盘读写活动',
  temperature: '硬件温度概览',
  frame: '游戏帧率 (FPS)',
  network: '网络上传/下载速率'
}

const moduleBooleanKeyMap: Record<MonitorModuleKey, keyof SettingsState> = {
  cpu: 'enableCpuMonitor',
  memory: 'enableMemoryMonitor',
  gpu: 'enableGpuMonitor',
  disk: 'enableDiskMonitor',
  temperature: 'enableTemperatureMonitor',
  frame: 'enableFrameStats',
  network: 'enableNetworkMonitor'
}

const updateTemperaturePanels = (panels: TemperaturePanelPreference[]) => {
  settings.value.temperaturePanels = panels.map((panel, index) => ({
    ...panel,
    order: index
  }))
  handleSettingsChange()
}

const updateMonitorModules = (modules: MonitorModulePreference[]) => {
  settings.value.monitorModules = modules.map((module, index) => ({
    ...module,
    order: index
  }))
}

const toggleTemperaturePanel = (key: TemperaturePanelKey, enabled: boolean) => {
  const panels = orderedTemperaturePanels.value.map(panel =>
    panel.key === key ? { ...panel, enabled } : panel
  )
  updateTemperaturePanels(panels)
}

const moveTemperaturePanel = (key: TemperaturePanelKey, direction: 'up' | 'down') => {
  const panels = [...orderedTemperaturePanels.value]
  const index = panels.findIndex(panel => panel.key === key)
  if (index === -1) return
  const targetIndex = direction === 'up' ? index - 1 : index + 1
  if (targetIndex < 0 || targetIndex >= panels.length) return
  const [panel] = panels.splice(index, 1)
  panels.splice(targetIndex, 0, panel)
  updateTemperaturePanels(panels)
}

const toggleMonitorModule = (key: MonitorModuleKey, enabled: boolean) => {
  const modules = orderedMonitorModules.value.map(module =>
    module.key === key ? { ...module, enabled } : module
  )
  updateMonitorModules(modules)
  const boolKey = moduleBooleanKeyMap[key]
  if (boolKey) {
    settings.value = {
      ...settings.value,
      [boolKey]: enabled
    } as SettingsState
  }
  handleSettingsChange()
}

const moveMonitorModule = (key: MonitorModuleKey, direction: 'up' | 'down') => {
  const modules = [...orderedMonitorModules.value]
  const index = modules.findIndex(module => module.key === key)
  if (index === -1) return
  const targetIndex = direction === 'up' ? index - 1 : index + 1
  if (targetIndex < 0 || targetIndex >= modules.length) return
  const [module] = modules.splice(index, 1)
  modules.splice(targetIndex, 0, module)
  updateMonitorModules(modules)
  handleSettingsChange()
}

const fontFamilyOptions = [
  {label: 'Inter (推荐)', value: 'Inter'},
  {label: 'JetBrains Mono', value: 'JetBrains Mono'},
  {label: 'SF Pro Display', value: 'SF Pro Display'},
  {label: '系统默认', value: 'system-ui'}
]

const handleSettingsChange = () => {
  settingsStore.applySettings()
}

const handleAutoStartUpdate = async (value: boolean) => {
  settings.value.autoStart = value
  await settingsStore.setAutoStart(value)
}

const changeBackgroundAccent = (color: string) => {
  settingsStore.updateSettings({backgroundAccent: color})
}

const changeForegroundColor = (color: string) => {
  settingsStore.updateSettings({foregroundColor: color})
}

const resetSettings = async () => {
  if (confirm('确定要重置所有设置吗？此操作不可撤销。')) {
    await settingsStore.resetSettings()
  }
}

const exportSettings = () => {
  const dataStr = settingsStore.exportSettings()
  const dataUri = 'data:application/json;charset=utf-8,' + encodeURIComponent(dataStr)
  const exportFileDefaultName = 'system-monitor-settings.json'
  const linkElement = document.createElement('a')
  linkElement.setAttribute('href', dataUri)
  linkElement.setAttribute('download', exportFileDefaultName)
  linkElement.click()
}

const closeWindow = async () => {
  try {
    await invoke('close_settings_window')
  } catch (error) {
    console.error('关闭设置窗口失败:', error)
    if (window.close) {
      window.close()
    }
  }
}

onMounted(() => {
  settingsStore.ensureInitialized()
})
</script>

<style scoped>
.settings-shell {
  min-height: 100vh;
  background: linear-gradient(135deg, #f5f7fa 0%, #e4e8f0 100%);
  color: #1f2937;
  font-family: 'Inter', system-ui, sans-serif;
  padding: 20px;
  box-sizing: border-box;
}

.settings-container {
  max-width: 1000px;
  margin: 0 auto;
  padding-bottom: 40px;
}

.settings-header {
  margin-bottom: 24px;
  background: rgba(255, 255, 255, 0.6);
  backdrop-filter: blur(12px);
  padding: 16px 24px;
  border-radius: 16px;
  border: 1px solid rgba(255, 255, 255, 0.8);
  box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.05);
}

.close-icon {
  font-size: 24px;
  line-height: 1;
  color: #6b7280;
}

.glass-card {
  background: rgba(255, 255, 255, 0.7);
  backdrop-filter: blur(12px);
  border-radius: 16px;
  border: 1px solid rgba(255, 255, 255, 0.6);
  box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.02), 0 2px 4px -1px rgba(0, 0, 0, 0.02);
  transition: transform 0.2s ease, box-shadow 0.2s ease;
}

.glass-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.05), 0 4px 6px -2px rgba(0, 0, 0, 0.025);
}

:deep(.n-card-header__main) {
  font-weight: 600;
  color: #374151;
}

.hint-text {
  font-size: 12px;
  color: #9ca3af;
  margin-top: 8px;
}

.slider-wrapper {
  display: flex;
  align-items: center;
  gap: 12px;
  width: 100%;
}

.value-tag {
  font-size: 12px;
  font-weight: 600;
  color: #6366f1;
  background: rgba(99, 102, 241, 0.1);
  padding: 2px 6px;
  border-radius: 4px;
  min-width: 36px;
  text-align: center;
}

.color-picker-wrapper {
  display: flex;
  flex-direction: column;
  gap: 12px;
  width: 100%;
}

.mini-color-picker {
  width: 100%;
}

.quick-colors {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.color-dot {
  width: 24px;
  height: 24px;
  border-radius: 50%;
  cursor: pointer;
  border: 2px solid transparent;
  transition: all 0.2s ease;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.color-dot:hover {
  transform: scale(1.1);
}

.color-dot.active {
  border-color: #6366f1;
  transform: scale(1.1);
  box-shadow: 0 0 0 2px rgba(99, 102, 241, 0.2);
}

.module-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.module-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 12px;
  background: rgba(255, 255, 255, 0.5);
  border-radius: 8px;
  transition: background-color 0.2s ease;
}

.module-item:hover {
  background: rgba(255, 255, 255, 0.8);
}

.module-item.disabled {
  opacity: 0.6;
  filter: grayscale(0.8);
}

.module-info {
  flex: 1;
}

.module-name {
  font-weight: 500;
  font-size: 14px;
  color: #374151;
}

.module-desc {
  font-size: 12px;
  color: #9ca3af;
  margin-top: 2px;
}

.module-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}

.card-subtitle {
  font-size: 12px;
  color: #9ca3af;
  font-weight: normal;
}

.settings-footer {
  margin-top: 32px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 20px 24px;
  background: rgba(255, 255, 255, 0.6);
  backdrop-filter: blur(12px);
  border-radius: 16px;
  border: 1px solid rgba(255, 255, 255, 0.8);
}

.footer-info {
  display: flex;
  flex-direction: column;
}

.app-name {
  font-weight: 600;
  color: #374151;
}

.app-version {
  font-size: 12px;
  color: #9ca3af;
}

@media (max-width: 600px) {
  .settings-shell {
    padding: 12px;
  }
  
  .settings-footer {
    flex-direction: column;
    gap: 16px;
    align-items: flex-start;
  }
}
</style>
