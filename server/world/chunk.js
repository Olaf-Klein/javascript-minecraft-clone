/**
 * Chunk representation and management
 */

const { CHUNK_SIZE, CHUNK_HEIGHT } = require('../../shared/constants/game');

class Chunk {
  constructor(x, z) {
    this.x = x;
    this.z = z;
    this.blocks = new Uint16Array(CHUNK_SIZE * CHUNK_HEIGHT * CHUNK_SIZE);
    this.modified = false;
    this.generated = false;
  }

  /**
   * Get block ID at position within chunk
   */
  getBlock(x, y, z) {
    if (x < 0 || x >= CHUNK_SIZE || y < 0 || y >= CHUNK_HEIGHT || z < 0 || z >= CHUNK_SIZE) {
      return 0; // Air
    }
    const index = this.getBlockIndex(x, y, z);
    return this.blocks[index];
  }

  /**
   * Set block ID at position within chunk
   */
  setBlock(x, y, z, blockId) {
    if (x < 0 || x >= CHUNK_SIZE || y < 0 || y >= CHUNK_HEIGHT || z < 0 || z >= CHUNK_SIZE) {
      return false;
    }
    const index = this.getBlockIndex(x, y, z);
    this.blocks[index] = blockId;
    this.modified = true;
    return true;
  }

  /**
   * Calculate array index from 3D coordinates
   */
  getBlockIndex(x, y, z) {
    return y * (CHUNK_SIZE * CHUNK_SIZE) + z * CHUNK_SIZE + x;
  }

  /**
   * Serialize chunk data for network transmission
   */
  serialize() {
    return {
      x: this.x,
      z: this.z,
      blocks: Array.from(this.blocks)
    };
  }

  /**
   * Deserialize chunk data
   */
  static deserialize(data) {
    const chunk = new Chunk(data.x, data.z);
    chunk.blocks = new Uint16Array(data.blocks);
    chunk.generated = true;
    return chunk;
  }

  /**
   * Compress chunk data for efficient storage/transmission
   */
  compress() {
    // Simple run-length encoding
    const compressed = [];
    let currentBlock = this.blocks[0];
    let count = 1;

    for (let i = 1; i < this.blocks.length; i++) {
      if (this.blocks[i] === currentBlock && count < 65535) {
        count++;
      } else {
        compressed.push(currentBlock, count);
        currentBlock = this.blocks[i];
        count = 1;
      }
    }
    compressed.push(currentBlock, count);

    return compressed;
  }

  /**
   * Decompress chunk data
   */
  static decompress(compressed) {
    const blocks = [];
    for (let i = 0; i < compressed.length; i += 2) {
      const blockId = compressed[i];
      const count = compressed[i + 1];
      for (let j = 0; j < count; j++) {
        blocks.push(blockId);
      }
    }
    return new Uint16Array(blocks);
  }
}

module.exports = Chunk;
