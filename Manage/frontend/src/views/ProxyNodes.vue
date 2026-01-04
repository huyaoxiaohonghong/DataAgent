<template>
  <div class="proxy-page fade-in">
    <div class="page-header">
      <div>
        <h1>配置管理</h1>
        <p class="page-subtitle">代理节点配置与测速</p>
      </div>
      <a-space>
        <a-popconfirm
          v-if="selectedNodeCount > 0"
          :title="`确定要删除选中的 ${selectedNodeCount} 个节点吗？`"
          @confirm="handleBatchDelete"
          ok-text="确定"
          cancel-text="取消"
        >
          <a-button danger :loading="batchDeleting">
            <template #icon><DeleteOutlined /></template>
            批量删除 ({{ selectedNodeCount }})
          </a-button>
        </a-popconfirm>
        <a-button 
          @click="handleCheckSelected" 
          :loading="checkingAll"
          :disabled="selectedNodeCount === 0"
        >
          <template #icon><ThunderboltOutlined /></template>
          {{ selectedNodeCount > 0 ? `检测 (${selectedNodeCount})` : '选择节点检测' }}
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
    <AnimatedList animationType="slideUp" :duration="0.5" :delay="0">
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
    </AnimatedList>

    <!-- 节点列表 -->
    <AnimatedList animationType="slideUp" :duration="0.6" :delay="150">
      <div class="glass-card table-card">
      <a-table
        :columns="columns"
        :data-source="groupedNodes"
        :loading="loading"
        :pagination="false"
        row-key="key"
        :row-selection="{ selectedRowKeys, onChange: onSelectChange, checkStrictly: false }"
        :default-expand-all-rows="true"
        :indent-size="20"
      >
        <template #bodyCell="{ column, record }">
          <!-- 名称列：分组行显示分组信息，节点行显示节点名称 -->
          <template v-if="column.key === 'name'">
            <template v-if="record.isGroup">
              <div class="group-row">
                <span class="group-name">
                  <FolderOutlined style="margin-right: 8px; color: #1890ff;" />
                  <strong>{{ record.name }}</strong>
                  <a-tag style="margin-left: 8px;">{{ record.nodeCount }} 个节点</a-tag>
                  <a-tag v-if="record.validCount > 0" color="success">{{ record.validCount }} 可用</a-tag>
                </span>
                <!-- 订阅信息显示 -->
                <span v-if="getGroupSubscriptionInfo(record.name)" class="subscription-info">
                  <a-tag color="blue">
                    剩余: {{ formatTraffic(getGroupSubscriptionInfo(record.name)!.total_bytes - getGroupSubscriptionInfo(record.name)!.upload_bytes - getGroupSubscriptionInfo(record.name)!.download_bytes) }}
                    / {{ formatTraffic(getGroupSubscriptionInfo(record.name)!.total_bytes) }}
                  </a-tag>
                  <a-tag :color="getGroupSubscriptionInfo(record.name)!.expire_timestamp && (getGroupSubscriptionInfo(record.name)!.expire_timestamp! * 1000 < Date.now()) ? 'red' : 'green'">
                    {{ formatExpireTime(getGroupSubscriptionInfo(record.name)!.expire_timestamp) }}
                  </a-tag>
                </span>
              </div>
            </template>
            <template v-else>
              {{ record.name }}
            </template>
          </template>
          
          <!-- 其他列：分组行不显示，节点行正常显示 -->
          <template v-if="column.key === 'protocol'">
            <span v-if="!record.isGroup">{{ record.protocol }}</span>
          </template>
          <template v-if="column.key === 'status'">
            <a-tag v-if="!record.isGroup" :color="getStatusColor(record.status)">
              {{ getStatusText(record.status) }}
            </a-tag>
          </template>
          <template v-if="column.key === 'latency'">
            <template v-if="!record.isGroup">
              <span v-if="record.latency" :class="getLatencyClass(record.latency)">
                {{ record.latency }}ms
              </span>
              <span v-else class="text-muted">-</span>
            </template>
          </template>
          <template v-if="column.key === 'address'">
            <code v-if="!record.isGroup" class="address-code">{{ record.address }}:{{ record.port }}</code>
          </template>
          <template v-if="column.key === 'last_check_at'">
            <template v-if="!record.isGroup">
              <span v-if="record.last_check_at">{{ formatDate(record.last_check_at) }}</span>
              <span v-else class="text-muted">从未检测</span>
            </template>
          </template>
          <template v-if="column.key === 'action'">
            <a-space v-if="!record.isGroup">
              <a-tooltip title="查看链接">
                <a-button type="text" size="small" @click="openLinkModal(record)">
                  <LinkOutlined />
                </a-button>
              </a-tooltip>
              <a-tooltip title="检测可用性">
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
    </AnimatedList>


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

    <!-- 查看链接弹窗 -->
    <a-modal
      v-model:open="showLinkModal"
      title="查看节点链接"
      :footer="null"
      width="520px"
    >
      <div v-if="currentViewNode" class="link-modal-content">
        <div class="link-info">
          <div class="link-label">节点名称</div>
          <div class="link-value">{{ currentViewNode.name }}</div>
        </div>
        
        <div class="link-info">
          <div class="link-label">协议类型</div>
          <div class="link-value">{{ currentViewNode.protocol.toUpperCase() }}</div>
        </div>

        <div class="link-info">
          <div class="link-label">订阅链接</div>
          <div class="link-box">
            <a-textarea
              :value="getNodeLink(currentViewNode)"
              :rows="3"
              readonly
              class="link-textarea"
            />
            <a-button type="primary" @click="copyNodeLink" style="margin-top: 8px;">
              <template #icon><CopyOutlined /></template>
              复制链接
            </a-button>
          </div>
        </div>

        <div class="link-info">
          <div class="link-label">扫码导入</div>
          <div class="qrcode-box">
            <canvas ref="qrcodeCanvas" class="qrcode-canvas"></canvas>
          </div>
        </div>
      </div>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, nextTick } from 'vue'
