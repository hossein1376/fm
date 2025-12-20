import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import api from '../services/api'

export const useAuthStore = defineStore('auth', () => {
  const token = ref<string | null>(localStorage.getItem('token'))
  const user = ref<any>(null)

  const isAuthenticated = computed(() => !!token.value)

  async function login(username: string, password: string) {
    try {
      const response = await api.login(username, password)
      token.value = response.token
      user.value = response.user
      localStorage.setItem('token', response.token)
      return true
    } catch (error) {
      console.error('Login failed:', error)
      return false
    }
  }

  async function register(username: string, password: string) {
    try {
      const response = await api.register(username, password)
      token.value = response.token
      user.value = response.user
      localStorage.setItem('token', response.token)
      return true
    } catch (error) {
      console.error('Registration failed:', error)
      return false
    }
  }

  function logout() {
    token.value = null
    user.value = null
    localStorage.removeItem('token')
  }

  return {
    token,
    user,
    isAuthenticated,
    login,
    register,
    logout
  }
})
