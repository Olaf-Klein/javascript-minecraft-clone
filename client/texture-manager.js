// Texture system for block rendering
const THREE = require('three');
const { BLOCK_TYPES } = require('../shared/constants');
const TextureGenerator = require('./texture-generator');

class TextureManager {
  constructor() {
    this.textureLoader = new THREE.TextureLoader();
    this.textures = new Map();
    this.normalMaps = new Map();
    this.roughnessMaps = new Map();
    this.metalnessMaps = new Map();
    this.textureAtlas = null;
    this.atlasSize = 512; // Size of texture atlas
    this.blockSize = 16; // Size of each block texture in atlas
    this.blocksPerRow = Math.floor(this.atlasSize / this.blockSize);

    // Initialize texture loading
    this.loadTextures();
  }

  async loadTextures() {
    console.log('Loading block textures...');

    // Load individual block textures
    await this.loadBlockTextures();

    // Create texture atlas for efficient rendering
    this.createTextureAtlas();

    console.log('Textures loaded successfully');
  }

  async loadBlockTextures() {
    const texturePromises = [];

    // Define texture mappings for blocks
    const textureMappings = {
      [BLOCK_TYPES.GRASS_BLOCK]: {
        top: 'grass_block_top.png',
        side: 'grass_block_side.png',
        bottom: 'dirt.png'
      },
      [BLOCK_TYPES.DIRT]: 'dirt.png',
      [BLOCK_TYPES.STONE]: 'stone.png',
      [BLOCK_TYPES.COBBLESTONE]: 'cobblestone.png',
      [BLOCK_TYPES.OAK_LOG]: {
        top: 'oak_log_top.png',
        side: 'oak_log.png'
      },
      [BLOCK_TYPES.OAK_LEAVES]: 'oak_leaves.png',
      [BLOCK_TYPES.OAK_PLANKS]: 'oak_planks.png',
      [BLOCK_TYPES.SAND]: 'sand.png',
      [BLOCK_TYPES.GRAVEL]: 'gravel.png',
      [BLOCK_TYPES.COAL_ORE]: 'coal_ore.png',
      [BLOCK_TYPES.IRON_ORE]: 'iron_ore.png',
      [BLOCK_TYPES.GOLD_ORE]: 'gold_ore.png',
      [BLOCK_TYPES.DIAMOND_ORE]: 'diamond_ore.png',
      [BLOCK_TYPES.BEDROCK]: 'bedrock.png',
      [BLOCK_TYPES.WATER]: 'water.png',
      [BLOCK_TYPES.LAVA]: 'lava.png',
      // Add more block textures as needed
    };

    // Load textures for each block
    for (const [blockId, texturePath] of Object.entries(textureMappings)) {
      if (typeof texturePath === 'string') {
        // Single texture for all faces
        texturePromises.push(this.loadTexture(blockId, texturePath));
      } else {
        // Different textures for different faces
        for (const [face, path] of Object.entries(texturePath)) {
          texturePromises.push(this.loadTexture(`${blockId}_${face}`, path));
        }
      }
    }

    await Promise.all(texturePromises);
  }

  async loadTexture(name, path) {
    return new Promise((resolve, reject) => {
      const fullPath = `assets/textures/blocks/${path}`;

      this.textureLoader.load(
        fullPath,
        (texture) => {
          // Configure texture
          texture.magFilter = THREE.NearestFilter;
          texture.minFilter = THREE.NearestFilter;
          texture.generateMipmaps = false;
          texture.wrapS = THREE.RepeatWrapping;
          texture.wrapT = THREE.RepeatWrapping;

          this.textures.set(name, texture);
          resolve(texture);
        },
        (progress) => {
          // Loading progress
        },
        (error) => {
          console.warn(`Failed to load texture: ${fullPath}, using procedural texture`);
          // Create procedural texture as fallback
          this.createProceduralTexture(name);
          resolve();
        }
      );
    });
  }

