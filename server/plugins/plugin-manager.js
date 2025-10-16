/**
 * Plugin manager - handles loading, unloading, and sandboxing of plugins
 */

const fs = require('fs');
const path = require('path');
const vm = require('vm');

class PluginManager {
  constructor(server, pluginsDir) {
    this.server = server;
    this.pluginsDir = pluginsDir;
    this.plugins = new Map();
    this.eventHandlers = new Map();
    
    // Initialize plugins directory
    if (!fs.existsSync(pluginsDir)) {
      fs.mkdirSync(pluginsDir, { recursive: true });
    }
  }

  /**
   * Load all plugins from plugins directory
   */
  loadAll() {
    console.log('Loading plugins...');
    
    const pluginFolders = fs.readdirSync(this.pluginsDir)
      .filter(f => fs.statSync(path.join(this.pluginsDir, f)).isDirectory());

    for (const folder of pluginFolders) {
      try {
        this.loadPlugin(folder);
      } catch (error) {
        console.error(`Failed to load plugin ${folder}:`, error);
      }
    }

    console.log(`Loaded ${this.plugins.size} plugins`);
  }

  /**
   * Load a single plugin
   */
  loadPlugin(pluginName) {
    const pluginDir = path.join(this.pluginsDir, pluginName);
    const manifestPath = path.join(pluginDir, 'plugin.json');
    const mainPath = path.join(pluginDir, 'main.js');

    // Check for required files
    if (!fs.existsSync(manifestPath)) {
      throw new Error(`Missing plugin.json for ${pluginName}`);
    }
    if (!fs.existsSync(mainPath)) {
      throw new Error(`Missing main.js for ${pluginName}`);
    }

    // Load manifest
    const manifest = JSON.parse(fs.readFileSync(manifestPath, 'utf8'));
    
    // Validate manifest
    this.validateManifest(manifest);

    // Check version compatibility
    if (manifest.apiVersion !== '1.0') {
      throw new Error(`Incompatible API version: ${manifest.apiVersion}`);
    }

    // Check dependencies
    if (manifest.dependencies) {
      for (const dep of manifest.dependencies) {
        if (!this.plugins.has(dep)) {
          throw new Error(`Missing dependency: ${dep}`);
        }
      }
    }

    // Load plugin code
    const code = fs.readFileSync(mainPath, 'utf8');

    // Create sandboxed API
    const api = this.createPluginAPI(pluginName);

    // Create sandbox context
    const sandbox = {
      console: {
        log: (...args) => console.log(`[${pluginName}]`, ...args),
        error: (...args) => console.error(`[${pluginName}]`, ...args),
        warn: (...args) => console.warn(`[${pluginName}]`, ...args)
      },
      require: (module) => {
        // Only allow specific safe modules
        const allowedModules = ['path', 'util'];
        if (allowedModules.includes(module)) {
          return require(module);
        }
        throw new Error(`Module ${module} is not allowed in plugins`);
      },
      api,
      exports: {}
    };

    // Execute plugin code in sandbox
    vm.createContext(sandbox);
    vm.runInContext(code, sandbox, {
      filename: mainPath,
      timeout: 5000
    });

    // Store plugin
    const plugin = {
      name: pluginName,
      manifest,
      exports: sandbox.exports,
      enabled: true
    };

    this.plugins.set(pluginName, plugin);

    // Call onEnable if it exists
    if (plugin.exports.onEnable) {
      plugin.exports.onEnable();
    }

    console.log(`Loaded plugin: ${pluginName} v${manifest.version}`);
  }

  /**
   * Validate plugin manifest
   */
  validateManifest(manifest) {
    const required = ['name', 'version', 'apiVersion', 'main'];
    for (const field of required) {
      if (!manifest[field]) {
        throw new Error(`Missing required field: ${field}`);
      }
    }
  }

