import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { enable, disable, isEnabled } from '@tauri-apps/plugin-autostart'
import { Window, getCurrentWindow } from '@tauri-apps/api/window'
import { LogicalSize } from '@tauri-apps/api/dpi'
import { emit, listen, type UnlistenFn } from '@tauri-apps/api/event'
import { useSystemStore } from './system'

export type TemperaturePanelKey =
  | 'cpu'
  | 'memory'
  | 'gpu'
  | 'vrm'
  | 'motherboard'
  | 'storage'

export type MonitorModuleKey =
  | 'cpu'
  | 'memory'
  | 'gpu'
  | 'disk'
  | 'temperature'
  | 'frame'
  | 'network'

export interface TemperaturePanelPreference {
  key: TemperaturePanelKey
  label: string
  enabled: boolean
  order: number
  description?: string
}

export interface MonitorModulePreference {
  key: MonitorModuleKey
  label: string
  enabled: boolean
  order: number
  description?: string
}

export interface SettingsState {
  autoStart: boolean
  showInTaskbar: boolean
  alwaysOnTop: boolean
  refreshInterval: number
  enableCpuMonitor: boolean
  enableMemoryMonitor: boolean
  enableGpuMonitor: boolean
  enableNetworkMonitor: boolean
  enableDiskMonitor: boolean
  enableTemperatureMonitor: boolean
  temperaturePanels: TemperaturePanelPreference[]
  monitorModules: MonitorModulePreference[]
  enableFrameStats: boolean
  opacity: number
  themeColor: string
  logLevel: string
  cacheTime: number
  windowWidth: number
  windowHeight: number
  cpuAlertThreshold: number
  memoryAlertThreshold: number
  cpuSmoothing: number
  memorySmoothing: number
  backgroundStyle: 'glass' | 'aurora' | 'midnight' | 'transparent'
  backgroundAccent: string
  foregroundColor: string
  fontFamily: string
}

const moduleBooleanMap: Record<MonitorModuleKey, keyof SettingsState> = {
  cpu: 'enableCpuMonitor',
  memory: 'enableMemoryMonitor',
  gpu: 'enableGpuMonitor',
  disk: 'enableDiskMonitor',
  temperature: 'enableTemperatureMonitor',
  frame: 'enableFrameStats',
  network: 'enableNetworkMonitor'
}

const createDefaultTemperaturePanels = (): TemperaturePanelPreference[] => [
  {
    key: 'cpu',
    label: 'CPU 温度',
    enabled: true,
    order: 0,
    description: '聚合 CPU 包与核心的最高温度'
  },
  {
    key: 'memory',
    label: '内存温度',
    enabled: true,
    order: 1,
    description: 'DIMM / Memory 温度传感器'
  },
  {
    key: 'gpu',
    label: 'GPU 温度',
    enabled: true,
    order: 2,
    description: '显卡核心/显存温度'
  }
]

const createDefaultMonitorModules = (): MonitorModulePreference[] => [
  {
    key: 'cpu',
    label: 'CPU',
    enabled: true,
    order: 0,
    description: '显示 CPU 使用率'
  },
  {
    key: 'memory',
    label: '内存',
    enabled: true,
    order: 1,
    description: '显示内存使用率'
  },
  {
    key: 'gpu',
    label: 'GPU',
    enabled: true,
    order: 2,
    description: '显示 GPU 使用率'
  },
  {
    key: 'disk',
    label: '磁盘',
    enabled: true,
    order: 3,
    description: '显示磁盘使用情况'
  },
  {
    key: 'temperature',
    label: '温度',
    enabled: true,
    order: 4,
    description: '显示各组件温度'
  },
  {
    key: 'frame',
    label: '帧率',
    enabled: false,
    order: 5,
    description: '显示帧率 / 帧时间'
  },
  {
    key: 'network',
    label: '网络',
    enabled: true,
    order: 6,
    description: '显示网络上下行速率'
  }
]

