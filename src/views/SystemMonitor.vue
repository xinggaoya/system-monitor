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
      <template v-for="(module, index) in moduleDisplays" :key="module.key">
        <span v-if="index > 0" class="data-divider"></span>
        <div class="data-group" :class="module.type">
          <span class="data-label">{{ module.label }}</span>
          
          <template v-if="module.type === 'text'">
            <span class="data-value" :class="module.valueClass" :title="module.tooltip">
              {{ module.value }}
            </span>
          </template>
          
          <template v-else-if="module.type === 'temperature'">
            <span class="data-value temperature-breakdown" :title="module.badge?.detail">
              {{ module.badge?.value ?? '--' }}
            </span>
          </template>
          
          <template v-else-if="module.type === 'frame'">
            <div class="frame-stats" :class="module.frame?.qualityClass" :title="module.frame?.detail">
              <span class="fps-value">{{ module.frame?.fpsText }}</span>
              <span class="frametime-value">{{ module.frame?.frameTimeText }}</span>
            </div>
          </template>
          
          <template v-else-if="module.type === 'network'">
            <div class="network-values">
              <div class="network-row download">
                <span class="arrow">↓</span>
                <span class="net-value">{{ module.network?.download }}</span>
              </div>
              <div class="network-row upload">
                <span class="arrow">↑</span>
                <span class="net-value">{{ module.network?.upload }}</span>
              </div>
            </div>
          </template>
        </div>
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
import type { TemperatureInfo, TemperatureCategory } from '@/stores/system'
import { useSettingsStore, type SettingsState, type TemperaturePanelPreference, type TemperaturePanelKey, type MonitorModulePreference, type MonitorModuleKey } from '@/stores/settings'

