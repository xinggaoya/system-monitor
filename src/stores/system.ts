import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

// 系统信息接口定义
export interface SystemInfo {
  cpu_usage: number
  memory: MemoryInfo
  network: NetworkInfo
  disk: DiskInfo
  system: SystemDetails
  temperatures: TemperatureInfo[]
}

export interface MemoryInfo {
  total: number
  used: number
  available: number
  usage_percent: number
  swap_total: number
  swap_used: number
}

export interface NetworkInfo {
  interfaces: NetworkInterface[]
  total_received: number
  total_transmitted: number
}

export interface NetworkInterface {
  name: string
  received: number
  transmitted: number
  receive_rate: number
  transmit_rate: number
}

export interface DiskInfo {
  disks: Disk[]
}

export interface Disk {
  name: string
  mount_point: string
  file_system: string
  total_space: number
  available_space: number
  used_space: number
  usage_percent: number
}

export interface SystemDetails {
  name?: string
  kernel_version?: string
  os_version?: string
  host_name?: string
  cpu_count: number
  cpu_brand?: string
  cpu_frequency?: number
}

export interface TemperatureInfo {
  label: string
  temperature: number
  max?: number
  critical?: number
}

export interface GpuInfo {
  name: string
  usage_percent: number
  memory: GpuMemoryInfo
  temperature?: number
  frequency?: number
}

export interface GpuMemoryInfo {
  total: number
  used: number
  usage_percent: number
}

export interface MonitorConfig {
  refresh_interval: number
  enable_cpu: boolean
  enable_memory: boolean
  enable_network: boolean
  enable_disk: boolean
  enable_temperature: boolean
  enable_gpu: boolean
}

export const useSystemStore = defineStore('system', () => {
  // 状态
  const systemInfo = ref<SystemInfo | null>(null)
  const gpuInfo = ref<GpuInfo | null>(null)
  const isMonitoring = ref(false)
  const lastUpdate = ref<Date | null>(null)
  const config = ref<MonitorConfig>({
    refresh_interval: 1000,
    enable_cpu: true,
    enable_memory: true,
    enable_network: true,
    enable_disk: true,
    enable_temperature: true,
    enable_gpu: true
  })
  const error = ref<string | null>(null)

  // 计算属性
  const memoryUsageText = computed(() => {
    if (!systemInfo.value?.memory) return '0%'
    return `${systemInfo.value.memory.usage_percent.toFixed(1)}%`
  })

  const cpuUsageText = computed(() => {
    if (!systemInfo.value) return '0%'
    return `${systemInfo.value.cpu_usage.toFixed(1)}%`
  })

  const formattedMemory = computed(() => {
    if (!systemInfo.value?.memory) return { used: '0 B', total: '0 B' }
    const { used, total } = systemInfo.value.memory
    return {
      used: formatBytes(used),
      total: formatBytes(total)
    }
  })

  const networkSpeed = computed(() => {
    if (!systemInfo.value?.network.interfaces.length) return { download: '0 B/s', upload: '0 B/s' }

    let totalDownload = 0
    let totalUpload = 0

    systemInfo.value.network.interfaces.forEach(iface => {
      totalDownload += iface.receive_rate
      totalUpload += iface.transmit_rate
    })

    return {
      download: formatBytes(totalDownload) + '/s',
      upload: formatBytes(totalUpload) + '/s'
    }
  })

  const highestTemperature = computed(() => {
    if (!systemInfo.value?.temperatures.length) return null
    return Math.max(...systemInfo.value.temperatures.map(t => t.temperature))
  })

  // 方法
  const fetchSystemInfo = async () => {
    try {
      error.value = null
      const info = await invoke<SystemInfo>('get_system_info')
      systemInfo.value = info
      lastUpdate.value = new Date()
    } catch (err) {
      console.error('获取系统信息失败:', err)
      error.value = err as string
    }
  }

  const fetchGpuInfo = async () => {
    try {
      const info = await invoke<GpuInfo | null>('get_gpu_info')
      gpuInfo.value = info
    } catch (err) {
      console.error('获取GPU信息失败:', err)
    }
  }

  const getCurrentData = async () => {
    try {
      const data = await invoke<SystemInfo | null>('get_current_data')
      if (data) {
        systemInfo.value = data
        lastUpdate.value = new Date()
      }
    } catch (err) {
      console.error('获取当前数据失败:', err)
    }
  }

  const updateConfig = async (newConfig: Partial<MonitorConfig>) => {
    try {
      config.value = { ...config.value, ...newConfig }
      await invoke('update_monitor_config', { config: config.value })
    } catch (err) {
      error.value = err as string
      console.error('更新配置失败:', err)
    }
  }

  const startMonitoring = async () => {
    isMonitoring.value = true
    await fetchSystemInfo()
    await fetchGpuInfo()
  }

  const stopMonitoring = () => {
    isMonitoring.value = false
  }

  const toggleMonitoring = () => {
    if (isMonitoring.value) {
      stopMonitoring()
    } else {
      startMonitoring()
    }
  }

  // 测试数据刷新机制
  const testDataRefresh = async (): Promise<string> => {
    try {
      const result = await invoke<string>('test_data_refresh')
      return result
    } catch (err) {
      console.error('测试数据刷新失败:', err)
      return `测试失败: ${err}`
    }
  }

  // GPU监控相关方法
  const getGpuMonitorStatus = async (): Promise<[boolean, string | null]> => {
    try {
      const status = await invoke<[boolean, string | null]>('get_gpu_monitor_status')
      return status
    } catch (err) {
      console.error('获取GPU监控状态失败:', err)
      return [false, `获取状态失败: ${err}`]
    }
  }

  const getGpuNames = async (): Promise<string[]> => {
    try {
      const names = await invoke<string[]>('get_gpu_names')
      return names
    } catch (err) {
      console.error('获取GPU名称失败:', err)
      return []
    }
  }

  const getDetailedGpuInfo = async (deviceIndex: number = 0): Promise<string> => {
    try {
      const info = await invoke<string>('get_detailed_gpu_info', { deviceIndex })
      return info
    } catch (err) {
      console.error('获取详细GPU信息失败:', err)
      return 'GPU监控不可用'
    }
  }

  // 工具函数
  const formatBytes = (bytes: number): string => {
    if (bytes === 0) return '0 B'
    const k = 1024
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
    const i = Math.floor(Math.log(bytes) / Math.log(k))
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
  }

  const formatTemperature = (temp: number): string => {
    return `${temp.toFixed(1)}°C`
  }

  return {
    // 状态
    systemInfo,
    gpuInfo,
    isMonitoring,
    lastUpdate,
    config,
    error,

    // 计算属性
    memoryUsageText,
    cpuUsageText,
    formattedMemory,
    networkSpeed,
    highestTemperature,

    // 方法
    fetchSystemInfo,
    fetchGpuInfo,
    getCurrentData,
    updateConfig,
    startMonitoring,
    stopMonitoring,
    toggleMonitoring,
    testDataRefresh,
    formatBytes,
    formatTemperature,

    // GPU监控相关方法
    getGpuMonitorStatus,
    getGpuNames,
    getDetailedGpuInfo
  }
})