<template>
  <div class="users-page fade-in">
    <div class="page-header">
      <div>
        <h1>用户管理</h1>
        <p class="page-subtitle">管理系统用户账号</p>
      </div>
      <a-button type="primary" @click="openCreateModal">
        <template #icon><PlusOutlined /></template>
        添加用户
      </a-button>
    </div>

    <AnimatedList animationType="slideUp" :duration="0.6" :delay="100">
      <div class="glass-card table-card">
        <a-table
          :columns="columns"
          :data-source="users"
          :loading="loading"
          :pagination="{ pageSize: 10 }"
          row-key="id"
        >
          <template #bodyCell="{ column, record }">
            <template v-if="column.key === 'created_at'">
              {{ formatDate(record.created_at) }}
            </template>
            <template v-if="column.key === 'action'">
              <a-space>
                <a-button type="link" size="small" @click="openEditModal(record)">编辑</a-button>
                <a-popconfirm
                  title="确定要删除这个用户吗？"
                  @confirm="handleDeleteUser(record.id)"
                  ok-text="确定"
                  cancel-text="取消"
                >
                  <a-button type="link" size="small" danger :loading="deleteLoading === record.id">删除</a-button>
                </a-popconfirm>
              </a-space>
            </template>
          </template>
        </a-table>
      </div>
    </AnimatedList>

    <!-- 创建/编辑用户弹窗 -->
    <a-modal
      v-model:open="showModal"
      :title="editingUser ? '编辑用户' : '添加用户'"
      @ok="handleSubmit"
      :confirmLoading="submitLoading"
    >
      <a-form :model="formState" layout="vertical">
        <a-form-item label="用户名" required>
          <a-input v-model:value="formState.username" placeholder="请输入用户名" />
        </a-form-item>
        <a-form-item :label="editingUser ? '新密码（留空则不修改）' : '密码'" :required="!editingUser">
          <a-input-password v-model:value="formState.password" :placeholder="editingUser ? '留空则不修改密码' : '请输入密码'" />
        </a-form-item>
      </a-form>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { message } from 'ant-design-vue'
import { userApi } from '@/api/user'
import type { User } from '@/types'
import { PlusOutlined } from '@ant-design/icons-vue'
import AnimatedList from '@/components/animations/AnimatedList.vue'
import dayjs from 'dayjs'

const loading = ref(false)
const submitLoading = ref(false)
const deleteLoading = ref<number | null>(null)
const showModal = ref(false)
const users = ref<User[]>([])
const editingUser = ref<User | null>(null)

const formState = reactive({
  username: '',
  password: '',
})

const columns = [
  { title: 'ID', dataIndex: 'id', key: 'id', width: 80 },
  { title: '用户名', dataIndex: 'username', key: 'username' },
  { title: '创建时间', dataIndex: 'created_at', key: 'created_at' },
  { title: '操作', key: 'action', width: 150 },
]

function formatDate(date: string) {
  return dayjs(date).format('YYYY-MM-DD HH:mm')
}

function openCreateModal() {
  editingUser.value = null
  formState.username = ''
  formState.password = ''
  showModal.value = true
}

function openEditModal(user: User) {
  editingUser.value = user
  formState.username = user.username
  formState.password = ''
  showModal.value = true
}

async function loadUsers() {
  loading.value = true
  try {
    const response = await userApi.listUsers()
    if (response.success && response.data) {
      users.value = response.data
    }
  } catch (error) {
    message.error('加载用户列表失败')
  } finally {
    loading.value = false
  }
}

async function handleSubmit() {
  if (!formState.username) {
    message.warning('请输入用户名')
    return
  }
  
  if (!editingUser.value && !formState.password) {
    message.warning('请输入密码')
    return
  }

  submitLoading.value = true
  try {
    if (editingUser.value) {
      // 编辑用户
      const response = await userApi.updateUser(
        editingUser.value.id,
        formState.username,
        formState.password || undefined
      )
      if (response.success) {
        message.success('用户更新成功')
        showModal.value = false
        await loadUsers()
      } else {
        message.error(response.message)
      }
    } else {
      // 创建用户
      const response = await userApi.createUser(formState.username, formState.password)
      if (response.success) {
        message.success('用户创建成功')
        showModal.value = false
        formState.username = ''
        formState.password = ''
        await loadUsers()
      } else {
        message.error(response.message)
      }
    }
  } catch (error) {
    message.error(editingUser.value ? '更新用户失败' : '创建用户失败')
  } finally {
    submitLoading.value = false
  }
}

async function handleDeleteUser(userId: number) {
  deleteLoading.value = userId
  try {
    const response = await userApi.deleteUser(userId)
    if (response.success) {
      message.success('用户删除成功')
      await loadUsers()
    } else {
      message.error(response.message)
    }
  } catch (error) {
    message.error('删除用户失败')
  } finally {
    deleteLoading.value = null
  }
}

onMounted(() => {
  loadUsers()
})
</script>

<style scoped>
.users-page {
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