  /**
   * Create plugin API - sandboxed interface for plugins
   */
  createPluginAPI(pluginName) {
    return {
      // Event system
      on: (event, handler) => {
        if (!this.eventHandlers.has(event)) {
          this.eventHandlers.set(event, []);
        }
        this.eventHandlers.get(event).push({
          plugin: pluginName,
          handler
        });
      },

      // Player API
      getPlayers: () => {
        return Array.from(this.server.players.values()).map(p => ({
          uuid: p.uuid,
          username: p.username,
          position: { x: p.x, y: p.y, z: p.z },
          gameMode: p.gameMode
        }));
      },

      getPlayer: (uuid) => {
        const player = this.server.players.get(uuid);
        if (!player) return null;
        return {
          uuid: player.uuid,
          username: player.username,
          position: { x: player.x, y: player.y, z: player.z },
          gameMode: player.gameMode
        };
      },

      sendMessage: (uuid, message) => {
        const player = this.server.players.get(uuid);
        if (player) {
          this.server.sendPacket(player, {
            type: 'CHAT_MESSAGE_SERVER',
            data: { message }
          });
        }
      },

      broadcast: (message) => {
        this.server.broadcast({
          type: 'CHAT_MESSAGE_SERVER',
          data: { message }
        });
      },

      // World API
      getBlock: (x, y, z) => {
        return this.server.world.getBlock(x, y, z);
      },

      setBlock: (x, y, z, blockId) => {
        return this.server.world.setBlock(x, y, z, blockId);
      },

      // Command registration
      registerCommand: (name, handler) => {
        // Store command handler
        // Implementation would be in command system
        console.log(`Plugin ${pluginName} registered command: ${name}`);
      },

      // Config API
      getConfig: () => {
        const configPath = path.join(this.pluginsDir, pluginName, 'config.json');
        if (fs.existsSync(configPath)) {
          return JSON.parse(fs.readFileSync(configPath, 'utf8'));
        }
        return {};
      },

      saveConfig: (config) => {
        const configPath = path.join(this.pluginsDir, pluginName, 'config.json');
        fs.writeFileSync(configPath, JSON.stringify(config, null, 2));
      }
    };
  }

  /**
   * Emit an event to all plugins
   */
  emitEvent(event, data) {
    const handlers = this.eventHandlers.get(event);
    if (!handlers) return;

    for (const { plugin, handler } of handlers) {
      try {
        handler(data);
      } catch (error) {
        console.error(`Error in plugin ${plugin} handling event ${event}:`, error);
      }
    }
  }

  /**
   * Handle plugin message from client
   */
  handleMessage(player, channel, data) {
    this.emitEvent('pluginMessage', {
      player: {
        uuid: player.uuid,
        username: player.username
      },
      channel,
      data
    });
  }

  /**
   * Unload a plugin
   */
  unloadPlugin(pluginName) {
    const plugin = this.plugins.get(pluginName);
    if (!plugin) {
      throw new Error(`Plugin ${pluginName} not found`);
    }

    // Call onDisable if it exists
    if (plugin.exports.onDisable) {
      plugin.exports.onDisable();
    }

    // Remove event handlers
    for (const [event, handlers] of this.eventHandlers.entries()) {
      this.eventHandlers.set(
        event,
        handlers.filter(h => h.plugin !== pluginName)
      );
    }

    this.plugins.delete(pluginName);
    console.log(`Unloaded plugin: ${pluginName}`);
  }

  /**
   * Reload a plugin
   */
  reloadPlugin(pluginName) {
    if (this.plugins.has(pluginName)) {
      this.unloadPlugin(pluginName);
    }
    this.loadPlugin(pluginName);
  }

  /**
   * Get plugin info
   */
  getPluginInfo(pluginName) {
    const plugin = this.plugins.get(pluginName);
    if (!plugin) return null;

    return {
      name: plugin.name,
      version: plugin.manifest.version,
      description: plugin.manifest.description,
      author: plugin.manifest.author,
      enabled: plugin.enabled
    };
  }

  /**
   * List all plugins
   */
  listPlugins() {
    return Array.from(this.plugins.values()).map(p => ({
      name: p.name,
      version: p.manifest.version,
      enabled: p.enabled
    }));
  }

  /**
   * Unload all plugins
   */
  unloadAll() {
    for (const pluginName of this.plugins.keys()) {
      try {
        this.unloadPlugin(pluginName);
      } catch (error) {
        console.error(`Error unloading plugin ${pluginName}:`, error);
      }
    }
  }
}

module.exports = PluginManager;
