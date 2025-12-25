<template>
  <div class="login-container">
    <!-- 像素雪花背景 -->
    <PixelSnow 
      :snowflakeCount="180"
      color="rgba(255, 255, 255, 0.9)"
      :minSize="2"
      :maxSize="6"
      :speed="1.2"
      :wind="0.3"
      :pixelated="true"
    />
    
    <div class="login-card glass-card fade-in">
      <div class="login-header">
        <CloudOutlined class="login-logo" />
        <h1 class="gradient-text">
          <SplitText text="Edge Manager" :delay="60" :duration="400" />
        </h1>
        <p class="login-subtitle">{{ isAddAccountMode ? '添加新账号' : '全栈边缘管理系统' }}</p>
      </div>
      
      <!-- 显示已登录的账号 -->
      <div v-if="authStore.sessions.length > 0 && !isAddAccountMode" class="logged-in-accounts">
        <p class="accounts-title">已登录账号</p>
        <div class="account-list">
          <div 
            v-for="session in authStore.sessions" 
            :key="session.user_id"
            class="account-item"
            @click="switchToAccount(session.user_id)"
          >
            <a-avatar :size="32"><template #icon><UserOutlined /></template></a-avatar>
            <span class="account-name">{{ session.username }}</span>
            <RightOutlined />
          </div>
        </div>
        <a-divider>或登录其他账号</a-divider>
      </div>
      
      <a-form
        :model="formState"
        @finish="handleLoginClick"
        layout="vertical"
        class="login-form"
      >
        <a-form-item
          name="username"
          :rules="[{ required: true, message: '请输入用户名' }]"
        >
          <a-input
            v-model:value="formState.username"
            placeholder="用户名"
            size="large"
          >
            <template #prefix>
              <UserOutlined />
            </template>
          </a-input>
        </a-form-item>
        <a-form-item
          name="password"
          :rules="[{ required: true, message: '请输入密码' }]"
        >
          <a-input-password
            v-model:value="formState.password"
            placeholder="密码"
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
            class="login-button"
          >
            {{ isAddAccountMode ? '添加账号' : '登录' }}
          </a-button>
        </a-form-item>
      </a-form>
      
      <div v-if="isAddAccountMode" class="back-link">
        <a @click="router.back()">返回</a>
      </div>
      
      <div class="login-footer">
        <p>默认账号: admin / admin123</p>
      </div>
    </div>
    
    <!-- 弹窗式滑动验证 -->
    <SliderCaptcha 
      v-model:visible="captchaVisible" 
      @success="handleCaptchaSuccess" 
    />
  </div>
</template>

<script setup lang="ts">
import { reactive, ref, computed, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { message } from 'ant-design-vue'
import { useAuthStore } from '@/stores/auth'
import { UserOutlined, LockOutlined, CloudOutlined, RightOutlined } from '@ant-design/icons-vue'
import SliderCaptcha from '@/components/SliderCaptcha.vue'
import PixelSnow from '@/components/backgrounds/PixelSnow.vue'
import SplitText from '@/components/animations/SplitText.vue'

const router = useRouter()
const route = useRoute()
const authStore = useAuthStore()
const loading = ref(false)
const captchaVisible = ref(false)

const isAddAccountMode = computed(() => route.query.addAccount === 'true')

const formState = reactive({
  username: '',
  password: '',
})

onMounted(() => {
  authStore.init()
})

function switchToAccount(userId: number) {
  authStore.switchUser(userId)
  message.success(`已切换到用户: ${authStore.username}`)
  router.push('/')
}

// 点击登录按钮，触发验证码弹窗
function handleLoginClick() {
  captchaVisible.value = true
}

// 验证成功后，自动执行登录
async function handleCaptchaSuccess() {
  loading.value = true
  try {
    const response = await authStore.login(formState.username, formState.password)
    if (response.success) {
      message.success('登录成功')
      router.push('/')
    } else {
      message.error(response.message || '用户名或密码错误')
    }
  } catch (error) {
    message.error('登录失败，请稍后重试')
  } finally {
    loading.value = false
  }
}
</script>

<style scoped>
.login-container {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
  overflow: hidden;
  background: #0b0b0b;
}

.login-card {
  width: 100%;
  max-width: 420px;
  padding: 48px 40px;
  z-index: 1;
}

.login-header {
  text-align: center;
  margin-bottom: 32px;
}

.login-logo {
  font-size: 48px;
  color: #1890ff;
  margin-bottom: 16px;
}

.login-header h1 {
  font-size: 28px;
  margin-bottom: 8px;
}

.login-subtitle {
  color: rgba(255, 255, 255, 0.45);
  font-size: 14px;
}

.logged-in-accounts {
  margin-bottom: 24px;
}

.accounts-title {
  font-size: 12px;
  color: rgba(255, 255, 255, 0.45);
  margin-bottom: 12px;
}

.account-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.account-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.3s;
}

.account-item:hover {
  background: rgba(255, 255, 255, 0.1);
}

.account-name {
  flex: 1;
  color: #fff;
}

.login-form :deep(.ant-input-affix-wrapper),
.login-form :deep(.ant-input) {
  background: rgba(255, 255, 255, 0.05);
  border-color: rgba(255, 255, 255, 0.1);
}

.login-form :deep(.ant-input-affix-wrapper:hover),
.login-form :deep(.ant-input:hover) {
  border-color: #1890ff;
}

.login-button {
  height: 48px;
  font-size: 16px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border: none;
}

.login-button:hover:not(:disabled) {
  background: linear-gradient(135deg, #764ba2 0%, #667eea 100%);
}

.login-button:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.back-link {
  text-align: center;
  margin-top: 16px;
}

.back-link a {
  color: #1890ff;
  cursor: pointer;
}

.login-footer {
  text-align: center;
  margin-top: 24px;
  color: rgba(255, 255, 255, 0.35);
  font-size: 12px;
}
</style>
