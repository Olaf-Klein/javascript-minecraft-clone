/**
 * Main server entry point
 */

const fs = require('fs');
const path = require('path');
const World = require('./world/world');
const GameServer = require('./network/server');
const PluginManager = require('./plugins/plugin-manager');

// Load configuration
const configPath = path.join(__dirname, 'config.json');
let config = {
  port: 3000,
  maxPlayers: 20,
  renderDistance: 8,
  worldName: 'world',
  seed: 12345,
  pluginsEnabled: true,
  pluginsDir: path.join(__dirname, 'plugins', 'plugins-data')
};

if (fs.existsSync(configPath)) {
  const loadedConfig = JSON.parse(fs.readFileSync(configPath, 'utf8'));
  config = { ...config, ...loadedConfig };
}

console.log('=================================');
console.log('JavaScript Minecraft Server');
console.log('=================================');
console.log('Starting server...');
console.log(`World: ${config.worldName}`);
console.log(`Seed: ${config.seed}`);

// Initialize world
const world = new World(config.worldName, config.seed);

// Initialize server
const server = new GameServer(world, config);

// Initialize plugin manager
let pluginManager = null;
if (config.pluginsEnabled) {
  pluginManager = new PluginManager(server, config.pluginsDir);
  server.pluginManager = pluginManager;
  
  try {
    pluginManager.loadAll();
  } catch (error) {
    console.error('Error loading plugins:', error);
  }
}

// Start server
server.start();

// Auto-save interval
if (config.autoSave) {
  setInterval(() => {
    console.log('Auto-saving world...');
    world.saveAll();
  }, config.autoSaveInterval || 300000); // Default 5 minutes
}

// Handle shutdown gracefully
const shutdown = () => {
  console.log('\nShutting down server...');
  
  server.stop();
  
  if (pluginManager) {
    pluginManager.unloadAll();
  }
  
  process.exit(0);
};

process.on('SIGINT', shutdown);
process.on('SIGTERM', shutdown);

// Handle errors
process.on('uncaughtException', (error) => {
  console.error('Uncaught exception:', error);
  shutdown();
});

process.on('unhandledRejection', (reason, promise) => {
  console.error('Unhandled rejection at:', promise, 'reason:', reason);
});

console.log('Server is ready!');
console.log(`Listening on port ${config.port}`);
console.log('Press Ctrl+C to stop');
