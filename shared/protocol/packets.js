/**
 * Network packet definitions for client-server communication
 */

// Packet types
const PacketType = {
  // Client -> Server
  HANDSHAKE: 0x00,
  LOGIN: 0x01,
  PLAYER_POSITION: 0x02,
  PLAYER_LOOK: 0x03,
  PLAYER_POSITION_AND_LOOK: 0x04,
  PLAYER_DIGGING: 0x05,
  PLAYER_BLOCK_PLACEMENT: 0x06,
  CHAT_MESSAGE: 0x07,
  CLIENT_SETTINGS: 0x08,
  KEEP_ALIVE: 0x09,

  // Server -> Client
  DISCONNECT: 0x10,
  CHUNK_DATA: 0x11,
  BLOCK_CHANGE: 0x12,
  MULTI_BLOCK_CHANGE: 0x13,
  SPAWN_PLAYER: 0x14,
  DESTROY_ENTITIES: 0x15,
  ENTITY_POSITION: 0x16,
  ENTITY_LOOK: 0x17,
  ENTITY_POSITION_AND_LOOK: 0x18,
  PLAYER_INFO: 0x19,
  CHAT_MESSAGE_SERVER: 0x1A,
  TIME_UPDATE: 0x1B,
  WORLD_INFO: 0x1C,
  SERVER_KEEP_ALIVE: 0x1D,

  // Bidirectional
  PLUGIN_MESSAGE: 0x20
};

class Packet {
  constructor(type, data = {}) {
    this.type = type;
    this.data = data;
    this.timestamp = Date.now();
  }

  /**
   * Serialize packet to JSON string
   */
  serialize() {
    return JSON.stringify({
      type: this.type,
      data: this.data,
      timestamp: this.timestamp
    });
  }

  /**
   * Deserialize packet from JSON string
   */
  static deserialize(json) {
    const obj = JSON.parse(json);
    const packet = new Packet(obj.type, obj.data);
    packet.timestamp = obj.timestamp;
    return packet;
  }
}

/**
 * Packet builders for common packet types
 */
class PacketBuilder {
  static handshake(protocolVersion, username) {
    return new Packet(PacketType.HANDSHAKE, {
      protocolVersion,
      username
    });
  }

  static login(username, uuid) {
    return new Packet(PacketType.LOGIN, {
      username,
      uuid
    });
  }

  static playerPosition(x, y, z, onGround) {
    return new Packet(PacketType.PLAYER_POSITION, {
      x, y, z, onGround
    });
  }

  static playerLook(yaw, pitch, onGround) {
    return new Packet(PacketType.PLAYER_LOOK, {
      yaw, pitch, onGround
    });
  }

  static playerPositionAndLook(x, y, z, yaw, pitch, onGround) {
    return new Packet(PacketType.PLAYER_POSITION_AND_LOOK, {
      x, y, z, yaw, pitch, onGround
    });
  }

  static blockPlacement(x, y, z, face, blockId) {
    return new Packet(PacketType.PLAYER_BLOCK_PLACEMENT, {
      x, y, z, face, blockId
    });
  }

  static blockDigging(x, y, z, face, status) {
    return new Packet(PacketType.PLAYER_DIGGING, {
      x, y, z, face, status
    });
  }

  static chatMessage(message) {
    return new Packet(PacketType.CHAT_MESSAGE, {
      message
    });
  }

  static disconnect(reason) {
    return new Packet(PacketType.DISCONNECT, {
      reason
    });
  }

  static chunkData(chunkX, chunkZ, blocks, biomes) {
    return new Packet(PacketType.CHUNK_DATA, {
      chunkX,
      chunkZ,
      blocks,
      biomes
    });
  }

  static blockChange(x, y, z, blockId) {
    return new Packet(PacketType.BLOCK_CHANGE, {
      x, y, z, blockId
    });
  }

  static multiBlockChange(chunkX, chunkZ, changes) {
    return new Packet(PacketType.MULTI_BLOCK_CHANGE, {
      chunkX,
      chunkZ,
      changes // Array of {x, y, z, blockId}
    });
  }

  static spawnPlayer(entityId, uuid, username, x, y, z, yaw, pitch) {
    return new Packet(PacketType.SPAWN_PLAYER, {
      entityId,
      uuid,
      username,
      x, y, z, yaw, pitch
    });
  }

  static destroyEntities(entityIds) {
    return new Packet(PacketType.DESTROY_ENTITIES, {
      entityIds
    });
  }

  static worldInfo(seed, gameMode, difficulty, maxPlayers) {
    return new Packet(PacketType.WORLD_INFO, {
      seed,
      gameMode,
      difficulty,
      maxPlayers
    });
  }

  static timeUpdate(worldAge, timeOfDay) {
    return new Packet(PacketType.TIME_UPDATE, {
      worldAge,
      timeOfDay
    });
  }

  static keepAlive(keepAliveId) {
    return new Packet(PacketType.KEEP_ALIVE, {
      keepAliveId
    });
  }

  static pluginMessage(channel, data) {
    return new Packet(PacketType.PLUGIN_MESSAGE, {
      channel,
      data
    });
  }
}

module.exports = {
  PacketType,
  Packet,
  PacketBuilder
};
