import { createRouter, createWebHashHistory } from 'vue-router' 

let routes = [
  {
    path: '/',
    name: 'Home',
  component: () => import('../pages/HomePage.vue').then(m => m.default)
  },
  {
    path: '/mypages',
    name: 'MyPages',
  component: () => import('../pages/MyPages.vue').then(m => m.default)
  },

  
  {
    path: '/collect', 
    name: 'Collect',
  component: () => import('../pages/CollectPage.vue').then(m => m.default)
  },
  {
    path: '/play',
    name: 'PlayPage',
  component: () => import('../pages/PlayPage.vue').then(m => m.default)
  }
  
]

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    ...routes.map(route => ({
    ...route,
    component: route.component,
    meta: { requiresAuth: false }
  })),
  {
    path: '/:pathMatch(.*)*',
    redirect: '/'
  }
]
})

export default router
