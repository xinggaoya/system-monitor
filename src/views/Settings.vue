<template>
  <div class="settings-shell">
    <n-page-header
        class="settings-header"
        title="系统监控设置"
        subtitle="自定义刷新策略和外观，让状态一目了然"
    >
      <template #extra>
        <n-button round secondary size="small" @click="closeWindow">关闭</n-button>
      </template>
    </n-page-header>

    <n-form ref="formRef" :model="settings" :rules="rules" :label-width="120">
      <n-space vertical size="large">
        <n-grid cols="1 960:2" x-gap="16" y-gap="16">
          <n-gi>
            <n-card title="通用设置" class="settings-card">
              <n-form-item label="开机自启动" path="autoStart">
                <n-switch
                    v-model:value="settings.autoStart"
                    @update:value="handleAutoStartUpdate"
                />
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
          </n-gi>

          <n-gi>
            <n-card title="监控策略" class="settings-card">
              <n-form-item label="刷新频率" path="refreshInterval">
                <n-select
                    class="setting-select"
                    v-model:value="settings.refreshInterval"
                    :options="refreshIntervalOptions"
                    size="small"
                    @update:value="handleSettingsChange"
                />
              </n-form-item>
              <n-form-item label="启用 CPU 监控" path="enableCpuMonitor">
                <n-switch
                    v-model:value="settings.enableCpuMonitor"
                    @update:value="handleSettingsChange"
                />
              </n-form-item>
              <n-form-item label="启用内存监控" path="enableMemoryMonitor">
                <n-switch
                    v-model:value="settings.enableMemoryMonitor"
                    @update:value="handleSettingsChange"
                />
              </n-form-item>
              <n-form-item label="启用 GPU 监控" path="enableGpuMonitor">
                <n-switch
                    v-model:value="settings.enableGpuMonitor"
                    @update:value="handleSettingsChange"
                />
              </n-form-item>
              <n-form-item label="启用网络监控" path="enableNetworkMonitor">
                <n-switch
                    v-model:value="settings.enableNetworkMonitor"
                    @update:value="handleSettingsChange"
                />
              </n-form-item>
              <n-form-item label="启用磁盘监控" path="enableDiskMonitor">
                <n-switch
                    v-model:value="settings.enableDiskMonitor"
                    @update:value="handleSettingsChange"
                />
              </n-form-item>
              <n-form-item label="启用温度监控" path="enableTemperatureMonitor">
                <n-switch
                    v-model:value="settings.enableTemperatureMonitor"
                    @update:value="handleSettingsChange"
                />
              </n-form-item>
            </n-card>
          </n-gi>

          <n-gi>
            <n-card title="性能控制" class="settings-card">
              <n-form-item label="CPU 平滑" path="cpuSmoothing">
                <div class="setting-control">
                  <n-slider
                      v-model:value="settings.cpuSmoothing"
                      :min="10"
                      :max="90"
                      :step="5"
                      @update:value="handleSettingsChange"
                  />
                  <span class="setting-value">{{ settings.cpuSmoothing }}%</span>
                </div>
              </n-form-item>
              <n-form-item label="CPU 警戒线" path="cpuAlertThreshold">
                <div class="setting-control">
                  <n-slider
                      v-model:value="settings.cpuAlertThreshold"
                      :min="40"
                      :max="100"
                      :step="5"
                      @update:value="handleSettingsChange"
                  />
                  <span class="setting-value">{{ settings.cpuAlertThreshold }}%</span>
                </div>
              </n-form-item>
              <n-form-item label="内存平滑" path="memorySmoothing">
                <div class="setting-control">
                  <n-slider
                      v-model:value="settings.memorySmoothing"
                      :min="10"
                      :max="90"
                      :step="5"
                      @update:value="handleSettingsChange"
                  />
                  <span class="setting-value">{{ settings.memorySmoothing }}%</span>
                </div>
              </n-form-item>
              <n-form-item label="内存警戒线" path="memoryAlertThreshold">
                <div class="setting-control">
                  <n-slider
                      v-model:value="settings.memoryAlertThreshold"
                      :min="40"
                      :max="100"
                      :step="5"
                      @update:value="handleSettingsChange"
                  />
                  <span class="setting-value">{{ settings.memoryAlertThreshold }}%</span>
                </div>
              </n-form-item>
            </n-card>
          </n-gi>

          <n-gi>
            <n-card title="背景设置" class="settings-card">
              <n-form-item label="透明度" path="opacity">
                <div class="setting-control">
                  <n-slider
                      class="opacity-slider"
                      v-model:value="settings.opacity"
                      :min="10"
                      :max="100"
                      :step="1"
                      @update:value="handleSettingsChange"
                  />
                  <span class="setting-value">{{ settings.opacity }}%</span>
                </div>
              </n-form-item>
              <n-form-item label="背景风格" path="backgroundStyle">
                <n-select
                    class="setting-select"
                    v-model:value="settings.backgroundStyle"
                    :options="backgroundStyleOptions"
                    size="small"
                    @update:value="handleSettingsChange"
                />
              </n-form-item>
              <n-form-item label="主色调" path="backgroundAccent">
                <div class="setting-control color-control">
                  <n-color-picker
                      v-model:value="settings.backgroundAccent"
                      :modes="['hex']"
                      :show-alpha="false"
                      size="small"
                      @update:value="changeBackgroundAccent"
                  />
                  <div class="quick-colors">
                    <n-button
                        v-for="color in accentColors"
                        :key="color.name"
                        quaternary
                        round
                        size="tiny"
                        :style="{ backgroundColor: color.value, color: '#fff' }"
                        :class="['color-chip', { active: settings.backgroundAccent === color.value }]"
                        @click="changeBackgroundAccent(color.value)"
                    >
                      {{ color.name }}
                    </n-button>
                  </div>
                </div>
              </n-form-item>
            </n-card>
          </n-gi>

          <n-gi>
            <n-card title="字体设置" class="settings-card">
              <n-form-item label="字体颜色" path="foregroundColor">
                <div class="setting-control color-control">
                  <n-color-picker
                      v-model:value="settings.foregroundColor"
                      :modes="['hex']"
                      :show-alpha="false"
                      size="small"
                      @update:value="changeForegroundColor"
                  />
                  <div class="quick-colors">
                    <n-button
                        v-for="color in foregroundColors"
                        :key="color.name"
                        quaternary
                        round
                        size="tiny"
                        :style="{ backgroundColor: color.value, color: color.text }"
                        :class="['color-chip', { active: settings.foregroundColor === color.value }]"
                        @click="changeForegroundColor(color.value)"
                    >
                      {{ color.name }}
                    </n-button>
                  </div>
                </div>
              </n-form-item>
              <n-form-item label="字体样式" path="fontFamily">
                <n-select
                    class="setting-select"
                    v-model:value="settings.fontFamily"
                    :options="fontFamilyOptions"
                    size="small"
                    @update:value="handleSettingsChange"
                />
              </n-form-item>
            </n-card>
          </n-gi>

          <n-gi>
            <n-card title="高级设置" class="settings-card">
              <n-form-item label="日志级别" path="logLevel">
                <n-select
                    class="setting-select"
                    v-model:value="settings.logLevel"
                    :options="logLevelOptions"
                    size="small"
                    @update:value="handleSettingsChange"
                />
              </n-form-item>
              <n-form-item label="数据缓存时间" path="cacheTime">
                <n-select
                    class="setting-select"
                    v-model:value="settings.cacheTime"
                    :options="cacheTimeOptions"
                    size="small"
                    @update:value="handleSettingsChange"
                />
              </n-form-item>
            </n-card>
          </n-gi>
        </n-grid>

        <n-card class="settings-footer-card">
          <div class="footer-info">
            <div>系统监控 v1.0.0</div>
            <span>基于 Tauri · Vue 3 · Naive UI</span>
          </div>
          <n-space>
            <n-button tertiary @click="resetSettings">重置设置</n-button>
            <n-button type="primary" @click="exportSettings">导出配置</n-button>
          </n-space>
        </n-card>
      </n-space>
    </n-form>
  </div>
