# Modding Guide

## Plugin System

The JavaScript Minecraft Clone includes a powerful plugin system that allows you to extend the game without modifying the core code.

## Creating a Plugin

### 1. Plugin Structure

Create a directory in `server/plugins/plugins-data/` with your plugin name:

```
my-plugin/
├── plugin.json    # Plugin manifest
├── main.js        # Plugin entry point
└── config.json    # Optional configuration (created automatically)
```

### 2. Plugin Manifest (plugin.json)

```json
{
  "name": "my-plugin",
  "version": "1.0.0",
  "description": "My awesome plugin",
  "author": "Your Name",
  "apiVersion": "1.0",
  "main": "main.js",
  "dependencies": []
}
```

**Required Fields**:
- `name`: Unique plugin identifier
- `version`: Plugin version (semver)
- `apiVersion`: Compatible API version
- `main`: Entry point file

**Optional Fields**:
- `description`: What the plugin does
- `author`: Creator name
- `dependencies`: Array of required plugin names

### 3. Plugin Code (main.js)

```javascript
// Basic plugin structure
exports.onEnable = function() {
  console.log('Plugin enabled!');
  
  // Register event handlers
  api.on('playerJoin', (data) => {
    api.broadcast(`Welcome ${data.username}!`);
  });
  
  // Register commands
  api.registerCommand('hello', (player, args) => {
    api.sendMessage(player.uuid, 'Hello, world!');
  });
};

exports.onDisable = function() {
  console.log('Plugin disabled!');
};
```

## Plugin API

### Events

Register event handlers to respond to game events:

```javascript
api.on('eventName', (data) => {
  // Handle event
});
```

**Available Events**:
- `playerJoin`: When a player connects
  - `data`: `{ uuid, username }`
- `playerLeave`: When a player disconnects
  - `data`: `{ uuid, username }`
- `playerMove`: When a player moves
  - `data`: `{ uuid, x, y, z }`
- `blockBreak`: When a block is broken
  - `data`: `{ uuid, x, y, z, blockId }`
- `blockPlace`: When a block is placed
  - `data`: `{ uuid, x, y, z, blockId }`
- `chatMessage`: When a chat message is sent
  - `data`: `{ uuid, message }`
- `pluginMessage`: Custom plugin messages from client
  - `data`: `{ player, channel, data }`

### Player API

```javascript
// Get all online players
const players = api.getPlayers();
// Returns: [{ uuid, username, position: {x, y, z}, gameMode }, ...]

// Get specific player
const player = api.getPlayer(uuid);
// Returns: { uuid, username, position: {x, y, z}, gameMode } or null

// Send message to player
api.sendMessage(uuid, 'Hello!');

// Broadcast message to all players
api.broadcast('Server announcement!');
```

### World API

```javascript
// Get block at position
const blockId = api.getBlock(x, y, z);

// Set block at position
api.setBlock(x, y, z, blockId);

// Block IDs are defined in the block registry
// Common blocks:
// 0 = Air, 1 = Stone, 2 = Dirt, 3 = Grass
```

### Command API

```javascript
// Register a command
api.registerCommand('mycommand', (player, args) => {
  // player: { uuid, username }
  // args: array of command arguments
  
  api.sendMessage(player.uuid, `You ran: ${args.join(' ')}`);
});
```

### Configuration API

```javascript
// Get plugin configuration
const config = api.getConfig();

// Modify and save configuration
config.mySetting = 'newValue';
api.saveConfig(config);
```

## Example Plugins

### Welcome Plugin

Sends a welcome message when players join:

```javascript
// plugin.json
{
  "name": "welcome-plugin",
  "version": "1.0.0",
  "apiVersion": "1.0",
  "main": "main.js"
}

// main.js
exports.onEnable = function() {
  api.on('playerJoin', (data) => {
    api.broadcast(`§e${data.username} joined the game!`);
    api.sendMessage(data.uuid, 'Welcome to the server!');
  });
  
  api.on('playerLeave', (data) => {
    api.broadcast(`§e${data.username} left the game!`);
  });
};
```

### Teleport Command Plugin

Adds a /tp command:

```javascript
// plugin.json
{
  "name": "teleport-plugin",
  "version": "1.0.0",
  "apiVersion": "1.0",
  "main": "main.js"
}

// main.js
exports.onEnable = function() {
  api.registerCommand('tp', (player, args) => {
    if (args.length !== 3) {
      api.sendMessage(player.uuid, 'Usage: /tp <x> <y> <z>');
      return;
    }
    
    const x = parseFloat(args[0]);
    const y = parseFloat(args[1]);
    const z = parseFloat(args[2]);
    
    if (isNaN(x) || isNaN(y) || isNaN(z)) {
      api.sendMessage(player.uuid, 'Invalid coordinates!');
      return;
    }
    
    // Teleport player (would need to be implemented in API)
    api.sendMessage(player.uuid, `Teleported to ${x}, ${y}, ${z}`);
  });
};
```

### Auto-Save Plugin

Automatically saves the world periodically:

```javascript
// plugin.json
{
  "name": "auto-save",
  "version": "1.0.0",
  "apiVersion": "1.0",
  "main": "main.js"
}

// main.js
let saveInterval;

exports.onEnable = function() {
  const config = api.getConfig();
  
  // Default to 5 minutes if not configured
  const intervalMs = (config.intervalMinutes || 5) * 60 * 1000;
  
  saveInterval = setInterval(() => {
    api.broadcast('§6Auto-saving world...');
    // Save would be triggered through API
  }, intervalMs);
};

exports.onDisable = function() {
  if (saveInterval) {
    clearInterval(saveInterval);
  }
};
```

## Security Considerations

### Sandboxing

Plugins run in a sandboxed VM environment with limited access:

**Allowed**:
- Plugin API functions
- Standard JavaScript features
- Limited Node.js modules (`path`, `util`)

**Not Allowed**:
- File system access (except through config API)
- Network access
- Process manipulation
- Arbitrary require() calls

### Best Practices

1. **Validate Input**: Always validate user input in commands
2. **Error Handling**: Use try-catch to prevent crashes
3. **Performance**: Avoid heavy computations in event handlers
4. **Resource Cleanup**: Clean up timers/resources in onDisable
5. **Configuration**: Use config API for user-configurable values

## Loading/Unloading Plugins

Plugins are automatically loaded on server start. To manage plugins at runtime:

```javascript
// Load a plugin
pluginManager.loadPlugin('plugin-name');

// Unload a plugin
pluginManager.unloadPlugin('plugin-name');

// Reload a plugin
pluginManager.reloadPlugin('plugin-name');
```

## Debugging

Use console logging in your plugin:

```javascript
console.log('Debug info');    // Shows as [plugin-name] Debug info
console.error('Error info');   // Shows as error with plugin name
console.warn('Warning info');  // Shows as warning with plugin name
```

## Version Compatibility

Always specify the correct `apiVersion` in your manifest. The server will reject plugins with incompatible API versions.

Current API Version: **1.0**

## Support

For issues or questions:
1. Check the examples above
2. Review the plugin manager source code
3. Open an issue on the repository
