<template>
  <div
      ref="monitorRef"
      class="floating-monitor"
      data-tauri-drag-region
      :style="monitorStyles"
      @contextmenu.prevent="handleContextMenu"
  >
    <!-- 加载状态 -->
    <div v-if="loading" class="loading-state">
      <div class="loading-spinner"></div>
    </div>

    <!-- 错误状态 -->
    <div v-else-if="error" class="error-state">
      <div class="error-icon">⚠️</div>
    </div>

    <!-- 数据显示 -->
    <div v-else class="monitor-data">
      <template v-if="settings.enableCpuMonitor">
        <span class="data-label">CPU</span>
        <span class="data-value" :class="cpuStateClass">{{ getCpuUsage }}%</span>
      </template>

      <span
          v-if="settings.enableCpuMonitor && settings.enableMemoryMonitor"
          class="data-divider"
      >|</span>

      <template v-if="settings.enableMemoryMonitor">
        <span class="data-label">内存</span>
        <span class="data-value" :class="memoryStateClass">{{ getMemoryUsage }}%</span>
      </template>

      <template v-if="settings.enableGpuMonitor">
        <span class="data-divider">|</span>
        <span class="data-label">GPU</span>
        <span v-if="gpuInfo" class="data-value">{{ Math.round(gpuInfo.usage_percent) }}%</span>
        <span v-else class="data-value">--</span>
      </template>

      <template v-if="settings.enableDiskMonitor">
        <span class="data-divider">|</span>
        <span class="data-label">磁盘</span>
        <span class="data-value" :title="diskInfo.detail">{{ diskInfo.value }}</span>
      </template>

      <template v-if="settings.enableTemperatureMonitor">
        <span class="data-divider">|</span>
        <span class="data-label">温度</span>
        <span class="data-value" :title="temperatureInfo.detail">{{ temperatureInfo.value }}</span>
      </template>

      <template v-if="showNetwork">
        <span class="data-divider">|</span>
        <span class="data-label">网络</span>
        <span class="data-value network-values">
          <div class="network-download">↓{{ networkSpeed.download }}</div>
          <div class="network-upload">↑{{ networkSpeed.upload }}</div>
        </span>
      </template>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, nextTick, onMounted, onUnmounted, ref, watch } from 'vue'
import { storeToRefs } from 'pinia'
import { getCurrentWindow, type Window } from '@tauri-apps/api/window'
import { LogicalSize } from '@tauri-apps/api/dpi'
import { useSystemMonitor } from '@/composables/useSystemMonitor'
import { useSystemStore } from '@/stores/system'
import { useSettingsStore, type SettingsState } from '@/stores/settings'

const {
  systemInfo,
  gpuInfo,
  error,
  networkSpeed
} = useSystemMonitor(true)

const systemStore = useSystemStore()
const settingsStore = useSettingsStore()
const { settings } = storeToRefs(settingsStore)

let mainWindow: Window | null = null
const ensureMainWindow = () => {
  if (mainWindow) return mainWindow
  if (typeof window === 'undefined') return null
  if (!('__TAURI_INTERNALS__' in window)) return null
  mainWindow = getCurrentWindow()
  return mainWindow
}

const loading = computed(() => !systemInfo.value && !error.value)
const monitorRef = ref<HTMLElement | null>(null)
let resizeObserver: ResizeObserver | null = null
let resizeRaf = 0
const lastAppliedSize = ref<{ width: number; height: number }>({ width: 0, height: 0 })
const MIN_WIDTH = 200
const MIN_HEIGHT = 48
const WIDTH_BUFFER = 24
const HEIGHT_BUFFER = 16

const cpuDisplay = ref<number | null>(null)
const memoryDisplay = ref<number | null>(null)

const clamp = (value: number, min: number, max: number) => Math.min(max, Math.max(min, value))

const easeValue = (prev: number | null, next: number, smoothingPercent: number) => {
  if (!Number.isFinite(next)) return prev ?? null
  if (prev === null) return Number(next.toFixed(1))
  const lerp = clamp((100 - smoothingPercent) / 100, 0.05, 1)
  return Number((prev + (next - prev) * lerp).toFixed(1))
}

