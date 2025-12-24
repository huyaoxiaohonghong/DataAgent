import api from './client'
import type { LoginResponse, ApiResponse, Session } from '@/types'

export const authApi = {
    async login(username: string, password: string): Promise<LoginResponse> {
        const response = await api.post<LoginResponse>('/auth/login', {
            username,
            password,
        })
        return response.data
    },

    async logout(): Promise<void> {
        await api.post('/auth/logout')
    },

    async checkSession(): Promise<ApiResponse<Session>> {
        const response = await api.get<ApiResponse<Session>>('/auth/check')
        return response.data
    },
}