</template>

<script setup lang="ts">
import {onMounted, ref} from 'vue'
import {storeToRefs} from 'pinia'
import {invoke} from '@tauri-apps/api/core'
import {useSettingsStore} from '@/stores/settings'
import type {FormInst, FormRules} from 'naive-ui'

const settingsStore = useSettingsStore()
const {settings} = storeToRefs(settingsStore)
const formRef = ref<FormInst | null>(null)

const rules: FormRules = {}

const refreshIntervalOptions = [
  {label: '0.5 秒', value: 500},
  {label: '1 秒', value: 1000},
  {label: '2 秒', value: 2000},
  {label: '5 秒', value: 5000}
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
  {label: '玻璃质感', value: 'glass'},
  {label: '极光渐变', value: 'aurora'},
  {label: '午夜暗色', value: 'midnight'},
  {label: '纯透明', value: 'transparent'}
]

const accentColors = [
  {name: '曜石', value: '#0f172a'},
  {name: '深蓝', value: '#1d4ed8'},
  {name: '薄荷', value: '#14b8a6'},
  {name: '极光紫', value: '#8b5cf6'},
  {name: '熔岩', value: '#ef4444'},
  {name: '夕阳', value: '#f97316'},
  {name: '青碧', value: '#06b6d4'},
  {name: '星辉', value: '#6366f1'}
]

