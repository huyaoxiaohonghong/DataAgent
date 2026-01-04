<template>
  <div class="dashboard-container">
    <!-- 顶部导航 -->
    <header class="dashboard-header glass-card">
      <div class="header-left">
        <div class="logo-mini">
          <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M12 2L2 7L12 12L22 7L12 2Z" stroke="currentColor" stroke-width="2"/>
            <path d="M2 17L12 22L22 17" stroke="currentColor" stroke-width="2"/>
            <path d="M2 12L12 17L22 12" stroke="currentColor" stroke-width="2"/>
          </svg>
        </div>
        <span class="app-title">DataAgent Client</span>
      </div>
      
      <div class="header-right">
        <a-button type="text" @click="handleSync" :loading="syncStore.syncing">
          <template #icon><SyncOutlined :spin="syncStore.syncing" /></template>
          同步数据
        </a-button>
        <a-dropdown>
          <a-button type="text">
            <template #icon><UserOutlined /></template>
            {{ authStore.user?.username || '用户' }}
            <DownOutlined />
          </a-button>
          <template #overlay>
            <a-menu>
              <a-menu-item key="logout" @click="handleLogout">
                <LogoutOutlined /> 退出登录
              </a-menu-item>
            </a-menu>
          </template>
        </a-dropdown>
      </div>
    </header>

    <!-- 主内容区 -->
    <main class="dashboard-main">
      <!-- 状态卡片 -->
      <div class="stats-grid">
        <div class="stat-card glass-card">
          <div class="stat-icon users">
            <TeamOutlined />
          </div>
          <div class="stat-info">
            <span class="stat-value">{{ syncStore.users.length }}</span>
            <span class="stat-label">同步用户</span>
          </div>
        </div>
        
        <div class="stat-card glass-card">
          <div class="stat-icon configs">
            <SettingOutlined />
          </div>
          <div class="stat-info">
            <span class="stat-value">{{ syncStore.configs.length }}</span>
            <span class="stat-label">配置项</span>
          </div>
        </div>
        
        <div class="stat-card glass-card">
          <div class="stat-icon sync">
            <CloudSyncOutlined />
          </div>
          <div class="stat-info">
            <span class="stat-value" :class="{ syncing: syncStore.syncing }">
              {{ syncStore.syncing ? '同步中...' : '已同步' }}
            </span>
            <span class="stat-label">{{ formatTime(syncStore.lastSyncTime) }}</span>
          </div>
        </div>
        
        <div class="stat-card glass-card">
          <div class="stat-icon status" :class="connectionStatus">
            <ApiOutlined />
          </div>
          <div class="stat-info">
            <span class="stat-value">{{ connectionStatus === 'online' ? '在线' : '离线' }}</span>
            <span class="stat-label">服务器连接</span>
          </div>
        </div>
      </div>

      <!-- 快捷操作 -->
      <div class="quick-actions">
        <h2 class="section-title">快捷操作</h2>
        <div class="actions-grid">
          <router-link to="/users" class="action-card glass-card">
            <TeamOutlined />
            <span>用户管理</span>
          </router-link>
          <router-link to="/config" class="action-card glass-card">
            <SettingOutlined />
            <span>配置管理</span>
          </router-link>
        </div>
      </div>

      <!-- 最近同步的用户 -->
      <div class="recent-section">
        <h2 class="section-title">最近同步的用户</h2>
        <div class="users-table glass-card">
          <a-table
            :dataSource="recentUsers"
            :columns="userColumns"
            :pagination="false"
            size="middle"
            :loading="syncStore.syncing"
          >
            <template #bodyCell="{ column, record }">
              <template v-if="column.key === 'role'">
                <a-tag :color="record.role === 'admin' ? 'purple' : 'blue'">
                  {{ record.role }}
                </a-tag>
              </template>
              <template v-if="column.key === 'status'">
                <span class="status-tag" :class="record.status || 'success'">
                  {{ record.status === 'disabled' ? '禁用' : '正常' }}
                </span>
              </template>
            </template>
          </a-table>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { message } from 'ant-design-vue'
