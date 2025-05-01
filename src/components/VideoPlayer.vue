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
import HlsPlugin ,{ EVENT }from 'xgplayer-hls'

 

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
      localStorage.setItem('videoPlaybackState', JSON.stringify({  index,  time  }))
      // 通知父组件更新当前索引
      this.$emit('update:currentIndex', index)
    },
    

 initPlayer(videoObj: typeof this.videos[0], index: number, seekTime?: number) { 
        
      if (this.player) {
        this.player.destroy()
      } 
      console.log(`initPlayer url:${videoObj.url}`);
      
      this.player = new Player({
        id: 'mse',
        url: videoObj.url,
        autoplay: true,
        fluid: true,
        lang: 'zh-cn',
        playbackRate: [0.5, 0.75, 1, 1.5, 2],
        plugins: [HlsPlugin],
        pip: true,
        hls: {
          retryCount: 3,
          maxRetryCount: 3
        }
      })

 
      // 监听播放器error事件
      this.player.on('error', (err: any) => {
        console.error('播放器发生错误:', err)
        this.$emit('stream-error', { type: 'player', error: err })
      })

 
        this.player.on('timeupdate', async () => {
                if (this.player) {
                  const currentTime = this.player.currentTime
                  const duration = this.player.duration 
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


          this.player.on('core_event', ({ eventName, ...rest }) => {
             
              if (eventName === EVENT.TTFB){ 
                let urls = [
                "https://svipsvip.ffzy-online5.com/20241014/33611_a776d3cf/2000k/hls/ab4462a5ee7fac19531097fc07f827b4.ts",
                "https://vip.lz-cdn14.com/20230706/26161_9c714caa/2000k/hls/c3177d5997c0368270.ts",
                "https://vip.lz-cdn14.com/20230713/26624_997d352d/2000k/hls/d7d9bb24b610588300.ts",
                "https://vip.lz-cdn14.com/20230720/27001_6739b38c/2000k/hls/c0d6c3790760798266.ts"
                // "https://svipsvip.ffzy-online5.com/20241014/33611_a776d3cf/2000k/hls/bee9924d3ec68d835f582a5afe6ea0ea.ts",
                // "https://svipsvip.ffzy-online5.com/20241014/33611_a776d3cf/2000k/hls/6dbb72bbcbaf8108d6dcb9b69b27e29b.ts",
                // "https://svipsvip.ffzy-online5.com/20241014/33611_a776d3cf/2000k/hls/ac6a88c91e57034ce35f4ca6f110aca1.ts",
                // "https://svipsvip.ffzy-online5.com/20241014/33611_a776d3cf/2000k/hls/93c38d342c4f690983e9d574432f9a75.ts",
                // "https://svipsvip.ffzy-online5.com/20241014/33611_a776d3cf/2000k/hls/e8b1eb709f7ff8bd0b33ed6c44b34fff.ts",
                // "https://svipsvip.ffzy-online5.com/20241014/33611_a776d3cf/2000k/hls/e157150337e26b123408429c546a9f24.ts",
                // "https://svipsvip.ffzy-online5.com/20241014/33611_a776d3cf/2000k/hls/9d12a3cbb49b62a640a114ed9c036054.ts"
                ]
               
                // ffmpeg -i "输入URL" -c copy -f null - 2> log.txt -----------
                // 非凡资源_广告
                let is_ffzy  = rest.url == urls[0] 
                if(is_ffzy){ 
                  setInterval(() => {
                    // 跳过广告 
                    this.player?.seek(this.player.currentTime+21)
                  },29000);
                }
                // 量子资源_广告
                let is_lziplayer  = (rest.url == urls[1]  || rest.url == urls[2] || rest.url == urls[3])
                // console.log(`当前的秒：${this.player.currentTime},方式广告：${is_lziplayer}`);
                // console.log(`当前的url：${rest.url}`); 
                if(is_lziplayer){ 
                  setInterval(() => {
                    // 跳过广告 
                    this.player?.seek(this.player.currentTime+22)
                  },28000);
                }


                 
              }
          })

         
    // -----------
    }, 
    changeVideo(index: number) {
      try {   
        console.log(`changeVideo ${JSON.stringify(this.videos[index])} `);
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
                      this.player?.play()
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
