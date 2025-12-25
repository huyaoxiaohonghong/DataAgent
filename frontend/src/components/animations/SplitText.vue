<template>
  <div class="split-text-container" ref="containerRef">
    <span
      v-for="(item, index) in elements"
      :key="index"
      :class="['split-text-element', { 'is-word': splitBy === 'word' }]"
      :style="getDelayStyle(index)"
    >
      {{ item }}
    </span>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, onMounted } from 'vue';

interface Props {
  text: string;
  splitBy?: 'char' | 'word';
  delay?: number;
  duration?: number;
  animationType?: 'fadeUp' | 'fadeIn' | 'scale' | 'blur';
}

const props = withDefaults(defineProps<Props>(), {
  splitBy: 'char',
  delay: 50,
  duration: 500,
  animationType: 'fadeUp'
});

const containerRef = ref<HTMLDivElement | null>(null);
const isVisible = ref(false);

const elements = computed(() => {
  if (props.splitBy === 'word') {
    return props.text.split(' ');
  }
  return props.text.split('');
});

const getDelayStyle = (index: number) => {
  return {
    '--delay': `${index * props.delay}ms`,
    '--duration': `${props.duration}ms`,
    animationPlayState: isVisible.value ? 'running' : 'paused'
  };
};

onMounted(() => {
  // 使用 IntersectionObserver 实现滚动触发动画
  const observer = new IntersectionObserver(
    (entries) => {
      entries.forEach((entry) => {
        if (entry.isIntersecting) {
          isVisible.value = true;
          observer.disconnect();
        }
      });
    },
    { threshold: 0.1 }
  );

  if (containerRef.value) {
    observer.observe(containerRef.value);
  }
});
</script>

<style scoped>
.split-text-container {
  display: inline;
}

.split-text-element {
  display: inline-block;
  opacity: 0;
  animation: var(--animation, fadeUp) var(--duration, 500ms) ease forwards;
  animation-delay: var(--delay, 0ms);
}

.split-text-element.is-word {
  margin-right: 0.3em;
}

@keyframes fadeUp {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

@keyframes scale {
  from {
    opacity: 0;
    transform: scale(0.5);
  }
  to {
    opacity: 1;
    transform: scale(1);
  }
}

@keyframes blur {
  from {
    opacity: 0;
    filter: blur(10px);
  }
  to {
    opacity: 1;
    filter: blur(0);
  }
}
</style>
