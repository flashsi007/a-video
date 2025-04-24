<template>
  <div>
    <!-- 播放器容器 -->
    <div id="mse" class="w-full h-96 rounded-lg overflow-hidden"></div>
  </div>
</template>

<script lang="ts">
import { defineComponent  } from 'vue'
import Player from 'xgplayer'
import 'xgplayer/dist/index.min.css'
import HlsPlugin from 'xgplayer-hls'

export default defineComponent({
  name: 'VideoPlayer',
  props: {
    videos: {
      type: Array as () => Array<{
        url: string
        intro: number
        outro: number
      }>,
      required: true
    },
    currentIndex: {
      type: Number,
      default: 0
    }
  },
  
  data() {
    return {
      player: null as Player | null,
      currentVideoIndex: 0
    }
  },
  
  methods: {
    updateVideos(newVideos: typeof this.videos) {
      if (this.player && this.player.currentSrc) {
        const currentTime = this.player.currentTime
        const currentIndex = this.currentIndex
        this.initPlayer(newVideos[currentIndex], currentIndex, currentTime)
      }
    },
    
    loadPlaybackState() {
      const savedState = localStorage.getItem('videoPlaybackState')
      if (savedState) {
        try {
          const { index, time } = JSON.parse(savedState)
          if (index >= 0 && index < this.videos.length) {
            this.currentVideoIndex = index
            return { index, time }
          }
        } catch (e) {
          console.error('Failed to parse playback state', e)
        }
      }
      return null
    },
    
    savePlaybackState(index: number, time: number) {
      localStorage.setItem('videoPlaybackState', JSON.stringify({
        index,
        time
      }))
      // 通知父组件更新当前索引
      this.$emit('update:currentIndex', index)
    },
    
    initPlayer(videoObj: typeof this.videos[0], index: number, seekTime?: number) {
      if (this.player) {
        this.player.destroy()
      }
      
      this.player = new Player({
        id: 'mse',
        url: videoObj.url,
        autoplay: true,
        fluid: true,
        lang: 'zh-cn',
        playbackRate: [0.5, 0.75, 1, 1.5, 2],
        plugins: [HlsPlugin],
        pip: true,
        playNext: {
        urlList:  this.videos.map(video => video.url),
    },
        hls: {
          retryCount: 3,
          maxRetryCount: 3
        }
      })

      this.player.on('timeupdate', () => {
        if (this.player) {
          const currentTime = this.player.currentTime
          const duration = this.player.duration
          // @ts-ignore
          const currentSrc = this.player.currentSrc || ''
          
          this.savePlaybackState(index, currentTime)
          
     

          if (currentTime < videoObj.intro) {
            this.player.currentTime = videoObj.intro
          }
          else if (duration - currentTime < videoObj.outro) {
            if (this.currentVideoIndex < this.videos.length - 1) {
              this.playNext(this.currentVideoIndex + 1)
            }
          }
        }
      })

      this.player.on('ended', () => {
        if (this.currentVideoIndex < this.videos.length - 1) {
          this.playNext(this.currentVideoIndex + 1)
        } else {
          localStorage.removeItem('videoPlaybackState')
        }
      })

      if (seekTime) {
        this.player.once('canplay', () => {
          this.player?.seek(seekTime)
        })
      }
    },
    
    changeVideo(index: number) {
      try {   
         
         this.currentVideoIndex = index
         this.initPlayer(this.videos[index], index) 

        } catch (error) {
          console.error('视频切换失败:', error)
        } 
      },
      
      
      playNext(index: number ) {
        try { 
          this.currentVideoIndex = index
          console.log('当前播放的index：',index);

          
          const currentTime = this.player?.currentTime ||0 
          localStorage.setItem('videoPlaybackState', JSON.stringify({    index,   currentTime  }))

      // 通知父组件更新当前索引
      this.$emit('update:updateVideoIndex', index)

         this.player?.playNext({index,url: this.videos[index].url  }) 
      } catch (error) {
        console.error('视频切换失败:', error)
      }
    }
  },
  
  watch: {
    currentIndex(newVal: number) {
      this.playNext(newVal)
    }
  },
  
  mounted() {
    if (this.videos.length > 0) {
      const savedState = this.loadPlaybackState()
      const index = savedState?.index ?? this.currentIndex
      this.initPlayer(this.videos[index], index, savedState?.time)
    }
  }
})
</script>
