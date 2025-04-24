<template>
  <div class="min-h-screen bg-background text-white font-sans">
    <!-- 顶部信息栏  sticky top-0 z-30-->
    <div class="bg-background-light p-4 flex 
                  justify-between items-center 
                  border-b border-gray-800 
                  shadow-card ">
      <div class="flex items-center space-x-3">
        <VideoCamera class="text-primary" style="width: 2rem; height: 2rem;" />
        <span class="font-semibold text-lg tracking-wide">影视播放器</span>
      </div>
      <div class="text-gray-400 text-sm">
        <a href="http://ffzy5.tv/" target="_blank" class="hover:text-primary transition-colors underline underline-offset-2">ffzy5.tv</a>
      </div>
    </div>

    <!-- 主内容区 -->
    <div class="max-w-7xl mx-auto px-4 py-6">
      <!-- 视频播放器 -->
      <div class="mb-8 bg-black rounded-2xl overflow-hidden shadow-2xl border-2 border-background-card ring-2 ring-primary/10">
        <VideoPlayer 
          ref="videoPlayer"
          :videos="videos" 
          :currentIndex="currentVideoIndex"
          @update:currentIndex="(index: number) => currentVideoIndex = index"
          @update:updateVideoIndex="(index: number) => updateVideoIndex(index)"
        />
      </div>

      <!-- 视频设置表单 -->
      <div class="bg-background-card rounded-2xl p-8 mb-8 shadow-card border border-background-light">
        <h2 class="text-xl font-semibold mb-6 text-primary flex items-center">
          <Setting class="mr-2" style="width: 1.2rem; height: 1.2rem;" />
          视频设置
        </h2>
        <el-form class="space-y-8">
          <el-form-item label="视频URL列表" class="block">
            <div class="text-gray-400 text-sm mb-2">每行输入一个视频URL</div>
            <el-input
              type="textarea"
              :rows="5"
              v-model="videoUrls"
              placeholder="https://example.com/video.m3u8"
              @change="updateVideos"
              class="w-full bg-background-light border-background-card text-gray-200 focus:ring-primary focus:border-primary"
            />
          </el-form-item>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
            <el-form-item label="片头时间(秒)" class="block">
              <div class="flex items-center justify-between mb-1">
                <span class="text-gray-400 text-sm">跳过片头</span>
                <span class="text-primary font-medium">{{introTime}}秒</span>
              </div>
              <el-slider
                v-model="introTime"
                :max="300"
                show-input
                class="custom-slider"
              />
            </el-form-item>
            <el-form-item label="片尾时间(秒)" class="block">
              <div class="flex items-center justify-between mb-1">
                <span class="text-gray-400 text-sm">跳过片尾</span>
                <span class="text-primary font-medium">{{outroTime}}秒</span>
              </div>
              <el-slider
                v-model="outroTime"
                :max="300"
                show-input
                class="custom-slider"
              />
            </el-form-item>
          </div>
        </el-form>
      </div>

      <!-- 视频列表 -->
      <div class="mb-8">
        <div class="flex items-center justify-between mb-6">
          <h2 class="text-xl font-semibold text-primary flex items-center">
            <Film class="mr-2" style="width: 1.2rem; height: 1.2rem;" />
            视频列表 <span class="ml-2 text-sm text-gray-400">({{videos.length}}个视频)</span>
          </h2>
        </div>
        <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-6">
          <div 
            v-for="(video, index) in videos" 
            :key="index"
            @click="playVideo(index)"
            :class="[
              'group relative p-4 border rounded-2xl cursor-pointer transition-all duration-300 shadow-card',
              'hover:bg-primary/10 hover:border-primary hover:scale-105 transform',
              videoIndex === index 
                ? 'bg-gradient-to-br from-primary/20 to-primary/5 border-primary ring-2 ring-primary/40' 
                : 'bg-background-card border-background-light'
            ]"
          >
            <!-- 视频缩略图区域 --> 
            <div class="aspect-video bg-background-light rounded-lg mb-3 flex items-center justify-center overflow-hidden shadow-inner">
              <VideoCamera class="text-primary" style="width: 2rem; height: 2rem;" />
            </div>
            <div class="space-y-2">
              <h3 class="font-medium text-gray-200 group-hover:text-primary truncate">
                视频 {{ index + 1 }}
              </h3>
              <p class="text-xs text-gray-500 truncate">
                ID: {{ video.url.split('/')?.slice(-2, -1)[0] }}
              </p>
              <div class="pt-1">
                <span 
                  v-if="videoIndex === index" 
                  class="inline-flex items-center px-2 py-1 text-xs font-medium rounded-full bg-primary/20 text-primary animate-pulse"
                >
                  <span class="w-1.5 h-1.5 bg-primary rounded-full mr-1 animate-ping"></span>
                  正在播放
                </span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, watchEffect } from 'vue'