watch(
  () => systemInfo.value?.cpu_usage,
  (usage) => {
    if (typeof usage !== 'number') return
    cpuDisplay.value = easeValue(cpuDisplay.value, usage, settings.value.cpuSmoothing)
  },
  { immediate: true }
)

watch(
  () => settings.value.cpuSmoothing,
  () => {
    if (typeof systemInfo.value?.cpu_usage === 'number') {
      cpuDisplay.value = Number(systemInfo.value.cpu_usage.toFixed(1))
    }
  }
)

watch(
  () => systemInfo.value?.memory?.usage_percent,
  (usage) => {
    if (typeof usage !== 'number') return
    memoryDisplay.value = easeValue(memoryDisplay.value, usage, settings.value.memorySmoothing)
  },
  { immediate: true }
)

watch(
  () => settings.value.memorySmoothing,
  () => {
    const usage = systemInfo.value?.memory?.usage_percent
    if (typeof usage === 'number') {
      memoryDisplay.value = Number(usage.toFixed(1))
    }
  }
)

const getCpuUsage = computed(() => {
  if (!settings.value.enableCpuMonitor || cpuDisplay.value === null) return '--'
  return cpuDisplay.value.toFixed(1)
})

const getMemoryUsage = computed(() => {
  if (!settings.value.enableMemoryMonitor || memoryDisplay.value === null) return '--'
  return memoryDisplay.value.toFixed(1)
})

const cpuStateClass = computed(() => {
  if (!settings.value.enableCpuMonitor || cpuDisplay.value === null) return 'muted'
  if (cpuDisplay.value >= settings.value.cpuAlertThreshold) return 'critical'
  if (cpuDisplay.value >= settings.value.cpuAlertThreshold - 10) return 'warn'
  return 'normal'
})

const memoryStateClass = computed(() => {
  if (!settings.value.enableMemoryMonitor || memoryDisplay.value === null) return 'muted'
  if (memoryDisplay.value >= settings.value.memoryAlertThreshold) return 'critical'
  if (memoryDisplay.value >= settings.value.memoryAlertThreshold - 10) return 'warn'
  return 'normal'
})

const monitorOpacity = computed(() => clamp(settings.value.opacity / 100, 0.1, 1))

const hexToRgba = (hexColor: string, alpha: number) => {
  const hex = hexColor?.replace('#', '') || '3b82f6'
  const normalized = hex.length === 3 ? hex.split('').map((c) => c + c).join('') : hex.padEnd(6, '0')
  const bigint = parseInt(normalized, 16)
  const r = (bigint >> 16) & 255
  const g = (bigint >> 8) & 255
  const b = bigint & 255
  return `rgba(${r}, ${g}, ${b}, ${alpha})`
}

const fontStack = computed(() => {
  const preferred = settings.value.fontFamily?.trim()
  if (!preferred) {
    return 'Inter, "Segoe UI", system-ui, -apple-system, sans-serif'
  }
  return `${preferred}, "Segoe UI", system-ui, -apple-system, sans-serif`
})

const buildBackground = (style: SettingsState['backgroundStyle'], accent: string) => {
  switch (style) {
    case 'aurora':
      return `linear-gradient(120deg, ${hexToRgba(accent, 0.65)} 0%, rgba(56, 189, 248, 0.25) 45%, rgba(14, 165, 233, 0.2) 100%)`
    case 'midnight':
      return `linear-gradient(135deg, rgba(15, 23, 42, 0.95) 0%, ${hexToRgba(accent, 0.55)} 100%)`
    case 'transparent':
      return 'transparent'
    case 'glass':
    default:
      return `linear-gradient(120deg, ${hexToRgba(accent, 0.55)} 0%, ${hexToRgba(accent, 0.25)} 100%)`
  }
}

