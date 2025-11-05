import { ref, onMounted, onUnmounted, watch, toRefs } from 'vue'
import { useSystemStore } from '@/stores/system'

export function useSystemMonitor(autoStart = true, interval = 1000) {
  const systemStore = useSystemStore()
  const isPolling = ref(false)
  let pollInterval: number | null = null

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
    highestTemperature
  } = toRefs(systemStore)

  // 开始轮询
  const startPolling = () => {
    if (isPolling.value) return

    isPolling.value = true
    pollInterval = window.setInterval(async () => {
      // 直接调用 fetchSystemInfo 来获取最新数据，而不是读取缓存
      await systemStore.fetchSystemInfo()
      // 每5次轮询更新一次GPU信息（减少GPU查询频率）
      if (Math.random() < 0.2) {
        await systemStore.fetchGpuInfo()
      }
    }, interval)
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
  }

  // 更新轮询间隔
  const updateInterval = (newInterval: number) => {
    if (isPolling.value) {
      stopPolling()
      startPolling()
    }
  }

  // 监听配置变化
  watch(
    () => systemStore.config.refresh_interval,
    (newInterval) => {
      updateInterval(newInterval)
    }
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
    highestTemperature
  }
}