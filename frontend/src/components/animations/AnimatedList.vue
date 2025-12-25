<template>
  <div class="animated-list-container" ref="containerRef">
    <!-- 模式1: 传入 items 数组，使用 slot 渲染每个项 -->
    <TransitionGroup
      v-if="items && items.length > 0"
      :name="animationType"
      tag="div"
      class="animated-list-wrapper"
      :style="wrapperStyle"
      @before-enter="onBeforeEnter"
      @enter="onEnter"
      @leave="onLeave"
    >
      <div
        v-for="(item, index) in displayItems"
        :key="getItemKey(item, index)"
        class="animated-list-item"
        :data-index="index"
        :style="getItemStyle(index)"
      >
        <slot name="item" :item="item" :index="index">
          {{ item }}
        </slot>
      </div>
    </TransitionGroup>
    
    <!-- 模式2: 不传入 items，直接包裹子内容并添加入场动画 -->
    <Transition
      v-else
      :name="animationType"
      appear
      @before-enter="onBeforeEnterWrapper"
      @enter="onEnterWrapper"
    >
      <div v-if="isVisible" class="animated-list-content" :style="contentStyle">
        <slot></slot>
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue';
import gsap from 'gsap';

interface Props {
  items?: any[];
  itemKey?: string | ((item: any, index: number) => string | number);
  animationType?: 'fade' | 'slide' | 'scale' | 'slideUp' | 'slideRight' | 'blur';
  staggerDelay?: number;
  duration?: number;
  easing?: string;
  threshold?: number;
  once?: boolean;
  delay?: number; // 初始延迟
}

const props = withDefaults(defineProps<Props>(), {
  items: () => [],
  animationType: 'slideUp',
  staggerDelay: 80,
  duration: 0.5,
  easing: 'power2.out',
  threshold: 0.1,
  once: true,
  delay: 0
});

const containerRef = ref<HTMLDivElement | null>(null);
const isVisible = ref(false);
const hasAnimated = ref(false);

const displayItems = computed(() => {
  if (!isVisible.value && props.once && !hasAnimated.value) {
    return [];
  }
  return props.items;
});

const wrapperStyle = computed(() => ({
  '--stagger-delay': `${props.staggerDelay}ms`,
  '--duration': `${props.duration}s`,
}));

const contentStyle = computed(() => ({
  '--duration': `${props.duration}s`,
  '--delay': `${props.delay}ms`,
}));

const getItemKey = (item: any, index: number): string | number => {
  if (typeof props.itemKey === 'function') {
    return props.itemKey(item, index);
  }
  if (typeof props.itemKey === 'string' && item && typeof item === 'object') {
    return item[props.itemKey] ?? index;
  }
  return index;
};

const getItemStyle = (index: number) => ({
  '--item-index': index,
  transitionDelay: `${index * props.staggerDelay}ms`,
});

// GSAP 动画钩子 (用于列表项)
const onBeforeEnter = (el: Element) => {
  const htmlEl = el as HTMLElement;
  gsap.set(htmlEl, getInitialState());
};

const onEnter = (el: Element, done: () => void) => {
  const htmlEl = el as HTMLElement;
  const index = parseInt(htmlEl.dataset.index || '0');
  
  gsap.to(htmlEl, {
    ...getFinalState(),
    duration: props.duration,
    delay: props.delay / 1000 + index * (props.staggerDelay / 1000),
    ease: props.easing,
    onComplete: done
  });
};

const onLeave = (el: Element, done: () => void) => {
  const htmlEl = el as HTMLElement;
  
  gsap.to(htmlEl, {
    ...getInitialState(),
    duration: props.duration * 0.5,
    ease: props.easing,
    onComplete: done
  });
};

// GSAP 动画钩子 (用于整体包裹内容)
const onBeforeEnterWrapper = (el: Element) => {
  const htmlEl = el as HTMLElement;
  gsap.set(htmlEl, getInitialState());
};

