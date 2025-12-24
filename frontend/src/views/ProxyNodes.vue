<template>
  <div class="proxy-page fade-in">
    <div class="page-header">
      <div>
        <h1>配置管理</h1>
        <p class="page-subtitle">代理节点配置与测速</p>
      </div>
      <a-space>
        <a-popconfirm
          v-if="selectedRowKeys.length > 0"
          :title="`确定要删除选中的 ${selectedRowKeys.length} 个节点吗？`"
          @confirm="handleBatchDelete"
          ok-text="确定"
          cancel-text="取消"
        >
          <a-button danger :loading="batchDeleting">
            <template #icon><DeleteOutlined /></template>
            批量删除 ({{ selectedRowKeys.length }})
          </a-button>
        </a-popconfirm>
        <a-button @click="handleCheckAll" :loading="checkingAll">
          <template #icon><ThunderboltOutlined /></template>
          全部测速
        </a-button>
        <a-button @click="showImportModal = true">
          <template #icon><ImportOutlined /></template>
          导入订阅
        </a-button>
        <a-button type="primary" @click="openCreateModal">
          <template #icon><PlusOutlined /></template>
          添加节点
        </a-button>
      </a-space>
    </div>

    <!-- 统计卡片 -->
    <a-row :gutter="16" class="stats-row">
      <a-col :span="6">
        <div class="stat-card glass-card">
          <div class="stat-icon total"><CloudServerOutlined /></div>
          <div class="stat-info">
            <div class="stat-value">{{ nodes.length }}</div>
            <div class="stat-label">总节点数</div>
          </div>
        </div>
      </a-col>
      <a-col :span="6">
        <div class="stat-card glass-card">
          <div class="stat-icon valid"><CheckCircleOutlined /></div>
          <div class="stat-info">
            <div class="stat-value">{{ validCount }}</div>
            <div class="stat-label">可用节点</div>
          </div>
        </div>
      </a-col>
      <a-col :span="6">
        <div class="stat-card glass-card">
          <div class="stat-icon invalid"><CloseCircleOutlined /></div>
          <div class="stat-info">
            <div class="stat-value">{{ invalidCount }}</div>
            <div class="stat-label">不可用</div>
          </div>
        </div>
      </a-col>
      <a-col :span="6">
        <div class="stat-card glass-card">
          <div class="stat-icon unknown"><QuestionCircleOutlined /></div>
          <div class="stat-info">
            <div class="stat-value">{{ unknownCount }}</div>
            <div class="stat-label">未检测</div>
          </div>
        </div>
      </a-col>
    </a-row>

    <!-- 节点列表 -->
    <div class="glass-card table-card">
      <a-table
        :columns="columns"
        :data-source="nodes"
        :loading="loading"
        :pagination="{ pageSize: 10 }"
        row-key="id"
        :row-selection="{ selectedRowKeys, onChange: onSelectChange }"
      >
        <template #bodyCell="{ column, record }">
          <template v-if="column.key === 'status'">
            <a-tag :color="getStatusColor(record.status)">
              {{ getStatusText(record.status) }}
            </a-tag>
          </template>
          <template v-if="column.key === 'latency'">
            <span v-if="record.latency" :class="getLatencyClass(record.latency)">
              {{ record.latency }}ms
            </span>
            <span v-else class="text-muted">-</span>
          </template>
          <template v-if="column.key === 'address'">
            <code class="address-code">{{ record.address }}:{{ record.port }}</code>
          </template>
          <template v-if="column.key === 'last_check_at'">
            <span v-if="record.last_check_at">{{ formatDate(record.last_check_at) }}</span>
            <span v-else class="text-muted">从未检测</span>
          </template>
          <template v-if="column.key === 'action'">
            <a-space>
              <a-tooltip title="测速">
                <a-button 
                  type="text" 
                  size="small" 
                  @click="handleCheck(record.id)"
                  :loading="checkingId === record.id"
                >
                  <ThunderboltOutlined />
                </a-button>
              </a-tooltip>
              <a-button type="text" size="small" @click="openEditModal(record)">
                <EditOutlined />
              </a-button>
              <a-popconfirm
                title="确定要删除这个节点吗？"
                @confirm="handleDelete(record.id)"
                ok-text="确定"
                cancel-text="取消"
              >
                <a-button type="text" size="small" danger>
                  <DeleteOutlined />
                </a-button>
              </a-popconfirm>
            </a-space>
          </template>
        </template>
      </a-table>
    </div>

    <!-- 创建/编辑弹窗 -->
    <a-modal
      v-model:open="showModal"
      :title="editingNode ? '编辑节点' : '添加节点'"
      @ok="handleSubmit"
      :confirmLoading="submitLoading"
      width="520px"
    >
      <a-form :model="formState" layout="vertical">
        <a-row :gutter="16">
          <a-col :span="12">
            <a-form-item label="节点名称" required>
              <a-input v-model:value="formState.name" placeholder="如：香港节点1" />
            </a-form-item>
          </a-col>
          <a-col :span="12">
            <a-form-item label="协议类型" required>
              <a-select v-model:value="formState.protocol" placeholder="选择协议">
                <a-select-option value="http">HTTP</a-select-option>
                <a-select-option value="https">HTTPS</a-select-option>
                <a-select-option value="socks5">SOCKS5</a-select-option>
                <a-select-option value="vmess">VMess</a-select-option>
                <a-select-option value="trojan">Trojan</a-select-option>
                <a-select-option value="ss">Shadowsocks</a-select-option>
              </a-select>
            </a-form-item>
          </a-col>
        </a-row>
        <a-row :gutter="16">
          <a-col :span="16">
            <a-form-item label="服务器地址" required>
              <a-input v-model:value="formState.address" placeholder="如：192.168.1.1 或 example.com" />
            </a-form-item>
          </a-col>
          <a-col :span="8">
            <a-form-item label="端口" required>
              <a-input-number v-model:value="formState.port" :min="1" :max="65535" style="width: 100%" />
            </a-form-item>
          </a-col>
        </a-row>
        <a-row :gutter="16">
          <a-col :span="12">
            <a-form-item label="用户名">
              <a-input v-model:value="formState.username" placeholder="可选" />
            </a-form-item>
          </a-col>
          <a-col :span="12">
            <a-form-item label="密码">
              <a-input-password v-model:value="formState.password" placeholder="可选" />
            </a-form-item>
          </a-col>
        </a-row>
        <a-form-item label="额外配置 (JSON)">
          <a-textarea 
            v-model:value="formState.extra_config" 
            placeholder='{"uuid": "xxx", "alterId": 0}'
            :rows="3"
          />
        </a-form-item>
      </a-form>
    </a-modal>

    <!-- 导入订阅弹窗 -->
    <a-modal
      v-model:open="showImportModal"
      title="导入订阅"
      @ok="handleImport"
      :confirmLoading="importLoading"
      width="600px"
    >
      <!-- 分组名称 -->
      <a-form layout="vertical" style="margin-bottom: 16px;">
        <a-form-item label="分组名称" required>
          <a-input
            v-model:value="importGroupName"
            placeholder="为导入的节点命名分组，方便管理"
            size="large"
          >
            <template #prefix>
              <FolderOutlined />
            </template>
          </a-input>
          <div class="import-hint">分组名称将应用到所有导入的节点</div>
        </a-form-item>
      </a-form>
      
      <a-tabs v-model:activeKey="importType">
        <a-tab-pane key="url" tab="订阅链接">
          <a-form layout="vertical">
            <a-form-item label="订阅 URL">
              <a-input
                v-model:value="importUrl"
                placeholder="https://example.com/subscribe"
                size="large"
              />
              <div class="import-hint">输入订阅链接，将自动获取并解析节点</div>
            </a-form-item>
          </a-form>
        </a-tab-pane>
        <a-tab-pane key="content" tab="订阅内容">
          <a-form layout="vertical">
            <a-form-item label="订阅内容">
              <a-textarea
                v-model:value="importContent"
                placeholder="粘贴 Base64 编码的订阅内容或节点链接（每行一个）"
                :rows="8"
                size="large"
              />
              <div class="import-hint">
                支持格式：vmess://, trojan://, ss://, http://, https://, socks5://
              </div>
            </a-form-item>
          </a-form>
        </a-tab-pane>
      </a-tabs>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import { message } from 'ant-design-vue'
