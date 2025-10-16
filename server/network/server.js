/**
 * WebSocket server for multiplayer functionality
 */

const WebSocket = require('ws');
const { v4: uuidv4 } = require('uuid');
const { Packet, PacketBuilder, PacketType } = require('../../shared/protocol/packets');
const { PROTOCOL_VERSION, TICK_RATE } = require('../../shared/constants/game');

class GameServer {
  constructor(world, config) {
    this.world = world;
    this.config = config;
    this.players = new Map();
    this.nextEntityId = 1;
    this.tickInterval = null;
    this.lastKeepAlive = Date.now();
  }

  /**
   * Start the server
   */
  start() {
    this.wss = new WebSocket.Server({ 
      port: this.config.port,
      maxPayload: 1024 * 1024 // 1MB
    });

    console.log(`Server starting on port ${this.config.port}`);
    console.log(`Protocol version: ${PROTOCOL_VERSION}`);
    console.log(`Max players: ${this.config.maxPlayers}`);

    this.wss.on('connection', (ws) => this.handleConnection(ws));
    
    // Start game tick
    this.tickInterval = setInterval(() => this.tick(), 1000 / TICK_RATE);

    console.log('Server started successfully!');
  }

  /**
   * Handle new client connection
   */
  handleConnection(ws) {
    console.log('New connection attempt');

    let player = null;

    ws.on('message', (data) => {
      try {
        const packet = Packet.deserialize(data.toString());
        
        if (!player && packet.type !== PacketType.HANDSHAKE) {
          this.disconnect(ws, 'Invalid handshake');
          return;
        }

        this.handlePacket(ws, player, packet);
      } catch (error) {
        console.error('Error handling packet:', error);
      }
    });

    ws.on('close', () => {
      if (player) {
        console.log(`Player ${player.username} disconnected`);
        this.removePlayer(player);
      }
    });

    ws.on('error', (error) => {
      console.error('WebSocket error:', error);
    });
  }

  /**
   * Handle incoming packet
   */
  handlePacket(ws, player, packet) {
    switch (packet.type) {
      case PacketType.HANDSHAKE:
        this.handleHandshake(ws, packet);
        break;
      case PacketType.LOGIN:
        player = this.handleLogin(ws, packet);
        break;
      case PacketType.PLAYER_POSITION:
      case PacketType.PLAYER_LOOK:
      case PacketType.PLAYER_POSITION_AND_LOOK:
        this.handlePlayerMovement(player, packet);
        break;
      case PacketType.PLAYER_DIGGING:
        this.handleBlockBreak(player, packet);
        break;
      case PacketType.PLAYER_BLOCK_PLACEMENT:
        this.handleBlockPlace(player, packet);
        break;
      case PacketType.CHAT_MESSAGE:
        this.handleChatMessage(player, packet);
        break;
      case PacketType.KEEP_ALIVE:
        player.lastKeepAlive = Date.now();
        break;
      case PacketType.PLUGIN_MESSAGE:
        this.handlePluginMessage(player, packet);
        break;
    }
  }

  /**
   * Handle handshake
   */
  handleHandshake(ws, packet) {
    if (packet.data.protocolVersion !== PROTOCOL_VERSION) {
      this.disconnect(ws, `Protocol version mismatch. Server: ${PROTOCOL_VERSION}, Client: ${packet.data.protocolVersion}`);
      return;
    }

    console.log(`Handshake from ${packet.data.username}`);
    // Handshake successful, wait for login
  }

  /**
   * Handle login
   */
  handleLogin(ws, packet) {
    if (this.players.size >= this.config.maxPlayers) {
      this.disconnect(ws, 'Server full');
      return null;
    }

    const player = {
      ws,
      entityId: this.nextEntityId++,
      uuid: packet.data.uuid || uuidv4(),
      username: packet.data.username,
      x: 0,
      y: 80,
      z: 0,
      yaw: 0,
      pitch: 0,
      gameMode: 1, // Creative by default
      lastKeepAlive: Date.now()
    };

    this.players.set(player.uuid, player);
    console.log(`Player ${player.username} logged in (${this.players.size}/${this.config.maxPlayers})`);

    // Send world info
    this.sendPacket(player, PacketBuilder.worldInfo(
      this.world.seed,
      player.gameMode,
      0, // peaceful
      this.config.maxPlayers
    ));

    // Send spawn position
    this.sendPacket(player, PacketBuilder.playerPositionAndLook(
      player.x, player.y, player.z,
      player.yaw, player.pitch,
      true
    ));

    // Send chunks around spawn
    this.sendChunksAroundPlayer(player);

    // Notify other players
    this.broadcastExcept(player, PacketBuilder.spawnPlayer(
      player.entityId,
      player.uuid,
      player.username,
      player.x, player.y, player.z,
      player.yaw, player.pitch
    ));

    // Send existing players to new player
    for (const otherPlayer of this.players.values()) {
      if (otherPlayer.uuid !== player.uuid) {
        this.sendPacket(player, PacketBuilder.spawnPlayer(
          otherPlayer.entityId,
          otherPlayer.uuid,
          otherPlayer.username,
          otherPlayer.x, otherPlayer.y, otherPlayer.z,
          otherPlayer.yaw, otherPlayer.pitch
        ));
      }
    }

    return player;
  }

