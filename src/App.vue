<script setup lang="ts">
import { ref } from "vue"
import { invoke } from "@tauri-apps/api/core"
import { RouterView } from 'vue-router'
import { Window } from "@tauri-apps/api/window"
// @ts-ignore
import { Webview  } from "@tauri-apps/api/webview"
const appWindow = Window.getCurrent();
const greetMsg = ref("")
const name = ref("")
// @ts-ignore
async function greet() {
  greetMsg.value = await invoke("greet", { name: name.value })
}

 

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
</script>

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
                 shadow-card flex  z-50 
                 border-b justify-center space-x-4" 
    style="margin-top:2rem;background:rgba(30,32,38,0.95); border-bottom:1px solid #23272f;">
      <router-link  to="/"    class="flex justify-center items-center" >
          <VideoCamera class="text-primary mr-2" style="width: 1.5rem; height:1.5rem;" />
          <span class="font-semibold text-lg tracking-wide">影视资源库</span>
      </router-link>
     
     

      <router-link   to="/collect"  class="flex justify-center items-center"   >
        <Film class="text-primary mr-2" style="width: 1.5rem; height:1.5rem;" />
        <span class="font-semibold text-lg tracking-wide">片源更新</span> 
      </router-link>


      <router-link  to="/mypages"  class="flex justify-center items-center" >
        <Setting class="text-primary mr-2" style="width: 1.5rem; height:1.5rem;" />
        <span class="font-semibold text-lg tracking-wide">设置</span>
      </router-link>

    </nav>
     
   
    <!-- 内容区域 -->
    <main class="pt-28 p-8 max-w-7xl mx-auto">
      <RouterView />
    </main>
  </div>
</template>
<script lang="ts">
// @ts-ignore  
import {  VideoCamera, Film,Setting } from '@element-plus/icons-vue' 
</script>
<style>
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
