import { createRouter, createWebHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'
import { useAuthStore } from '@/stores/auth'

const routes: RouteRecordRaw[] = [
    {
        path: '/login',
        name: 'Login',
        component: () => import('@/views/Login.vue'),
        meta: { requiresAuth: false },
    },
    {
        path: '/',
        component: () => import('@/layouts/MainLayout.vue'),
        meta: { requiresAuth: true },
        children: [
            {
                path: '',
                name: 'Dashboard',
                component: () => import('@/views/Dashboard.vue'),
            },
            {
                path: 'users',
                name: 'Users',
                component: () => import('@/views/Users.vue'),
            },
            {
                path: 'logs',
                name: 'Logs',
                component: () => import('@/views/Logs.vue'),
            },
            {
                path: 'profile',
                name: 'Profile',
                component: () => import('@/views/Profile.vue'),
            },
            {
                path: 'proxy',
                name: 'ProxyNodes',
                component: () => import('@/views/ProxyNodes.vue'),
            },
        ],
    },
]

const router = createRouter({
    history: createWebHistory(),
    routes,
})

router.beforeEach((to, _from, next) => {
    const authStore = useAuthStore()

    // 初始化 store（从 localStorage 加载会话）
    authStore.init()

    // 检查是否需要认证
    if (to.meta.requiresAuth !== false && !authStore.isAuthenticated) {
        next({ name: 'Login' })
    } else if (to.name === 'Login' && authStore.isAuthenticated && !to.query.addAccount) {
        // 如果已登录且不是添加账号模式，跳转到首页
        next({ name: 'Dashboard' })
    } else {
        next()
    }
})

export default router
