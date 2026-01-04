<template>
  <a-config-provider :theme="themeConfig">
    <router-view />
  </a-config-provider>
</template>

<script setup lang="ts">
import { theme } from 'ant-design-vue'

// Ant Design Vue 暗色主题配置
const themeConfig = {
  algorithm: theme.darkAlgorithm,
  token: {
    colorPrimary: '#6366f1',
    colorBgContainer: 'rgba(255, 255, 255, 0.05)',
    colorBgElevated: '#1a1a2e',
    colorBorder: 'rgba(255, 255, 255, 0.1)',
    colorText: '#f8fafc',
    colorTextSecondary: '#94a3b8',
    borderRadius: 8
  }
}
</script>

<style>
#app {
  min-height: 100vh;
}
</style>