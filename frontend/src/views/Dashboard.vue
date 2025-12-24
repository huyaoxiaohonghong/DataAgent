<template>
  <div class="dashboard fade-in">
    <div class="page-header">
      <h1>仪表盘</h1>
      <p class="page-subtitle">欢迎回来, {{ authStore.session?.username }}</p>
    </div>

    <a-row :gutter="[24, 24]" class="stats-row">
      <a-col :xs="24" :sm="12" :lg="6">
        <div class="stat-card glass-card">
          <div class="stat-icon users">
            <TeamOutlined />
          </div>
          <div class="stat-content">
            <div class="stat-value">{{ stats.totalUsers }}</div>
            <div class="stat-label">总用户数</div>
          </div>
        </div>
      </a-col>
      <a-col :xs="24" :sm="12" :lg="6">
        <div class="stat-card glass-card">
          <div class="stat-icon logs">
            <FileTextOutlined />
          </div>
          <div class="stat-content">
            <div class="stat-value">{{ stats.totalLogs }}</div>
            <div class="stat-label">操作日志</div>
          </div>
        </div>
      </a-col>
      <a-col :xs="24" :sm="12" :lg="6">
        <div class="stat-card glass-card">
          <div class="stat-icon active">
            <ThunderboltOutlined />
          </div>
          <div class="stat-content">
            <div class="stat-value">正常</div>
            <div class="stat-label">系统状态</div>
          </div>
        </div>
      </a-col>
      <a-col :xs="24" :sm="12" :lg="6">
        <div class="stat-card glass-card">
          <div class="stat-icon edge">
            <CloudOutlined />
          </div>
          <div class="stat-content">
            <div class="stat-value">边缘</div>
            <div class="stat-label">运行环境</div>
          </div>
        </div>
      </a-col>
    </a-row>

    <a-row :gutter="[24, 24]">
      <a-col :xs="24" :lg="16">
        <div class="glass-card card-section">
          <h3>最近操作日志</h3>
          <a-table
            :columns="logColumns"
            :data-source="recentLogs"
            :pagination="false"
            :loading="loading"
            size="small"
          />
        </div>
      </a-col>
      <a-col :xs="24" :lg="8">
        <div class="glass-card card-section">
          <h3>系统信息</h3>
          <a-descriptions :column="1" size="small">
            <a-descriptions-item label="运行平台">Cloudflare Workers</a-descriptions-item>
            <a-descriptions-item label="数据库">D1 (SQLite)</a-descriptions-item>
            <a-descriptions-item label="缓存">KV Store</a-descriptions-item>
            <a-descriptions-item label="前端">Vue 3 + Ant Design</a-descriptions-item>
            <a-descriptions-item label="后端">Rust Worker</a-descriptions-item>
          </a-descriptions>
        </div>
      </a-col>
    </a-row>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { useAuthStore } from '@/stores/auth'
import { userApi } from '@/api/user'
import { logsApi } from '@/api/logs'
import type { LogEntry } from '@/types'
import {
  TeamOutlined,
  FileTextOutlined,
  ThunderboltOutlined,
  CloudOutlined,
} from '@ant-design/icons-vue'

const authStore = useAuthStore()
const loading = ref(false)
const recentLogs = ref<LogEntry[]>([])

const stats = reactive({
  totalUsers: 0,
  totalLogs: 0,
})

const logColumns = [
  { title: 'ID', dataIndex: 'id', key: 'id', width: 60 },
  { title: '操作', dataIndex: 'action', key: 'action' },
  { title: '时间', dataIndex: 'timestamp', key: 'timestamp' },
]

onMounted(async () => {
  loading.value = true
  try {
    const [usersRes, logsRes] = await Promise.all([
      userApi.listUsers(),
      logsApi.getLogs(10),
    ])
    if (usersRes.success && usersRes.data) {
      stats.totalUsers = usersRes.data.length
    }
    if (logsRes.success && logsRes.data) {
      recentLogs.value = logsRes.data
      stats.totalLogs = logsRes.data.length
    }
  } catch (error) {
    console.error('Failed to load dashboard data:', error)
  } finally {
    loading.value = false
  }
})
</script>

<style scoped>
.dashboard {
  padding: 0;
}

.page-header {
  margin-bottom: 32px;
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

.stats-row {
  margin-bottom: 24px;
}

.stat-card {
  padding: 24px;
  display: flex;
  align-items: center;
  gap: 20px;
  transition: transform 0.3s, box-shadow 0.3s;
}

.stat-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 12px 40px rgba(0, 0, 0, 0.3);
}

.stat-icon {
  width: 56px;
  height: 56px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 24px;
}

.stat-icon.users {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: #fff;
}

.stat-icon.logs {
  background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
  color: #fff;
}

.stat-icon.active {
  background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%);
  color: #fff;
}

.stat-icon.edge {
  background: linear-gradient(135deg, #43e97b 0%, #38f9d7 100%);
  color: #fff;
}

.stat-value {
  font-size: 28px;
  font-weight: 700;
  color: #fff;
}

.stat-label {
  color: rgba(255, 255, 255, 0.45);
  font-size: 14px;
}

.card-section {
  padding: 24px;
}

.card-section h3 {
  font-size: 16px;
  font-weight: 600;
  color: #fff;
  margin-bottom: 16px;
}
</style>