const onEnterWrapper = (el: Element, done: () => void) => {
  const htmlEl = el as HTMLElement;
  
  gsap.to(htmlEl, {
    ...getFinalState(),
    duration: props.duration,
    delay: props.delay / 1000,
    ease: props.easing,
    onComplete: done
  });
};

const getInitialState = () => {
  switch (props.animationType) {
    case 'fade':
      return { opacity: 0 };
    case 'slide':
      return { opacity: 0, x: -30 };
    case 'slideUp':
      return { opacity: 0, y: 30 };
    case 'slideRight':
      return { opacity: 0, x: 30 };
    case 'scale':
      return { opacity: 0, scale: 0.8 };
    case 'blur':
      return { opacity: 0, filter: 'blur(10px)' };
    default:
      return { opacity: 0 };
  }
};

const getFinalState = () => {
  switch (props.animationType) {
    case 'fade':
      return { opacity: 1 };
    case 'slide':
      return { opacity: 1, x: 0 };
    case 'slideUp':
      return { opacity: 1, y: 0 };
    case 'slideRight':
      return { opacity: 1, x: 0 };
    case 'scale':
      return { opacity: 1, scale: 1 };
    case 'blur':
      return { opacity: 1, filter: 'blur(0px)' };
    default:
      return { opacity: 1 };
  }
};

onMounted(() => {
  const observer = new IntersectionObserver(
    (entries) => {
      entries.forEach((entry) => {
        if (entry.isIntersecting) {
          isVisible.value = true;
          hasAnimated.value = true;
          if (props.once) {
            observer.disconnect();
          }
        } else if (!props.once) {
          isVisible.value = false;
        }
      });
    },
    { threshold: props.threshold }
  );

  if (containerRef.value) {
    observer.observe(containerRef.value);
  }
});

// 监听 items 变化，触发动画
watch(() => props.items, () => {
  if (isVisible.value) {
    hasAnimated.value = true;
  }
}, { deep: true });
</script>

<style scoped>
.animated-list-container {
  width: 100%;
}

.animated-list-wrapper {
  position: relative;
}

.animated-list-item {
  will-change: transform, opacity;
}

.animated-list-content {
  width: 100%;
  will-change: transform, opacity;
}

/* CSS 过渡备用方案 */
.fade-enter-active,
.fade-leave-active {
  transition: opacity var(--duration, 0.5s) ease;
  transition-delay: var(--delay, 0ms);
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

.slide-enter-active,
.slide-leave-active {
  transition: all var(--duration, 0.5s) ease;
  transition-delay: var(--delay, 0ms);
}

.slide-enter-from {
  opacity: 0;
  transform: translateX(-30px);
}

.slide-leave-to {
  opacity: 0;
  transform: translateX(30px);
}

.slideUp-enter-active,
.slideUp-leave-active {
  transition: all var(--duration, 0.5s) ease;
  transition-delay: var(--delay, 0ms);
}

.slideUp-enter-from {
  opacity: 0;
  transform: translateY(30px);
}

.slideUp-leave-to {
  opacity: 0;
  transform: translateY(-30px);
}

.slideRight-enter-active,
.slideRight-leave-active {
  transition: all var(--duration, 0.5s) ease;
  transition-delay: var(--delay, 0ms);
}

.slideRight-enter-from {
  opacity: 0;
  transform: translateX(30px);
}

.slideRight-leave-to {
  opacity: 0;
  transform: translateX(-30px);
}

.scale-enter-active,
.scale-leave-active {
  transition: all var(--duration, 0.5s) ease;
  transition-delay: var(--delay, 0ms);
}

.scale-enter-from,
.scale-leave-to {
  opacity: 0;
  transform: scale(0.8);
}

.blur-enter-active,
.blur-leave-active {
  transition: all var(--duration, 0.5s) ease;
  transition-delay: var(--delay, 0ms);
}

.blur-enter-from,
.blur-leave-to {
  opacity: 0;
  filter: blur(10px);
}
</style>
