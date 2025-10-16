/**
 * Network client for connecting to the game server
 */

export class NetworkClient {
  constructor(serverAddress, username) {
    this.serverAddress = serverAddress;
    this.username = username;
    this.ws = null;
    this.connected = false;
    this.eventHandlers = new Map();
    this.keepAliveInterval = null;
  }

  /**
   * Connect to server
   */
  async connect() {
    return new Promise((resolve, reject) => {
      try {
        this.ws = new WebSocket(this.serverAddress);

        this.ws.onopen = () => {
          console.log('Connected to server');
          this.connected = true;
          
          // Send handshake
          this.send({
            type: 0x00, // HANDSHAKE
            data: {
              protocolVersion: '1.21.10',
              username: this.username
            }
          });

          // Send login
          this.send({
            type: 0x01, // LOGIN
            data: {
              username: this.username,
              uuid: this.generateUUID()
            }
          });

          // Start keep-alive
          this.keepAliveInterval = setInterval(() => {
            this.send({
              type: 0x09, // KEEP_ALIVE
              data: {
                keepAliveId: Date.now()
              }
            });
          }, 5000);

          resolve();
        };

        this.ws.onmessage = (event) => {
          this.handleMessage(event.data);
        };

        this.ws.onerror = (error) => {
          console.error('WebSocket error:', error);
          reject(error);
        };

        this.ws.onclose = () => {
          console.log('Disconnected from server');
          this.connected = false;
          if (this.keepAliveInterval) {
            clearInterval(this.keepAliveInterval);
          }
          this.emit('disconnect');
        };

      } catch (error) {
        reject(error);
      }
    });
  }

  /**
   * Handle incoming message
   */
  handleMessage(data) {
    try {
      const packet = JSON.parse(data);
      
      switch (packet.type) {
        case 0x10: // DISCONNECT
          console.log('Disconnected:', packet.data.reason);
          this.emit('disconnect', packet.data);
          break;
        case 0x11: // CHUNK_DATA
          this.emit('chunkData', packet.data);
          break;
        case 0x12: // BLOCK_CHANGE
          this.emit('blockChange', packet.data);
          break;
        case 0x13: // MULTI_BLOCK_CHANGE
          this.emit('multiBlockChange', packet.data);
          break;
        case 0x14: // SPAWN_PLAYER
          this.emit('spawnPlayer', packet.data);
          break;
        case 0x15: // DESTROY_ENTITIES
          this.emit('destroyEntities', packet.data);
          break;
        case 0x18: // ENTITY_POSITION_AND_LOOK
          this.emit('playerPosition', packet.data);
          break;
        case 0x1A: // CHAT_MESSAGE_SERVER
          this.emit('chatMessage', packet.data);
          break;
        case 0x1B: // TIME_UPDATE
          this.emit('timeUpdate', packet.data);
          break;
        case 0x1C: // WORLD_INFO
          this.emit('worldInfo', packet.data);
          break;
        case 0x1D: // SERVER_KEEP_ALIVE
          // Respond to keep-alive
          this.send({
            type: 0x09,
            data: packet.data
          });
          break;
      }
    } catch (error) {
      console.error('Error handling message:', error);
    }
  }

  /**
   * Send packet to server
   */
  send(packet) {
    if (this.ws && this.ws.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify({
        ...packet,
        timestamp: Date.now()
      }));
    }
  }

  /**
   * Send player position
   */
  sendPosition(x, y, z, onGround) {
    this.send({
      type: 0x02, // PLAYER_POSITION
      data: { x, y, z, onGround }
    });
  }

  /**
   * Send player look
   */
  sendLook(yaw, pitch, onGround) {
    this.send({
      type: 0x03, // PLAYER_LOOK
      data: { yaw, pitch, onGround }
    });
  }

  /**
   * Send block break
   */
  breakBlock(x, y, z) {
    this.send({
      type: 0x05, // PLAYER_DIGGING
      data: {
        x, y, z,
        face: 0,
        status: 2 // Finished digging
      }
    });
  }

  /**
   * Send block placement
   */
  placeBlock(x, y, z, blockId) {
    this.send({
      type: 0x06, // PLAYER_BLOCK_PLACEMENT
      data: {
        x, y, z,
        face: 1, // Top
        blockId
      }
    });
  }

  /**
   * Send chat message
   */
  sendChatMessage(message) {
    this.send({
      type: 0x07, // CHAT_MESSAGE
      data: { message }
    });
  }

  /**
   * Register event handler
   */
  on(event, handler) {
    if (!this.eventHandlers.has(event)) {
      this.eventHandlers.set(event, []);
    }
    this.eventHandlers.get(event).push(handler);
  }

  /**
   * Emit event
   */
  emit(event, data) {
    const handlers = this.eventHandlers.get(event);
    if (handlers) {
      handlers.forEach(handler => handler(data));
    }
  }

  /**
   * Generate UUID
   */
  generateUUID() {
    return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, (c) => {
      const r = Math.random() * 16 | 0;
      const v = c === 'x' ? r : (r & 0x3 | 0x8);
      return v.toString(16);
    });
  }

  /**
   * Disconnect from server
   */
  disconnect() {
    if (this.ws) {
      this.ws.close();
    }
    if (this.keepAliveInterval) {
      clearInterval(this.keepAliveInterval);
    }
  }
}