const normalizeTemperaturePanels = (
  panels?: TemperaturePanelPreference[]
): TemperaturePanelPreference[] => {
  const defaults = createDefaultTemperaturePanels()
  if (!Array.isArray(panels) || !panels.length) {
    return defaults
  }
  const map = new Map(defaults.map(panel => [panel.key, {...panel}]))
  panels.forEach(panel => {
    if (!panel?.key) return
    if (!map.has(panel.key)) return
    map.set(panel.key, {
      ...map.get(panel.key)!,
      ...panel
    })
  })
  return Array.from(map.values())
    .sort((a, b) => (a.order ?? 0) - (b.order ?? 0))
    .map((panel, index) => ({...panel, order: index}))
}

const normalizeMonitorModules = (
  modules?: MonitorModulePreference[]
): MonitorModulePreference[] => {
  const defaults = createDefaultMonitorModules()
  if (!Array.isArray(modules) || !modules.length) {
    return defaults
  }
  const map = new Map(defaults.map(module => [module.key, {...module}]))
  modules.forEach(module => {
    if (!module?.key) return
    if (!map.has(module.key)) return
    map.set(module.key, {
      ...map.get(module.key)!,
      ...module
    })
  })
  return Array.from(map.values())
    .sort((a, b) => (a.order ?? 0) - (b.order ?? 0))
    .map((module, index) => ({...module, order: index}))
}

const syncMonitorModulesWithBooleans = (state: SettingsState): MonitorModulePreference[] => {
  return (state.monitorModules ?? createDefaultMonitorModules()).map(module => {
    const boolKey = moduleBooleanMap[module.key]
    const enabled = boolKey ? (state[boolKey] as boolean) : module.enabled
    return {
      ...module,
      enabled
    }
  })
}

const createDefaultSettings = (): SettingsState => ({
  autoStart: false,
  showInTaskbar: false,
  alwaysOnTop: true,
  refreshInterval: 1000,
  enableCpuMonitor: true,
  enableMemoryMonitor: true,
  enableGpuMonitor: true,
  enableNetworkMonitor: true,
  enableDiskMonitor: true,
  enableTemperatureMonitor: true,
  temperaturePanels: createDefaultTemperaturePanels(),
  monitorModules: createDefaultMonitorModules(),
  enableFrameStats: false,
  opacity: 90,
  themeColor: '#3b82f6',
  logLevel: 'info',
  cacheTime: 600,
  windowWidth: 600,
  windowHeight: 40,
  cpuAlertThreshold: 85,
  memoryAlertThreshold: 80,
  cpuSmoothing: 60,
  memorySmoothing: 55,
  backgroundStyle: 'glass',
  backgroundAccent: '#3b82f6',
  foregroundColor: '#ffffff',
  fontFamily: 'Inter'
})

const SETTINGS_LOCAL_KEY = 'system-monitor-settings'
const SETTINGS_UPDATED_EVENT = 'settings:updated'

