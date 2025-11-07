<template>
  <div
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
import { computed, onMounted, onUnmounted, ref, watch } from 'vue'
import { storeToRefs } from 'pinia'
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

const loading = computed(() => !systemInfo.value && !error.value)

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

const showNetwork = computed(() => settings.value.enableNetworkMonitor && !!systemInfo.value?.network)

const handleContextMenu = (e: MouseEvent) => {
  e.preventDefault()
}

onMounted(async () => {
  await settingsStore.ensureInitialized()
  const [isAvailable] = await systemStore.getGpuMonitorStatus()
  if (isAvailable) {
    await systemStore.getGpuNames()
    await systemStore.getDetailedGpuInfo(0)
  }
})

onUnmounted(() => {
  // 预留清理逻辑
})
</script>

<style scoped>
.floating-monitor {
  height: 42px;
  font-weight: 600;
  border-radius: 22px;
  line-height: 42px;
  padding: 0 20px;
  display: flex;
  align-items: center;
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
