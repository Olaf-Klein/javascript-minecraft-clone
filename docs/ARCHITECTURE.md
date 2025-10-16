# Architecture Documentation

## Overview

The JavaScript Minecraft Clone is built with a client-server architecture, optimized for multiplayer gameplay and extensibility through a plugin system.

## System Components

### Client

**Technology**: Three.js + WebSockets

**Components**:
- **Renderer**: Handles 3D graphics using Three.js
  - Chunk-based mesh generation
  - Face culling for performance
  - Dynamic chunk loading/unloading
  
- **Player Controller**: 
  - First-person controls
  - Physics simulation (gravity, jumping)
  - Input handling
  
- **Network Client**:
  - WebSocket connection management
  - Packet serialization/deserialization
  - Event-driven architecture

### Server

**Technology**: Node.js + WebSockets + SQLite

**Components**:
- **World System**:
  - Procedural generation using simplex noise
  - Chunk management
  - Persistent storage with SQLite
  
- **Network Server**:
  - WebSocket server
  - Player session management
  - Packet routing and broadcasting
  
- **Plugin Manager**:
  - Sandboxed plugin execution
  - Event system
  - API for world/player manipulation

### Shared

**Components**:
- **Block Registry**: Central registry of all block types
- **Protocol**: Network packet definitions
- **Constants**: Shared game constants

## Data Flow

### Connection Flow
```
Client -> Handshake -> Server
Server validates protocol version
Client -> Login -> Server
Server creates player session
Server -> World Info -> Client
Server -> Chunk Data -> Client (multiple)
Client renders world
```

### Block Updates
```
Client detects block interaction
Client -> Block Change -> Server
Server validates and updates world
Server -> Block Change -> All Clients
Clients update local rendering
```

### Plugin Events
```
Game Event occurs
Server emits event to PluginManager
PluginManager calls registered handlers
Plugins can modify game state
```

## Performance Optimizations

### Client
- **Frustum Culling**: Only render visible chunks
- **Face Culling**: Only render exposed block faces
- **Batch Rendering**: Combine blocks into chunk meshes
- **Lazy Loading**: Load chunks as needed

### Server
- **Chunk Unloading**: Remove distant chunks from memory
- **Delta Compression**: Run-length encoding for chunk data
- **Spatial Partitioning**: Chunk-based world storage
- **Database Indexing**: Efficient world persistence

## Security

### Plugin Sandboxing
- VM-based execution context
- Limited module access
- Timeout protection
- API-only world access

### Network
- Protocol version checking
- Keep-alive mechanism
- Max packet size limits
- Input validation

## Scalability

### Horizontal Scaling (Future)
- Distributed chunk servers
- Load balancing
- Shared world database
- Redis for session management

### Vertical Scaling
- Worker threads for generation
- Async I/O operations
- Connection pooling
- Memory management

## Extension Points

### Adding Blocks
1. Register in `shared/blocks/registry.js`
2. Add texture/material in client renderer
3. Update generator if needed for natural generation

### Adding Entities
1. Create entity class
2. Add network packets for sync
3. Implement client-side rendering
4. Add server-side logic

### Creating Plugins
1. Create plugin directory
2. Add `plugin.json` manifest
3. Implement `main.js` with API calls
4. Use event system for hooks

## Future Improvements

- Advanced lighting system
- Entity system (mobs, items)
- Inventory and crafting
- Biome-specific generation
- Advanced physics
- Audio system
- Particle effects
- Advanced UI (menus, HUD)
