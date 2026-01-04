import { defineStore } from 'pinia'
import { ref } from 'vue'
import { api } from '../api'

export interface UserInfo {
    id: number
    username: string
    role: string
    status: string
    created_at: string
}

export interface ConfigInfo {
    id: number
    key: string
    value: string
    description: string
    updated_at: string
}

export const useSyncStore = defineStore('sync', () => {
    const users = ref<UserInfo[]>([])
    const configs = ref<ConfigInfo[]>([])
    const lastSyncTime = ref<string | null>(null)
    const syncing = ref(false)
    const syncError = ref<string | null>(null)

    async function syncUsers() {
        syncing.value = true
        syncError.value = null
        try {
            const response = await api.users.list()
            users.value = response.users || response
            lastSyncTime.value = new Date().toISOString()
        } catch (error: any) {
            syncError.value = error.message || '同步用户失败'
        } finally {
            syncing.value = false
        }
    }

    async function syncConfigs() {
        syncing.value = true
        syncError.value = null
        try {
            const response = await api.config.list()
            configs.value = response.configs || response
            lastSyncTime.value = new Date().toISOString()
        } catch (error: any) {
            syncError.value = error.message || '同步配置失败'
        } finally {
            syncing.value = false
        }
    }

    async function syncAll() {
        syncing.value = true
        syncError.value = null
        try {
            await Promise.all([syncUsers(), syncConfigs()])
            lastSyncTime.value = new Date().toISOString()
        } catch (error: any) {
            syncError.value = error.message || '同步失败'
        } finally {
            syncing.value = false
        }
    }

    return {
        users,
        configs,
        lastSyncTime,
        syncing,
        syncError,
        syncUsers,
        syncConfigs,
        syncAll
    }
})
