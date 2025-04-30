<template>
  <div class=" bg-background text-white font-sans ">
     <div>
       <div v-for="(vodTypItem,index) in vodTypes" :key="index" >
          
        <div >
            <h2 class="text-xl font-semibold  flex items-center">
              <Film class="mr-2 text-[#FF4c4c] " style="width: 1.2rem; height: 1.2rem;" />
              <span class="text-[#FF4c4c]">{{vodTypItem.title}}  </span>
            </h2>  
          </div>

           <div>
                <div v-for="(item) in vodTypItem.list" :key="item.id" class="border-gray-800  bg-background-light   p-[1.2rem] h-16 mt-3 mb-3 border-b shadow-card rounded-lg"  >
                       <div  class="flex  "> 
                          <h2> {{item.name}} </h2> 
                          <div  class=" mx-4 flex-1">  
                             <div>
                              <el-progress 
                                           class="custom-el-progress"
                                          :percentage="item.progress.percent" 
                                          :stroke-width="18"  
                                          color="#FF4c4c"  
                                          :show-text="false"   
                                          :indeterminate="item.collecting"  striped   striped-flow    :duration="1.2"  />
                             </div>
                          </div> 
                          <div class="w-20 "> 
                                <button class="w-full  bg-primary text-white rounded-lg font-medium shadow-card hover:bg-primary/80 transition  disabled:opacity-60 disabled:cursor-not-allowed" 
                                :disabled="item.disabled" @click="startCollect(item,{parentName:vodTypItem.title,childid:item.id})" >
                                  {{ item.collecting ? '采集中...' : '开始采集' }}
                                </button>
                        </div> 
                       </div>
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
interface Progress {
  percent: number;
  current: number;
  total: number;
  message: string;
}
 
let unlisten: UnlistenFn | null = null;
// @ts-ignore
const progressAddress = ref<any>(null);

const vodTypes = ref<Array<any>>([
      { title: "动漫", list: [],  },
      { title: "电影", list: [], },
      { title: "电视剧", list:[]  },
      { title: "综艺", list: [] }
    ])


    const loadCache = (key: string, defaultValue: any) => {
        try {
          const cached = localStorage.getItem(key)
          return cached ? JSON.parse(cached) : defaultValue
        } catch {
          return defaultValue
        }
      }

const startCollect = async (params:any,data:any) => { 
  try { 
    progressAddress.value = data 
    localStorage.setItem('progressAddress', JSON.stringify(data))
    vodTypes.value.forEach((item:any) => {
      item.list.forEach(($item:any) => {
        $item.disabled  = true 
          if($item.id == data.childid){
            $item.collecting = true 
            $item.progress = { percent: 0, current: 0, total: 0, message: '准备采集...' }
          }else{
            $item.collecting = false 
            $item.progress = { percent: 0, current: 0, total: 0, message: '' }
          }
        })  

    }) 
 
    await invoke('crawl_ffzy',{ params });
  } catch (e) {
    // progress.value.message = '采集启动失败';
    // collecting.value = false;
  }
};

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


        vodTypes.value.forEach((item:any) => {
      item.list.forEach(($item:any) => {
        $item.disabled  = false 
        $item.collecting = false 
        $item.progress = { percent: 0, current: 0, total: 0, message: '' }
        })  

    }) 
     
      } catch (e) {
        console.log(e);
      }
    }
getVodTypes()

onMounted(async () => {
  
  unlisten = await listen<Progress>('ffzy_progress', (event) => {
    let payload = event.payload;
    let strs = String(payload).split("/");   


    progressAddress.value = loadCache('progressAddress', progressAddress.value)
    
    vodTypes.value.forEach((item:any) => {
      item.list.forEach(($item:any) => {
        $item.disabled  = true 
          if($item.id == progressAddress.value.childid){
            $item.collecting = true 
            $item.progress = {  percent: Number(strs[0]) / Number(strs[1]) * 100,  
                                current: Number(strs[0]),  
                                total: Number(strs[1]),   
                                message: '正在采集...'  
                              }
           if ($item.progress.percent >= 100){
               clear()
               throw new Error("采集完成"); 
           }       

          }else{
            $item.collecting = false 
            $item.progress = { percent: 0, current: 0, total: 0, message: '' }
          }
        })  

    }) 
    console.log(vodTypes.value);
    
    // if (progress.value.percent >= 100) {
    //   collecting.value = false;
    //   progress.value.message = '采集完成';
    // }
  });
});

const clear = () => {
  progressAddress.value = null
  localStorage.removeItem('progressAddress')
  vodTypes.value.forEach((item:any) => {
      item.list.forEach(($item:any) => {
        $item.disabled  = false
        $item.collecting = false
        $item.progress = { percent: 0, current: 0, total: 0, message: '' }
        }) 
  })
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