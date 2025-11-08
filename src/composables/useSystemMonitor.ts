import { ref, onMounted, onUnmounted, watch, toRefs } from 'vue'
import { storeToRefs } from 'pinia'
import { useSystemStore } from '@/stores/system'
import { useSettingsStore } from '@/stores/settings'

export function useSystemMonitor(autoStart = true, interval = 1000) {
  const systemStore = useSystemStore()
  const settingsStore = useSettingsStore()
  const isPolling = ref(false)
  const pollDelay = ref(interval)
  let pollInterval: number | null = null
  let lastFrameSample = 0

  // 获取store的响应式引用
  const {
    systemInfo,
    gpuInfo,
    isMonitoring,
    lastUpdate,
    error,
    memoryUsageText,
    cpuUsageText,
    formattedMemory,
    networkSpeed,
    highestTemperature,
    frameStats,
    frameError
  } = toRefs(systemStore)

  const maybeFetchFrameStats = async () => {
    if (!settings.value.enableFrameStats) return
    const now = Date.now()
    if (now - lastFrameSample < Math.max(pollDelay.value, 1200)) return
    lastFrameSample = now
    await systemStore.fetchFrameStats()
  }

  // 开始轮询
  const startPolling = () => {
    if (isPolling.value) return

    isPolling.value = true
    pollInterval = window.setInterval(async () => {
      await systemStore.fetchSystemInfo()
      if (Math.random() < 0.2) {
        await systemStore.fetchGpuInfo()
      }
      if (settings.value.enableFrameStats) {
        await maybeFetchFrameStats()
      }
    }, pollDelay.value)
  }

  // 停止轮询
  const stopPolling = () => {
    if (pollInterval) {
      clearInterval(pollInterval)
      pollInterval = null
    }
    isPolling.value = false
  }

  // 切换轮询状态
  const togglePolling = () => {
    if (isPolling.value) {
      stopPolling()
    } else {
      startPolling()
    }
  }

  // 手动刷新
  const refresh = async () => {
    await systemStore.fetchSystemInfo()
    await systemStore.fetchGpuInfo()
    await maybeFetchFrameStats()
  }

  // 更新轮询间隔
  const updateInterval = (newInterval: number) => {
    if (!newInterval || Number.isNaN(newInterval)) return
    pollDelay.value = newInterval
    if (isPolling.value) {
      stopPolling()
      startPolling()
    }
  }

  // 监听设置中的刷新间隔，动态调整轮询频率
  const { settings } = storeToRefs(settingsStore)
  watch(
    () => settings.value.refreshInterval,
    (newInterval) => {
      updateInterval(newInterval)
    },
    { immediate: true }
  )

  watch(
    () => settings.value.enableFrameStats,
    async (enabled) => {
      if (enabled) {
        lastFrameSample = 0
        await maybeFetchFrameStats()
      }
    },
    { immediate: true }
  )

  // 生命周期
  onMounted(async () => {
    if (autoStart) {
      await systemStore.startMonitoring()
      startPolling()
    }
  })

  onUnmounted(() => {
    stopPolling()
    systemStore.stopMonitoring()
  })

  return {
    // 状态
    isPolling,

    // 方法
    startPolling,
    stopPolling,
    togglePolling,
    refresh,
    updateInterval,

    // store的响应式引用
    systemInfo,
    gpuInfo,
    isMonitoring,
    lastUpdate,
    error,
    memoryUsageText,
    cpuUsageText,
    formattedMemory,
    networkSpeed,
    highestTemperature,
    frameStats,
    frameError
  }
}