const monitorStyles = computed(() => {
  const accent = settings.value.backgroundAccent || '#3b82f6'
  const foreground = settings.value.foregroundColor || '#ffffff'
  const style = (settings.value.backgroundStyle || 'glass') as SettingsState['backgroundStyle']
  const background = buildBackground(style, accent)
  const border = style === 'transparent' ? '1px solid transparent' : `1px solid ${hexToRgba(accent, 0.35)}`
  const backdropFilter = style === 'transparent' ? 'none' : 'blur(18px)'
  return {
    '--monitor-accent': hexToRgba(accent, 0.5),
    '--monitor-foreground': foreground,
    background,
    border,
    opacity: monitorOpacity.value,
    color: foreground,
    fontFamily: fontStack.value,
    boxShadow: 'none',
    backdropFilter
  }
})

const scheduleWindowResize = (width: number, height: number) => {
  if (typeof window === 'undefined') return
  if (resizeRaf) {
    window.cancelAnimationFrame(resizeRaf)
  }
  resizeRaf = window.requestAnimationFrame(() => {
    resizeRaf = 0
    applyWindowSize(width, height)
  })
}

const extractEntrySize = (entry: ResizeObserverEntry) => {
  const box = Array.isArray(entry.borderBoxSize)
    ? entry.borderBoxSize[0]
    : entry.borderBoxSize

  if (box && typeof box === 'object') {
    return {
      width: box.inlineSize ?? entry.contentRect.width,
      height: box.blockSize ?? entry.contentRect.height
    }
  }

  return {
    width: entry.contentRect.width,
    height: entry.contentRect.height
  }
}

const applySizeBuffer = (width: number, height: number) => {
  const el = monitorRef.value
  const contentWidth = el ? Math.max(width, el.scrollWidth) : width
  const contentHeight = el ? Math.max(height, el.scrollHeight) : height
  return {
    width: contentWidth + WIDTH_BUFFER,
    height: contentHeight + HEIGHT_BUFFER
  }
}

const applyWindowSize = async (rawWidth: number, rawHeight: number) => {
  const win = ensureMainWindow()
  if (!win) return
  const buffered = applySizeBuffer(rawWidth, rawHeight)
  const width = Math.max(Math.ceil(buffered.width), MIN_WIDTH)
  const height = Math.max(Math.ceil(buffered.height), MIN_HEIGHT)
  if (
    width === lastAppliedSize.value.width &&
    height === lastAppliedSize.value.height
  ) {
    return
  }
  lastAppliedSize.value = { width, height }
  try {
    const logicalSize = new LogicalSize(width, height)
    await win.setSize(logicalSize)
    await win.setMinSize(logicalSize)
    await win.setMaxSize(logicalSize)
  } catch (err) {
    console.error('更新窗口尺寸失败:', err)
  }
}

const startSizeObserver = () => {
  if (!monitorRef.value || typeof ResizeObserver === 'undefined') return
  resizeObserver?.disconnect()
  resizeObserver = new ResizeObserver((entries) => {
    const entry = entries[0]
    if (!entry) return
    const { width, height } = extractEntrySize(entry)
    scheduleWindowResize(width, height)
  })
  resizeObserver.observe(monitorRef.value)
  const rect = monitorRef.value.getBoundingClientRect()
  scheduleWindowResize(rect.width, rect.height)
}

watch(
  () => settings.value.alwaysOnTop,
  async (value) => {
    const win = ensureMainWindow()
    if (!win) return
    try {
      await win.setAlwaysOnTop(value)
    } catch (err) {
      console.error('更新窗口置顶失败:', err)
    }
  },
  { immediate: true }
)

watch(
  () => settings.value.showInTaskbar,
  async (show) => {
    const win = ensureMainWindow()
    if (!win) return
    try {
      await win.setSkipTaskbar(!show)
    } catch (err) {
      console.error('更新任务栏显示状态失败:', err)
    }
  },
  { immediate: true }
)

