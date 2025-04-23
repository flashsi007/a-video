<template>
  <div class="p-8 h-screen">
     <p class="p-2">
      http://ffzy5.tv/
     </p>

    <div class="mb-4">
    <VideoPlayer 
      ref="videoPlayer"
      :videos="videos" 
      :currentIndex="currentVideoIndex"
      @update:currentIndex="(index: number) => currentVideoIndex = index"
    />
    </div>

    
    <el-form>
      <el-form-item label="视频URL列表">
        <el-input
          type="textarea"
          :rows="5"
          v-model="videoUrls"
          placeholder="每行一个视频URL"
          @change="updateVideos"
        />
      </el-form-item>
      
      <el-form-item label="片头时间(秒)">
        <el-slider
          v-model="introTime"
          :max="300"
          show-input
        />
      </el-form-item>
      
      <el-form-item label="片尾时间(秒)">
        <el-slider
          v-model="outroTime"
          :max="300"
          show-input
        />
      </el-form-item>
    </el-form>

   

    <!-- 视频列表 -->
    <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-3 gap-3 lg:grid-cols-5 xl:grid-cols-6  p-4 bg-gray-50 rounded-xl mt-6">
      <div 
        v-for="(video, index) in videos" 
        :key="index"
        @click="playVideo(index)"
        :class="[
          'group relative p-4 border rounded-xl cursor-pointer transition-all duration-300 shadow-md',
          'hover:shadow-lg hover:-translate-y-1',
          currentVideoIndex === index 
            ? 'bg-gradient-to-br from-yellow-100 to-yellow-50 border-yellow-300 ring-2 ring-yellow-200' 
            : 'bg-white border-gray-200 hover:border-gray-300'
        ]"
      >
        <!-- 缩略图占位 --> 
        <div class="flex ">
           <div class="mr-2">
              <VideoCamera style="width: 1.5rem; height: 1.5rem;" />
           </div>
          <h3 class="font-medium text-gray-800 group-hover:text-blue-600">
              视频 {{ index + 1 }}
            </h3>
        </div>
        
        <div class="flex items-center justify-between">
          <div>
            <p class=" text-xs text-gray-500 mt-1">
              ID: {{ video.url.split('/').slice(-2, -1)[0] }}
            </p>
             
          </div> 
        </div>

        <div>
          <span v-if="currentVideoIndex === index" class="  py-1 text-xs font-medium rounded-full bg-yellow-100 text-yellow-800 animate-pulse">
            正在播放
          </span>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref,   watchEffect } from 'vue'
import { ElInput, ElSlider, ElForm, ElFormItem } from 'element-plus'
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
      videoPlayer.value?.changeVideo(index)
    }

    return {
      videoUrls,
      introTime,
      outroTime,
      currentVideoIndex,
      videos,
      videoPlayer,
      updateVideos,
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