import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { api } from '../api'

export interface User {
    id: number
    username: string
    role: string
    created_at: string
}

export const useAuthStore = defineStore('auth', () => {
    const token = ref<string | null>(localStorage.getItem('token'))
    const user = ref<User | null>(null)
    const loading = ref(false)

    const isAuthenticated = computed(() => !!token.value)

    async function login(username: string, password: string) {
        loading.value = true
        try {
            const response = await api.auth.login(username, password)
            token.value = response.token
            user.value = response.user
            localStorage.setItem('token', response.token)
            return { success: true }
        } catch (error: any) {
            return { success: false, message: error.message || '登录失败' }
        } finally {
            loading.value = false
        }
    }

    async function logout() {
        try {
            await api.auth.logout()
        } catch {
            // 忽略登出错误
        } finally {
            token.value = null
            user.value = null
            localStorage.removeItem('token')
        }
    }

    async function checkAuth() {
        if (!token.value) return false
        try {
            const response = await api.auth.check()
            user.value = response.user
            return true
        } catch {
            token.value = null
            user.value = null
            localStorage.removeItem('token')
            return false
        }
    }

    return {
        token,
        user,
        loading,
        isAuthenticated,
        login,
        logout,
        checkAuth
    }
})