const diskInfo = computed(() => {
  if (!settings.value.enableDiskMonitor) {
    return { value: '--', detail: '' }
  }
  const disks = systemInfo.value?.disk?.disks
  if (!disks || !disks.length) {
    return { value: '--', detail: '' }
  }
  const busiest = [...disks].sort((a, b) => b.usage_percent - a.usage_percent)[0]
  const label = busiest.mount_point || busiest.name || 'Disk'
  const used = systemStore.formatBytes?.(busiest.used_space) ?? ''
  const total = systemStore.formatBytes?.(busiest.total_space) ?? ''
  return {
    value: `${Math.round(busiest.usage_percent)}%`,
    detail: `${label} · ${used}/${total}`
  }
})

const temperatureInfo = computed(() => {
  if (!settings.value.enableTemperatureMonitor) {
    return { value: '--', detail: '' }
  }
  const temps = systemInfo.value?.temperatures
  if (!temps || !temps.length) {
    return { value: '--', detail: '' }
  }
  const hottest = temps.reduce((max, current) => {
    return current.temperature > max.temperature ? current : max
  })
  const formatted = systemStore.formatTemperature?.(hottest.temperature) ?? `${hottest.temperature.toFixed(1)}°C`
  const extra: string[] = []
  if (hottest.max) extra.push(`Max ${hottest.max.toFixed(0)}°C`)
  if (hottest.critical) extra.push(`Critical ${hottest.critical.toFixed(0)}°C`)
  return {
    value: formatted,
    detail: `${hottest.label}${extra.length ? ` · ${extra.join(' / ')}` : ''}`
  }
})

const showNetwork = computed(() => settings.value.enableNetworkMonitor && !!systemInfo.value?.network)

const handleContextMenu = (e: MouseEvent) => {
  e.preventDefault()
}

onMounted(async () => {
  await settingsStore.ensureInitialized()
  await nextTick()
  startSizeObserver()
  const [isAvailable] = await systemStore.getGpuMonitorStatus()
  if (isAvailable) {
    await systemStore.getGpuNames()
    await systemStore.getDetailedGpuInfo(0)
  }
})

onUnmounted(() => {
  if (resizeObserver) {
    resizeObserver.disconnect()
    resizeObserver = null
  }
  if (resizeRaf && typeof window !== 'undefined') {
    window.cancelAnimationFrame(resizeRaf)
    resizeRaf = 0
  }
})
</script>

<style scoped>
.floating-monitor {
  min-height: 40px;
  font-weight: 600;
  border-radius: 22px;
  padding: 8px 20px;
  display: inline-flex;
  align-items: center;
  gap: 8px;
  width: fit-content;
  max-width: calc(100vw - 12px);
  white-space: nowrap;
  border: 1px solid transparent;
  transition: opacity 0.2s ease;
}

.loading-state,
.error-state,
.monitor-data {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  width: 100%;
  pointer-events: none;
}

.loading-spinner {
  width: 16px;
  height: 16px;
  border: 2px solid rgba(255, 255, 255, 0.3);
  border-top: 2px solid white;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

.error-icon {
  font-size: 20px;
}

.data-label {
  font-size: 15px;
  font-weight: 600;
  color: var(--monitor-foreground, rgba(255, 255, 255, 0.92));
  text-shadow: 0 1px 3px rgba(0, 0, 0, 0.5);
}

.data-value {
  font-size: 14px;
  font-weight: 700;
  color: var(--monitor-foreground, #ffffff);
  min-width: 36px;
  letter-spacing: 0.4px;
  transition: color 0.2s ease, text-shadow 0.2s ease;
}

.data-value.warn {
  color: #facc15;
  text-shadow: 0 0 8px rgba(250, 204, 21, 0.65);
}

.data-value.critical {
  color: #f87171;
  text-shadow: 0 0 10px rgba(248, 113, 113, 0.75);
}

.data-value.muted {
  color: rgba(255, 255, 255, 0.55);
}

.data-divider {
  color: var(--monitor-accent, rgba(255, 255, 255, 0.35));
  font-size: 14px;
  font-weight: 400;
  margin: 0 4px;
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.4);
}

.network-values {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  line-height: 1.1;
  font-size: 12px;
}

.network-download,
.network-upload {
  letter-spacing: 0.2px;
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}
</style>
