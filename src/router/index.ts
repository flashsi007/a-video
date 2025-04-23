import { createRouter, createWebHistory } from 'vue-router'
import HomePage from '../pages/HomePage.vue'
import MyPages from '../pages/MyPages.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'Home',
      component: HomePage
    },
    {
      path: '/mypages',
      name: 'MyPages',
      component: MyPages
    }
  ]
})

export default router
