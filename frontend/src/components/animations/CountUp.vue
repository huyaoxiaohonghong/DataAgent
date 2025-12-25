<template>
  <span class="count-up">{{ displayValue }}</span>
</template>

<script setup lang="ts">
import { ref, watch, onMounted } from 'vue';

interface Props {
  endValue: number;
  duration?: number;
  startValue?: number;
  decimals?: number;
  prefix?: string;
  suffix?: string;
  separator?: string;
}

const props = withDefaults(defineProps<Props>(), {
  duration: 2000,
  startValue: 0,
  decimals: 0,
  prefix: '',
  suffix: '',
  separator: ','
});

const currentValue = ref(props.startValue);
const isAnimating = ref(false);

const displayValue = ref('');

const formatNumber = (num: number): string => {
  const fixed = num.toFixed(props.decimals);
  const parts = fixed.split('.');
  parts[0] = parts[0].replace(/\B(?=(\d{3})+(?!\d))/g, props.separator);
  return props.prefix + parts.join('.') + props.suffix;
};

const easeOutQuart = (t: number): number => {
  return 1 - Math.pow(1 - t, 4);
};

const animate = () => {
  if (isAnimating.value) return;
  isAnimating.value = true;

  const startTime = performance.now();
  const startVal = props.startValue;
  const endVal = props.endValue;
  const duration = props.duration;

  const step = (currentTime: number) => {
    const elapsed = currentTime - startTime;
    const progress = Math.min(elapsed / duration, 1);
    const easedProgress = easeOutQuart(progress);
    
    currentValue.value = startVal + (endVal - startVal) * easedProgress;
    displayValue.value = formatNumber(currentValue.value);

    if (progress < 1) {
      requestAnimationFrame(step);
    } else {
      isAnimating.value = false;
      currentValue.value = endVal;
      displayValue.value = formatNumber(endVal);
    }
  };

  requestAnimationFrame(step);
};

onMounted(() => {
  displayValue.value = formatNumber(props.startValue);
  
  // 使用 IntersectionObserver 实现滚动触发
  const observer = new IntersectionObserver(
    (entries) => {
      entries.forEach((entry) => {
        if (entry.isIntersecting) {
          animate();
          observer.disconnect();
        }
      });
    },
    { threshold: 0.1 }
  );

  const el = document.querySelector('.count-up');
  if (el) {
    observer.observe(el);
  }
});

watch(() => props.endValue, () => {
  animate();
});
</script>

<style scoped>
.count-up {
  font-variant-numeric: tabular-nums;
}
</style>
