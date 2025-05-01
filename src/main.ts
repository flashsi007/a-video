import { createApp } from 'vue'
import { createPinia } from 'pinia'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import './assets/tailwindcss.css'
import App from './App.vue'
import router from './router'
import * as ElementPlusIconsVue from '@element-plus/icons-vue'

// 添加调试日志
console.log('Initializing Vue application...')

const app = createApp(App)

// 确保按顺序初始化插件
try {
  app.use(createPinia())
  console.log('Pinia initialized')
  
  app.use(router)
  console.log('Router initialized')
  
  app.use(ElementPlus)
  console.log('ElementPlus initialized')
  
  // 注册图标组件
  for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
    app.component(key, component)
  }
  console.log('Icons registered')
  
  app.mount('#app')
  console.log('Application mounted')
} catch (error) {
  console.error('Initialization failed:', error)
}