const foregroundColors = [
  {name: '纯白', value: '#ffffff', text: '#111827'},
  {name: '冷白', value: '#f3f4f6', text: '#1f2937'},
  {name: '蓝灰', value: '#cbd5f5', text: '#0f172a'},
  {name: '暖金', value: '#facc15', text: '#1f2937'},
  {name: '霓虹绿', value: '#a3e635', text: '#0f172a'},
  {name: '珊瑚', value: '#fb7185', text: '#0f172a'}
]

const fontFamilyOptions = [
  {label: 'Inter', value: 'Inter'},
  {label: 'JetBrains Mono', value: 'JetBrains Mono'},
  {label: 'SF Pro Display', value: 'SF Pro Display'},
  {label: 'System UI', value: 'system-ui'}
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
  if (confirm('确定要重置所有设置吗？')) {
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
  padding: 32px;
  background: radial-gradient(circle at top, rgba(119, 136, 255, 0.12), transparent 45%),
  linear-gradient(135deg, #f7f9fc 0%, #eef2ff 100%);
}

.settings-header {
  border-radius: 16px;
  background: rgba(255, 255, 255, 0.85);
  box-shadow: 0 12px 30px rgba(15, 23, 42, 0.08);
  margin-bottom: 24px;
  padding: 16px 24px;
}

.settings-card {
  border-radius: 18px;
  box-shadow: 0 12px 30px rgba(15, 23, 42, 0.08);
  border: none;
}

.setting-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  padding: 12px 0;
  border-bottom: 1px solid rgba(99, 102, 241, 0.08);
}

.setting-row:last-child {
  border-bottom: none;
  padding-bottom: 0;
}

.setting-meta {
  flex: 1;
}

.setting-title {
  margin: 0;
  font-weight: 600;
  color: #1f2937;
}

.setting-desc {
  margin: 4px 0 0;
  font-size: 12px;
  color: #6b7280;
}

.setting-control {
  display: flex;
  align-items: center;
  gap: 12px;
  width: 100%;
}

.setting-value {
  font-weight: 600;
  color: #6366f1;
  width: 48px;
  text-align: right;
}

:deep(.setting-control .n-slider) {
  flex: 1;
  min-width: 220px;
}

.opacity-slider {
  width: 180px;
}

.setting-select {
  min-width: 140px;
}

.color-control {
  flex-wrap: wrap;
}

.quick-colors {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.color-chip {
  font-size: 12px;
  color: #fff;
  border: none;
  box-shadow: none;
}

.color-chip.active {
  box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.3);
}

.settings-footer-card {
  display: flex;
  align-items: center;
  justify-content: space-between;
  border-radius: 18px;
  box-shadow: 0 12px 30px rgba(15, 23, 42, 0.08);
}

.footer-info {
  display: flex;
  flex-direction: column;
  color: #6b7280;
}

@media (max-width: 768px) {
  .settings-shell {
    padding: 16px;
  }

  .setting-row {
    flex-direction: column;
    align-items: flex-start;
  }

  .setting-control {
    width: 100%;
  }

  .setting-select,
  .opacity-slider {
    width: 100%;
  }

  .settings-footer-card {
    flex-direction: column;
    gap: 16px;
  }
}
</style>
