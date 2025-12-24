<template>
  <div class="profile-page fade-in">
    <div class="page-header">
      <h1>个人信息</h1>
      <p class="page-subtitle">查看您的账号详情</p>
    </div>

    <a-row :gutter="24">
      <a-col :xs="24" :lg="8">
        <div class="glass-card profile-card">
          <div class="profile-avatar">
            <a-avatar :size="100">
              <template #icon><UserOutlined /></template>
            </a-avatar>
          </div>
          <h2 class="profile-name">{{ user?.username }}</h2>
          <p class="profile-role">系统管理员</p>
        </div>
      </a-col>
      <a-col :xs="24" :lg="16">
        <div class="glass-card info-card">
          <h3>账号详情</h3>
          <a-descriptions :column="1" :loading="loading">
            <a-descriptions-item label="用户ID">{{ user?.id }}</a-descriptions-item>
            <a-descriptions-item label="用户名">{{ user?.username }}</a-descriptions-item>
            <a-descriptions-item label="创建时间">{{ formatDate(user?.created_at) }}</a-descriptions-item>
            <a-descriptions-item label="会话有效期">
              {{ sessionExpiry }}
            </a-descriptions-item>
          </a-descriptions>
        </div>
      </a-col>
    </a-row>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useAuthStore } from '@/stores/auth'
import { userApi } from '@/api/user'
import type { User } from '@/types'
import { UserOutlined } from '@ant-design/icons-vue'
import dayjs from 'dayjs'

const authStore = useAuthStore()
const loading = ref(false)
const user = ref<User | null>(null)

const sessionExpiry = computed(() => {
  if (!authStore.session) return '未知'
  return dayjs(authStore.session.expires_at * 1000).format('YYYY-MM-DD HH:mm:ss')
})

function formatDate(date?: string) {
  if (!date) return '-'
  return dayjs(date).format('YYYY-MM-DD HH:mm:ss')
}

onMounted(async () => {
  loading.value = true
  try {
    const response = await userApi.getProfile()
    if (response.success && response.data) {
      user.value = response.data
    }
  } catch (error) {
    console.error('Failed to load profile:', error)
  } finally {
    loading.value = false
  }
})
</script>

<style scoped>
.profile-page {
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

.profile-card {
  padding: 40px 24px;
  text-align: center;
}

.profile-avatar {
  margin-bottom: 20px;
}

.profile-avatar :deep(.ant-avatar) {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.profile-name {
  font-size: 24px;
  font-weight: 600;
  color: #fff;
  margin-bottom: 8px;
}

.profile-role {
  color: rgba(255, 255, 255, 0.45);
  font-size: 14px;
}

.info-card {
  padding: 24px;
}

.info-card h3 {
  font-size: 16px;
  font-weight: 600;
  color: #fff;
  margin-bottom: 24px;
}
</style>