import { proxyApi, type ProxyNode } from '@/api/proxy'
import dayjs from 'dayjs'
import {
  PlusOutlined,
  ThunderboltOutlined,
  EditOutlined,
  DeleteOutlined,
  CloudServerOutlined,
  CheckCircleOutlined,
  CloseCircleOutlined,
  QuestionCircleOutlined,
  ImportOutlined,
  FolderOutlined,
} from '@ant-design/icons-vue'

const loading = ref(false)
const submitLoading = ref(false)
const checkingId = ref<number | null>(null)
const checkingAll = ref(false)
const showModal = ref(false)
const showImportModal = ref(false)
const importLoading = ref(false)
const importType = ref('url')
const importUrl = ref('')
const importContent = ref('')
const importGroupName = ref('')
const nodes = ref<ProxyNode[]>([])
const editingNode = ref<ProxyNode | null>(null)
const selectedRowKeys = ref<number[]>([])
const batchDeleting = ref(false)

const formState = reactive({
  name: '',
  protocol: 'http',
  address: '',
  port: 80,
  username: '',
  password: '',
  extra_config: '',
})

const columns = [
  { title: '名称', dataIndex: 'name', key: 'name', width: 150 },
  { title: '分组', dataIndex: 'group_name', key: 'group_name', width: 120 },
  { title: '协议', dataIndex: 'protocol', key: 'protocol', width: 80 },
  { title: '地址', key: 'address', width: 180 },
  { title: '状态', key: 'status', width: 80 },
  { title: '延迟', key: 'latency', width: 80 },
  { title: '上次检测', key: 'last_check_at', width: 140 },
  { title: '操作', key: 'action', width: 120 },
]