  /**
   * Send chunks around player
   */
  sendChunksAroundPlayer(player) {
    const playerChunkX = Math.floor(player.x / 16);
    const playerChunkZ = Math.floor(player.z / 16);
    const renderDistance = this.config.renderDistance || 8;

    for (let dx = -renderDistance; dx <= renderDistance; dx++) {
      for (let dz = -renderDistance; dz <= renderDistance; dz++) {
        const chunkX = playerChunkX + dx;
        const chunkZ = playerChunkZ + dz;
        const chunk = this.world.getChunk(chunkX, chunkZ);
        
        this.sendPacket(player, PacketBuilder.chunkData(
          chunkX, chunkZ,
          chunk.compress(),
          [] // biomes - simplified
        ));
      }
    }
  }

  /**
   * Handle player movement
   */
  handlePlayerMovement(player, packet) {
    if (packet.data.x !== undefined) {
      player.x = packet.data.x;
      player.y = packet.data.y;
      player.z = packet.data.z;
    }
    if (packet.data.yaw !== undefined) {
      player.yaw = packet.data.yaw;
      player.pitch = packet.data.pitch;
    }

    // Broadcast to other players
    this.broadcastExcept(player, PacketBuilder.playerPositionAndLook(
      player.x, player.y, player.z,
      player.yaw, player.pitch,
      packet.data.onGround
    ));
  }

  /**
   * Handle block break
   */
  handleBlockBreak(player, packet) {
    const { x, y, z } = packet.data;
    this.world.setBlock(x, y, z, 0); // Air

    // Broadcast block change
    this.broadcast(PacketBuilder.blockChange(x, y, z, 0));
  }

  /**
   * Handle block placement
   */
  handleBlockPlace(player, packet) {
    const { x, y, z, blockId } = packet.data;
    this.world.setBlock(x, y, z, blockId);

    // Broadcast block change
    this.broadcast(PacketBuilder.blockChange(x, y, z, blockId));
  }

  /**
   * Handle chat message
   */
  handleChatMessage(player, packet) {
    const message = `<${player.username}> ${packet.data.message}`;
    console.log(message);

    // Broadcast to all players
    this.broadcast(PacketBuilder.chatMessage(message));
  }

  /**
   * Handle plugin message
   */
  handlePluginMessage(player, packet) {
    // Forward to plugin system
    if (this.pluginManager) {
      this.pluginManager.handleMessage(player, packet.data.channel, packet.data.data);
    }
  }

  /**
   * Remove player from server
   */
  removePlayer(player) {
    this.players.delete(player.uuid);
    
    // Notify other players
    this.broadcast(PacketBuilder.destroyEntities([player.entityId]));
  }

  /**
   * Send packet to player
   */
  sendPacket(player, packet) {
    if (player.ws.readyState === WebSocket.OPEN) {
      player.ws.send(packet.serialize());
    }
  }

  /**
   * Broadcast packet to all players
   */
  broadcast(packet) {
    const data = packet.serialize();
    for (const player of this.players.values()) {
      if (player.ws.readyState === WebSocket.OPEN) {
        player.ws.send(data);
      }
    }
  }

  /**
   * Broadcast packet to all players except one
   */
  broadcastExcept(excludePlayer, packet) {
    const data = packet.serialize();
    for (const player of this.players.values()) {
      if (player.uuid !== excludePlayer.uuid && player.ws.readyState === WebSocket.OPEN) {
        player.ws.send(data);
      }
    }
  }

  /**
   * Disconnect client
   */
  disconnect(ws, reason) {
    console.log(`Disconnecting client: ${reason}`);
    if (ws.readyState === WebSocket.OPEN) {
      ws.send(PacketBuilder.disconnect(reason).serialize());
      ws.close();
    }
  }

  /**
   * Game tick - runs at TICK_RATE per second
   */
  tick() {
    // Update world
    this.world.tick();

    // Send time update periodically
    if (this.world.worldAge % 20 === 0) {
      this.broadcast(PacketBuilder.timeUpdate(this.world.worldAge, this.world.timeOfDay));
    }

    // Check keep-alive
    const now = Date.now();
    if (now - this.lastKeepAlive > 5000) {
      const keepAliveId = now;
      this.broadcast(PacketBuilder.keepAlive(keepAliveId));
      this.lastKeepAlive = now;
    }

    // Remove disconnected players
    for (const player of this.players.values()) {
      if (now - player.lastKeepAlive > 30000) {
        console.log(`Player ${player.username} timed out`);
        this.removePlayer(player);
      }
    }

    // Unload distant chunks
    const playerPositions = Array.from(this.players.values()).map(p => ({ x: p.x, z: p.z }));
    this.world.unloadDistantChunks(playerPositions, this.config.renderDistance || 8);
  }

  /**
   * Stop the server
   */
  stop() {
    console.log('Stopping server...');
    
    if (this.tickInterval) {
      clearInterval(this.tickInterval);
    }

    // Disconnect all players
    for (const player of this.players.values()) {
      this.disconnect(player.ws, 'Server shutting down');
    }

    this.world.close();
    this.wss.close();
    
    console.log('Server stopped');
  }
}

module.exports = GameServer;
