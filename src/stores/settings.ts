import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { enable, disable, isEnabled } from '@tauri-apps/plugin-autostart'
import { Window, getCurrentWindow } from '@tauri-apps/api/window'
import { LogicalSize } from '@tauri-apps/api/dpi'
import { emit, listen, type UnlistenFn } from '@tauri-apps/api/event'
import { useSystemStore } from './system'

export interface SettingsState {
  autoStart: boolean
  showInTaskbar: boolean
  alwaysOnTop: boolean
  refreshInterval: number
  enableCpuMonitor: boolean
  enableMemoryMonitor: boolean
  enableGpuMonitor: boolean
  enableNetworkMonitor: boolean
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

const defaultSettings: SettingsState = {
  autoStart: false,
  showInTaskbar: false,
  alwaysOnTop: true,
  refreshInterval: 1000,
  enableCpuMonitor: true,
  enableMemoryMonitor: true,
  enableGpuMonitor: true,
  enableNetworkMonitor: true,
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
}

const SETTINGS_LOCAL_KEY = 'system-monitor-settings'
const SETTINGS_UPDATED_EVENT = 'settings:updated'

export const useSettingsStore = defineStore('settings', () => {
  const settings = ref<SettingsState>({ ...defaultSettings })
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
      if (!mainWindow) return
      await mainWindow.setAlwaysOnTop(settings.value.alwaysOnTop)
      await mainWindow.setSkipTaskbar(!settings.value.showInTaskbar)
      if (settings.value.windowWidth && settings.value.windowHeight) {
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
      console.error('应用窗口偏好失败:', err)
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
        settings.value = { ...defaultSettings, ...(saved as Partial<SettingsState>) }
      } else {
        const local = localStorage.getItem(SETTINGS_LOCAL_KEY)
        if (local) {
          settings.value = { ...defaultSettings, ...JSON.parse(local) }
        }
      }
      await syncAutoStartState()
      await applySettings()
    } catch (err) {
      console.error('加载设置失败:', err)
      error.value = `${err}`
      const local = localStorage.getItem(SETTINGS_LOCAL_KEY)
      if (local) {
        settings.value = { ...defaultSettings, ...JSON.parse(local) }
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
    settings.value = { ...defaultSettings }
    await setAutoStart(defaultSettings.autoStart)
  }

  const exportSettings = () => JSON.stringify(settings.value, null, 2)

  return {
    settings,
    loading,
    initialized,
    error,
    defaultSettings,
    ensureInitialized,
    loadSettings,
    updateSettings,
    applySettings,
    setAutoStart,
    resetSettings,
    exportSettings
  }
})
