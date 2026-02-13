import type { App } from 'vue'
import type { RouteRecordRaw } from 'vue-router'
import { createRouter, createWebHashHistory } from 'vue-router'
import { WindowLayout } from '@/views/window'
import { ChatLayout } from '@/views/unitest/layout'

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'Root',
    component: ChatLayout,
    redirect: '/chat',
    children: [
      {
        path: '/chat/:uuid?',
        name: 'Chat',
        component: () => import('@/views/unitest/index.vue'),
      },
    ],
  },
  {
    path: '/unitestagent/:uuid?',
    name: 'Unitestagent',
    component: ChatLayout,
    redirect: '/unitestagent',
    children: [
      {
        path: '/unitestagent/:uuid?',
        name: 'Unitestagent',
        component: () => import('@/views/unitest/index.vue'),
      },
    ],
  },
  {
    path: '/window',
    name: 'Window',
    component: WindowLayout,
    redirect: '/window',
    children: [
      {
        path: '/window/prompt-store',
        name: 'PromptStore',
        component: () => import('@/components/common/PromptStore/index.vue'),
      },
      {
        path: '/window/project-store',
        name: 'ProjectStore',
        component: () => import('@/components/common/ProjStore/index.vue'),
      },
    ],
  },
  {
    path: '/404',
    name: '404',
    component: () => import('@/views/exception/404/index.vue'),
  },
  {
    path: '/500',
    name: '500',
    component: () => import('@/views/exception/500/index.vue'),
  },
  {
    path: '/:pathMatch(.*)*',
    name: 'notFound',
    redirect: '/404',
  },
]

export const router = createRouter({
  history: createWebHashHistory(),
  routes,
  scrollBehavior: () => ({ left: 0, top: 0 }),
})

export async function setupRouter(app: App) {
  app.use(router)
  await router.isReady()
} 