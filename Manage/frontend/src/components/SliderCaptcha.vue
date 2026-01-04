<template>
  <a-modal
    :open="visible" 
    :footer="null"
    :maskClosable="true"
    :closable="true"
    width="340px"
    @cancel="handleCancel"
    class="captcha-modal"
    :bodyStyle="{ padding: '20px' }"
  >
    <div class="captcha-title">安全验证</div>
    <div class="captcha-subtitle">拖动滑块完成拼图</div>
    
    <div class="slider-captcha" :class="{ verified: isVerified, failed: isFailed }">
      <div class="captcha-container">
        <!-- 背景图层 -->
        <div class="captcha-bg">
          <img :src="currentImage" class="bg-image" alt="verification background">
          
          <!-- 目标缺口 -->
          <div class="target-slot" :style="{ left: `${targetPosition}px`, top: `${targetY}px` }"></div>
          
          <!-- 拼图块 -->
          <div 
            class="puzzle-piece" 
            :style="{ 
              left: `${sliderPosition}px`, 
              top: `${targetY}px`,
              backgroundPosition: `-${targetPosition}px -${targetY}px`
            }"
          >
            <!-- 拼图内容使用背景图的切片 -->
            <img :src="currentImage" :style="{ left: `-${targetPosition}px`, top: `-${targetY}px` }">
          </div>
          
          <div class="refresh-btn" @click="reset">
            <ReloadOutlined />
          </div>
        </div>
        
        <!-- 滑块轨道 -->
        <div class="slider-track-container">
          <div class="slider-track">
            <div class="slider-fill" :style="{ width: `${sliderPosition}px` }"></div>
            <div class="slider-text" v-if="!isVerified && !isDragging">向右滑动填充拼图</div>
            
            <!-- 滑块按钮 -->
            <div 
              class="slider-handle"
              :style="{ left: `${sliderPosition}px` }"
              @mousedown="startDrag"
              @touchstart="startDrag"
            >
              <template v-if="isVerified">
                <CheckOutlined />
              </template>
              <template v-else-if="isFailed">
                <CloseOutlined />
              </template>
              <template v-else>
                <ArrowRightOutlined />
              </template>
            </div>
          </div>
        </div>
      </div>
    </div>
  </a-modal>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { CheckOutlined, CloseOutlined, ArrowRightOutlined, ReloadOutlined } from '@ant-design/icons-vue'

const props = defineProps<{
  visible: boolean
}>()

const emit = defineEmits<{
  (e: 'update:visible', val: boolean): void
  (e: 'success'): void
  (e: 'fail'): void
}>()

// 随机背景图
const images = [
  'https://images.unsplash.com/photo-1579546929518-9e396f3cc809?ixlib=rb-1.2.1&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=340&q=80',
  'https://images.unsplash.com/photo-1557683316-973673baf926?ixlib=rb-1.2.1&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=340&q=80',
  'https://images.unsplash.com/photo-1557682250-33bd709cbe85?ixlib=rb-1.2.1&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=340&q=80',
  'https://images.unsplash.com/photo-1558591710-4b4a1ae0f04d?ixlib=rb-1.2.1&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=340&q=80'
]

const currentImage = ref(images[0])
const sliderPosition = ref(0)
const isDragging = ref(false)
const isVerified = ref(false)
const isFailed = ref(false)
const startX = ref(0)
const trackWidth = 300 // 轨道总宽
const handleWidth = 40 // 滑块宽度
const maxSlide = trackWidth - handleWidth

// 目标位置
const targetPosition = ref(0)
const targetY = ref(0)
const tolerance = 5 

watch(() => props.visible, (val) => {
  if (val) {
    reset()
  }
})

function handleCancel() {
  emit('update:visible', false)
}

function initTarget() {
  // 随机切换图片
  currentImage.value = images[Math.floor(Math.random() * images.length)]
  
  // 随机横坐标: 50px 到 maxSlide - 10px
  targetPosition.value = 50 + Math.floor(Math.random() * (maxSlide - 60))
  // 随机纵坐标: 10px 到 90px (图片高度140px)
  targetY.value = 10 + Math.floor(Math.random() * 80)
}

function startDrag(e: MouseEvent | TouchEvent) {
  if (isVerified.value) return
  
  isDragging.value = true
  isFailed.value = false
  
  if (e instanceof MouseEvent) {
    startX.value = e.clientX - sliderPosition.value
  } else {
    startX.value = e.touches[0].clientX - sliderPosition.value
  }
  
  document.addEventListener('mousemove', onDrag)
  document.addEventListener('mouseup', endDrag)
  document.addEventListener('touchmove', onDrag)
  document.addEventListener('touchend', endDrag)
}

