<template>
  <div class="min-h-screen bg-background text-white font-sans flex flex-col items-center justify-center">
    <div class="bg-background-light p-6 rounded-2xl shadow-card border border-background-card w-full max-w-lg mt-16">
      <h2 class="text-xl font-semibold mb-8 text-primary flex items-center justify-center">
        <Setting class="mr-2" style="width: 1.5rem; height: 1.5rem;" />
        采集页面
      </h2>
      <button class="w-full px-6 py-3 bg-primary text-white rounded-xl font-medium shadow-card hover:bg-primary/80 transition mb-8 text-lg disabled:opacity-60 disabled:cursor-not-allowed" :disabled="collecting" @click="startCollect">
        {{ collecting ? '采集中...' : '开始采集' }}
      </button>
      <div v-if="progress.total > 0" class="w-full">
        <div class="flex justify-between mb-2">
          <span class="text-gray-400 text-sm">进度: {{ progress.current }}/{{ progress.total }}</span>
          <span class="text-primary font-medium">{{ progress.percent.toFixed(1) }}%</span>
        </div>
        <el-progress
          :percentage="progress.percent"
          :stroke-width="18"
          color="#409EFF"
          :show-text="false"
          class="custom-el-progress"
          :indeterminate="collecting"
          striped
          striped-flow
          :duration="1.2"
        />
        <div class="mt-3 text-gray-400 text-sm text-center">{{ progress.message }}</div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen, UnlistenFn } from '@tauri-apps/api/event';
import { ElProgress } from 'element-plus';
import 'element-plus/es/components/progress/style/css';
// @ts-ignore
import { Setting } from '@element-plus/icons-vue';

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
  unlisten = await listen<Progress>('ffzy_progress', (event) => {
    let payload = event.payload;
    let strs = String(payload).split("/");
    progress.value = {
      percent: Number(strs[0]) / Number(strs[1]) * 100,
      current: Number(strs[0]),
      total: Number(strs[1]),
      message: '正在采集...'
    };
    if (progress.value.percent >= 100) {
      collecting.value = false;
      progress.value.message = '采集完成';
    }
  });
});

onUnmounted(() => {
  if (unlisten) unlisten();
});
</script>

<style scoped>
.custom-el-progress {
  margin-top: 8px;
  border-radius: 12px;
  background: #23272f;
  box-shadow: 0 2px 8px rgba(64,158,255,0.08);
}
.bg-background {
  background-color: #181a20;
}
.bg-background-light {
  background-color: #23272f;
}
.bg-background-card {
  background-color: #23272f;
}
.text-primary {
  color: #409EFF;
}
.shadow-card {
  box-shadow: 0 4px 24px 0 rgba(0,0,0,0.18);
}
.border-background-card {
  border-color: #23272f;
}
.border-background-light {
  border-color: #23272f;
}
.rounded-2xl {
  border-radius: 1rem;
}
</style>