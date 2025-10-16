// Client-side code for the Minecraft clone
const { io } = require('socket.io-client');
const VoxelEngine = require('./voxel');
const { World } = require('../shared/world');

class GameClient {
  constructor() {
    this.socket = null;
    this.myPlayer = null;
    this.otherPlayers = {};
    this.world = new World();
    this.voxelEngine = new VoxelEngine();
    this.voxelEngine.animate();
    this.isConnected = false;

    // Make this available globally for UI
    window.gameClient = this;
  }

  connectToServer(address) {
    if (this.socket) {
      this.socket.disconnect();
    }

    console.log('Connecting to:', address);
    this.socket = io(`http://${address}`);

    this.socket.on('connect', () => {
      console.log('Connected to server');
      this.isConnected = true;
      if (window.uiManager) {
        window.uiManager.enterGame('multiplayer', address);
      }
    });

    this.socket.on('connectionRejected', (data) => {
      console.error('Connection rejected:', data.reason);
      this.isConnected = false;
      if (window.uiManager) {
        window.uiManager.showConnectionError(data.reason);
      }
    });

    this.socket.on('worldData', (data) => {
      console.log('Received world data');
      // Load chunks into local world
      for (const [key, blocks] of Object.entries(data)) {
        const [x, z] = key.split(',').map(Number);
        const chunk = this.world.getChunk(x, z);
        chunk.blocks = blocks;
      }
      // TODO: Render the world
    });

    this.socket.on('playerJoined', (player) => {
      if (player.id === this.socket.id) {
        this.myPlayer = player;
      } else {
        this.otherPlayers[player.id] = player;
        console.log('Player joined:', player);
      }
    });

    this.socket.on('playerMoved', (data) => {
      if (this.otherPlayers[data.id]) {
        this.otherPlayers[data.id].position = data.position;
        // TODO: Update player position in scene
      }
    });

    this.socket.on('playerLeft', (playerId) => {
      delete this.otherPlayers[playerId];
      console.log('Player left:', playerId);
    });

    this.socket.on('blockUpdated', (data) => {
      this.world.setBlock(data.x, data.y, data.z, data.type);
      console.log('Block updated:', data);
      // TODO: Update rendering
    });

    this.socket.on('chatMessage', (data) => {
      if (window.uiManager) {
        window.uiManager.addChatMessage(`${data.player}: ${data.message}`);
      }
    });

    this.socket.on('disconnect', () => {
      console.log('Disconnected from server');
      this.isConnected = false;
      if (window.uiManager) {
        window.uiManager.disconnectFromServer();
      }
    });
  }

  enterGame(mode, serverAddress) {
    console.log('Entering game:', mode, serverAddress);
    // Game is now active, UI is handled by UIManager
  }

  disconnect() {
    if (this.socket) {
      this.socket.disconnect();
      this.socket = null;
    }
    this.isConnected = false;
    this.myPlayer = null;
    this.otherPlayers = {};
  }

  sendPlayerMove(position) {
    if (this.socket && this.isConnected) {
      this.socket.emit('playerMove', { position });
    }
  }

  sendBlockUpdate(x, y, z, type) {
    if (this.socket && this.isConnected) {
      this.socket.emit('blockUpdate', { x, y, z, type });
    }
  }

  sendChatMessage(message) {
    if (this.socket && this.isConnected) {
      this.socket.emit('chatMessage', { message });
    }
  }
}

// Initialize game client
const gameClient = new GameClient();

// Handle player movement simulation (for testing)
setInterval(() => {
  if (gameClient.myPlayer && gameClient.isConnected) {
    // Simulate movement
    gameClient.myPlayer.position.x += 0.01;
    gameClient.sendPlayerMove(gameClient.myPlayer.position);
  }
}, 100);