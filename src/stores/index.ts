import { createPinia } from 'pinia'

const pinia = createPinia()

export default pinia

// 导出所有store
export { useSystemStore } from './system'
export type { SystemInfo, MemoryInfo, NetworkInfo, DiskInfo, SystemDetails, TemperatureInfo, GpuInfo, GpuMemoryInfo, MonitorConfig } from './system'