const validCount = computed(() => nodes.value.filter(n => n.status === 'valid').length)
const invalidCount = computed(() => nodes.value.filter(n => n.status === 'invalid').length)
const unknownCount = computed(() => nodes.value.filter(n => n.status === 'unknown').length)

function formatDate(date: string) {
  return dayjs(date).format('YYYY-MM-DD HH:mm')
}

function getStatusColor(status: string) {
  switch (status) {
    case 'valid': return 'success'
    case 'invalid': return 'error'
    default: return 'default'
  }
}

function getStatusText(status: string) {
  switch (status) {
    case 'valid': return '可用'
    case 'invalid': return '不可用'
    default: return '未检测'
  }
}

function getLatencyClass(latency: number) {
  if (latency < 100) return 'latency-fast'
  if (latency < 300) return 'latency-medium'
  return 'latency-slow'
}

function openCreateModal() {
  editingNode.value = null
  Object.assign(formState, {
    name: '',
    protocol: 'http',
    address: '',
    port: 80,
    username: '',
    password: '',
    extra_config: '',
  })
  showModal.value = true
}

function openEditModal(node: ProxyNode) {
  editingNode.value = node
  Object.assign(formState, {
    name: node.name,
    protocol: node.protocol,
    address: node.address,
    port: node.port,
    username: node.username || '',
    password: node.password || '',
    extra_config: node.extra_config || '',
  })
  showModal.value = true
}

async function loadNodes() {
  loading.value = true
  try {
    const response = await proxyApi.listNodes()
    if (response.success && response.data) {
      nodes.value = response.data
    }
  } catch (error) {
    message.error('加载节点列表失败')
  } finally {
    loading.value = false
  }
}

async function handleSubmit() {
  if (!formState.name || !formState.address || !formState.port) {
    message.warning('请填写必要信息')
    return
  }

  submitLoading.value = true
  try {
    const data = {
      name: formState.name,
      protocol: formState.protocol,
      address: formState.address,
      port: formState.port,
      username: formState.username || undefined,
      password: formState.password || undefined,
      extra_config: formState.extra_config || undefined,
    }

    if (editingNode.value) {
      const response = await proxyApi.updateNode(editingNode.value.id, data)
      if (response.success) {
        message.success('节点更新成功')
        showModal.value = false
        await loadNodes()
      } else {
        message.error(response.message)
      }
    } else {
      const response = await proxyApi.createNode(data)
      if (response.success) {
        message.success('节点创建成功')
        showModal.value = false
        await loadNodes()
      } else {
        message.error(response.message)
      }
    }
  } catch (error) {
    message.error('操作失败')
  } finally {
    submitLoading.value = false
  }
}

