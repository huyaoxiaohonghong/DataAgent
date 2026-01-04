export interface User {
    id: number
    username: string
    created_at: string
}

export interface Session {
    user_id: number
    username: string
    expires_at: number
}

export interface LogEntry {
    id: number
    user_id: number
    action: string
    timestamp: string
}

export interface LoginResponse {
    success: boolean
    token?: string
    message: string
    user_id?: number
    username?: string
}

export interface ApiResponse<T> {
    success: boolean
    data?: T
    message: string
}

// 存储在本地的用户会话信息
export interface StoredSession {
    token: string
    user_id: number
    username: string
    expires_at: number
}
