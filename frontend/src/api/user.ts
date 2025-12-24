import api from './client'
import type { ApiResponse, User } from '@/types'

export const userApi = {
    async getProfile(): Promise<ApiResponse<User>> {
        const response = await api.get<ApiResponse<User>>('/user/profile')
        return response.data
    },

    async listUsers(): Promise<ApiResponse<User[]>> {
        const response = await api.get<ApiResponse<User[]>>('/users')
        return response.data
    },

    async createUser(username: string, password: string): Promise<ApiResponse<void>> {
        const response = await api.post<ApiResponse<void>>('/users', {
            username,
            password,
        })
        return response.data
    },

    async deleteUser(userId: number): Promise<ApiResponse<void>> {
        const response = await api.delete<ApiResponse<void>>(`/users/${userId}`)
        return response.data
    },

    async updateUser(userId: number, username: string, password?: string): Promise<ApiResponse<void>> {
        const response = await api.put<ApiResponse<void>>(`/users/${userId}`, {
            username,
            password: password || null,
        })
        return response.data
    },
}