  createProceduralTexture(name) {
    let canvas;

    // Generate procedural texture based on block name
    if (name.includes('grass_block_top')) {
      canvas = TextureGenerator.createGrassBlockTop();
    } else if (name.includes('grass_block_side')) {
      canvas = TextureGenerator.createGrassBlockSide();
    } else if (name.includes('dirt')) {
      canvas = TextureGenerator.createDirt();
    } else if (name.includes('stone')) {
      canvas = TextureGenerator.createStone();
    } else if (name.includes('oak_log_top')) {
      canvas = TextureGenerator.createOakLogTop();
    } else if (name.includes('oak_log')) {
      canvas = TextureGenerator.createOakLog();
    } else if (name.includes('oak_leaves')) {
      canvas = TextureGenerator.createOakLeaves();
    } else if (name.includes('oak_planks')) {
      canvas = TextureGenerator.createOakPlanks();
    } else if (name.includes('sand')) {
      canvas = TextureGenerator.createSand();
    } else if (name.includes('coal_ore')) {
      canvas = TextureGenerator.createCoalOre();
    } else if (name.includes('iron_ore')) {
      canvas = TextureGenerator.createIronOre();
    } else if (name.includes('gold_ore')) {
      canvas = TextureGenerator.createGoldOre();
    } else if (name.includes('diamond_ore')) {
      canvas = TextureGenerator.createDiamondOre();
    } else if (name.includes('bedrock')) {
      canvas = TextureGenerator.createBedrock();
    } else if (name.includes('water')) {
      canvas = TextureGenerator.createWater();
    } else {
      // Generic fallback
      canvas = this.createFallbackTexture(name);
    }

    const texture = new THREE.CanvasTexture(canvas);
    texture.magFilter = THREE.NearestFilter;
    texture.minFilter = THREE.NearestFilter;
    texture.generateMipmaps = false;

    this.textures.set(name, texture);
  }

  createFallbackTexture(name) {
    // Create a simple colored texture as fallback
    const canvas = document.createElement('canvas');
    canvas.width = 16;
    canvas.height = 16;
    const ctx = canvas.getContext('2d');

    // Generate a pseudo-random color based on name
    const hash = name.split('').reduce((a, b) => {
      a = ((a << 5) - a) + b.charCodeAt(0);
      return a & a;
    }, 0);

    const r = (hash & 0xff) % 256;
    const g = ((hash >> 8) & 0xff) % 256;
    const b = ((hash >> 16) & 0xff) % 256;

    ctx.fillStyle = `rgb(${r}, ${g}, ${b})`;
    ctx.fillRect(0, 0, 16, 16);

    // Add some texture variation
    ctx.fillStyle = `rgb(${Math.min(255, r + 30)}, ${Math.min(255, g + 30)}, ${Math.min(255, b + 30)})`;
    for (let i = 0; i < 16; i += 4) {
      for (let j = 0; j < 16; j += 4) {
        if ((i + j) % 8 === 0) {
          ctx.fillRect(i, j, 2, 2);
        }
      }
    }

    return canvas;
  }

  createTextureAtlas() {
    const canvas = document.createElement('canvas');
    canvas.width = this.atlasSize;
    canvas.height = this.atlasSize;
    const ctx = canvas.getContext('2d');

    const textureArray = Array.from(this.textures.entries());
    const atlasMap = new Map();

    textureArray.forEach(([name, texture], index) => {
      const x = (index % this.blocksPerRow) * this.blockSize;
      const y = Math.floor(index / this.blocksPerRow) * this.blockSize;

      // Draw texture to atlas
      if (texture.image) {
        ctx.drawImage(texture.image, x, y, this.blockSize, this.blockSize);
      }

      // Store UV coordinates for this texture
      atlasMap.set(name, {
        u: x / this.atlasSize,
        v: y / this.atlasSize,
        width: this.blockSize / this.atlasSize,
        height: this.blockSize / this.atlasSize
      });
    });

    // Create atlas texture
    this.textureAtlas = new THREE.CanvasTexture(canvas);
    this.textureAtlas.magFilter = THREE.NearestFilter;
    this.textureAtlas.minFilter = THREE.NearestFilter;
    this.textureAtlas.generateMipmaps = false;

    this.atlasMap = atlasMap;
  }

