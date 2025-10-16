// World management for Minecraft clone
const { BLOCK_TYPES, CHUNK_SIZE, WORLD_HEIGHT } = require('./constants');

class Chunk {
  constructor(x, z) {
    this.x = x;
    this.z = z;
    this.blocks = new Array(CHUNK_SIZE * WORLD_HEIGHT * CHUNK_SIZE).fill(BLOCK_TYPES.AIR);
  }

  getBlock(x, y, z) {
    if (x < 0 || x >= CHUNK_SIZE || y < 0 || y >= WORLD_HEIGHT || z < 0 || z >= CHUNK_SIZE) return BLOCK_TYPES.AIR;
    return this.blocks[x + y * CHUNK_SIZE + z * CHUNK_SIZE * WORLD_HEIGHT];
  }

  setBlock(x, y, z, type) {
    if (x < 0 || x >= CHUNK_SIZE || y < 0 || y >= WORLD_HEIGHT || z < 0 || z >= CHUNK_SIZE) return;
    this.blocks[x + y * CHUNK_SIZE + z * CHUNK_SIZE * WORLD_HEIGHT] = type;
  }
}

class World {
  constructor() {
    this.chunks = new Map();
  }

  getChunkKey(x, z) {
    return `${x},${z}`;
  }

  getChunk(x, z) {
    const key = this.getChunkKey(x, z);
    if (!this.chunks.has(key)) {
      this.chunks.set(key, new Chunk(x, z));
      this.generateChunk(this.chunks.get(key));
    }
    return this.chunks.get(key);
  }

  generateChunk(chunk) {
    // Simple flat world generation
    for (let x = 0; x < CHUNK_SIZE; x++) {
      for (let z = 0; z < CHUNK_SIZE; z++) {
        chunk.setBlock(x, 0, z, BLOCK_TYPES.STONE);
        chunk.setBlock(x, 1, z, BLOCK_TYPES.DIRT);
        chunk.setBlock(x, 2, z, BLOCK_TYPES.GRASS);
      }
    }
  }

  getBlock(worldX, worldY, worldZ) {
    const chunkX = Math.floor(worldX / CHUNK_SIZE);
    const chunkZ = Math.floor(worldZ / CHUNK_SIZE);
    const chunk = this.getChunk(chunkX, chunkZ);
    const localX = worldX - chunkX * CHUNK_SIZE;
    const localZ = worldZ - chunkZ * CHUNK_SIZE;
    return chunk.getBlock(localX, worldY, localZ);
  }

  setBlock(worldX, worldY, worldZ, type) {
    const chunkX = Math.floor(worldX / CHUNK_SIZE);
    const chunkZ = Math.floor(worldZ / CHUNK_SIZE);
    const chunk = this.getChunk(chunkX, chunkZ);
    const localX = worldX - chunkX * CHUNK_SIZE;
    const localZ = worldZ - chunkZ * CHUNK_SIZE;
    chunk.setBlock(localX, worldY, localZ, type);
  }
}

module.exports = { World, Chunk };