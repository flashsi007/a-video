<template>
  <div class=" bg-background text-white font-sans ">
    <div class='mb-5'>
      <button @click="clear"  class="w-full  bg-primary text-white  rounded-lg   font-medium shadow-card  hover:bg-primary/80 transition   disabled:opacity-60 disabled:cursor-not-allowed">
         重置
       </button>
    </div>
     <div> 
       <div v-for="(vodTypItem,index) in vodTypes" :key="index" >
          
        <div class="flex ">
            <h2 class="text-xl font-semibold  flex items-center">
              <VideoCamera class="mr-2 text-[#FF4c4c] " style="width: 1.2rem; height: 1.2rem;" />
              <span class="text-[#FF4c4c]">{{vodTypItem.title}}  </span>
            </h2>  
            <div class="flex-1 mx-5"> 
              <div>
                    <el-progress  
                        class="custom-el-progress"
                        text-inside
                       :percentage="vodTypItem.progress.percent" 
                       :stroke-width="18"  
                       color="#FF4c4c"  
                       :show-text="false"   
                       :indeterminate="vodTypItem.collecting"  striped   striped-flow    :duration="1.2"  />
                  </div>
            </div>

            <div class="w-32 mx-5">
              <button class="w-full  bg-primary text-white rounded-lg font-medium shadow-card hover:bg-primary/80 transition  disabled:opacity-60 disabled:cursor-not-allowed" 
                                :disabled="vodTypItem.disabled" @click="startCollect(vodTypItem )" >
                                  {{ vodTypItem.collecting ? (vodTypItem.progress.total+'/'+vodTypItem.progress.current) : '开始采集' }}
                                </button>
            </div>

          </div>

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
 
let unlisten: UnlistenFn | null = null; 
interface vodType {
  title: string,
  status: number,
  collecting: boolean,
  disabled: boolean,
  progress: {
    percent: number,
    current: number,
    total: number,
    message: string
  },
}

const loadCache = (key: string, defaultValue: any) => {
        try {
          const cached = localStorage.getItem(key)
          return cached ? JSON.parse(cached) : defaultValue
        } catch {
          return defaultValue
        }
      }
const vodTypes = ref<vodType[]>([
  { title: "当天采集", status: 0,   collecting: false, disabled: false, progress: { percent: 0, current: 0, total: 0, message: '' } },
  { title: "一周采集", status: 1,   collecting: false, disabled: false, progress: { percent: 0, current: 0, total: 0, message: '' } },
  { title: "所有采集", status: 2,   collecting: false, disabled: false, progress: { percent: 0, current: 0, total: 0, message: '' } }
])

const vodType = ref<vodType>(vodTypes.value[0])


  

const startCollect = async (params:vodType ) => { 
  try { 
    vodType.value = params  
    localStorage.setItem('vodType', JSON.stringify(params))
     await invoke('crawl_ffzy',{ status:params.status });
  } catch (e) {  
    console.log(e);
    
  }
};

  

onMounted(async () => {
  unlisten = await listen('ffzy_progress', (event) => {
    const {current,message,percent,total} = event.payload as any
     vodType.value = loadCache('vodType', JSON.stringify(vodType.value))
     vodTypes.value.forEach((item) => {
      item.disabled = true
      if(item.title == vodType.value.title){
        item.collecting = true
        item.progress = {percent,current,total,message}
      }
     })

     // 采集完成
     if(current == total){
        clear()
     }


  })
})

const clear = () => {
  vodTypes.value = [
  { title: "当天采集", status: 0,   collecting: false, disabled: false, progress: { percent: 0, current: 0, total: 0, message: '' } },
  { title: "一周采集", status: 1,   collecting: false, disabled: false, progress: { percent: 0, current: 0, total: 0, message: '' } },
  { title: "所有采集", status: 2,   collecting: false, disabled: false, progress: { percent: 0, current: 0, total: 0, message: '' } }
]
    localStorage.removeItem('vodType')
    vodType.value = vodTypes.value[0]
}
onUnmounted(() => {
 
  if (unlisten) unlisten();
});
</script>

<style scoped>
.custom-el-progress {
  /* margin-top: 8px; */
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