  getBlockMaterial(blockType) {
    const blockName = Object.keys(BLOCK_TYPES).find(key => BLOCK_TYPES[key] === blockType);

    if (!blockName) {
      // Fallback material
      return new THREE.MeshLambertMaterial({ color: 0xff00ff });
    }

    // Create PBR material with textures
    const material = new THREE.MeshStandardMaterial({
      map: this.textureAtlas,
      transparent: blockType === BLOCK_TYPES.WATER || blockType === BLOCK_TYPES.GLASS,
      opacity: blockType === BLOCK_TYPES.WATER ? 0.8 : 1.0,
      roughness: this.getBlockRoughness(blockType),
      metalness: this.getBlockMetalness(blockType),
    });

    // Set UV transform for this block's texture
    const textureInfo = this.atlasMap.get(blockName.toLowerCase());
    if (textureInfo) {
      material.map.offset.set(textureInfo.u, textureInfo.v);
      material.map.repeat.set(textureInfo.width, textureInfo.height);
    }

    return material;
  }

  getBlockRoughness(blockType) {
    // Define roughness values for different block types
    const roughnessMap = {
      [BLOCK_TYPES.STONE]: 0.8,
      [BLOCK_TYPES.COBBLESTONE]: 0.9,
      [BLOCK_TYPES.OAK_PLANKS]: 0.7,
      [BLOCK_TYPES.OAK_LOG]: 0.8,
      [BLOCK_TYPES.GRASS_BLOCK]: 0.8,
      [BLOCK_TYPES.DIRT]: 0.9,
      [BLOCK_TYPES.SAND]: 0.6,
      [BLOCK_TYPES.GRAVEL]: 0.9,
      [BLOCK_TYPES.WATER]: 0.0,
      [BLOCK_TYPES.GLASS]: 0.0,
      [BLOCK_TYPES.GOLD_ORE]: 0.3,
      [BLOCK_TYPES.DIAMOND_ORE]: 0.2,
    };

    return roughnessMap[blockType] || 0.8;
  }

  getBlockMetalness(blockType) {
    // Define metalness values for different block types
    const metalnessMap = {
      [BLOCK_TYPES.GOLD_ORE]: 0.8,
      [BLOCK_TYPES.IRON_ORE]: 0.9,
      [BLOCK_TYPES.DIAMOND_ORE]: 0.1,
      [BLOCK_TYPES.ANCIENT_DEBRIS]: 0.9,
      [BLOCK_TYPES.NETHERITE_BLOCK]: 0.9,
      [BLOCK_TYPES.GOLD_BLOCK]: 0.8,
      [BLOCK_TYPES.IRON_BLOCK]: 0.9,
      [BLOCK_TYPES.DIAMOND_BLOCK]: 0.1,
    };

    return metalnessMap[blockType] || 0.0;
  }

  // Get UV coordinates for a specific face of a block
  getFaceUV(blockType, face) {
    const blockName = Object.keys(BLOCK_TYPES).find(key => BLOCK_TYPES[key] === blockType);
    if (!blockName) return null;

    // Handle blocks with different textures per face
    let textureName = blockName.toLowerCase();
    if (face && this.atlasMap.has(`${textureName}_${face}`)) {
      textureName = `${textureName}_${face}`;
    }

    return this.atlasMap.get(textureName);
  }

  dispose() {
    // Clean up textures
    this.textures.forEach(texture => texture.dispose());
    if (this.textureAtlas) {
      this.textureAtlas.dispose();
    }
  }
}

module.exports = TextureManager;