async function handleDelete(id: number) {
  try {
    const response = await proxyApi.deleteNode(id)
    if (response.success) {
      message.success('节点删除成功')
      selectedRowKeys.value = selectedRowKeys.value.filter(key => key !== id)
      await loadNodes()
    } else {
      message.error(response.message)
    }
  } catch (error) {
    message.error('删除失败')
  }
}

function onSelectChange(keys: number[]) {
  selectedRowKeys.value = keys
}

async function handleBatchDelete() {
  if (selectedRowKeys.value.length === 0) {
    message.warning('请先选择要删除的节点')
    return
  }

  batchDeleting.value = true
  try {
    const response = await proxyApi.batchDeleteNodes(selectedRowKeys.value)
    if (response.success && response.data) {
      message.success(`成功删除 ${response.data.deleted} 个节点`)
      selectedRowKeys.value = []
      await loadNodes()
    } else {
      message.error(response.message)
    }
  } catch (error) {
    message.error('批量删除失败')
  } finally {
    batchDeleting.value = false
  }
}

async function handleCheck(id: number) {
  checkingId.value = id
  try {
    const response = await proxyApi.checkNode(id)
    if (response.success && response.data) {
      const result = response.data
      message.info(`${result.message}${result.latency ? ` (${result.latency}ms)` : ''}`)
      await loadNodes()
    } else {
      message.error(response.message)
    }
  } catch (error) {
    message.error('检测失败')
  } finally {
    checkingId.value = null
  }
}

async function handleCheckAll() {
  checkingAll.value = true
  try {
    const response = await proxyApi.checkAllNodes()
    if (response.success && response.data) {
      const results = response.data
      const validCount = results.filter(r => r.status === 'valid').length
      message.success(`检测完成：${validCount}/${results.length} 个节点可用`)
      await loadNodes()
    } else {
      message.error(response.message)
    }
  } catch (error) {
    message.error('批量检测失败')
  } finally {
    checkingAll.value = false
  }
}

async function handleImport() {
  if (!importGroupName.value.trim()) {
    message.warning('请输入分组名称')
    return
  }
  if (importType.value === 'url' && !importUrl.value) {
    message.warning('请输入订阅链接')
    return
  }
  if (importType.value === 'content' && !importContent.value) {
    message.warning('请输入订阅内容')
    return
  }

  importLoading.value = true
  try {
    const data = {
      group_name: importGroupName.value.trim(),
      ...(importType.value === 'url' 
        ? { url: importUrl.value }
        : { content: importContent.value })
    }
    
    const response = await proxyApi.importSubscription(data)
    if (response.success && response.data) {
      const result = response.data
      message.success(`导入完成：成功 ${result.success} 个，失败 ${result.failed} 个`)
      showImportModal.value = false
      importUrl.value = ''
      importContent.value = ''
      importGroupName.value = ''
      await loadNodes()
    } else {
      message.error(response.message)
    }
  } catch (error) {
    message.error('导入失败')
  } finally {
    importLoading.value = false
  }
}

onMounted(() => {
  loadNodes()
})
</script>

<style scoped>
.proxy-page {
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

.stats-row {
  margin-bottom: 24px;
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

.stat-icon.total {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: #fff;
}

.stat-icon.valid {
  background: linear-gradient(135deg, #52c41a 0%, #389e0d 100%);
  color: #fff;
}

.stat-icon.invalid {
  background: linear-gradient(135deg, #ff4d4f 0%, #cf1322 100%);
  color: #fff;
}

.stat-icon.unknown {
  background: linear-gradient(135deg, #faad14 0%, #d48806 100%);
  color: #fff;
}

.stat-value {
  font-size: 28px;
  font-weight: 600;
  color: #fff;
}

.stat-label {
  font-size: 14px;
  color: rgba(255, 255, 255, 0.45);
}

.table-card {
  padding: 24px;
}

.address-code {
  background: rgba(255, 255, 255, 0.1);
  padding: 2px 8px;
  border-radius: 4px;
  font-family: monospace;
  font-size: 12px;
}

.text-muted {
  color: rgba(255, 255, 255, 0.35);
}

.latency-fast {
  color: #52c41a;
  font-weight: 500;
}

.latency-medium {
  color: #faad14;
  font-weight: 500;
}

.latency-slow {
  color: #ff4d4f;
  font-weight: 500;
}

.import-hint {
  margin-top: 8px;
  font-size: 12px;
  color: rgba(255, 255, 255, 0.45);
}
</style>
