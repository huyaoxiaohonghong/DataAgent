<template>
  <div class="logs-page fade-in">
    <div class="page-header">
      <div>
        <h1>系统日志</h1>
        <p class="page-subtitle">查看系统操作记录</p>
      </div>
      <a-button @click="loadLogs">
        <template #icon><ReloadOutlined /></template>
        刷新
      </a-button>
    </div>

    <AnimatedList animationType="slideUp" :duration="0.6" :delay="100">
      <div class="glass-card table-card">
        <a-table
          :columns="columns"
          :data-source="logs"
          :loading="loading"
          :pagination="{ pageSize: 20 }"
          row-key="id"
        >
          <template #bodyCell="{ column, record }">
            <template v-if="column.key === 'timestamp'">
              {{ formatDate(record.timestamp) }}
            </template>
            <template v-if="column.key === 'action'">
              <a-tag :color="getActionColor(record.action)">
                {{ record.action }}
              </a-tag>
            </template>
          </template>
        </a-table>
      </div>
    </AnimatedList>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { message } from 'ant-design-vue'
import { logsApi } from '@/api/logs'
import type { LogEntry } from '@/types'
import { ReloadOutlined } from '@ant-design/icons-vue'
import AnimatedList from '@/components/animations/AnimatedList.vue'
import dayjs from 'dayjs'

const loading = ref(false)
const logs = ref<LogEntry[]>([])

const columns = [
  { title: 'ID', dataIndex: 'id', key: 'id', width: 80 },
  { title: '用户ID', dataIndex: 'user_id', key: 'user_id', width: 100 },
  { title: '操作', dataIndex: 'action', key: 'action' },
  { title: '时间', dataIndex: 'timestamp', key: 'timestamp', width: 180 },
]

function formatDate(date: string) {
  return dayjs(date).format('YYYY-MM-DD HH:mm:ss')
}

function getActionColor(action: string): string {
  if (action.includes('login')) return 'green'
  if (action.includes('logout')) return 'orange'
  if (action.includes('create')) return 'blue'
  if (action.includes('delete')) return 'red'
  return 'default'
}

async function loadLogs() {
  loading.value = true
  try {
    const response = await logsApi.getLogs(100)
    if (response.success && response.data) {
      logs.value = response.data
    }
  } catch (error) {
    message.error('加载日志失败')
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  loadLogs()
})
</script>

<style scoped>
.logs-page {
  padding: 0;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
}

.page-header h1 {
  font-size: 28px;
  font-weight: 600;
  color: #fff;
  margin-bottom: 8px;
}

.page-subtitle {
  color: rgba(255, 255, 255, 0.45);
  font-size: 14px;
}

.table-card {
  padding: 24px;
}
</style>
