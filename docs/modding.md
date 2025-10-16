# Modding API Documentation

The JavaScript Minecraft Clone supports a comprehensive modding system that allows you to extend the game with new blocks, items, gameplay mechanics, and more.

## Getting Started

Mods are JavaScript files placed in the `mods/` directory. Each mod is a Node.js module that exports an object with specific methods and properties.

## Basic Mod Structure

```javascript
module.exports = {
  name: 'My Awesome Mod',
  version: '1.0.0',
  description: 'Adds awesome features to the game',

  // Called when the mod is loaded
  async initialize(api) {
    // Your mod initialization code here
  },

  // Called when the mod is unloaded (optional)
  async cleanup() {
    // Cleanup code here
  },
};
```

## Mod API

The `api` parameter passed to `initialize()` provides access to various game systems:

### Event System

```javascript
// Listen for events
api.on('event-name', (data) => {
  // Handle event
});

// Stop listening for events
api.off('event-name');

// Emit custom events
api.emit('custom-event', data);
```

#### Available Events

- `world-generation` (chunk): Called when a new chunk is generated
- `block-break` (blockType, position): Called when a block is broken
- `block-place` (blockType, position): Called when a block is placed
- `player-move` (player, position): Called when a player moves
- `player-chat` (player, message): Called when a player sends a chat message

### Block Registration

```javascript
const blockId = api.registerBlock('my_block', {
  hardness: 2.0,        // How long it takes to break
  tool: 'pickaxe',      // Required tool ('pickaxe', 'axe', 'shovel', 'hoe', null)
  drops: 'my_item',     // What it drops when broken (item ID or block ID)
  transparent: false,   // Whether light passes through
  texture: 'block.png', // Texture file path
  color: 0x00ff00,      // Fallback color if no texture
});
```

### Item Registration

```javascript
api.registerItem('my_item', {
  name: 'My Item',      // Display name
  maxStack: 64,         // Maximum stack size
  rarity: 'common',     // 'common', 'uncommon', 'rare', 'epic'
  texture: 'item.png',  // Item texture
});
```

### World Access

```javascript
const world = api.getWorld();

// Get block at position
const blockType = world.getBlock(x, y, z);

// Set block at position
world.setBlock(x, y, z, blockType);
```

### Logging

```javascript
api.log('This is a log message');
api.error('This is an error message');
```

## Block Properties

When registering blocks, you can specify:

- `hardness`: Time to break (in seconds with bare hands)
- `tool`: Required tool type
- `drops`: Item/block dropped when broken
- `transparent`: Whether the block is transparent
- `texture`: Path to texture file
- `color`: Hex color for rendering
- `lightLevel`: Light emitted (0-15)
- `flammable`: Whether the block can catch fire
- `liquid`: Whether the block is a liquid

## Tool Effectiveness

Different tools are effective against different materials:

- **Pickaxe**: Stone, metal, crystal blocks
- **Axe**: Wood, plant, organic blocks
- **Shovel**: Dirt, sand, gravel, snow
- **Hoe**: Plant, organic, sculk blocks
- **Sword**: Plant, organic blocks (for harvesting)
- **Shears**: Wool, plant, organic blocks

## Example Mod: Custom Tree

```javascript
module.exports = {
  name: 'Custom Tree Mod',
  version: '1.0.0',

  async initialize(api) {
    // Register custom log block
    const cherryLogId = api.registerBlock('cherry_log', {
      hardness: 2.0,
      tool: 'axe',
      drops: 'cherry_log',
      transparent: false,
      color: 0x8B4513,
    });

    // Register custom leaves
    const cherryLeavesId = api.registerBlock('cherry_leaves', {
      hardness: 0.2,
      tool: null,
      drops: null,
      transparent: true,
      color: 0x228B22,
    });

    // Register sapling item
    api.registerItem('cherry_sapling', {
      name: 'Cherry Sapling',
      maxStack: 64,
      texture: 'cherry_sapling.png',
    });

    // Hook into world generation to add trees
    api.on('world-generation', (chunk) => {
      this.generateCherryTrees(chunk, api, cherryLogId, cherryLeavesId);
    });

    api.log('Cherry Tree Mod loaded!');
  },

  generateCherryTrees(chunk, api, logId, leavesId) {
    const world = api.getWorld();

    // Simple tree generation logic
    for (let x = 1; x < 15; x++) {
      for (let z = 1; z < 15; z++) {
        if (Math.random() < 0.01) { // 1% chance
          const height = 60 + Math.floor(Math.random() * 20);
          const worldX = chunk.x * 16 + x;
          const worldZ = chunk.z * 16 + z;

          // Check if we can place a tree here
          if (world.getBlock(worldX, height, worldZ) === 0) { // Air
            // Generate trunk
            for (let y = height; y < height + 5; y++) {
              world.setBlock(worldX, y, worldZ, logId);
            }

            // Generate leaves
            for (let dx = -2; dx <= 2; dx++) {
              for (let dz = -2; dz <= 2; dz++) {
                for (let dy = 3; dy <= 5; dy++) {
                  if (Math.abs(dx) + Math.abs(dz) + Math.abs(dy - 4) <= 3) {
                    world.setBlock(worldX + dx, height + dy, worldZ + dz, leavesId);
                  }
                }
              }
            }
          }
        }
      }
    }
  },
};
```

## Hot Reloading

Mods support hot reloading - changes to mod files are automatically detected and the mod is reloaded without restarting the server. This makes development much faster.

## Best Practices

1. **Error Handling**: Always wrap your code in try-catch blocks
2. **Performance**: Don't do expensive operations in frequently called event handlers
3. **Compatibility**: Check if blocks/items exist before using them
4. **Cleanup**: Implement the `cleanup()` method to remove event listeners and clean up resources
5. **Documentation**: Comment your code and provide clear mod descriptions

## Distribution

Mods can be distributed as:
- Single `.js` files in the `mods/` directory
- Directories with an `index.js` file and additional assets
- NPM packages (future feature)

## API Versioning

The mod API follows semantic versioning. Breaking changes will be clearly documented in release notes.