function onDrag(e: MouseEvent | TouchEvent) {
  if (!isDragging.value) return
  
  let clientX: number
  if (e instanceof MouseEvent) {
    clientX = e.clientX
  } else {
    clientX = e.touches[0].clientX
  }
  
  let newPosition = clientX - startX.value
  
  // 限制范围
  newPosition = Math.max(0, Math.min(newPosition, maxSlide))
  sliderPosition.value = newPosition
}

function endDrag() {
  if (!isDragging.value) return
  
  isDragging.value = false
  
  document.removeEventListener('mousemove', onDrag)
  document.removeEventListener('mouseup', endDrag)
  document.removeEventListener('touchmove', onDrag)
  document.removeEventListener('touchend', endDrag)
  
  const diff = Math.abs(sliderPosition.value - targetPosition.value)
  
  if (diff <= tolerance) {
    isVerified.value = true
    sliderPosition.value = targetPosition.value
    setTimeout(() => {
      emit('success')
      emit('update:visible', false)
    }, 500)
  } else {
    isFailed.value = true
    emit('fail')
    
    setTimeout(() => {
      if (isFailed.value) {
        sliderPosition.value = 0
        isFailed.value = false
      }
    }, 500)
  }
}

function reset() {
  sliderPosition.value = 0
  isVerified.value = false
  isFailed.value = false
  initTarget()
}

// 暴露方法以便手动重置
defineExpose({ reset })
</script>

<style scoped>
.captcha-title {
  font-size: 18px;
  font-weight: 600;
  margin-bottom: 4px;
  text-align: center;
  color: #333;
}

.captcha-subtitle {
  font-size: 14px;
  color: #666;
  margin-bottom: 20px;
  text-align: center;
}

.captcha-container {
  position: relative;
  border-radius: 4px;
}

.captcha-bg {
  position: relative;
  width: 300px;
  height: 140px;
  background: #f0f2f5;
  border-radius: 4px;
  overflow: hidden;
  margin: 0 auto 20px;
  box-shadow: 0 2px 8px rgba(0,0,0,0.1);
}

.bg-image {
  width: 100%;
  height: 100%;
  object-fit: cover;
  display: block;
}

.target-slot {
  position: absolute;
  width: 40px;
  height: 40px;
  background: rgba(0,0,0,0.5);
  box-shadow: 0 0 5px rgba(255,255,255,0.5);
  z-index: 1;
}

.puzzle-piece {
  position: absolute;
  width: 40px;
  height: 40px;
  z-index: 2;
  box-shadow: 0 0 10px rgba(0,0,0,0.2);
  border: 1px solid rgba(255,255,255,0.6);
  overflow: hidden;
}

.puzzle-piece img {
  position: absolute;
  width: 300px; /* Must match container width */
  height: 140px; /* Must match container height */
  object-fit: cover;
}

.refresh-btn {
  position: absolute;
  top: 5px;
  right: 5px;
  width: 24px;
  height: 24px;
  background: rgba(0,0,0,0.3);
  color: #fff;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
  cursor: pointer;
  z-index: 5;
  transition: all 0.3s;
}

.refresh-btn:hover {
  background: rgba(0,0,0,0.6);
  transform: rotate(180deg);
}

.slider-track-container {
  width: 300px;
  margin: 0 auto;
}

.slider-track {
  position: relative;
  height: 40px;
  background: #f7f9fa;
  border: 1px solid #e4e7eb;
  border-radius: 4px;
  box-shadow: inset 0 2px 2px rgba(0,0,0,0.05);
}

.slider-fill {
  position: absolute;
  left: 0;
  top: 0;
  height: 100%;
  background: #d1e9ff;
  border: 1px solid #1890ff;
  border-radius: 4px 0 0 4px;
  transition: width 0.1s;
}

.verified .slider-fill {
  background: #dff6dd;
  border-color: #52c41a;
}

.failed .slider-fill {
  background: #ffccc7;
  border-color: #ff4d4f;
}

.slider-text {
  position: absolute;
  width: 100%;
  text-align: center;
  line-height: 38px;
  color: #666;
  font-size: 14px;
  pointer-events: none;
}

.slider-handle {
  position: absolute;
  top: -1px; /* Align border */
  width: 40px;
  height: 40px;
  background: #fff;
  border: 1px solid #e4e7eb;
  border-radius: 4px;
  box-shadow: 0 0 5px rgba(0,0,0,0.1);
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: grab;
  font-size: 16px;
  color: #666;
  z-index: 3;
  transition: background 0.2s;
}

.slider-handle:hover {
  background: #1890ff;
  border-color: #1890ff;
  color: #fff;
}

.slider-handle:active {
  cursor: grabbing;
}

.verified .slider-handle {
  background: #52c41a;
  border-color: #52c41a;
  color: #fff;
  cursor: default;
}

.failed .slider-handle {
  background: #ff4d4f;
  border-color: #ff4d4f;
  color: #fff;
}
</style>

<style>
/* Global styles for the captcha modal */
.captcha-modal .ant-modal-content {
  border-radius: 8px;
  overflow: hidden;
}
</style>