import { message } from 'ant-design-vue'
import { proxyApi, type ProxyNode, type SubscriptionInfo } from '@/api/proxy'
import dayjs from 'dayjs'
import QRCode from 'qrcode'
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
  LinkOutlined,
  CopyOutlined,
} from '@ant-design/icons-vue'
import AnimatedList from '@/components/animations/AnimatedList.vue'

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
const selectedRowKeys = ref<(string | number)[]>([])
const batchDeleting = ref(false)

// 链接查看弹窗相关
const showLinkModal = ref(false)
const currentViewNode = ref<ProxyNode | null>(null)
const qrcodeCanvas = ref<HTMLCanvasElement | null>(null)

// 订阅信息，按分组名称索引
const subscriptionInfoMap = ref<Map<string, SubscriptionInfo>>(new Map())

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
  { title: '名称', dataIndex: 'name', key: 'name', width: 200 },
  { title: '协议', dataIndex: 'protocol', key: 'protocol', width: 80 },
  { title: '地址', key: 'address', width: 180 },
  { title: '状态', key: 'status', width: 80 },
  { title: '延迟', key: 'latency', width: 80 },
  { title: '上次检测', key: 'last_check_at', width: 140 },
  { title: '操作', key: 'action', width: 120 },
]

// 分组节点接口
interface GroupNode {
  key: string
  name: string
  isGroup: boolean
  nodeCount: number
  validCount: number
  children: (ProxyNode & { key: number })[]
}

type TreeNode = GroupNode | (ProxyNode & { key: number })

// 将节点按分组转换为树形结构
const groupedNodes = computed<TreeNode[]>(() => {
  const groups = new Map<string, GroupNode>()
  
  for (const node of nodes.value) {
    const groupName = node.group_name || '未分组'
    
    if (!groups.has(groupName)) {
      groups.set(groupName, {
        key: `group-${groupName}`,
        name: groupName,
        isGroup: true,
        nodeCount: 0,
        validCount: 0,
        children: [],
      })
    }
    
    const group = groups.get(groupName)!
    group.nodeCount++
    if (node.status === 'valid') {
      group.validCount++
    }
    group.children.push({ ...node, key: node.id })
  }
  
  // 转换为数组并排序，未分组放最后
  return Array.from(groups.values()).sort((a, b) => {
    if (a.name === '未分组') return 1
    if (b.name === '未分组') return -1
    return a.name.localeCompare(b.name)
  })
})

const validCount = computed(() => nodes.value.filter(n => n.status === 'valid').length)
const invalidCount = computed(() => nodes.value.filter(n => n.status === 'invalid').length)
const unknownCount = computed(() => nodes.value.filter(n => n.status === 'unknown').length)
const selectedNodeCount = computed(() => selectedRowKeys.value.filter(key => typeof key === 'number').length)

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

// 格式化流量显示
function formatTraffic(bytes: number): string {
  if (bytes <= 0) return '0 B'
  const units = ['B', 'KB', 'MB', 'GB', 'TB']
  const exp = Math.min(Math.floor(Math.log(bytes) / Math.log(1024)), units.length - 1)
  return `${(bytes / Math.pow(1024, exp)).toFixed(2)} ${units[exp]}`
}

// 格式化剩余时间
function formatExpireTime(timestamp: number | undefined): string {
  if (!timestamp) return '永久'
  const now = Date.now() / 1000
  const diff = timestamp - now
  if (diff <= 0) return '已过期'
  
  const days = Math.floor(diff / 86400)
  if (days > 30) {
    const months = Math.floor(days / 30)
    return `${months} 个月`
  }
  return `${days} 天`
}

