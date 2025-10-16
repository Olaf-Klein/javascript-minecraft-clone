/**
 * Procedural world generation using simplex noise
 */

const SimplexNoise = require('noise-simplex');
const Chunk = require('./chunk');
const blockRegistry = require('../../shared/blocks/registry');
const { CHUNK_SIZE, CHUNK_HEIGHT, SEA_LEVEL } = require('../../shared/constants/game');

class WorldGenerator {
  constructor(seed) {
    this.seed = seed;
    this.noise = new SimplexNoise(seed);
    this.noiseScale = 0.01;
    this.heightScale = 32;
    this.biomeScale = 0.005;

    // Cache block IDs
    this.blockIds = {
      air: blockRegistry.getByName('air').id,
      stone: blockRegistry.getByName('stone').id,
      dirt: blockRegistry.getByName('dirt').id,
      grass: blockRegistry.getByName('grass_block').id,
      sand: blockRegistry.getByName('sand').id,
      water: blockRegistry.getByName('water').id,
      bedrock: blockRegistry.getByName('bedrock').id,
      coal_ore: blockRegistry.getByName('coal_ore').id,
      iron_ore: blockRegistry.getByName('iron_ore').id,
      gold_ore: blockRegistry.getByName('gold_ore').id,
      diamond_ore: blockRegistry.getByName('diamond_ore').id,
      oak_log: blockRegistry.getByName('oak_log').id,
      oak_leaves: blockRegistry.getByName('oak_leaves').id
    };
  }

  /**
   * Generate a chunk at the given coordinates
   */
  generateChunk(chunkX, chunkZ) {
    const chunk = new Chunk(chunkX, chunkZ);

    // Generate terrain
    for (let x = 0; x < CHUNK_SIZE; x++) {
      for (let z = 0; z < CHUNK_SIZE; z++) {
        const worldX = chunkX * CHUNK_SIZE + x;
        const worldZ = chunkZ * CHUNK_SIZE + z;

        // Calculate terrain height using multiple octaves of noise
        const height = this.getTerrainHeight(worldX, worldZ);
        const biome = this.getBiome(worldX, worldZ);

        // Fill column
        this.generateColumn(chunk, x, z, height, biome, worldX, worldZ);
      }
    }

    // Generate features (trees, etc.)
    this.generateFeatures(chunk, chunkX, chunkZ);

    chunk.generated = true;
    return chunk;
  }

  /**
   * Calculate terrain height at world coordinates
   */
  getTerrainHeight(x, z) {
    let height = SEA_LEVEL;
    
    // Multiple octaves for varied terrain
    height += this.noise.noise2D(x * this.noiseScale, z * this.noiseScale) * this.heightScale;
    height += this.noise.noise2D(x * this.noiseScale * 2, z * this.noiseScale * 2) * (this.heightScale / 2);
    height += this.noise.noise2D(x * this.noiseScale * 4, z * this.noiseScale * 4) * (this.heightScale / 4);

    return Math.floor(height);
  }

  /**
   * Determine biome at world coordinates
   */
  getBiome(x, z) {
    const temperature = this.noise.noise2D(x * this.biomeScale, z * this.biomeScale);
    const humidity = this.noise.noise2D(x * this.biomeScale + 1000, z * this.biomeScale + 1000);

    if (temperature < -0.3) return 'snowy';
    if (temperature > 0.5 && humidity < -0.3) return 'desert';
    if (humidity > 0.3) return 'forest';
    return 'plains';
  }

  /**
   * Generate a vertical column of blocks
   */
  generateColumn(chunk, x, z, height, biome, worldX, worldZ) {
    for (let y = 0; y < CHUNK_HEIGHT; y++) {
      let blockId = this.blockIds.air;

      if (y === 0) {
        // Bedrock layer
        blockId = this.blockIds.bedrock;
      } else if (y < height - 4) {
        // Stone layer
        blockId = this.blockIds.stone;
        
        // Add ores
        if (this.shouldPlaceOre(worldX, y, worldZ, 0.01, 1, 64)) {
          blockId = this.blockIds.coal_ore;
        } else if (this.shouldPlaceOre(worldX, y, worldZ, 0.008, 1, 32)) {
          blockId = this.blockIds.iron_ore;
        } else if (this.shouldPlaceOre(worldX, y, worldZ, 0.005, 1, 16)) {
          blockId = this.blockIds.gold_ore;
        } else if (this.shouldPlaceOre(worldX, y, worldZ, 0.002, 1, 12)) {
          blockId = this.blockIds.diamond_ore;
        }
      } else if (y < height - 1) {
        // Dirt layer
        blockId = this.blockIds.dirt;
      } else if (y < height) {
        // Surface block
        if (biome === 'desert') {
          blockId = this.blockIds.sand;
        } else if (y >= SEA_LEVEL) {
          blockId = this.blockIds.grass;
        } else {
          blockId = this.blockIds.sand;
        }
      } else if (y < SEA_LEVEL) {
        // Water
        blockId = this.blockIds.water;
      }

      chunk.setBlock(x, y, z, blockId);
    }
  }

  /**
   * Check if ore should be placed at position
   */
  shouldPlaceOre(x, y, z, frequency, minY, maxY) {
    if (y < minY || y > maxY) return false;
    const oreNoise = this.noise.noise3D(x * 0.1, y * 0.1, z * 0.1);
    return oreNoise > (1 - frequency);
  }

  /**
   * Generate features like trees
   */
  generateFeatures(chunk, chunkX, chunkZ) {
    // Simple tree generation
    for (let attempt = 0; attempt < 3; attempt++) {
      const x = Math.floor(Math.random() * (CHUNK_SIZE - 4)) + 2;
      const z = Math.floor(Math.random() * (CHUNK_SIZE - 4)) + 2;
      
      // Find surface
      let y = CHUNK_HEIGHT - 1;
      while (y > 0 && chunk.getBlock(x, y, z) === this.blockIds.air) {
        y--;
      }
      
      // Check if it's grass
      if (chunk.getBlock(x, y, z) === this.blockIds.grass && y > SEA_LEVEL) {
        this.generateTree(chunk, x, y + 1, z);
      }
    }
  }

  /**
   * Generate a simple tree
   */
  generateTree(chunk, x, y, z) {
    const trunkHeight = 5;
    
    // Trunk
    for (let i = 0; i < trunkHeight; i++) {
      chunk.setBlock(x, y + i, z, this.blockIds.oak_log);
    }
    
    // Leaves
    const leavesY = y + trunkHeight;
    for (let dx = -2; dx <= 2; dx++) {
      for (let dy = -2; dy <= 1; dy++) {
        for (let dz = -2; dz <= 2; dz++) {
          if (dx === 0 && dy < 0 && dz === 0) continue; // Skip trunk
          const distance = Math.abs(dx) + Math.abs(dy) + Math.abs(dz);
          if (distance < 4) {
            const lx = x + dx;
            const ly = leavesY + dy;
            const lz = z + dz;
            if (lx >= 0 && lx < CHUNK_SIZE && lz >= 0 && lz < CHUNK_SIZE && ly >= 0 && ly < CHUNK_HEIGHT) {
              if (chunk.getBlock(lx, ly, lz) === this.blockIds.air) {
                chunk.setBlock(lx, ly, lz, this.blockIds.oak_leaves);
              }
            }
          }
        }
      }
    }
  }
}

module.exports = WorldGenerator;
