<script setup lang="ts">
import { ref } from "vue"
import { invoke } from "@tauri-apps/api/core"
import { RouterView } from 'vue-router'

const greetMsg = ref("")
const name = ref("")
// @ts-ignore
async function greet() {
  greetMsg.value = await invoke("greet", { name: name.value })
}
</script>

<template>
  <div class="min-h-screen">
    <!-- 顶部导航栏 -->
    <nav class="fixed top-0 left-0 right-0 bg-white shadow-md p-4 flex justify-center space-x-4 z-10">
      <router-link 
        to="/" 
        class="px-4 py-2 rounded hover:bg-gray-100 transition"
      >
        Home
      </router-link>
      <router-link 
        to="/mypages" 
        class="px-4 py-2 rounded hover:bg-gray-100 transition"
      >
        My Pages
      </router-link>
    </nav>

    <!-- 内容区域 -->
    <main class="pt-20 p-8">
      <RouterView />
    </main>
  </div>
</template>
