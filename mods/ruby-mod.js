// Example mod: Adds a custom ruby block
module.exports = {
  name: 'Ruby Mod',
  version: '1.0.0',
  description: 'Adds ruby ore and blocks to the game',

  // Called when the mod is loaded
  async initialize(api) {
    api.log('Ruby Mod initializing...');

    // Register event hooks
    api.on('world-generation', (chunk) => {
      this.generateRubyOre(chunk, api);
    });

    api.on('block-break', (blockType, position) => {
      if (blockType === this.rubyOreId) {
        api.log(`Ruby ore broken at ${position.x}, ${position.y}, ${position.z}`);
      }
    });

    // Register custom blocks
    this.rubyOreId = api.registerBlock('ruby_ore', {
      hardness: 3.0,
      tool: 'pickaxe',
      drops: 'ruby',
      transparent: false,
      texture: 'ruby_ore.png',
      color: 0xff0040, // Ruby red
    });

    this.rubyBlockId = api.registerBlock('ruby_block', {
      hardness: 5.0,
      tool: 'pickaxe',
      drops: this.rubyBlockId,
      transparent: false,
      texture: 'ruby_block.png',
      color: 0xff0060,
    });

    // Register items
    api.registerItem('ruby', {
      name: 'Ruby',
      maxStack: 64,
      rarity: 'rare',
      texture: 'ruby_item.png',
    });

    api.log('Ruby Mod initialized successfully!');
  },

  // Called when the mod is unloaded
  async cleanup() {
    console.log('Ruby Mod cleaning up...');
  },

  // Custom ruby ore generation
  generateRubyOre(chunk, api) {
    const world = api.getWorld();
    if (!world) return;

    // Simple ore generation logic
    for (let x = 0; x < 16; x++) {
      for (let z = 0; z < 16; z++) {
        for (let y = 10; y < 50; y++) {
          // 5% chance to generate ruby ore in stone
          if (Math.random() < 0.05) {
            const blockType = world.getBlock(chunk.x * 16 + x, y, chunk.z * 16 + z);
            if (blockType === 14) { // STONE
              world.setBlock(chunk.x * 16 + x, y, chunk.z * 16 + z, this.rubyOreId);
            }
          }
        }
      }
    }
  },
};