# JavaScript Minecraft Clone

A JavaScript-based Minecraft clone featuring version 1.21.10 block compatibility, multiplayer support, and extensible mod/plugin system.

## Features

- **Block Compatibility**: All vanilla blocks from Minecraft 1.21.10
- **World Generation**: Procedural terrain generation with multiple biomes
- **Multiplayer**: LAN and internet multiplayer with WebSocket communication
- **Dedicated Servers**: Pterodactyl-compatible for easy deployment
- **Mod/Plugin Support**: Extensible API with sandboxing and version control
- **Performance**: Optimized chunk loading and rendering for large worlds
- **Modern UI**: Intuitive inventory, chat, and player management

## Tech Stack

- **Client**: Three.js for 3D rendering, Vite for development
- **Server**: Node.js with WebSocket support
- **Storage**: SQLite for world persistence
- **Networking**: WebSockets for real-time communication

## Installation

```bash
# Install dependencies
npm install

# Start the server
npm start

# In another terminal, start the client (development mode)
npm run client:dev

# Or build the client for production
npm run client:build
```

## Project Structure

```
javascript-minecraft-clone/
├── client/          # Client-side code
│   ├── src/         # Source files
│   │   ├── engine/  # Rendering engine
│   │   ├── ui/      # User interface
│   │   └── network/ # Client networking
│   └── index.html   # Entry point
├── server/          # Server-side code
│   ├── world/       # World generation and management
│   ├── network/     # Server networking
│   ├── plugins/     # Plugin system
│   └── index.js     # Server entry point
├── shared/          # Shared code between client and server
│   ├── blocks/      # Block definitions
│   ├── protocol/    # Network protocol
│   └── constants/   # Game constants
└── docs/            # Documentation

```

## Configuration

### Server Configuration

Edit `server/config.json` to configure:
- Port number
- Max players
- World settings
- Plugin directory

### Pterodactyl Deployment

A Pterodactyl egg configuration is provided in `pterodactyl/egg.json` for easy server deployment.

## Mod/Plugin Development

See [docs/MODDING.md](docs/MODDING.md) for information on creating mods and plugins.

## Architecture

### Block Registry
All blocks are defined in `shared/blocks/registry.js` with properties matching Minecraft 1.21.10 specifications.

### World System
- Chunk-based world management (16x256x16 chunks)
- Procedural generation using simplex noise
- Persistent storage with SQLite

### Networking
- Client-server architecture using WebSockets
- Efficient delta compression for chunk updates
- Player authentication and session management

### Plugin API
- Event-driven architecture
- Sandboxed execution environment
- Version compatibility checking

## Performance

- Frustum culling for efficient rendering
- Chunk LOD (Level of Detail) system
- Worker threads for world generation
- Lazy chunk loading/unloading

## Contributing

Contributions are welcome! Please read the contributing guidelines before submitting pull requests.

## License

MIT License - see LICENSE file for details
