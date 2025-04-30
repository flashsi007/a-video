import { defineStore } from 'pinia'

export const useVideoStore = defineStore('video', {
  state: () => ({
    selectedVideo: null as { id: number; title: string; video_urls: string[] } | null,
  }),
  actions: {
    setSelectedVideo(video: { id: number; title: string; video_urls: string[] }) {
      this.selectedVideo = video
    },
  },
})