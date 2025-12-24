import api from './client'
import type { ApiResponse } from '@/types'

export interface ProxyNode {
    id: number
    name: string
    protocol: string
    address: string
    port: number
    username?: string
    password?: string
    extra_config?: string
    group_name?: string
    status: string
    latency?: number
    last_check_at?: string
    created_at: string
    updated_at: string
}

export interface CreateProxyNodeRequest {
    name: string
    protocol: string
    address: string
    port: number
    username?: string
    password?: string
    extra_config?: string
    group_name?: string
}

export interface ProxyCheckResult {
    id: number
    status: string
    latency?: number
    message: string
}

export interface ImportResult {
    total: number
    success: number
    failed: number
    nodes: string[]
}

export interface BatchDeleteResult {
    deleted: number
}

export const proxyApi = {
    async listNodes(): Promise<ApiResponse<ProxyNode[]>> {
        const response = await api.get<ApiResponse<ProxyNode[]>>('/proxy/nodes')
        return response.data
    },

    async getNode(id: number): Promise<ApiResponse<ProxyNode>> {
        const response = await api.get<ApiResponse<ProxyNode>>(`/proxy/nodes/${id}`)
        return response.data
    },

    async createNode(data: CreateProxyNodeRequest): Promise<ApiResponse<{ id: number }>> {
        const response = await api.post<ApiResponse<{ id: number }>>('/proxy/nodes', data)
        return response.data
    },

    async updateNode(id: number, data: CreateProxyNodeRequest): Promise<ApiResponse<void>> {
        const response = await api.put<ApiResponse<void>>(`/proxy/nodes/${id}`, data)
        return response.data
    },

    async deleteNode(id: number): Promise<ApiResponse<void>> {
        const response = await api.delete<ApiResponse<void>>(`/proxy/nodes/${id}`)
        return response.data
    },

    async checkNode(id: number): Promise<ApiResponse<ProxyCheckResult>> {
        const response = await api.post<ApiResponse<ProxyCheckResult>>(`/proxy/nodes/${id}/check`)
        return response.data
    },

    async checkAllNodes(): Promise<ApiResponse<ProxyCheckResult[]>> {
        const response = await api.post<ApiResponse<ProxyCheckResult[]>>('/proxy/nodes/check-all')
        return response.data
    },

    async importSubscription(data: { url?: string; content?: string; group_name?: string }): Promise<ApiResponse<ImportResult>> {
        const response = await api.post<ApiResponse<ImportResult>>('/proxy/import', data)
        return response.data
    },

    async batchDeleteNodes(ids: number[]): Promise<ApiResponse<BatchDeleteResult>> {
        const response = await api.post<ApiResponse<BatchDeleteResult>>('/proxy/nodes/batch-delete', { ids })
        return response.data
    },
}

