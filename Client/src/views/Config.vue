<template>
  <div class="page-container">
    <header class="page-header">
      <a-button type="text" @click="$router.back()">
        <template #icon><ArrowLeftOutlined /></template>
        返回
      </a-button>
      <h1 class="page-title">配置管理</h1>
      <a-button type="primary" @click="handleSync" :loading="syncStore.syncing">
        <template #icon><SyncOutlined /></template>
        同步
      </a-button>
    </header>

    <main class="page-content">
      <div class="config-card glass-card">
        <div class="card-header">
          <a-input-search
            v-model:value="searchText"
            placeholder="搜索配置项..."
            style="width: 280px"
            allow-clear
          />
          <span class="sync-info">
            上次同步: {{ formatTime(syncStore.lastSyncTime) }}
          </span>
        </div>

        <a-table
          :dataSource="filteredConfigs"
          :columns="columns"
          :loading="syncStore.syncing"
          :pagination="{ pageSize: 10, showTotal: (total: number) => `共 ${total} 条` }"
          row-key="id"
        >
          <template #bodyCell="{ column, record }">
            <template v-if="column.key === 'key'">
              <a-tag color="blue">{{ record.key }}</a-tag>
            </template>
            <template v-if="column.key === 'value'">
              <a-typography-text
                :ellipsis="{ tooltip: record.value }"
                style="max-width: 300px"
              >
                {{ record.value }}
              </a-typography-text>
            </template>
            <template v-if="column.key === 'updated_at'">
              {{ formatDate(record.updated_at) }}
            </template>
          </template>
        </a-table>

        <!-- 空状态 -->
        <div v-if="filteredConfigs.length === 0 && !syncStore.syncing" class="empty-state">
          <SettingOutlined class="empty-icon" />
          <p>暂无配置数据</p>
          <a-button type="primary" @click="handleSync">
            立即同步
          </a-button>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { message } from 'ant-design-vue'
import { ArrowLeftOutlined, SyncOutlined, SettingOutlined } from '@ant-design/icons-vue'
import { useSyncStore } from '../stores'

const syncStore = useSyncStore()
const searchText = ref('')

const columns = [
  { title: 'ID', dataIndex: 'id', key: 'id', width: 80 },
  { title: '配置键', dataIndex: 'key', key: 'key', width: 200 },
  { title: '配置值', dataIndex: 'value', key: 'value' },
  { title: '描述', dataIndex: 'description', key: 'description' },
  { title: '更新时间', dataIndex: 'updated_at', key: 'updated_at', width: 180 }
]

const filteredConfigs = computed(() => {
  if (!searchText.value) return syncStore.configs
  const search = searchText.value.toLowerCase()
  return syncStore.configs.filter(config => 
    config.key.toLowerCase().includes(search) ||
    config.value.toLowerCase().includes(search) ||
    (config.description && config.description.toLowerCase().includes(search))
  )
})

function formatTime(time: string | null) {
  if (!time) return '从未'
  return new Date(time).toLocaleString('zh-CN')
}

function formatDate(date: string) {
  if (!date) return '-'
  return new Date(date).toLocaleDateString('zh-CN')
}

async function handleSync() {
  await syncStore.syncConfigs()
  if (syncStore.syncError) {
    message.error(syncStore.syncError)
  } else {
    message.success('配置数据同步成功！')
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

.config-card {
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

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 20px;
  color: var(--text-muted);
}

.empty-icon {
  font-size: 48px;
  margin-bottom: 16px;
  opacity: 0.5;
}

.empty-state p {
  margin-bottom: 16px;
  font-size: 14px;
}
</style>