// 获取分组的订阅信息
function getGroupSubscriptionInfo(groupName: string): SubscriptionInfo | undefined {
  return subscriptionInfoMap.value.get(groupName)
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
    const [nodesResponse, subInfoResponse] = await Promise.all([
      proxyApi.listNodes(),
      proxyApi.listSubscriptionInfo()
    ])
    
    if (nodesResponse.success && nodesResponse.data) {
      nodes.value = nodesResponse.data
    }
    
    // 加载订阅信息并建立索引
    if (subInfoResponse.success && subInfoResponse.data) {
      const map = new Map<string, SubscriptionInfo>()
      for (const info of subInfoResponse.data) {
        map.set(info.group_name, info)
      }
      subscriptionInfoMap.value = map
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

function onSelectChange(keys: (string | number)[]) {
  selectedRowKeys.value = keys
}

// 获取实际选中的节点 ID（过滤掉分组 key）
function getSelectedNodeIds(): number[] {
  return selectedRowKeys.value.filter((key): key is number => typeof key === 'number')
}

async function handleBatchDelete() {
  const nodeIds = getSelectedNodeIds()
  if (nodeIds.length === 0) {
    message.warning('请先选择要删除的节点')
    return
  }

  batchDeleting.value = true
  try {
    const response = await proxyApi.batchDeleteNodes(nodeIds)
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

// 获取节点的原始链接
function getNodeLink(node: ProxyNode): string {
  // 优先使用 extra_config 中存储的原始链接
  if (node.extra_config) {
    // 如果 extra_config 是一个链接格式，直接返回
    if (node.extra_config.startsWith('vmess://') || 
        node.extra_config.startsWith('trojan://') || 
        node.extra_config.startsWith('ss://') ||
        node.extra_config.startsWith('http://') ||
        node.extra_config.startsWith('https://') ||
        node.extra_config.startsWith('socks')) {
      return node.extra_config
    }
    // 如果是 VMess JSON，需要构建完整链接
    if (node.protocol === 'vmess') {
      try {
        // extra_config 存储的是解码后的 JSON
        const encoded = btoa(node.extra_config)
        return `vmess://${encoded}`
      } catch {
        // 如果编码失败，返回原始配置
      }
    }
  }
  
  // 如果没有原始链接，根据协议构建基本链接
  if (node.protocol === 'http' || node.protocol === 'https') {
    const auth = node.username ? `${node.username}:${node.password || ''}@` : ''
    return `${node.protocol}://${auth}${node.address}:${node.port}`
  }
  if (node.protocol === 'socks5') {
    const auth = node.username ? `${node.username}:${node.password || ''}@` : ''
    return `socks5://${auth}${node.address}:${node.port}`
  }
  
  return `${node.protocol}://${node.address}:${node.port}`
}

// 打开链接查看弹窗
async function openLinkModal(node: ProxyNode) {
  currentViewNode.value = node
  showLinkModal.value = true
  
  // 等待 DOM 更新后生成二维码
  await nextTick()
  generateQRCode()
}

// 生成二维码
function generateQRCode() {
  if (!currentViewNode.value || !qrcodeCanvas.value) return
  
  const link = getNodeLink(currentViewNode.value)
  QRCode.toCanvas(qrcodeCanvas.value, link, {
    width: 200,
    margin: 2,
    color: {
      dark: '#000000',
      light: '#ffffff'
    }
  })
}

// 复制节点链接
function copyNodeLink() {
  if (!currentViewNode.value) return
  
  const link = getNodeLink(currentViewNode.value)
  navigator.clipboard.writeText(link).then(() => {
    message.success('链接已复制到剪贴板')
  }).catch(() => {
    message.error('复制失败，请手动复制')
  })
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

async function handleCheckSelected() {
  const nodeIds = getSelectedNodeIds()
  if (nodeIds.length === 0) {
    message.warning('请先选择要检测的节点')
    return
  }

  checkingAll.value = true
  try {
    let successCount = 0
    let validCount = 0
    
    // 逐个检测选中的节点
    for (const id of nodeIds) {
      try {
        const response = await proxyApi.checkNode(id)
        if (response.success && response.data) {
          successCount++
          if (response.data.status === 'valid') {
            validCount++
          }
        }
      } catch {
        // 单个节点检测失败，继续检测其他节点
      }
    }
    
    message.success(`检测完成：${validCount}/${successCount} 个节点可用`)
    await loadNodes()
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

.group-row {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.group-name {
  display: inline-flex;
  align-items: center;
}

.group-name strong {
  font-size: 14px;
  color: #fff;
}

.subscription-info {
  display: flex;
  align-items: center;
  gap: 4px;
  margin-left: 24px;
}

/* 链接查看弹窗样式 */
.link-modal-content {
  padding: 8px 0;
}

.link-info {
  margin-bottom: 20px;
}

.link-label {
  font-size: 12px;
  color: rgba(255, 255, 255, 0.45);
  margin-bottom: 8px;
}

.link-value {
  font-size: 14px;
  color: #fff;
}

.link-box {
  display: flex;
  flex-direction: column;
}

.link-textarea {
  font-family: monospace;
  font-size: 12px;
  word-break: break-all;
}

.qrcode-box {
  display: flex;
  justify-content: center;
  padding: 16px;
  background: #fff;
  border-radius: 8px;
}

.qrcode-canvas {
  display: block;
}
</style>
