<template>
  <div class="login-container">
    <div class="login-bg">
      <div class="bg-orb orb-1"></div>
      <div class="bg-orb orb-2"></div>
      <div class="bg-orb orb-3"></div>
    </div>
    
    <div class="login-card glass-card">
      <div class="login-header">
        <div class="logo">
          <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M12 2L2 7L12 12L22 7L12 2Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M2 17L12 22L22 17" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M2 12L12 17L22 12" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </div>
        <h1>DataAgent Client</h1>
        <p>连接到管理端同步数据</p>
      </div>

      <a-form
        :model="formState"
        @finish="handleLogin"
        layout="vertical"
        class="login-form"
      >
        <a-form-item label="服务器地址" name="serverUrl">
          <a-input
            v-model:value="formState.serverUrl"
            placeholder="https://your-worker.workers.dev/api"
            size="large"
          >
            <template #prefix>
              <GlobalOutlined />
            </template>
          </a-input>
        </a-form-item>

        <a-form-item
          label="用户名"
          name="username"
          :rules="[{ required: true, message: '请输入用户名' }]"
        >
          <a-input
            v-model:value="formState.username"
            placeholder="请输入用户名"
            size="large"
          >
            <template #prefix>
              <UserOutlined />
            </template>
          </a-input>
        </a-form-item>

        <a-form-item
          label="密码"
          name="password"
          :rules="[{ required: true, message: '请输入密码' }]"
        >
          <a-input-password
            v-model:value="formState.password"
            placeholder="请输入密码"
            size="large"
          >
            <template #prefix>
              <LockOutlined />
            </template>
          </a-input-password>
        </a-form-item>

        <a-form-item>
          <a-button
            type="primary"
            html-type="submit"
            size="large"
            block
            :loading="loading"
          >
            连接并登录
          </a-button>
        </a-form-item>
      </a-form>

      <div class="login-footer">
        <p>© 2026 DataAgent - Edge Management System</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, ref } from 'vue'
import { useRouter } from 'vue-router'
import { message } from 'ant-design-vue'
import { UserOutlined, LockOutlined, GlobalOutlined } from '@ant-design/icons-vue'
import { useAuthStore } from '../stores'
import { api } from '../api'

const router = useRouter()
const authStore = useAuthStore()

const formState = reactive({
  serverUrl: api.getBaseUrl(),
  username: '',
  password: ''
})

const loading = ref(false)

async function handleLogin() {
  loading.value = true
  
  // 保存服务器地址
  if (formState.serverUrl) {
    api.setBaseUrl(formState.serverUrl)
  }

  const result = await authStore.login(formState.username, formState.password)
  
  if (result.success) {
    message.success('登录成功！')
    router.push('/dashboard')
  } else {
    message.error(result.message || '登录失败')
  }
  
  loading.value = false
}
</script>

<style scoped>
.login-container {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 20px;
  position: relative;
  overflow: hidden;
}

.login-bg {
  position: absolute;
  inset: 0;
  overflow: hidden;
}

.bg-orb {
  position: absolute;
  border-radius: 50%;
  filter: blur(80px);
  opacity: 0.6;
  animation: float 10s ease-in-out infinite;
}

.orb-1 {
  width: 400px;
  height: 400px;
  background: linear-gradient(135deg, #6366f1, #8b5cf6);
  top: -100px;
  left: -100px;
}

.orb-2 {
  width: 300px;
  height: 300px;
  background: linear-gradient(135deg, #3b82f6, #06b6d4);
  bottom: -50px;
  right: -50px;
  animation-delay: -3s;
}

.orb-3 {
  width: 200px;
  height: 200px;
  background: linear-gradient(135deg, #8b5cf6, #ec4899);
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  animation-delay: -6s;
}

@keyframes float {
  0%, 100% { transform: translate(0, 0) scale(1); }
  33% { transform: translate(30px, -30px) scale(1.05); }
  66% { transform: translate(-20px, 20px) scale(0.95); }
}

.login-card {
  width: 100%;
  max-width: 420px;
  padding: 40px;
  position: relative;
  z-index: 1;
}

.login-header {
  text-align: center;
  margin-bottom: 32px;
}

.logo {
  width: 64px;
  height: 64px;
  margin: 0 auto 16px;
  background: linear-gradient(135deg, var(--primary-color), var(--primary-hover));
  border-radius: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
}

.logo svg {
  width: 32px;
  height: 32px;
}

.login-header h1 {
  font-size: 24px;
  font-weight: 700;
  color: var(--text-primary);
  margin-bottom: 8px;
}

.login-header p {
  color: var(--text-secondary);
  font-size: 14px;
}

.login-form {
  margin-top: 24px;
}

.login-form :deep(.ant-form-item-label > label) {
  color: var(--text-secondary);
}

.login-form :deep(.ant-input-affix-wrapper) {
  background: rgba(255, 255, 255, 0.05);
  border-color: var(--glass-border);
}

.login-form :deep(.ant-input) {
  background: transparent;
  color: var(--text-primary);
}

.login-form :deep(.anticon) {
  color: var(--text-muted);
}

.login-footer {
  text-align: center;
  margin-top: 24px;
  padding-top: 24px;
  border-top: 1px solid var(--glass-border);
}

.login-footer p {
  color: var(--text-muted);
  font-size: 12px;
}
</style>
