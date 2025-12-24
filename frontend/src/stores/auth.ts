import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { authApi } from '@/api/auth'
import type { StoredSession } from '@/types'

const SESSIONS_KEY = 'edge_sessions'
const CURRENT_USER_KEY = 'edge_current_user'

export const useAuthStore = defineStore('auth', () => {
    // 所有已登录的会话
    const sessions = ref<StoredSession[]>([])
    // 当前活动用户的 ID
    const currentUserId = ref<number | null>(null)

    // 当前活动会话
    const currentSession = computed(() => {
        return sessions.value.find(s => s.user_id === currentUserId.value) || null
    })

    // 当前 token
    const token = computed(() => currentSession.value?.token || null)

    // 是否已登录
    const isAuthenticated = computed(() => !!currentSession.value)

    // 当前用户名
    const username = computed(() => currentSession.value?.username || '')

    // 初始化 - 从 localStorage 加载
    function init() {
        try {
            const storedSessions = localStorage.getItem(SESSIONS_KEY)
            if (storedSessions) {
                const parsed = JSON.parse(storedSessions) as StoredSession[]
                // 过滤掉过期的会话
                const now = Math.floor(Date.now() / 1000)
                sessions.value = parsed.filter(s => s.expires_at > now)
                saveToStorage()
            }

            const storedCurrentUser = localStorage.getItem(CURRENT_USER_KEY)
            if (storedCurrentUser) {
                const userId = parseInt(storedCurrentUser)
                // 确保用户仍然在会话列表中
                if (sessions.value.find(s => s.user_id === userId)) {
                    currentUserId.value = userId
                } else if (sessions.value.length > 0) {
                    currentUserId.value = sessions.value[0].user_id
                }
            } else if (sessions.value.length > 0) {
                currentUserId.value = sessions.value[0].user_id
            }
        } catch (e) {
            console.error('Failed to load sessions:', e)
            sessions.value = []
            currentUserId.value = null
        }
    }

    // 保存到 localStorage
    function saveToStorage() {
        localStorage.setItem(SESSIONS_KEY, JSON.stringify(sessions.value))
        if (currentUserId.value) {
            localStorage.setItem(CURRENT_USER_KEY, currentUserId.value.toString())
        } else {
            localStorage.removeItem(CURRENT_USER_KEY)
        }
    }

    // 登录
    async function login(usernameInput: string, password: string) {
        const response = await authApi.login(usernameInput, password)

        if (response.success && response.token && response.user_id && response.username) {
            // 解析 JWT 获取过期时间
            const payload = parseJwt(response.token)
            const expiresAt = payload?.exp || (Math.floor(Date.now() / 1000) + 86400)

            // 检查是否已经登录过
            const existingIndex = sessions.value.findIndex(s => s.user_id === response.user_id)

            const newSession: StoredSession = {
                token: response.token,
                user_id: response.user_id,
                username: response.username,
                expires_at: expiresAt,
            }

            if (existingIndex >= 0) {
                // 更新现有会话
                sessions.value[existingIndex] = newSession
            } else {
                // 添加新会话
                sessions.value.push(newSession)
            }

            currentUserId.value = response.user_id
            saveToStorage()
        }

        return response
    }

    // 登出当前用户
    async function logout() {
        if (!currentUserId.value) return

        try {
            await authApi.logout()
        } catch (e) {
            // 忽略错误，继续清理本地状态
        }

        // 从会话列表中移除
        sessions.value = sessions.value.filter(s => s.user_id !== currentUserId.value)

        // 切换到下一个用户或清空
        if (sessions.value.length > 0) {
            currentUserId.value = sessions.value[0].user_id
        } else {
            currentUserId.value = null
        }

        saveToStorage()
    }

    // 登出所有用户
    function logoutAll() {
        sessions.value = []
        currentUserId.value = null
        saveToStorage()
    }

    // 切换用户
    function switchUser(userId: number) {
        const session = sessions.value.find(s => s.user_id === userId)
        if (session) {
            currentUserId.value = userId
            saveToStorage()
        }
    }

    // 检查会话有效性
    async function checkSession() {
        if (!token.value) return false

        try {
            const response = await authApi.checkSession()
            return response.success
        } catch {
            // 会话无效，移除当前用户
            if (currentUserId.value) {
                sessions.value = sessions.value.filter(s => s.user_id !== currentUserId.value)
                if (sessions.value.length > 0) {
                    currentUserId.value = sessions.value[0].user_id
                } else {
                    currentUserId.value = null
                }
                saveToStorage()
            }
            return false
        }
    }

    // 解析 JWT
    function parseJwt(token: string): any {
        try {
            const base64Url = token.split('.')[1]
            const base64 = base64Url.replace(/-/g, '+').replace(/_/g, '/')
            const jsonPayload = decodeURIComponent(
                atob(base64)
                    .split('')
                    .map(c => '%' + ('00' + c.charCodeAt(0).toString(16)).slice(-2))
                    .join('')
            )
            return JSON.parse(jsonPayload)
        } catch {
            return null
        }
    }

    return {
        sessions,
        currentUserId,
        currentSession,
        token,
        isAuthenticated,
        username,
        init,
        login,
        logout,
        logoutAll,
        switchUser,
        checkSession,
    }
})