export const useSettingsStore = defineStore('settings', () => {
  const settings = ref<SettingsState>(createDefaultSettings())
  const initialized = ref(false)
  const loading = ref(false)
  const error = ref<string | null>(null)
  let syncUnlisten: UnlistenFn | null = null
  let resizeUnlisten: UnlistenFn | null = null
  let suppressResizeCapture = false
  let pendingSizePersist: number | null = null

  const getMainWindow = async () => {
    try {
      const mainWindow = await Window.getByLabel('main')
      if (mainWindow) return mainWindow
    } catch (err) {
      console.warn('获取主窗口失败，尝试使用当前窗口:', err)
    }
    try {
      return await getCurrentWindow()
    } catch {
      return null
    }
  }

  const persistSettings = async () => {
    for (const [key, value] of Object.entries(settings.value)) {
      await invoke('save_settings', { key, value })
    }
    localStorage.setItem(SETTINGS_LOCAL_KEY, JSON.stringify(settings.value))
  }

  const syncMonitorConfig = async () => {
    const systemStore = useSystemStore()
    await systemStore.updateConfig({
      refresh_interval: settings.value.refreshInterval,
      enable_cpu: settings.value.enableCpuMonitor,
      enable_memory: settings.value.enableMemoryMonitor,
      enable_gpu: settings.value.enableGpuMonitor,
      enable_network: settings.value.enableNetworkMonitor,
      enable_disk: settings.value.enableDiskMonitor,
      enable_temperature: settings.value.enableTemperatureMonitor,
      refresh_strategy: {
        Adaptive: {
          min_interval_ms: Math.max(250, Math.round(settings.value.refreshInterval * 0.5)),
          max_interval_ms: Math.max(2000, settings.value.refreshInterval * 6),
          cpu_threshold: settings.value.cpuAlertThreshold,
          memory_threshold: settings.value.memoryAlertThreshold,
          change_threshold: 5
        }
      }
    })
  }

  const syncWindowPreferences = async () => {
    try {
      const mainWindow = await getMainWindow()
      if (mainWindow && settings.value.windowWidth && settings.value.windowHeight) {
        try {
          suppressResizeCapture = true
          await mainWindow.setSize(new LogicalSize(
            settings.value.windowWidth,
            settings.value.windowHeight
          ))
        } finally {
          suppressResizeCapture = false
        }
      }
    } catch (err) {
      console.error('应用窗口尺寸失败:', err)
    }

    try {
      await invoke('apply_window_preferences', {
        always_on_top: settings.value.alwaysOnTop,
        show_in_taskbar: settings.value.showInTaskbar
      })
    } catch (err) {
      console.error('调用窗口偏好命令失败:', err)
    }
  }

  const broadcastSettingsUpdate = async () => {
    try {
      await emit(SETTINGS_UPDATED_EVENT, settings.value)
    } catch (err) {
      console.error('广播设置更新失败:', err)
    }
  }

  const ensureSettingsSync = async () => {
    if (syncUnlisten || typeof window === 'undefined') return
    try {
      syncUnlisten = await listen<SettingsState>(SETTINGS_UPDATED_EVENT, (event) => {
        if (!event.payload) return
        settings.value = { ...settings.value, ...event.payload }
        localStorage.setItem(SETTINGS_LOCAL_KEY, JSON.stringify(settings.value))
      })
      window.addEventListener('beforeunload', () => {
        if (syncUnlisten) {
          syncUnlisten()
          syncUnlisten = null
        }
      })
    } catch (err) {
      console.error('监听设置更新事件失败:', err)
    }
  }

  const scheduleWindowSizePersist = () => {
    if (typeof window === 'undefined') return
    if (pendingSizePersist) {
      window.clearTimeout(pendingSizePersist)
    }
    pendingSizePersist = window.setTimeout(async () => {
      pendingSizePersist = null
      try {
        await persistSettings()
        await broadcastSettingsUpdate()
      } catch (err) {
        console.error('保存窗口尺寸失败:', err)
      }
    }, 200)
  }

  const ensureWindowSizeWatcher = async () => {
    if (resizeUnlisten || typeof window === 'undefined') return
    try {
      const currentWindow = await getCurrentWindow()
      if (currentWindow.label !== 'main') return
      resizeUnlisten = await currentWindow.onResized(({ payload }) => {
        if (suppressResizeCapture) return
        if (!payload?.width || !payload?.height) return
        const width = Math.round(payload.width)
        const height = Math.round(payload.height)
        if (
          width === settings.value.windowWidth &&
          height === settings.value.windowHeight
        ) {
          return
        }
        settings.value.windowWidth = width
        settings.value.windowHeight = height
        localStorage.setItem(SETTINGS_LOCAL_KEY, JSON.stringify(settings.value))
        scheduleWindowSizePersist()
      })
      window.addEventListener('beforeunload', () => {
        if (resizeUnlisten) {
          resizeUnlisten()
          resizeUnlisten = null
        }
      })
    } catch (err) {
      console.error('监听窗口尺寸变化失败:', err)
    }
  }

  const applySettings = async () => {
    try {
      error.value = null
      settings.value.temperaturePanels = normalizeTemperaturePanels(settings.value.temperaturePanels)
      settings.value.monitorModules = normalizeMonitorModules(settings.value.monitorModules)
      settings.value.monitorModules = syncMonitorModulesWithBooleans(settings.value)
      await persistSettings()
      await syncMonitorConfig()
      await syncWindowPreferences()
      await broadcastSettingsUpdate()
    } catch (err) {
      console.error('同步设置失败:', err)
      error.value = `${err}`
    }
  }

  const syncAutoStartState = async () => {
    try {
      settings.value.autoStart = await isEnabled()
    } catch (err) {
      console.error('同步自动启动状态失败:', err)
    }
  }

  const loadSettings = async () => {
    if (loading.value) return
    loading.value = true
    try {
      error.value = null
      const saved = await invoke<Record<string, unknown>>('get_all_settings')
      if (saved && Object.keys(saved).length) {
        const merged = { ...createDefaultSettings(), ...(saved as Partial<SettingsState>) }
        merged.temperaturePanels = normalizeTemperaturePanels(
          (saved.temperaturePanels as TemperaturePanelPreference[]) ?? createDefaultTemperaturePanels()
        )
        merged.monitorModules = normalizeMonitorModules(
          (saved.monitorModules as MonitorModulePreference[]) ?? createDefaultMonitorModules()
        )
        merged.monitorModules = syncMonitorModulesWithBooleans(merged)
        settings.value = merged
      } else {
        const local = localStorage.getItem(SETTINGS_LOCAL_KEY)
        if (local) {
          const parsed = JSON.parse(local) as Partial<SettingsState>
          parsed.temperaturePanels = normalizeTemperaturePanels(parsed.temperaturePanels)
          parsed.monitorModules = normalizeMonitorModules(parsed.monitorModules as MonitorModulePreference[])
          const merged = { ...createDefaultSettings(), ...parsed }
          merged.monitorModules = syncMonitorModulesWithBooleans(merged)
          settings.value = merged
        }
      }
      await syncAutoStartState()
      await applySettings()
    } catch (err) {
      console.error('加载设置失败:', err)
      error.value = `${err}`
      const local = localStorage.getItem(SETTINGS_LOCAL_KEY)
      if (local) {
        const parsed = JSON.parse(local) as Partial<SettingsState>
        parsed.temperaturePanels = normalizeTemperaturePanels(parsed.temperaturePanels)
        parsed.monitorModules = normalizeMonitorModules(parsed.monitorModules as MonitorModulePreference[])
        const merged = { ...createDefaultSettings(), ...parsed }
        merged.monitorModules = syncMonitorModulesWithBooleans(merged)
        settings.value = merged
      }
    } finally {
      initialized.value = true
      loading.value = false
    }
  }

  const ensureInitialized = async () => {
    await ensureSettingsSync()
    await ensureWindowSizeWatcher()
    if (!initialized.value && !loading.value) {
      await loadSettings()
    }
  }

  const updateSettings = async (partial: Partial<SettingsState>) => {
    settings.value = { ...settings.value, ...partial }
    await applySettings()
  }

  const setAutoStart = async (enabled: boolean) => {
    try {
      if (enabled) {
        await enable()
      } else {
        await disable()
      }
      settings.value.autoStart = enabled
      await applySettings()
    } catch (err) {
      console.error('切换自动启动失败:', err)
      error.value = `${err}`
      settings.value.autoStart = !enabled
    } finally {
      await syncAutoStartState()
    }
  }

  const resetSettings = async () => {
    settings.value = createDefaultSettings()
    await setAutoStart(settings.value.autoStart)
  }

  const exportSettings = () => JSON.stringify(settings.value, null, 2)

  return {
    settings,
    loading,
    initialized,
    error,
    ensureInitialized,
    loadSettings,
    updateSettings,
    applySettings,
    setAutoStart,
    resetSettings,
    exportSettings
  }
})