import { ElInput, ElSlider, ElForm, ElFormItem } from 'element-plus'
import { VideoCamera, Setting, Film } from '@element-plus/icons-vue'
import VideoPlayer from '@/components/VideoPlayer.vue' 

export default defineComponent({
  name: 'HomePage',
  components: {
    VideoPlayer,
    ElInput,
    ElSlider,
    ElForm,
    ElFormItem
  },
  setup() {
    // 从localStorage读取缓存
    const loadCache = (key: string, defaultValue: any) => {
      try {
        const cached = localStorage.getItem(key)
        return cached ? JSON.parse(cached) : defaultValue
      } catch {
        return defaultValue
      }
    }

    const videoPlayer = ref<typeof VideoPlayer>()
    const videoUrls = ref(loadCache('videoUrls', ``))

    const introTime = ref(loadCache('introTime', 0))
    const outroTime = ref(loadCache('outroTime', 0))
    const currentVideoIndex = ref(loadCache('currentVideoIndex', 0))
    const videoIndex = ref(loadCache('videoIndex', 0))
    const videos = ref<Array<{url: string, intro: number, outro: number}>>([])

    const updateVideos = () => {
      const newVideos = videoUrls.value 
        .split('\n')
        .filter((url: string) => url.trim())
        .map((url: string) => ({
          url: url.trim(),
          intro: introTime.value,
          outro: outroTime.value
        }))


      
      videos.value = newVideos


      videoPlayer.value?.updateVideos(newVideos)
    }

    const updateVideoIndex = (index: number) => {
      // currentVideoIndex.value = index
      videoIndex.value = index
      console.log( 'videoIndex', videoIndex.value);
      
    }
    // 初始化videos
    updateVideos()

    // 监听数据变化并更新缓存
    watchEffect(() => {
      localStorage.setItem('videoUrls', JSON.stringify(videoUrls.value))
      localStorage.setItem('introTime', JSON.stringify(introTime.value))
      localStorage.setItem('outroTime', JSON.stringify(outroTime.value))
      localStorage.setItem('currentVideoIndex', JSON.stringify(currentVideoIndex.value)) 
      updateVideos()
    })

    const playVideo = (index: number) => {
      currentVideoIndex.value = index
      videoIndex.value = index
      videoPlayer.value?.changeVideo(index)
    }



    return {
      videoUrls,
      introTime,
      outroTime,
      currentVideoIndex,
      videoIndex,
      videos,
      videoPlayer,
      updateVideos,
      updateVideoIndex,
      playVideo
    }
  }
})
</script>

/*
https://svipsvip.ffzy-online5.com/20241014/33610_ef152a79/index.m3u8
https://svipsvip.ffzy-online5.com/20241014/33611_a776d3cf/index.m3u8
https://svipsvip.ffzy-online5.com/20241014/33612_804d1c03/index.m3u8
https://svipsvip.ffzy-online5.com/20241014/33613_d957c740/index.m3u8
https://svipsvip.ffzy-online5.com/20241014/33614_0d8a8951/index.m3u8
*/