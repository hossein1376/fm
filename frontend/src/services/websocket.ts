class WebSocketService {
  private ws: WebSocket | null = null
  private reconnectTimeout: number = 5000
  private reconnectTimer: any = null

  connect() {
    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:'
    const wsUrl = `${protocol}//${window.location.host}/ws`

    this.ws = new WebSocket(wsUrl)

    this.ws.onopen = () => {
      console.log('WebSocket connected')
    }

    this.ws.onmessage = (event) => {
      try {
        const message = JSON.parse(event.data)
        console.log('WebSocket message:', message)
        // Emit custom event for components to listen
        window.dispatchEvent(new CustomEvent('ws-message', { detail: message }))
      } catch (e) {
        console.error('Failed to parse WebSocket message', e)
      }
    }

    this.ws.onerror = (error) => {
      console.error('WebSocket error:', error)
    }

    this.ws.onclose = () => {
      console.log('WebSocket disconnected')
      this.reconnect()
    }
  }

  disconnect() {
    if (this.reconnectTimer) {
      clearTimeout(this.reconnectTimer)
    }
    if (this.ws) {
      this.ws.close()
      this.ws = null
    }
  }

  reconnect() {
    if (this.reconnectTimer) {
      clearTimeout(this.reconnectTimer)
    }
    this.reconnectTimer = setTimeout(() => {
      console.log('Reconnecting WebSocket...')
      this.connect()
    }, this.reconnectTimeout)
  }

  send(message: any) {
    if (this.ws && this.ws.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify(message))
    } else {
      console.error('WebSocket is not connected')
    }
  }
}

export default new WebSocketService()
