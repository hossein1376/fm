import { defineStore } from 'pinia'
import { ref } from 'vue'
import api from '../services/api'

export const useHostStore = defineStore('host', () => {
  const hosts = ref<any[]>([])
  const currentHost = ref<any>(null)

  async function fetchHosts() {
    try {
      hosts.value = await api.getHosts()
    } catch (error) {
      console.error('Failed to fetch hosts:', error)
    }
  }

  async function createHost(hostData: any) {
    try {
      const newHost = await api.createHost(hostData)
      hosts.value.push(newHost)
      return true
    } catch (error) {
      console.error('Failed to create host:', error)
      return false
    }
  }

  async function deleteHost(id: string) {
    try {
      await api.deleteHost(id)
      hosts.value = hosts.value.filter(h => h.id !== id)
      return true
    } catch (error) {
      console.error('Failed to delete host:', error)
      return false
    }
  }

  function setCurrentHost(host: any) {
    currentHost.value = host
  }

  return {
    hosts,
    currentHost,
    fetchHosts,
    createHost,
    deleteHost,
    setCurrentHost
  }
})
