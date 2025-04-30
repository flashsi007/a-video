import { createRouter, createWebHistory } from 'vue-router' 

let routes = [
  {
    path: '/',
    name: 'Home',
    component: import('../pages/HomePage.vue')
  },
  {
    path: '/mypages',
    name: 'MyPages',
    component: import('../pages/MyPages.vue')
  },

  
  {
    path: '/collect', 
    name: 'Collect',
    component: import('../pages/CollectPage.vue')
  },
  {
    path: '/pla',
    name: 'PlayPage',
    component: import('../pages/PlayPage.vue')
  }
  
]

const router = createRouter({ history: createWebHistory(""),   routes  })

export default router
