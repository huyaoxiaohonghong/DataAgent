<template>
  <a-layout class="main-layout">
    <a-layout-sider v-model:collapsed="collapsed" collapsible :trigger="null" class="sider">
      <div class="logo">
        <CloudOutlined class="logo-icon" />
        <span v-if="!collapsed" class="logo-text">Edge Manager</span>
      </div>
      <a-menu
        v-model:selectedKeys="selectedKeys"
        theme="dark"
        mode="inline"
        @click="handleMenuClick"
      >
        <a-menu-item key="Dashboard">
          <DashboardOutlined />
          <span>仪表盘</span>
        </a-menu-item>
        <a-menu-item key="ProxyNodes">
          <CloudServerOutlined />
          <span>配置管理</span>
        </a-menu-item>
        <a-menu-item key="Users">
          <TeamOutlined />
          <span>用户管理</span>
        </a-menu-item>
        <a-menu-item key="Logs">
          <FileTextOutlined />
          <span>系统日志</span>
        </a-menu-item>
      </a-menu>
    </a-layout-sider>
    <a-layout>
      <a-layout-header class="header">
        <div class="header-left">
          <a-button type="text" @click="collapsed = !collapsed">
            <MenuFoldOutlined v-if="!collapsed" />
            <MenuUnfoldOutlined v-else />
          </a-button>
        </div>
        <div class="header-right">
          <a-dropdown>
            <a-space class="user-info">
              <a-avatar :size="32">
                <template #icon><UserOutlined /></template>
              </a-avatar>
              <span class="username">{{ authStore.username }}</span>
              <a-badge v-if="authStore.sessions.length > 1" :count="authStore.sessions.length" :offset="[0, -5]" />
            </a-space>
            <template #overlay>
              <a-menu>
                <!-- 已登录的用户列表 -->
                <template v-if="authStore.sessions.length > 1">
                  <a-menu-item-group title="切换用户">
                    <a-menu-item 
                      v-for="session in authStore.sessions" 
                      :key="session.user_id"
                      @click="switchToUser(session.user_id)"
                    >
                      <CheckOutlined v-if="session.user_id === authStore.currentUserId" />
                      <UserOutlined v-else />
                      {{ session.username }}
                    </a-menu-item>
                  </a-menu-item-group>
                  <a-menu-divider />
                </template>
                
                <a-menu-item key="profile" @click="router.push('/profile')">
                  <UserOutlined />
                  个人信息
                </a-menu-item>
                <a-menu-item key="add-account" @click="addAnotherAccount">
                  <PlusOutlined />
                  添加账号
                </a-menu-item>
                <a-menu-divider />
                <a-menu-item key="logout" @click="handleLogout">
                  <LogoutOutlined />
                  退出当前账号
                </a-menu-item>
                <a-menu-item v-if="authStore.sessions.length > 1" key="logout-all" danger @click="handleLogoutAll">
                  <LogoutOutlined />
                  退出所有账号
                </a-menu-item>
              </a-menu>
            </template>
          </a-dropdown>
        </div>
      </a-layout-header>
      <a-layout-content class="content">
        <router-view />
      </a-layout-content>
    </a-layout>
  </a-layout>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { message } from 'ant-design-vue'
import {
  DashboardOutlined,
  TeamOutlined,
  FileTextOutlined,
  MenuFoldOutlined,
  MenuUnfoldOutlined,
  UserOutlined,
  LogoutOutlined,
  CloudOutlined,
  CloudServerOutlined,
  PlusOutlined,
  CheckOutlined,
} from '@ant-design/icons-vue'

const router = useRouter()
const route = useRoute()
const authStore = useAuthStore()

const collapsed = ref(false)
const selectedKeys = computed(() => [route.name as string])

function handleMenuClick({ key }: { key: string }) {
  router.push({ name: key })
}

function switchToUser(userId: number) {
  authStore.switchUser(userId)
  message.success(`已切换到用户: ${authStore.username}`)
  // 刷新当前页面数据
  router.go(0)
}

function addAnotherAccount() {
  // 跳转到登录页，但不清除现有会话
  router.push('/login?addAccount=true')
}

async function handleLogout() {
  await authStore.logout()
  if (authStore.isAuthenticated) {
    // 还有其他用户登录，刷新页面
    message.success(`已退出，当前用户: ${authStore.username}`)
    router.go(0)
  } else {
    router.push('/login')
  }
}

function handleLogoutAll() {
  authStore.logoutAll()
  router.push('/login')
}
</script>

<style scoped>
.main-layout {
  min-height: 100vh;
}

.sider {
  background: rgba(0, 21, 41, 0.95) !important;
  backdrop-filter: blur(10px);
}

.logo {
  height: 64px;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 12px;
  padding: 16px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.logo-icon {
  font-size: 28px;
  color: #1890ff;
}

.logo-text {
  font-size: 18px;
  font-weight: 600;
  color: #fff;
  white-space: nowrap;
}

.header {
  background: rgba(0, 21, 41, 0.8) !important;
  backdrop-filter: blur(10px);
  padding: 0 24px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.header-left,
.header-right {
  display: flex;
  align-items: center;
}

.user-info {
  cursor: pointer;
  color: rgba(255, 255, 255, 0.85);
}

.username {
  color: rgba(255, 255, 255, 0.85);
}

.content {
  margin: 24px;
  padding: 24px;
  background: rgba(255, 255, 255, 0.02);
  border-radius: 12px;
  min-height: calc(100vh - 64px - 48px);
}
</style>
