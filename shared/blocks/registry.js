/**
 * Block Registry - Minecraft 1.21.10 compatible block definitions
 */

class BlockRegistry {
  constructor() {
    this.blocks = new Map();
    this.blocksByName = new Map();
    this.nextId = 0;
    this.initializeBlocks();
  }

  /**
   * Register a new block type
   */
  register(name, properties) {
    const id = this.nextId++;
    const block = {
      id,
      name,
      ...properties
    };
    this.blocks.set(id, block);
    this.blocksByName.set(name, block);
    return block;
  }

  /**
   * Get block by ID
   */
  getById(id) {
    return this.blocks.get(id);
  }

  /**
   * Get block by name
   */
  getByName(name) {
    return this.blocksByName.get(name);
  }

  /**
   * Initialize all vanilla blocks from Minecraft 1.21.10
   */
  initializeBlocks() {
    // Air (must be ID 0)
    this.register('air', {
      transparent: true,
      solid: false,
      lightLevel: 0,
      textures: {}
    });

    // Stone types
    this.register('stone', {
      transparent: false,
      solid: true,
      hardness: 1.5,
      textures: { all: 'stone' }
    });

    this.register('granite', {
      transparent: false,
      solid: true,
      hardness: 1.5,
      textures: { all: 'granite' }
    });

    this.register('diorite', {
      transparent: false,
      solid: true,
      hardness: 1.5,
      textures: { all: 'diorite' }
    });

    this.register('andesite', {
      transparent: false,
      solid: true,
      hardness: 1.5,
      textures: { all: 'andesite' }
    });

    // Dirt and grass
    this.register('dirt', {
      transparent: false,
      solid: true,
      hardness: 0.5,
      textures: { all: 'dirt' }
    });

    this.register('grass_block', {
      transparent: false,
      solid: true,
      hardness: 0.6,
      textures: {
        top: 'grass_block_top',
        bottom: 'dirt',
        side: 'grass_block_side'
      }
    });

    this.register('coarse_dirt', {
      transparent: false,
      solid: true,
      hardness: 0.5,
      textures: { all: 'coarse_dirt' }
    });

    // Sand and gravel
    this.register('sand', {
      transparent: false,
      solid: true,
      hardness: 0.5,
      gravity: true,
      textures: { all: 'sand' }
    });

    this.register('red_sand', {
      transparent: false,
      solid: true,
      hardness: 0.5,
      gravity: true,
      textures: { all: 'red_sand' }
    });

    this.register('gravel', {
      transparent: false,
      solid: true,
      hardness: 0.6,
      gravity: true,
      textures: { all: 'gravel' }
    });

    // Wood types
    const woodTypes = ['oak', 'spruce', 'birch', 'jungle', 'acacia', 'dark_oak', 'cherry', 'mangrove'];
    woodTypes.forEach(wood => {
      this.register(`${wood}_log`, {
        transparent: false,
        solid: true,
        hardness: 2.0,
        textures: {
          top: `${wood}_log_top`,
          bottom: `${wood}_log_top`,
          side: `${wood}_log`
        }
      });

      this.register(`${wood}_planks`, {
        transparent: false,
        solid: true,
        hardness: 2.0,
        textures: { all: `${wood}_planks` }
      });
    });

    // Ores
    this.register('coal_ore', {
      transparent: false,
      solid: true,
      hardness: 3.0,
      textures: { all: 'coal_ore' }
    });

    this.register('iron_ore', {
      transparent: false,
      solid: true,
      hardness: 3.0,
      textures: { all: 'iron_ore' }
    });

    this.register('gold_ore', {
      transparent: false,
      solid: true,
      hardness: 3.0,
      textures: { all: 'gold_ore' }
    });

    this.register('diamond_ore', {
      transparent: false,
      solid: true,
      hardness: 3.0,
      textures: { all: 'diamond_ore' }
    });

    this.register('emerald_ore', {
      transparent: false,
      solid: true,
      hardness: 3.0,
      textures: { all: 'emerald_ore' }
    });

    this.register('redstone_ore', {
      transparent: false,
      solid: true,
      hardness: 3.0,
      lightLevel: 9,
      textures: { all: 'redstone_ore' }
    });

    this.register('lapis_ore', {
      transparent: false,
      solid: true,
      hardness: 3.0,
      textures: { all: 'lapis_ore' }
    });

    this.register('copper_ore', {
      transparent: false,
      solid: true,
      hardness: 3.0,
      textures: { all: 'copper_ore' }
    });

    // Deepslate variants
    this.register('deepslate', {
      transparent: false,
      solid: true,
      hardness: 3.0,
      textures: { all: 'deepslate' }
    });

    this.register('deepslate_coal_ore', {
      transparent: false,
      solid: true,
      hardness: 4.5,
      textures: { all: 'deepslate_coal_ore' }
    });

    this.register('deepslate_iron_ore', {
      transparent: false,
      solid: true,
      hardness: 4.5,
      textures: { all: 'deepslate_iron_ore' }
    });

    this.register('deepslate_gold_ore', {
      transparent: false,
      solid: true,
      hardness: 4.5,
      textures: { all: 'deepslate_gold_ore' }
    });

    this.register('deepslate_diamond_ore', {
      transparent: false,
      solid: true,
      hardness: 4.5,
      textures: { all: 'deepslate_diamond_ore' }
    });

    // Glass
    this.register('glass', {
      transparent: true,
      solid: true,
      hardness: 0.3,
      textures: { all: 'glass' }
    });

    // Liquids
    this.register('water', {
      transparent: true,
      solid: false,
      liquid: true,
      textures: { all: 'water_still' }
    });

    this.register('lava', {
      transparent: true,
      solid: false,
      liquid: true,
      lightLevel: 15,
      textures: { all: 'lava_still' }
    });

    // Leaves
    woodTypes.forEach(wood => {
      this.register(`${wood}_leaves`, {
        transparent: true,
        solid: true,
        hardness: 0.2,
        textures: { all: `${wood}_leaves` }
      });
    });

    // Wool colors
    const colors = ['white', 'orange', 'magenta', 'light_blue', 'yellow', 'lime', 
                    'pink', 'gray', 'light_gray', 'cyan', 'purple', 'blue', 
                    'brown', 'green', 'red', 'black'];
    colors.forEach(color => {
      this.register(`${color}_wool`, {
        transparent: false,
        solid: true,
        hardness: 0.8,
        textures: { all: `${color}_wool` }
      });
    });

    // Terracotta
    this.register('terracotta', {
      transparent: false,
      solid: true,
      hardness: 1.25,
      textures: { all: 'terracotta' }
    });

    // Concrete
    colors.forEach(color => {
      this.register(`${color}_concrete`, {
        transparent: false,
        solid: true,
        hardness: 1.8,
        textures: { all: `${color}_concrete` }
      });
    });

    // Bedrock
    this.register('bedrock', {
      transparent: false,
      solid: true,
      hardness: -1, // Unbreakable
      textures: { all: 'bedrock' }
    });

    // Add more blocks as needed...
  }

  /**
   * Get all registered blocks
   */
  getAllBlocks() {
    return Array.from(this.blocks.values());
  }
}

// Singleton instance
const blockRegistry = new BlockRegistry();

module.exports = blockRegistry;
