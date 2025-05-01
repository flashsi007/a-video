

<template>
  <!-- -->
  <div class="min-h-screen  bg-background  text-white font-sans">
    <!-- Tauri自定义窗口栏 -->
 
    <div class="tauri-titlebar z-50  group 
                fixed top-0 left-0 right-0 
                h-8 flex items-center px-4  
                transition-opacity duration-300  
                group-hover:opacity-100 
                pointer-events-none 
                group-hover:pointer-events-auto" 
                style="background:rgba(30,32,38,0.95); border-bottom:1px solid #23272f; -webkit-app-region: drag;">
      <span class="text-primary font-bold tracking-widest text-sm select-none  ">A-VIDEO</span>
      <div class="ml-auto flex space-x-2 tauri-titlebar-buttons" style="-webkit-app-region: no-drag;">
        <button @click="minimizeWindow" title="最小化" class="w-7 h-7 flex items-center justify-center rounded hover:bg-gray-700 transition" style="-webkit-app-region: no-drag;">
          <svg width="14" height="14" viewBox="0 0 14 14"><rect x="3" y="7" width="8" height="2" rx="1" fill="#fff"/></svg>
        </button>
        <button @click="toggleMaximizeWindow" title="最大化/还原" class="w-7 h-7 flex items-center justify-center rounded hover:bg-gray-700 transition" style="-webkit-app-region: no-drag;">
          <svg width="14" height="14" viewBox="0 0 14 14"><rect x="3" y="3" width="8" height="8" rx="1" fill="#fff"/></svg>
        </button>
        <button @click="closeWindow" title="关闭" class="w-7 h-7 flex items-center justify-center rounded hover:bg-red-600 transition" style="-webkit-app-region: no-drag;">
          <svg width="14" height="14" viewBox="0 0 14 14"><line x1="4" y1="4" x2="10" y2="10" stroke="#fff" stroke-width="2"/><line x1="10" y1="4" x2="4" y2="10" stroke="#fff" stroke-width="2"/></svg>
        </button>
      </div>
    </div>
   
    <!-- 顶部导航栏 --> 
    <nav class="fixed top-0 left-0 right-0   
                 shadow-card flex  z-50  py-2
                 border-b justify-center space-x-4" 
    style="margin-top:2rem;background:rgba(30,32,38,0.95); border-bottom:1px solid #23272f;">
      <div    class="flex justify-center items-center cursor-pointer"  @click="toPage('/')" >
          <VideoCamera class="text-primary mr-2" style="width: 1.5rem; height:1.5rem;" />
          <span class="font-semibold text-lg tracking-wide">影视资源库</span>
      </div>
     
      <div    class="flex justify-center items-center cursor-pointer"  @click="toPage('/play')" >
          <VideoCamera class="text-primary mr-2" style="width: 1.5rem; height:1.5rem;" />
          <span class="font-semibold text-lg tracking-wide">播放</span>
      </div>


      <div     class="flex justify-center items-center cursor-pointer"   @click="toPage('/collect')">
        <Film class="text-primary mr-2" style="width: 1.5rem; height:1.5rem;" />
        <span class="font-semibold text-lg tracking-wide">片源更新</span> 
      </div>


      <div  class="flex justify-center items-center cursor-pointer " @click="toPage('/mypages')">
        <Setting class="text-primary mr-2" style="width: 1.5rem; height:1.5rem;" />
        <span class="font-semibold text-lg tracking-wide">设置</span>
      </div>

    </nav>
     
   
    <!-- 内容区域 --> 
    <main class="pt-28 p-8 max-w-7xl mx-auto"> 
        <RouterView />
    </main>
  </div>
</template>
 

<script setup lang="ts">  
import {useRouter} from "vue-router" 
const router = useRouter();

const toPage = (path:string) => {
  router.push(path)
}

 



import { Window } from "@tauri-apps/api/window" 
const appWindow = Window.getCurrent();

 

// 窗口控制方法
async function minimizeWindow() {
  await appWindow.minimize()
}
async function toggleMaximizeWindow() {
  const isMax = await appWindow.isMaximized()
  if (isMax) {
    await appWindow.unmaximize()
  } else {
    await appWindow.maximize()
  }
}
async function closeWindow() {
  await appWindow.close()
}
 
// onMounted(() => { 
  
 
// })

</script>
<style scoped>
.tauri-titlebar {
  /* opacity: 0; */
  pointer-events: auto;
  -webkit-app-region: drag;
}
.tauri-titlebar.group:hover,
.tauri-titlebar:focus-within {
  opacity: 1;
  pointer-events: auto;
}
.tauri-titlebar-buttons {
  -webkit-app-region: no-drag;
  pointer-events: auto;
}
</style>