import {
  SyncOutlined,
  UserOutlined,
  DownOutlined,
  LogoutOutlined,
  TeamOutlined,
  SettingOutlined,
  CloudSyncOutlined,
  ApiOutlined
} from '@ant-design/icons-vue'
import { useAuthStore, useSyncStore } from '../stores'

const router = useRouter()
const authStore = useAuthStore()
const syncStore = useSyncStore()

const connectionStatus = computed(() => {
  return syncStore.syncError ? 'offline' : 'online'
})

const recentUsers = computed(() => {
  return syncStore.users.slice(0, 5)
})

const userColumns = [
  { title: '用户名', dataIndex: 'username', key: 'username' },
  { title: '角色', dataIndex: 'role', key: 'role' },
  { title: '状态', dataIndex: 'status', key: 'status' },
  { title: '创建时间', dataIndex: 'created_at', key: 'created_at' }
]

function formatTime(time: string | null) {
  if (!time) return '从未同步'
  const date = new Date(time)
  return date.toLocaleString('zh-CN')
}

async function handleSync() {
  await syncStore.syncAll()
  if (syncStore.syncError) {
    message.error(syncStore.syncError)
  } else {
    message.success('同步成功！')
  }
}

async function handleLogout() {
  await authStore.logout()
  router.push('/login')
}

onMounted(() => {
  // 自动同步
  syncStore.syncAll()
})
</script>

<style scoped>
.dashboard-container {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

.dashboard-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 24px;
  margin: 16px;
  border-radius: var(--border-radius-lg);
}

.header-left {
  display: flex;
  align-items: center;
  gap: 12px;
}

.logo-mini {
  width: 36px;
  height: 36px;
  background: linear-gradient(135deg, var(--primary-color), var(--primary-hover));
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
}

.logo-mini svg {
  width: 20px;
  height: 20px;
}

.app-title {
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
}

.header-right {
  display: flex;
  align-items: center;
  gap: 8px;
}

.header-right :deep(.ant-btn) {
  color: var(--text-secondary);
}

.dashboard-main {
  flex: 1;
  padding: 0 16px 24px;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
  gap: 16px;
  margin-bottom: 32px;
}

.stat-card {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 20px;
}

.stat-icon {
  width: 48px;
  height: 48px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 24px;
}

.stat-icon.users {
  background: rgba(99, 102, 241, 0.15);
  color: #6366f1;
}

.stat-icon.configs {
  background: rgba(245, 158, 11, 0.15);
  color: #f59e0b;
}

.stat-icon.sync {
  background: rgba(59, 130, 246, 0.15);
  color: #3b82f6;
}

.stat-icon.status.online {
  background: rgba(16, 185, 129, 0.15);
  color: #10b981;
}

.stat-icon.status.offline {
  background: rgba(239, 68, 68, 0.15);
  color: #ef4444;
}

.stat-info {
  display: flex;
  flex-direction: column;
}

.stat-value {
  font-size: 20px;
  font-weight: 600;
  color: var(--text-primary);
}

.stat-label {
  font-size: 13px;
  color: var(--text-secondary);
}

.section-title {
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 16px;
}

.quick-actions {
  margin-bottom: 32px;
}

.actions-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
  gap: 16px;
}

.action-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 12px;
  padding: 24px;
  text-decoration: none;
  color: var(--text-primary);
  font-size: 14px;
  font-weight: 500;
  transition: all var(--transition-normal);
}

.action-card :deep(.anticon) {
  font-size: 28px;
  color: var(--primary-color);
}

.action-card:hover {
  transform: translateY(-4px);
  border-color: var(--primary-color);
}

.recent-section {
  margin-bottom: 24px;
}

.users-table {
  padding: 16px;
  overflow: hidden;
}

.users-table :deep(.ant-table) {
  background: transparent;
}
</style>
