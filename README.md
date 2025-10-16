# JavaScript Minecraft Clone

A complete recreation of Minecraft in JavaScript, featuring a desktop client built with Electron and a dedicated server compatible with Pterodactyl.

## Features

- Desktop application (not browser-based)
- Full vanilla Minecraft blocks and mechanics (targeting 1.21.10 compatibility)
- LAN and dedicated server multiplayer
- World creation and management
- Mod and plugin support
- Pterodactyl panel compatibility for dedicated servers
- Configurable graphics settings with quality presets (Low/Medium/High/Ultra)
- Hardware auto-detection for optimal performance
- Advanced graphics features: ray tracing simulation, volumetric lighting, dynamic shadows

## Graphics Settings

The game includes comprehensive graphics configuration to ensure optimal performance across all hardware levels:

### Quality Presets
- **Low**: Basic rendering for low-end devices (30+ FPS on integrated graphics)
- **Medium**: Balanced settings for mainstream hardware (60+ FPS)
- **High**: Enhanced visuals with advanced effects (60+ FPS on mid-range GPUs)
- **Ultra**: Maximum quality with all features enabled (for high-end hardware)

### Advanced Features
- **Textures**: Procedural Minecraft-style textures for all blocks
- **Normal Mapping**: Surface detail and depth perception for realistic block surfaces
- **PBR Materials**: Physically-based rendering with roughness and metalness properties
- **Custom Shaders**: Built-in shader effects including:
  - Enhanced lighting with specular highlights
  - Ambient occlusion for realistic shadowing
  - Water shaders with refraction and wave animation
  - Glass shaders with chromatic aberration
  - Ray tracing simulation for reflective surfaces
- **Volumetric Lighting**: Atmospheric scattering and god rays effects
- **Dynamic Shadows**: High-quality shadow mapping with soft shadows
- **Tone Mapping**: Advanced color grading and exposure control

### Texture System
- **Procedural Textures**: Automatically generated Minecraft-style textures when real textures aren't available
- **Texture Atlas**: Efficient rendering with combined texture sheets
- **Block-Specific Materials**: Each block type has appropriate visual properties (roughness, metalness, transparency)

Settings are automatically saved and persist between sessions.

## Project Structure

- `client/` - Electron desktop application
- `server/` - Node.js dedicated server
- `shared/` - Common code and constants
- `mods/` - Mod and plugin directory
- `assets/` - Textures, sounds, and other assets
- `docs/` - Documentation

## Getting Started

1. Install dependencies:
   ```bash
   npm install
   cd client && npm install
   cd ../server && npm install
   cd ../shared && npm install
   ```

2. Start the server:
   ```bash
   npm run start:server
   ```

3. Start the client:
   ```bash
   npm run start:client
   ```

## Building Executables

To create standalone executable files for easy distribution:

1. Install all dependencies:
   ```bash
   npm run install:all
   ```

2. Check build setup:
   ```bash
   cd client && npm run check-setup
   ```

3. Build for your platform:
   ```bash
   # Windows
   cd client && npm run dist:win

   # macOS
   cd client && npm run dist:mac

   # Linux
   cd client && npm run dist:linux

   # All platforms
   cd client && npm run dist
   ```

The executables will be created in `client/dist/` and can be distributed to users for easy installation.

See `docs/build-executables.md` for detailed build instructions.

## Development

- Client uses Electron with Three.js for 3D rendering
- Server uses Node.js with Socket.IO for networking
- Shared code is bundled with Webpack

## Modding

The game features a comprehensive modding system that allows you to extend gameplay with custom blocks, items, and mechanics.

### Features
- **Hot Reloading**: Mods reload automatically when files are changed
- **Event System**: Hook into game events like world generation, block breaking, and player actions
- **Block/Item Registration**: Add custom blocks and items with full properties
- **World Manipulation**: Modify terrain generation and world data
- **Plugin API**: Clean API for creating complex modifications

### Creating Mods
Mods are JavaScript files placed in the `mods/` directory. See `docs/modding.md` for the complete API documentation and examples.

### Example Mod
```javascript
module.exports = {
  name: 'My First Mod',
  version: '1.0.0',

  async initialize(api) {
    // Register a custom block
    const myBlockId = api.registerBlock('my_block', {
      hardness: 2.0,
      tool: 'pickaxe',
      drops: 'my_item',
      color: 0xff0000,
    });

    // Register an item
    api.registerItem('my_item', {
      name: 'My Item',
      maxStack: 64,
    });

    api.log('My mod loaded!');
  },
};
```

Mods are loaded automatically on server startup and support hot reloading during development.

## Server Setup

For dedicated servers, use the server component. It's compatible with Pterodactyl panel.

### Pterodactyl Configuration
- **Egg Import**: Use `pterodactyl-egg.json` to import the server egg into Pterodactyl
- **Startup Command**: `npm start` (handled by egg)
- **Docker Image**: `ghcr.io/pterodactyl/images:node-18` (handled by egg)
- **Environment Variables**: Configurable via Pterodactyl panel (name, max players, game mode, etc.)
- **Installation**: Automatic via egg installation script

See `docs/pterodactyl-setup.md` for detailed setup instructions.

## License

MIT