const {
  systemInfo,
  gpuInfo,
  error,
  networkSpeed,
  frameStats,
  frameError
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

type TemperatureGroupMap = Record<TemperatureCategory, TemperatureInfo[]>

interface TemperatureBadge {
  key: string
  label: string
  sensorLabel: string
  value: string
  detail: string
  category: TemperatureCategory | 'mixed'
}

interface TemperatureBlock {
  key: string
  label: string
  badge: TemperatureBadge
}

interface ModuleDisplay {
  key: string
  label: string
  type: 'text' | 'temperature' | 'frame' | 'network'
  value?: string
  valueClass?: string
  tooltip?: string
  badge?: TemperatureBadge
  frame?: {
    fpsText: string
    frameTimeText: string
    detail: string
    qualityClass: string
  }
  network?: {
    download: string
    upload: string
  }
}

const clamp = (value: number, min: number, max: number) => Math.min(max, Math.max(min, value))

const easeValue = (prev: number | null, next: number, smoothingPercent: number) => {
  if (!Number.isFinite(next)) return prev ?? null
  if (prev === null) return Number(next.toFixed(1))
  const lerp = clamp((100 - smoothingPercent) / 100, 0.05, 1)
  return Number((prev + (next - prev) * lerp).toFixed(1))
}

const createTemperatureGroups = (): TemperatureGroupMap => ({
  'cpu-package': [],
  'cpu-core': [],
  memory: [],
  gpu: [],
  motherboard: [],
  vrm: [],
  storage: [],
  other: []
})

const includesKeyword = (target: string, keywords: string[]) => {
  return keywords.some((keyword) => target.includes(keyword))
}

const categorizeTemperatureLabel = (label: string): TemperatureCategory => {
  const normalized = label.toLowerCase()
  if (
    normalized.includes('cpu') &&
    includesKeyword(normalized, ['package', 'tdie', 'tctl', 'socket', 'die'])
  ) {
    return 'cpu-package'
  }
  if (
    normalized.includes('cpu') &&
    includesKeyword(normalized, ['core', '#', 'ccd', 'ccx', 'thread', 'l3'])
  ) {
    return 'cpu-core'
  }
  if (includesKeyword(normalized, ['dimm', 'memory', 'ram'])) {
    return 'memory'
  }
  if (includesKeyword(normalized, ['gpu', 'graphics', 'video'])) {
    return 'gpu'
  }
  if (includesKeyword(normalized, ['vrm', 'vcore', 'soc'])) {
    return 'vrm'
  }
  if (includesKeyword(normalized, ['pch', 'chipset', 'motherboard', 'board'])) {
    return 'motherboard'
  }
  if (includesKeyword(normalized, ['nvme', 'ssd', 'hdd', 'm.2', 'm2', 'drive', 'storage'])) {
    return 'storage'
  }
  return 'other'
}

const formatTemperatureValue = (temp: number) => {
  return systemStore.formatTemperature?.(temp) ?? `${temp.toFixed(1)}°C`
}

const buildTemperatureBadge = (
  key: string,
  label: string,
  sensors: TemperatureInfo[],
  category: TemperatureCategory | 'mixed'
): TemperatureBadge | null => {
  if (!sensors?.length) return null
  const hottest = sensors.reduce((max, current) => {
    return current.temperature > max.temperature ? current : max
  })
  const extra: string[] = []
  if (typeof hottest.max === 'number') extra.push(`Max ${hottest.max.toFixed(0)}°C`)
  if (typeof hottest.critical === 'number') extra.push(`Critical ${hottest.critical.toFixed(0)}°C`)
  return {
    key,
    label,
    sensorLabel: hottest.label,
    value: formatTemperatureValue(hottest.temperature),
    detail: `${hottest.label}${extra.length ? ` · ${extra.join(' / ')}` : ''}`,
    category
  }
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

const frameModuleState = computed(() => {
  if (!settings.value.enableFrameStats) {
    return {
      fpsText: '--',
      frameTimeText: '--',
      detail: '帧率采集未启用',
      qualityClass: 'state-idle'
    }
  }
  if (frameError.value) {
    return {
      fpsText: 'ERR',
      frameTimeText: '--',
      detail: frameError.value ?? '帧率采集失败',
      qualityClass: 'state-critical'
    }
  }
  const stats = frameStats.value
  if (!stats) {
    return {
      fpsText: '--',
      frameTimeText: '--',
      detail: '等待帧率采集',
      qualityClass: 'state-idle'
    }
  }
  const fps = stats.average_fps
  const fpsText = Number.isFinite(fps) ? `${fps.toFixed(0)}` : '--'
  const frameTimeText = fps > 0 ? `${(1000 / fps).toFixed(1)}ms` : '--'
  let qualityClass = 'state-idle'
  if (fps >= 70) {
    qualityClass = 'state-good'
  } else if (fps >= 45) {
    qualityClass = 'state-warn'
  } else if (fps > 0) {
    qualityClass = 'state-critical'
  }
  const sourceLabel = (() => {
    switch (stats.source) {
      case 'present_mon':
        return 'PresentMon'
      case 'missing_dependency':
        return '依赖缺失'
      default:
        return '未接入'
    }
  })()
  const detail = `来源: ${sourceLabel} · 样本 ${stats.sample_count} · 窗口 ${stats.duration_ms}ms`
  return {
    fpsText,
    frameTimeText,
    detail,
    qualityClass
  }
})

const temperatureGroups = computed<TemperatureGroupMap>(() => {
  const grouped = createTemperatureGroups()
  const temps = systemInfo.value?.temperatures ?? []
  temps.forEach((sensor) => {
    const category = sensor.category ?? categorizeTemperatureLabel(sensor.label)
    grouped[category].push(sensor)
  })
  return grouped
})

const aggregatedTemperatureBadge = computed(() => {
  const temps = systemInfo.value?.temperatures ?? []
  if (!temps.length) return null
  return buildTemperatureBadge('aggregate', '温度', temps, 'mixed')
})

const temperaturePanelPreferences = computed<TemperaturePanelPreference[]>(() => {
  const panels = settings.value.temperaturePanels ?? []
  return [...panels]
    .sort((a, b) => (a.order ?? 0) - (b.order ?? 0))
    .filter(panel => panel.enabled)
})

const monitorModulePreferences = computed<MonitorModulePreference[]>(() => {
  const modules = settings.value.monitorModules ?? []
  return [...modules]
    .sort((a, b) => (a.order ?? 0) - (b.order ?? 0))
    .filter(module => module.enabled)
})

const isModuleEnabledInSettings = (key: MonitorModuleKey): boolean => {
  switch (key) {
    case 'cpu':
      return settings.value.enableCpuMonitor
    case 'memory':
      return settings.value.enableMemoryMonitor
    case 'gpu':
      return settings.value.enableGpuMonitor
    case 'disk':
      return settings.value.enableDiskMonitor
    case 'temperature':
      return settings.value.enableTemperatureMonitor
    case 'frame':
      return settings.value.enableFrameStats
    case 'network':
      return settings.value.enableNetworkMonitor
    default:
      return true
  }
}

const activeMonitorModules = computed<MonitorModulePreference[]>(() => {
  return monitorModulePreferences.value.filter(module => isModuleEnabledInSettings(module.key))
})

const getPanelSensors = (key: TemperaturePanelKey): { sensors: TemperatureInfo[]; category: TemperatureCategory | 'mixed' } => {
  const groups = temperatureGroups.value
  switch (key) {
    case 'cpu': {
      if (groups['cpu-package'].length) {
        return { sensors: groups['cpu-package'], category: 'cpu-package' }
      }
      if (groups['cpu-core'].length) {
        return { sensors: groups['cpu-core'], category: 'cpu-core' }
      }
      return { sensors: [...groups['cpu-package'], ...groups['cpu-core']], category: 'mixed' }
    }
    case 'memory':
      return { sensors: groups.memory, category: 'memory' }
    case 'gpu':
      return { sensors: groups.gpu, category: 'gpu' }
    case 'vrm':
      return { sensors: groups.vrm, category: 'vrm' }
    case 'motherboard':
      return { sensors: groups.motherboard, category: 'motherboard' }
    case 'storage':
      return { sensors: groups.storage, category: 'storage' }
    default:
      return { sensors: [], category: 'other' }
  }
}

const temperatureBlocks = computed<TemperatureBlock[]>(() => {
  if (!settings.value.enableTemperatureMonitor) return []

  const blocks: TemperatureBlock[] = []
  for (const panel of temperaturePanelPreferences.value) {
    const { sensors, category } = getPanelSensors(panel.key)
    let badge: TemperatureBadge | null = null
    if (sensors.length) {
      badge = buildTemperatureBadge(panel.key, panel.label, sensors, category)
    } else {
      badge = {
        key: `${panel.key}-placeholder`,
        label: panel.label,
        sensorLabel: panel.label,
        value: '--',
        detail: '未检测到对应的温度传感器',
        category
      }
    }
    if (badge) {
      blocks.push({
        key: `temperature-${panel.key}`,
        label: panel.label,
        badge
      })
    }
  }

  if (!blocks.length) {
    const fallback = aggregatedTemperatureBadge.value
    if (fallback) {
      blocks.push({
        key: 'temperature-fallback',
        label: '温度',
        badge: fallback
      })
    }
  }

  return blocks
})

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

const moduleDisplays = computed<ModuleDisplay[]>(() => {
  const displays: ModuleDisplay[] = []

  for (const module of activeMonitorModules.value) {
    switch (module.key) {
      case 'cpu': {
        const value = settings.value.enableCpuMonitor && getCpuUsage.value !== '--'
          ? `${getCpuUsage.value}%`
          : '--'
        displays.push({
          key: 'module-cpu',
          label: 'CPU',
          type: 'text',
          value,
          valueClass: cpuStateClass.value === 'normal' ? undefined : cpuStateClass.value
        })
        break
      }
      case 'memory': {
        const value = settings.value.enableMemoryMonitor && getMemoryUsage.value !== '--'
          ? `${getMemoryUsage.value}%`
          : '--'
        displays.push({
          key: 'module-memory',
          label: 'MEM',
          type: 'text',
          value,
          valueClass: memoryStateClass.value === 'normal' ? undefined : memoryStateClass.value
        })
        break
      }
      case 'gpu': {
        const gpuValue = gpuInfo.value ? `${Math.round(gpuInfo.value.usage_percent)}%` : '--'
        displays.push({
          key: 'module-gpu',
          label: 'GPU',
          type: 'text',
          value: gpuValue
        })
        break
      }
      case 'disk': {
        displays.push({
          key: 'module-disk',
          label: 'DSK',
          type: 'text',
          value: diskInfo.value.value,
          tooltip: diskInfo.value.detail
        })
        break
      }
      case 'temperature': {
        const temps = temperatureBlocks.value
        if (temps.length) {
          temps.forEach((block) => {
            displays.push({
              key: block.key,
              label: block.label,
              type: 'temperature',
              badge: block.badge
            })
          })
        } else if (aggregatedTemperatureBadge.value) {
          displays.push({
            key: 'temperature-fallback',
            label: 'TMP',
            type: 'temperature',
            badge: aggregatedTemperatureBadge.value
          })
        }
        break
      }
      case 'frame': {
        const state = frameModuleState.value
        displays.push({
          key: 'module-frame',
          label: 'FPS',
          type: 'frame',
          frame: {
            fpsText: state.fpsText,
            frameTimeText: state.frameTimeText,
            detail: state.detail,
            qualityClass: state.qualityClass
          }
        })
        break
      }
      case 'network': {
        displays.push({
          key: 'module-network',
          label: 'NET',
          type: 'network',
          network: {
            download: networkSpeed.value.download,
            upload: networkSpeed.value.upload
          }
        })
        break
      }
      default:
        break
    }
  }

  if (!displays.length) {
    displays.push({
      key: 'module-placeholder',
      label: 'MONITOR',
      type: 'text',
      value: '--',
      valueClass: 'muted',
      tooltip: '未启用任何监控模块'
    })
  }

  return displays
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
    boxShadow: '0 4px 20px rgba(0, 0, 0, 0.15)',
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
  gap: 16px;
  width: fit-content;
  max-width: calc(100vw - 12px);
  white-space: nowrap;
  border: 1px solid transparent;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  user-select: none;
  cursor: default;
}

.loading-state,
.error-state,
.monitor-data {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 16px;
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

.data-group {
  display: flex;
  align-items: center;
  gap: 6px;
}

.data-label {
  font-size: 12px;
  font-weight: 700;
  text-transform: uppercase;
  color: var(--monitor-foreground, rgba(255, 255, 255, 0.7));
  opacity: 0.8;
  letter-spacing: 0.5px;
}

.data-value {
  font-size: 14px;
  font-weight: 700;
  color: var(--monitor-foreground, #ffffff);
  min-width: 36px;
  letter-spacing: 0.2px;
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
  width: 1px;
  height: 16px;
  background: var(--monitor-accent, rgba(255, 255, 255, 0.2));
  margin: 0 2px;
}

.network-values {
  display: flex;
  flex-direction: column;
  gap: 2px;
  font-size: 11px;
  line-height: 1;
}

.network-row {
  display: flex;
  align-items: center;
  gap: 2px;
}

.network-row .arrow {
  font-size: 9px;
  opacity: 0.7;
}

.network-row.download { color: #60a5fa; }
.network-row.upload { color: #a78bfa; }

.temperature-breakdown {
  font-variant-numeric: tabular-nums;
}

.frame-stats {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  line-height: 1;
  gap: 2px;
}

.fps-value {
  font-size: 14px;
  font-weight: 700;
}

.frametime-value {
  font-size: 10px;
  opacity: 0.7;
  font-weight: 500;
}

.frame-stats.state-good { color: #4ade80; }
.frame-stats.state-warn { color: #f97316; }
.frame-stats.state-critical { color: #f43f5e; }
.frame-stats.state-idle { opacity: 0.75; }

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}
</style>
