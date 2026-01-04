// API 基础配置
const API_BASE_URL = localStorage.getItem('apiBaseUrl') || 'http://localhost:8787/api'

interface RequestOptions {
    method?: string
    body?: any
    headers?: Record<string, string>
}

async function request<T>(endpoint: string, options: RequestOptions = {}): Promise<T> {
    const token = localStorage.getItem('token')

    const headers: Record<string, string> = {
        'Content-Type': 'application/json',
        ...options.headers
    }

    if (token) {
        headers['Authorization'] = `Bearer ${token}`
    }

    const response = await fetch(`${API_BASE_URL}${endpoint}`, {
        method: options.method || 'GET',
        headers,
        body: options.body ? JSON.stringify(options.body) : undefined,
        credentials: 'include'
    })

    if (!response.ok) {
        const error = await response.json().catch(() => ({ message: '请求失败' }))
        throw new Error(error.message || `HTTP ${response.status}`)
    }

    return response.json()
}

// API 模块
export const api = {
    // 设置 API 地址
    setBaseUrl(url: string) {
        localStorage.setItem('apiBaseUrl', url)
    },

    getBaseUrl() {
        return localStorage.getItem('apiBaseUrl') || API_BASE_URL
    },

    // 认证相关
    auth: {
        login: (username: string, password: string) =>
            request<{ token: string; user: any }>('/auth/login', {
                method: 'POST',
                body: { username, password }
            }),

        logout: () =>
            request<void>('/auth/logout', { method: 'POST' }),

        check: () =>
            request<{ user: any }>('/auth/check')
    },

    // 用户管理
    users: {
        list: () =>
            request<{ users: any[] }>('/users'),

        get: (id: number) =>
            request<{ user: any }>(`/users/${id}`),

        create: (data: any) =>
            request<{ user: any }>('/users', {
                method: 'POST',
                body: data
            }),

        update: (id: number, data: any) =>
            request<{ user: any }>(`/users/${id}`, {
                method: 'PUT',
                body: data
            }),

        delete: (id: number) =>
            request<void>(`/users/${id}`, { method: 'DELETE' })
    },

    // 配置管理
    config: {
        list: () =>
            request<{ configs: any[] }>('/config'),

        get: (key: string) =>
            request<{ config: any }>(`/config/${key}`),

        set: (key: string, value: any) =>
            request<{ config: any }>('/config', {
                method: 'POST',
                body: { key, value }
            })
    },

    // 日志
    logs: {
        list: (params?: { page?: number; limit?: number }) => {
            const query = new URLSearchParams()
            if (params?.page) query.set('page', String(params.page))
            if (params?.limit) query.set('limit', String(params.limit))
            const queryStr = query.toString()
            return request<{ logs: any[]; total: number }>(`/logs${queryStr ? `?${queryStr}` : ''}`)
        }
    }
}
