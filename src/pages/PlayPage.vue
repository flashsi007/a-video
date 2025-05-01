<template>
    <div class="min-h-screen bg-background text-white font-sans">
      <!-- 顶部信息栏  sticky top-0 z-30-->
      <div class="bg-background-light p-4 flex 
                    justify-between items-center 
                    border-b border-gray-800 
                    shadow-card ">
        <div class="flex items-center space-x-3">
          <VideoCamera class="text-primary" style="width: 2rem; height: 2rem;" />
          <span class="font-semibold text-lg tracking-wide">{{videoTitle}} 第 {{currentVideoIndex+1}} 集 </span>
        </div>
        <div class="text-gray-400 text-sm"></div>
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
            <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
              <el-form-item  class="block">
                <div class="  mb-1">
                  <span class="text-gray-400 text-sm mr-1">跳过片头</span>
                  <span class="text-primary font-medium">{{introTime}}秒</span>
                </div>
                <el-slider v-model="introTime" :max="300"  class="custom-slider"  />
              </el-form-item>
              <el-form-item   class="block">
                <div class=" mb-1">
                  <span class="text-gray-400 text-sm mr-1">跳过片尾</span>
                  <span class="text-primary font-medium">{{outroTime}}秒</span>
                </div>
                <el-slider  v-model="outroTime"  :max="300"   class="custom-slider"   />
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
            <!-- @vue-ignore -->
            <div  v-for="(video, index) in videos" 
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
                <!-- <p class="text-xs text-gray-500 truncate">
                  ID: {{ video.url.split('/')?.slice(-2, -1)[0] }}
                </p> -->
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
  
      <div v-if="false" >
          <a href="https://1080zyk4.com/" target="_blank" class="hover:text-primary transition-colors underline underline-offset-2">1080资源网</a>
          <hr/> 
          <a href="http://bdzy.com/" target="_blank" class="hover:text-primary transition-colors underline underline-offset-2">百度云资源采集网</a>
          <hr/> 
          <a href="http://www.tiankongzy.com/" target="_blank" class="hover:text-primary transition-colors underline underline-offset-2">天空资源采集网</a>
          <hr/> 
          <a href="https://huaweiba.live/" target="_blank" class="hover:text-primary transition-colors underline underline-offset-2">华为吧影视资源站</a>
          <hr/> 
          <a href="http://help.feisuzyapi.com/" target="_blank" class="hover:text-primary transition-colors underline underline-offset-2">飞速资源网</a>
          <hr/> 
          <a href="http://www.lzzy.tv" target="_blank" class="hover:text-primary transition-colors underline underline-offset-2">量子资源网</a>
          <hr/> 
          <a href="http://www.ffzy.tv" target="_blank" class="hover:text-primary transition-colors underline underline-offset-2">非凡资源网</a>
          <hr/> 
          <a href="http://www.wujinzy.net" target="_blank" class="hover:text-primary transition-colors underline underline-offset-2">无尽资源网</a>
          <hr/> 
          <a href="http://huyazy.net" target="_blank" class="hover:text-primary transition-colors underline underline-offset-2">虎牙资源网</a>
          <hr/> 
          <a href="https://www.xinlangzy.com" target="_blank" class="hover:text-primary transition-colors underline underline-offset-2">新浪资源网</a>
          <hr/> 
          <a href="https://360zy.vip/" target="_blank" class="hover:text-primary transition-colors underline underline-offset-2">360资源网</a>
          <hr/> 
          <a href="http://bdzy.com/" target="_blank" class="hover:text-primary transition-colors underline underline-offset-2">百度云资源</a>
          <hr/> 
          <a href="https://1080zyk1.com/" target="_blank" class="hover:text-primary transition-colors underline underline-offset-2">优质资源库</a>
          <hr/> 
          <a href="https://moduzy.cc/" target="_blank" class="hover:text-primary transition-colors underline underline-offset-2">魔都资源网</a>
          <hr/> 
          <a href="https://ikunzy.net/" target="_blank" class="hover:text-primary transition-colors underline underline-offset-2">ikun资源站</a>
          <hr/> 
          <a href="https://www.kuaiyunzy.com/" target="_blank" class="hover:text-primary transition-colors underline underline-offset-2">快云资源网</a>
          <hr/> 
          <a href="https://yszzq.leshizy.top/" target="_blank" class="hover:text-primary transition-colors underline underline-offset-2">乐视资源网</a>
          <hr/> 
          <a href="https://ikunzy.net/" target="_blank" class="hover:text-primary transition-colors underline underline-offset-2">ikun资源站</a>
          <hr/> 
          <a href="https://yycmszyw.com/" target="_blank" class="hover:text-primary transition-colors underline underline-offset-2">优优资源网</a>
          <hr/> 
          <a href="https://aosikazy1.com" target="_blank" class="hover:text-primary transition-colors underline underline-offset-2">奥斯卡资源站</a>
          <hr/> 
          <a href="https://gqzy.me/" target="_blank" class="hover:text-primary transition-colors underline underline-offset-2">搞起资源站</a>
          <hr/> 
          <a href="https://didizy.com/" target="_blank" class="hover:text-primary transition-colors underline underline-offset-2">滴滴资源站</a>
          <hr/> 
          <a href="https://www.taopianzy.com/" target="_blank" class="hover:text-primary transition-colors underline underline-offset-2">淘片资源网</a>
          <a href="https://taopianzy.vip" target="_blank" class="hover:text-primary transition-colors underline underline-offset-2">备用地址</a>
          <a href="https://taopianzy.net" target="_blank" class="hover:text-primary transition-colors underline underline-offset-2">备用地址</a>
          <hr/> 
          <a href="http://www.bbkdj.com/help" target="_blank" class="hover:text-primary transition-colors underline underline-offset-2">步步高升影视资源采集网站</a>
          <hr/> 
      </div>
  
    </div>
  </template>
  
  <script lang="ts"> 
   import {useRoute} from "vue-router" 
  import { defineComponent, ref, onMounted,watchEffect } from 'vue'
  import { ElInput, ElSlider, ElForm, ElFormItem } from 'element-plus'  
  import VideoPlayer from '@/components/VideoPlayer.vue'  
  export default defineComponent({
    name: 'PlayPage',
    components: {
      VideoPlayer,
      ElInput,
      ElSlider,
      ElForm,
      ElFormItem
    },
    setup() {
       const route = useRoute();
       let {title,video_urls} =  route.query 

       // 从localStorage读取缓存
       const loadCache = (key: string, defaultValue: any) => {
        try {
          const cached = localStorage.getItem(key)
          return cached ? JSON.parse(cached) : defaultValue
        } catch {
          return defaultValue
        }
      }

 
     
     

      const videoTitle = ref(loadCache('videoTitle', title))
      const videoPlayer = ref<typeof VideoPlayer>()
      const play_source = ref<Array<string> >(loadCache('play_source', video_urls)) 
      const introTime = ref(loadCache('introTime', 0))
      const outroTime = ref(loadCache('outroTime', 0))
      const currentVideoIndex = ref(loadCache('currentVideoIndex', 0))
      const videoIndex = ref(loadCache('videoIndex', 0))
      const videos = ref<Array<{url: string, intro: number, outro: number}>>([])
     
  
      
      onMounted(() => {  
        play_source.value = loadCache('play_source', video_urls)  as Array<string>
       videoTitle.value = title
        localStorage.setItem('play_source', JSON.stringify(play_source.value))
        localStorage.setItem('videoTitle', JSON.stringify(videoTitle.value))
         updateVideos()
         
      })


      const updateVideos = () => {
       
        
        const newVideos = play_source.value.map((url: string) => ({
            url: url.trim(),
            intro: introTime.value,
            outro: outroTime.value
          }))
        
        videos.value = newVideos 
        videoPlayer.value?.updateVideos(newVideos)
      }
  
      const updateVideoIndex = (index: number) => {
        currentVideoIndex.value = index
        videoIndex.value = index
      }
      // 初始化videos
      updateVideos()
  
      // 监听数据变化并更新缓存
      watchEffect(() => {
        localStorage.setItem('play_source', JSON.stringify(play_source.value))
        localStorage.setItem('introTime', JSON.stringify(introTime.value))
        localStorage.setItem('outroTime', JSON.stringify(outroTime.value))
        localStorage.setItem('videoTitle', JSON.stringify(videoTitle.value))
        localStorage.setItem('currentVideoIndex', JSON.stringify(currentVideoIndex.value)) 
        updateVideos()
      })
  
      const playVideo = (index: number) => {
        currentVideoIndex.value = index
        videoIndex.value = index
        videoPlayer.value?.changeVideo(index)
      }
  
  
  
      return {
        play_source,
        introTime,
        outroTime,
        currentVideoIndex,
        videoIndex,
        videos,
        videoPlayer,
        videoTitle,
        updateVideos,
        updateVideoIndex,
        playVideo
      }
    }
  })
  </script>

  <style>
   .el-slider__bar{
     background-color: #FF4C4C;
   }

   .el-slider__button{
    border: #FF4C4C solid 1px;  
   }
 </style>

 
  
 