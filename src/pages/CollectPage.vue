<template>
  <div class="collect-page flex flex-col items-center justify-center min-h-screen bg-gray-50">
    <h2 class="text-2xl font-bold mb-6">采集页面</h2>
    <button
      class="px-6 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition mb-8"
      :disabled="collecting"
      @click="startCollect"
    >
      {{ collecting ? '采集中...' : '开始采集' }}
    </button>
    <div v-if="progress.total > 0" class="w-full max-w-md">
      <div class="flex justify-between mb-1">
        <span>进度: {{ progress.current }}/{{ progress.total }}</span>
        <span>{{ progress.percent.toFixed(1) }}%</span>
      </div>
      <div class="w-full bg-gray-200 rounded-full h-4">
        <div
          class="bg-blue-500 h-4 rounded-full transition-all"
          :style="{ width: progress.percent + '%' }"
        ></div>
      </div>
      <div class="mt-2 text-gray-600">{{ progress.message }}</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen, UnlistenFn } from '@tauri-apps/api/event';

interface Progress {
  percent: number;
  current: number;
  total: number;
  message: string;
}

const collecting = ref(false);
const progress = ref<Progress>({ percent: 0, current: 0, total: 0, message: '' });
let unlisten: UnlistenFn | null = null;

const startCollect = async () => {
  collecting.value = true;
  progress.value = { percent: 0, current: 0, total: 0, message: '准备采集...' };
  try {
    await invoke('crawl_ffzy');
  } catch (e) {
    progress.value.message = '采集启动失败';
    collecting.value = false;
  }
};

onMounted(async () => {
  unlisten = await listen<Progress>('crawl-progress', (event) => {
    progress.value = event.payload;
    if (progress.value.percent >= 100) {
      collecting.value = false;
    }
  });
});

onUnmounted(() => {
  if (unlisten) unlisten();
});
</script>

<style scoped>
.collect-page {
  min-height: 100vh;
}
</style>