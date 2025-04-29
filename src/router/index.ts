import { createRouter, createWebHistory } from 'vue-router'
// import HomePage from '../pages/HomePage.vue'
import MyPages from '../pages/MyPages.vue'
import CollectPage from '../pages/CollectPage.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    // {
    //   path: '/',
    //   name: 'Home',
    //   component: HomePage
    // },
    {
      path: '/mypages',
      name: 'MyPages',
      component: MyPages
    },

    
    {
      path: '/collect',
      name: 'Collect',
      component: CollectPage
    },
    
    {
      path: '/',
      name: 'Collect',
      component: CollectPage
    },
  ]
})

export default router
