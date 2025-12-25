<template>
  <canvas ref="canvasRef" class="pixel-snow-canvas"></canvas>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue'

interface Props {
  // 雪花数量
  snowflakeCount?: number
  // 雪花颜色
  color?: string
  // 雪花最小尺寸
  minSize?: number
  // 雪花最大尺寸
  maxSize?: number
  // 下落速度
  speed?: number
  // 风力（水平偏移）
  wind?: number
  // 是否启用像素化效果
  pixelated?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  snowflakeCount: 150,
  color: '#ffffff',
  minSize: 2,
  maxSize: 6,
  speed: 1,
  wind: 0.5,
  pixelated: true
})

const canvasRef = ref<HTMLCanvasElement | null>(null)
let animationId: number | null = null
let snowflakes: Snowflake[] = []

interface Snowflake {
  x: number
  y: number
  size: number
  speed: number
  wind: number
  opacity: number
  wobble: number
  wobbleSpeed: number
}

function createSnowflake(canvas: HTMLCanvasElement, startFromTop = false): Snowflake {
  return {
    x: Math.random() * canvas.width,
    y: startFromTop ? -10 : Math.random() * canvas.height,
    size: Math.floor(Math.random() * (props.maxSize - props.minSize + 1)) + props.minSize,
    speed: (Math.random() * 0.5 + 0.5) * props.speed,
    wind: (Math.random() - 0.5) * props.wind,
    opacity: Math.random() * 0.5 + 0.5,
    wobble: 0,
    wobbleSpeed: Math.random() * 0.02 + 0.01
  }
}

function initSnowflakes(canvas: HTMLCanvasElement) {
  snowflakes = []
  for (let i = 0; i < props.snowflakeCount; i++) {
    snowflakes.push(createSnowflake(canvas, false))
  }
}

function drawPixelatedSnowflake(
  ctx: CanvasRenderingContext2D, 
  x: number, 
  y: number, 
  size: number, 
  opacity: number
) {
  ctx.globalAlpha = opacity
  
  // 像素化雪花图案 - 十字形或方形
  const pixelSize = Math.max(2, Math.floor(size / 2))
  
  if (size > 4) {
    // 大雪花 - 十字形像素图案
    // 中心
    ctx.fillRect(
      Math.floor(x / pixelSize) * pixelSize,
      Math.floor(y / pixelSize) * pixelSize,
      pixelSize,
      pixelSize
    )
    // 上
    ctx.fillRect(
      Math.floor(x / pixelSize) * pixelSize,
      Math.floor((y - pixelSize) / pixelSize) * pixelSize,
      pixelSize,
      pixelSize
    )
    // 下
    ctx.fillRect(
      Math.floor(x / pixelSize) * pixelSize,
      Math.floor((y + pixelSize) / pixelSize) * pixelSize,
      pixelSize,
      pixelSize
    )
    // 左
    ctx.fillRect(
      Math.floor((x - pixelSize) / pixelSize) * pixelSize,
      Math.floor(y / pixelSize) * pixelSize,
      pixelSize,
      pixelSize
    )
    // 右
    ctx.fillRect(
      Math.floor((x + pixelSize) / pixelSize) * pixelSize,
      Math.floor(y / pixelSize) * pixelSize,
      pixelSize,
      pixelSize
    )
  } else {
    // 小雪花 - 单个像素方块
    ctx.fillRect(
      Math.floor(x / pixelSize) * pixelSize,
      Math.floor(y / pixelSize) * pixelSize,
      pixelSize,
      pixelSize
    )
  }
  
  ctx.globalAlpha = 1
}

function drawNormalSnowflake(
  ctx: CanvasRenderingContext2D, 
  x: number, 
  y: number, 
  size: number, 
  opacity: number
) {
  ctx.globalAlpha = opacity
  ctx.beginPath()
  ctx.arc(x, y, size / 2, 0, Math.PI * 2)
  ctx.fill()
  ctx.globalAlpha = 1
}

function animate() {
  const canvas = canvasRef.value
  if (!canvas) return

  const ctx = canvas.getContext('2d')
  if (!ctx) return

  // 清除画布
  ctx.clearRect(0, 0, canvas.width, canvas.height)
  
  // 设置雪花颜色
  ctx.fillStyle = props.color

  // 更新和绘制雪花
  snowflakes.forEach((flake, index) => {
    // 更新位置
    flake.wobble += flake.wobbleSpeed
    flake.y += flake.speed
    flake.x += flake.wind + Math.sin(flake.wobble) * 0.5

    // 如果雪花超出画布底部，重置到顶部
    if (flake.y > canvas.height + 10) {
      snowflakes[index] = createSnowflake(canvas, true)
    }

    // 如果雪花超出画布左右两侧，从另一侧出现
    if (flake.x > canvas.width + 10) {
      flake.x = -10
    } else if (flake.x < -10) {
      flake.x = canvas.width + 10
    }

    // 绘制雪花
    if (props.pixelated) {
      drawPixelatedSnowflake(ctx, flake.x, flake.y, flake.size, flake.opacity)
    } else {
      drawNormalSnowflake(ctx, flake.x, flake.y, flake.size, flake.opacity)
    }
  })

  animationId = requestAnimationFrame(animate)
}

function resizeCanvas() {
  const canvas = canvasRef.value
  if (!canvas) return

  canvas.width = window.innerWidth
  canvas.height = window.innerHeight
  
  // 重新初始化雪花
  initSnowflakes(canvas)
}

onMounted(() => {
  const canvas = canvasRef.value
  if (!canvas) return

  resizeCanvas()
  window.addEventListener('resize', resizeCanvas)
  animate()
})

onUnmounted(() => {
  if (animationId) {
    cancelAnimationFrame(animationId)
  }
  window.removeEventListener('resize', resizeCanvas)
})

// 监听属性变化重新初始化
watch(() => [props.snowflakeCount, props.minSize, props.maxSize], () => {
  const canvas = canvasRef.value
  if (canvas) {
    initSnowflakes(canvas)
  }
})
</script>

<style scoped>
.pixel-snow-canvas {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
  z-index: 0;
}
</style>
