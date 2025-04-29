<template>
  <div class="min-h-screen bg-background text-white font-sans">
    <!-- 顶部信息栏 -->
    <div class=" border-gray-800  bg-background-light     border-b shadow-card rounded-lg">
        <div v-for="(item,index) in vodTypes" :key="index"> 
          <div class="flex space-x-2">
             <VideoCamera class="text-primary ml-2" style="width: 1.5rem; height: 1.5rem;" />
              <div>
                    <span>{{ item.title }} : </span>
                    <span v-for="vodTypeItem in item.list" :key="vodTypeItem.id">
                      <span @click="handleVodTypeClick(vodTypeItem.name,vodTypeItem.id)" 
                      :class="vodTypeItem.id== vodTypesItemId? 'vodTypeItem text-[1rem] mr-1 cursor-pointer text-primary':'vodTypeItem text-[1rem] mr-1 cursor-pointer'">
                        {{ vodTypeItem.name }}
                      </span>
                    </span>
              </div>
        </div> 
        </div>
        <div class=" text-gray-400 text-sm"></div>
      </div>
      <!-- 主内容区 -->
      <div class="max-w-7xl mx-auto px-4 py-6">
        <div class="mb-8">
          <div class="flex items-center justify-between mb-6">
            <h2 class="text-xl font-semibold text-primary flex items-center">
              <Film class="mr-2" style="width: 1.2rem; height: 1.2rem;" />
              视频列表 <span class="ml-2 text-sm text-gray-400">({{ videos.length }}个视频)</span>
            </h2>
            <div class="flex items-center space-x-2">
              <el-input v-model="searchKeyword" placeholder="搜索视频标题/演员/类型" clearable size="small"
                class="rounded-lg bg-background-light border border-background-card text-gray-200 focus:ring-primary focus:border-primary shadow-inner"
                style="width: 220px;" @keyup.enter.native="onSearch" />
              <el-button type="primary" size="small" @click="onSearch"
                class="rounded-lg px-5 py-2 font-medium shadow-card bg-primary hover:bg-primary/80 border-none">搜索</el-button>
            </div>
          </div>

          <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-3 lg:grid-cols-5 xl:grid-cols-5 gap-6">
            <div v-for="(video, index) in videos" @click="handleVodItem(video)" :key="video.vod_id || index" class="group relative p-4 border rounded-2xl cursor-pointer   transition-all duration-300  shadow-card hover:bg-primary/10 hover:border-primary hover:scale-105  transform bg-background-card border-background-light">
              <div
                class="aspect-video bg-background-light rounded-lg mb-3 flex items-center justify-center overflow-hidden shadow-inner w-full h-52">
                <img v-if="video.img_url" :src="video.img_url" alt="封面" class="object-cover w-full h-full" />
              </div>
              <div class="space-y-2">
                <h3 class="font-medium text-gray-200 group-hover:text-primary truncate">
                  {{ video.title }}
                </h3>
                <p class="text-xs text-gray-500 truncate">
                  类型: {{ video.type_name }}
                </p>
              </div>
            </div>
          </div>

          <div class="flex justify-center mt-8">
            <el-pagination @current-change="currentChange" background layout="prev, pager, next" :page-size="pageSize"
              :pager-count="7" :total="total"
              class="rounded-xl bg-background-card shadow-card px-6 py-3 border border-background-light text-gray-200"
              prev-text="上一页" next-text="下一页" />
          </div>
        </div>
      </div>
    </div>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted } from 'vue'
// @ts-ignore
import { VideoCamera, Setting, Film } from '@element-plus/icons-vue'
// @ts-ignore
import { invoke } from '@tauri-apps/api/core'
import { useRouter } from 'vue-router'




export default defineComponent({
  name: 'HomePage',
  components: {
    VideoCamera,
    Film
  },
  setup() {
    const videos = ref<Array<any>>([])
    const loading = ref(false)
    const currentPage = ref(1)
    const pageSize = ref(50)
    const total = ref(0)
    const searchKeyword = ref('')
    const vodTypes = ref<Array<any>>([
      { title: "动漫", list: [] },
      { title: "电影", list: [] },
      { title: "电视剧", list: [] },
      { title: "综艺", list: [] }
    ])
    const vodTypesItem = ref('')
    const vodTypesItemId = ref(null)
    const router = useRouter()
    
    const fetchVideos = async () => {
      loading.value = true
      try {
        let params: any = {
          page: currentPage.value,
          page_size: pageSize.value,
          keyword: searchKeyword.value
        }

        if (vodTypesItem.value) {
          params.type_name = (vodTypesItem.value).trim()
        }

        const result: any = await invoke('query_videos', {
          query: params
        })

        if (result && typeof result === 'object' && Array.isArray(result.data)) {
          videos.value = result.data
          total.value = result.total || 0
          pageSize.value = result.page_size || 50
          currentPage.value = result.pager_count || 1
        } else if (Array.isArray(result)) {
          videos.value = result
          total.value = result.length
        } else {
          videos.value = []
          total.value = 0
        }
      } catch (e) {
        videos.value = []
        total.value = 0
      } finally {
        loading.value = false
      }
    }

    const getVodTypes = async () => {
      try {
        let result: any = await invoke('get_vod_types')
        result = JSON.parse(result)


        for (let i = 0; i < result.length; i++) {
          let item = result[i]

          if (i <= 3) {
            vodTypes.value[0].list.push(item)
          }

          if (i > 3 && i <= 10) {
            vodTypes.value[1].list.push(item)
          }

          if (i > 10 && i <= 19) {
            vodTypes.value[2].list.push(item)
          }
          if (i > 19 && i <= 25) {
            vodTypes.value[3].list.push(item)
          }
        }
      } catch (e) {
        console.log(e);
      }
    }

    const handleVodItem = (video: any) => {
     let video_urls:string[] = video.video_urls .split(',')   
       let url_len = (video_urls[video_urls.length-1]).replace(/^["\[]+|["\]]+$/g, '');
      const match = video_urls[0].match(/(https?:\/\/[^"\s]+)/) || []; 
      const url = match[1];
        video_urls[0] = url;
        video_urls[video_urls.length-1] = url_len
        video_urls = video_urls.map((str)=>{
          let url = str.replace(/["\\]/g, '')
           if(url){
            return url
           } 
          return str
        })
 
        
 
      
      router.push({
        path: '/pla',
        query: {
          id: video.vod_id,
          title: video.title, 
          video_urls: video.video_urls
        }
      })  
    }

    const handleVodTypeClick = (type: string,id:any) => {
      currentPage.value = 1
      vodTypesItem.value = type
      vodTypesItemId.value = id
       fetchVideos()
    }

    const onPageChange = (page: number) => {
      currentPage.value = page
      fetchVideos()
    }

    const onSearch = () => {
      vodTypesItem.value = ""
      currentPage.value = 1
      fetchVideos()
    }

    const currentChange = (page: number) => {
      currentPage.value = page
      fetchVideos()
    }


    onMounted(() => {
      getVodTypes()
      fetchVideos()
    })
    return {
      videos,
      loading,
      currentPage,
      pageSize,
      total,
      searchKeyword,
      vodTypes,
      vodTypesItemId,
      handleVodItem,
      handleVodTypeClick,
      currentChange,
      onPageChange,
      getVodTypes,
      onSearch
    }
  }
})
</script>
<style>
.vodTypeItem:hover {
  color: #FF4c4c;
}
</style>