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
  constructor(seed = Math.random(), onChunkGenerated = null) {
    this.chunks = new Map();
    this.seed = seed;
    this.noise = this.createNoiseFunction(seed);
    this.onChunkGenerated = onChunkGenerated;
  }

  // Simple noise function for terrain generation
  createNoiseFunction(seed) {
    const hash = (x, z) => {
      const h = ((x * 73856093) ^ (z * 19349663) ^ seed) & 0x7fffffff;
      return (h * 1.0) / 0x7fffffff;
    };

    return (x, z, octaves = 4, persistence = 0.5, scale = 0.01) => {
      let value = 0;
      let amplitude = 1;
      let frequency = scale;
      let maxValue = 0;

      for (let i = 0; i < octaves; i++) {
        value += hash(Math.floor(x * frequency), Math.floor(z * frequency)) * amplitude;
        maxValue += amplitude;
        amplitude *= persistence;
        frequency *= 2;
      }

      return value / maxValue;
    };
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
    const chunkWorldX = chunk.x * CHUNK_SIZE;
    const chunkWorldZ = chunk.z * CHUNK_SIZE;

    // Generate terrain heightmap
    const heightMap = [];
    for (let x = 0; x < CHUNK_SIZE; x++) {
      heightMap[x] = [];
      for (let z = 0; z < CHUNK_SIZE; z++) {
        const worldX = chunkWorldX + x;
        const worldZ = chunkWorldZ + z;

        // Base terrain height (40-120)
        const baseHeight = 40 + this.noise(worldX, worldZ, 6, 0.5, 0.005) * 80;

        // Add some hills and valleys
        const hillNoise = this.noise(worldX, worldZ, 3, 0.7, 0.02) * 20;
        const valleyNoise = this.noise(worldX, worldZ, 2, 0.8, 0.01) * 15;

        heightMap[x][z] = Math.floor(baseHeight + hillNoise - valleyNoise);
      }
    }

    // Generate blocks
    for (let x = 0; x < CHUNK_SIZE; x++) {
      for (let z = 0; z < CHUNK_SIZE; z++) {
        const height = heightMap[x][z];

        for (let y = 0; y < WORLD_HEIGHT; y++) {
          if (y === 0) {
            // Bedrock layer
            chunk.setBlock(x, y, z, BLOCK_TYPES.BEDROCK);
          } else if (y < height - 3) {
            // Deep stone
            chunk.setBlock(x, y, z, BLOCK_TYPES.STONE);
          } else if (y < height - 1) {
            // Dirt layer
            chunk.setBlock(x, y, z, BLOCK_TYPES.DIRT);
          } else if (y === height - 1) {
            // Grass/dirt transition
            const surfaceNoise = this.noise(chunkWorldX + x, chunkWorldZ + z, 1, 1, 0.1);
            if (surfaceNoise > 0.3) {
              chunk.setBlock(x, y, z, BLOCK_TYPES.GRASS_BLOCK);
            } else {
              chunk.setBlock(x, y, z, BLOCK_TYPES.DIRT);
            }
          } else if (y === height) {
            // Surface blocks
            const surfaceType = this.noise(chunkWorldX + x, chunkWorldZ + z, 1, 1, 0.05);
            if (surfaceType > 0.7) {
              chunk.setBlock(x, y, z, BLOCK_TYPES.STONE);
            } else if (surfaceType > 0.4) {
              chunk.setBlock(x, y, z, BLOCK_TYPES.GRASS_BLOCK);
            } else {
              chunk.setBlock(x, y, z, BLOCK_TYPES.DIRT);
            }
          }
          // Above ground is air by default
        }

        // Add some basic ores
        this.generateOres(chunk, x, z, height);
      }
    }

    // Generate trees
    this.generateTrees(chunk, heightMap);

    // Generate caves (simple)
    this.generateCaves(chunk);

    // Emit chunk generation event for mods
    if (this.onChunkGenerated) {
      this.onChunkGenerated(chunk);
    }
  }

  generateOres(chunk, x, z, surfaceHeight) {
    for (let y = 1; y < surfaceHeight - 1; y++) {
      const worldX = chunk.x * CHUNK_SIZE + x;
      const worldZ = chunk.z * CHUNK_SIZE + z;

      // Coal ore (common, near surface)
      if (y > 5 && y < surfaceHeight - 5) {
        const coalNoise = this.noise(worldX, worldZ + y * 1000, 1, 1, 0.05);
        if (coalNoise > 0.85 && chunk.getBlock(x, y, z) === BLOCK_TYPES.STONE) {
          chunk.setBlock(x, y, z, BLOCK_TYPES.COAL_ORE);
        }
      }

      // Iron ore (medium depth)
      if (y > 10 && y < surfaceHeight - 10) {
        const ironNoise = this.noise(worldX + 1000, worldZ + y * 1000, 1, 1, 0.03);
        if (ironNoise > 0.9 && chunk.getBlock(x, y, z) === BLOCK_TYPES.STONE) {
          chunk.setBlock(x, y, z, BLOCK_TYPES.IRON_ORE);
        }
      }

      // Gold ore (deeper)
      if (y > 20 && y < surfaceHeight - 20) {
        const goldNoise = this.noise(worldX + 2000, worldZ + y * 1000, 1, 1, 0.02);
        if (goldNoise > 0.95 && chunk.getBlock(x, y, z) === BLOCK_TYPES.STONE) {
          chunk.setBlock(x, y, z, BLOCK_TYPES.GOLD_ORE);
        }
      }

      // Diamond ore (deep)
      if (y > 30 && y < surfaceHeight - 30) {
        const diamondNoise = this.noise(worldX + 3000, worldZ + y * 1000, 1, 1, 0.01);
        if (diamondNoise > 0.98 && chunk.getBlock(x, y, z) === BLOCK_TYPES.STONE) {
          chunk.setBlock(x, y, z, BLOCK_TYPES.DIAMOND_ORE);
        }
      }
    }
  }

  generateTrees(chunk, heightMap) {
    for (let x = 2; x < CHUNK_SIZE - 2; x++) {
      for (let z = 2; z < CHUNK_SIZE - 2; z++) {
        const worldX = chunk.x * CHUNK_SIZE + x;
        const worldZ = chunk.z * CHUNK_SIZE + z;

        const treeNoise = this.noise(worldX, worldZ, 1, 1, 0.1);
        if (treeNoise > 0.8) {
          const height = heightMap[x][z];
          if (height > 50 && height < 100) { // Only on reasonable terrain
            this.generateTree(chunk, x, height + 1, z);
          }
        }
      }
    }
  }

  generateTree(chunk, x, y, z) {
    const trunkHeight = 4 + Math.floor(Math.random() * 3);

    // Trunk
    for (let i = 0; i < trunkHeight; i++) {
      chunk.setBlock(x, y + i, z, BLOCK_TYPES.OAK_LOG);
    }

    // Leaves
    const leafY = y + trunkHeight - 1;
    for (let dx = -2; dx <= 2; dx++) {
      for (let dz = -2; dz <= 2; dz++) {
        for (let dy = -1; dy <= 1; dy++) {
          if (Math.abs(dx) + Math.abs(dz) + Math.abs(dy) <= 3) {
            const leafX = x + dx;
            const leafZ = z + dz;
            const leafYPos = leafY + dy;
            if (chunk.getBlock(leafX, leafYPos, leafZ) === BLOCK_TYPES.AIR) {
              chunk.setBlock(leafX, leafYPos, leafZ, BLOCK_TYPES.OAK_LEAVES);
            }
          }
        }
      }
    }
  }

  generateCaves(chunk) {
    const caveNoise = (x, y, z) => this.noise(x, y + z * 100, 3, 0.6, 0.02);

    for (let x = 0; x < CHUNK_SIZE; x++) {
      for (let z = 0; z < CHUNK_SIZE; z++) {
        for (let y = 5; y < WORLD_HEIGHT - 5; y++) {
          const worldX = chunk.x * CHUNK_SIZE + x;
          const worldY = y;
          const worldZ = chunk.z * CHUNK_SIZE + z;

          const caveValue = caveNoise(worldX, worldY, worldZ);
          if (caveValue > 0.7) {
            chunk.setBlock(x, y, z, BLOCK_TYPES.AIR);
          }
        }
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