import api from './client'
import type { ApiResponse, LogEntry } from '@/types'

export const logsApi = {
    async getLogs(limit: number = 100): Promise<ApiResponse<LogEntry[]>> {
        const response = await api.get<ApiResponse<LogEntry[]>>('/logs', {
            params: { limit },
        })
        return response.data
    },
}
