import axios, { AxiosInstance } from 'axios'

class ApiService {
  private api: AxiosInstance

  constructor() {
    this.api = axios.create({
      baseURL: '/api',
      headers: {
        'Content-Type': 'application/json'
      }
    })

    this.api.interceptors.request.use(config => {
      const token = localStorage.getItem('token')
      if (token) {
        config.headers.Authorization = `Bearer ${token}`
      }
      return config
    })
  }

  // Auth
  async register(username: string, password: string) {
    const response = await this.api.post('/auth/register', { username, password })
    return response.data
  }

  async login(username: string, password: string) {
    const response = await this.api.post('/auth/login', { username, password })
    return response.data
  }

  // Hosts
  async createHost(data: any) {
    const response = await this.api.post('/hosts', data)
    return response.data
  }

  async getHosts() {
    const response = await this.api.get('/hosts')
    return response.data
  }

  async getHost(id: string) {
    const response = await this.api.get(`/hosts/${id}`)
    return response.data
  }

  async deleteHost(id: string) {
    const response = await this.api.delete(`/hosts/${id}`)
    return response.data
  }

  // Files
  async browseFiles(hostId: string, path: string) {
    const response = await this.api.post('/files/browse', {
      host_id: hostId,
      path
    })
    return response.data
  }

  async downloadFile(hostId: string, path: string) {
    const response = await this.api.post('/files/download', {
      host_id: hostId,
      path
    }, {
      responseType: 'blob'
    })
    return response.data
  }

  async uploadFile(hostId: string, path: string, file: File) {
    const formData = new FormData()
    formData.append('host_id', hostId)
    formData.append('path', path)
    formData.append('file', file)

    const response = await this.api.post('/files/upload', formData, {
      headers: {
        'Content-Type': 'multipart/form-data'
      }
    })
    return response.data
  }

  async deleteFile(hostId: string, path: string) {
    const response = await this.api.post('/files/delete', {
      host_id: hostId,
      path
    })
    return response.data
  }

  async createDirectory(hostId: string, path: string) {
    const response = await this.api.post('/files/mkdir', {
      host_id: hostId,
      path
    })
    return response.data
  }
}

export default new ApiService()
