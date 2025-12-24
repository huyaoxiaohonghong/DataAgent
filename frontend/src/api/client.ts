import axios from 'axios'
import type { StoredSession } from '@/types'

const SESSIONS_KEY = 'edge_sessions'
const CURRENT_USER_KEY = 'edge_current_user'

const api = axios.create({
    baseURL: '/api',
    timeout: 10000,
    headers: {
        'Content-Type': 'application/json',
    },
})

// 获取当前活动用户的 token
function getCurrentToken(): string | null {
    try {
        const storedSessions = localStorage.getItem(SESSIONS_KEY)
        const currentUserId = localStorage.getItem(CURRENT_USER_KEY)

        if (!storedSessions || !currentUserId) return null

        const sessions: StoredSession[] = JSON.parse(storedSessions)
        const session = sessions.find(s => s.user_id === parseInt(currentUserId))

        if (session && session.expires_at > Math.floor(Date.now() / 1000)) {
            return session.token
        }

        return null
    } catch {
        return null
    }
}

// Request interceptor to add auth token
api.interceptors.request.use((config) => {
    const token = getCurrentToken()
    if (token) {
        config.headers.Authorization = `Bearer ${token}`
    }
    return config
})

// Response interceptor for error handling
api.interceptors.response.use(
    (response) => response,
    (error) => {
        if (error.response?.status === 401) {
            // 不自动跳转，让 store 处理
            console.warn('Unauthorized request')
        }
        return Promise.reject(error)
    }
)

export default api
