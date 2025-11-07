import { createRouter, createWebHistory } from 'vue-router'
import SystemMonitor from '@/views/SystemMonitor.vue'
import Settings from '@/views/Settings.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'SystemMonitor',
      component: SystemMonitor
    },
    {
      path: '/settings',
      name: 'Settings',
      component: Settings
    }
  ]
})

export default router