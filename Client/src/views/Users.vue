<template>
  <div class="page-container">
    <header class="page-header">
      <a-button type="text" @click="$router.back()">
        <template #icon><ArrowLeftOutlined /></template>
        返回
      </a-button>
      <h1 class="page-title">用户管理</h1>
      <a-button type="primary" @click="handleSync" :loading="syncStore.syncing">
        <template #icon><SyncOutlined /></template>
        同步
      </a-button>
    </header>

    <main class="page-content">
      <div class="users-card glass-card">
        <div class="card-header">
          <a-input-search
            v-model:value="searchText"
            placeholder="搜索用户..."
            style="width: 280px"
            allow-clear
          />
          <span class="sync-info">
            上次同步: {{ formatTime(syncStore.lastSyncTime) }}
          </span>
        </div>

        <a-table
          :dataSource="filteredUsers"
          :columns="columns"
          :loading="syncStore.syncing"
          :pagination="{ pageSize: 10, showTotal: (total: number) => `共 ${total} 条` }"
          row-key="id"
        >
          <template #bodyCell="{ column, record }">
            <template v-if="column.key === 'role'">
              <a-tag :color="getRoleColor(record.role)">
                {{ getRoleLabel(record.role) }}
              </a-tag>
            </template>
            <template v-if="column.key === 'status'">
              <span class="status-tag" :class="record.status === 'disabled' ? 'error' : 'success'">
                <span class="status-dot"></span>
                {{ record.status === 'disabled' ? '禁用' : '正常' }}
              </span>
            </template>
            <template v-if="column.key === 'created_at'">
              {{ formatDate(record.created_at) }}
            </template>
          </template>
        </a-table>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { message } from 'ant-design-vue'
import { ArrowLeftOutlined, SyncOutlined } from '@ant-design/icons-vue'
import { useSyncStore } from '../stores'

const syncStore = useSyncStore()
const searchText = ref('')

const columns = [
  { title: 'ID', dataIndex: 'id', key: 'id', width: 80 },
  { title: '用户名', dataIndex: 'username', key: 'username' },
  { title: '角色', dataIndex: 'role', key: 'role', width: 120 },
  { title: '状态', dataIndex: 'status', key: 'status', width: 100 },
  { title: '创建时间', dataIndex: 'created_at', key: 'created_at', width: 180 }
]

const filteredUsers = computed(() => {
  if (!searchText.value) return syncStore.users
  const search = searchText.value.toLowerCase()
  return syncStore.users.filter(user => 
    user.username.toLowerCase().includes(search) ||
    user.role.toLowerCase().includes(search)
  )
})

function getRoleColor(role: string) {
  const colors: Record<string, string> = {
    admin: 'purple',
    manager: 'blue',
    user: 'green'
  }
  return colors[role] || 'default'
}

function getRoleLabel(role: string) {
  const labels: Record<string, string> = {
    admin: '管理员',
    manager: '经理',
    user: '用户'
  }
  return labels[role] || role
}

function formatTime(time: string | null) {
  if (!time) return '从未'
  return new Date(time).toLocaleString('zh-CN')
}

function formatDate(date: string) {
  if (!date) return '-'
  return new Date(date).toLocaleDateString('zh-CN')
}

async function handleSync() {
  await syncStore.syncUsers()
  if (syncStore.syncError) {
    message.error(syncStore.syncError)
  } else {
    message.success('用户数据同步成功！')
  }
}
</script>

<style scoped>
.page-container {
  min-height: 100vh;
  padding: 16px;
}

.page-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 24px;
}

.page-header :deep(.ant-btn-text) {
  color: var(--text-secondary);
}

.page-title {
  font-size: 24px;
  font-weight: 700;
  background: linear-gradient(135deg, var(--text-primary), var(--primary-hover));
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.page-content {
  max-width: 1200px;
  margin: 0 auto;
}

.users-card {
  padding: 24px;
}

.card-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 20px;
}

.sync-info {
  font-size: 13px;
  color: var(--text-muted);
}

.status-tag {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 4px 12px;
  border-radius: 20px;
  font-size: 12px;
}

.status-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: currentColor;
}

.status-tag.success {
  background: rgba(16, 185, 129, 0.15);
  color: var(--success-color);
}

.status-tag.error {
  background: rgba(239, 68, 68, 0.15);
  color: var(--error-color);
}
</style>
