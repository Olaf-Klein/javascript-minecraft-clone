// Mod system for Minecraft clone
const fs = require('fs');
const path = require('path');

class ModManager {
  constructor() {
    this.mods = new Map();
    this.modPath = path.join(__dirname, '..', 'mods');
    this.eventHooks = new Map();
    this.blockRegistry = new Map();
    this.itemRegistry = new Map();
    this.loadedMods = new Set();
  }

  // Initialize the mod system
  async initialize() {
    console.log('Initializing mod system...');

    // Set up file watchers for hot-reloading
    this.setupFileWatchers();

    // Load all mods
    await this.loadAllMods();

    console.log(`Loaded ${this.loadedMods.size} mods`);
  }

  // Set up file watchers for hot-reloading
  setupFileWatchers() {
    if (!fs.existsSync(this.modPath)) {
      fs.mkdirSync(this.modPath, { recursive: true });
    }

    // Watch for new mod files/directories
    fs.watch(this.modPath, { recursive: true }, (eventType, filename) => {
      if (filename && (filename.endsWith('.js') || filename.endsWith('.json'))) {
        console.log(`Mod file changed: ${filename}, reloading...`);
        this.reloadMod(filename);
      }
    });
  }

  // Load all mods from the mods directory
  async loadAllMods() {
    const modFiles = fs.readdirSync(this.modPath);

    for (const file of modFiles) {
      const filePath = path.join(this.modPath, file);
      const stat = fs.statSync(filePath);

      if (stat.isDirectory()) {
        await this.loadModFromDirectory(filePath);
      } else if (file.endsWith('.js')) {
        await this.loadModFromFile(filePath);
      }
    }
  }

  // Load a mod from a directory
  async loadModFromDirectory(dirPath) {
    const modName = path.basename(dirPath);
    const mainFile = path.join(dirPath, 'index.js');

    if (fs.existsSync(mainFile)) {
      try {
        const mod = require(mainFile);
        await this.registerMod(modName, mod);
      } catch (error) {
        console.error(`Failed to load mod ${modName}:`, error);
      }
    }
  }

  // Load a mod from a single file
  async loadModFromFile(filePath) {
    const modName = path.basename(filePath, '.js');

    try {
      delete require.cache[require.resolve(filePath)]; // Clear cache for hot-reloading
      const mod = require(filePath);
      await this.registerMod(modName, mod);
    } catch (error) {
      console.error(`Failed to load mod ${modName}:`, error);
    }
  }

  // Register a mod with the system
  async registerMod(name, mod) {
    if (this.loadedMods.has(name)) {
      await this.unloadMod(name);
    }

    console.log(`Loading mod: ${name}`);

    // Initialize mod if it has an init function
    if (typeof mod.initialize === 'function') {
      await mod.initialize(this.getModAPI(name));
    }

    this.mods.set(name, mod);
    this.loadedMods.add(name);

    console.log(`Mod ${name} loaded successfully`);
  }

  // Unload a mod
  async unloadMod(name) {
    const mod = this.mods.get(name);
    if (mod && typeof mod.cleanup === 'function') {
      await mod.cleanup();
    }

    // Remove event hooks
    this.eventHooks.forEach((hooks, eventName) => {
      hooks.delete(name);
    });

    // Remove registered blocks/items
    this.blockRegistry.forEach((block, blockName) => {
      if (block.mod === name) {
        this.blockRegistry.delete(blockName);
      }
    });

    this.itemRegistry.forEach((item, itemName) => {
      if (item.mod === name) {
        this.itemRegistry.delete(itemName);
      }
    });

    this.mods.delete(name);
    this.loadedMods.delete(name);

    // Clear require cache
    const modPath = path.join(this.modPath, name);
    if (fs.existsSync(modPath + '.js')) {
      delete require.cache[require.resolve(modPath + '.js')];
    }

    console.log(`Mod ${name} unloaded`);
  }

  // Reload a specific mod
  async reloadMod(filename) {
    const modName = path.basename(filename, path.extname(filename));
    if (this.loadedMods.has(modName)) {
      console.log(`Reloading mod: ${modName}`);
      await this.unloadMod(modName);

      const filePath = path.join(this.modPath, filename);
      if (fs.existsSync(filePath)) {
        await this.loadModFromFile(filePath);
      }
    }
  }

  // Get API for mods to use
  getModAPI(modName) {
    return {
      // Event system
      on: (eventName, callback) => this.registerEventHook(modName, eventName, callback),
      off: (eventName) => this.unregisterEventHook(modName, eventName),
      emit: (eventName, ...args) => this.emitEvent(eventName, ...args),

      // Block registration
      registerBlock: (blockName, blockData) => this.registerBlock(modName, blockName, blockData),
      getBlock: (blockName) => this.blockRegistry.get(blockName),

      // Item registration
      registerItem: (itemName, itemData) => this.registerItem(modName, itemName, itemData),
      getItem: (itemName) => this.itemRegistry.get(itemName),

      // World manipulation
      getWorld: () => this.world,
      setWorld: (world) => { this.world = world; },

      // Utility functions
      log: (message) => console.log(`[${modName}] ${message}`),
      error: (message) => console.error(`[${modName}] ${message}`),
    };
  }

  // Event system
  registerEventHook(modName, eventName, callback) {
    if (!this.eventHooks.has(eventName)) {
      this.eventHooks.set(eventName, new Map());
    }
    this.eventHooks.get(eventName).set(modName, callback);
  }

  unregisterEventHook(modName, eventName) {
    const hooks = this.eventHooks.get(eventName);
    if (hooks) {
      hooks.delete(modName);
    }
  }

  emitEvent(eventName, ...args) {
    const hooks = this.eventHooks.get(eventName);
    if (hooks) {
      hooks.forEach((callback, modName) => {
        try {
          callback(...args);
        } catch (error) {
          console.error(`Error in mod ${modName} event ${eventName}:`, error);
        }
      });
    }
  }

  // Block registration
  registerBlock(modName, blockName, blockData) {
    const fullName = `${modName}:${blockName}`;
    this.blockRegistry.set(fullName, {
      ...blockData,
      mod: modName,
      name: fullName,
    });
    console.log(`Registered block: ${fullName}`);
    return fullName;
  }

  // Item registration
  registerItem(modName, itemName, itemData) {
    const fullName = `${modName}:${itemName}`;
    this.itemRegistry.set(fullName, {
      ...itemData,
      mod: modName,
      name: fullName,
    });
    console.log(`Registered item: ${fullName}`);
    return fullName;
  }

  // Get all registered blocks
  getAllBlocks() {
    return Array.from(this.blockRegistry.values());
  }

  // Get all registered items
  getAllItems() {
    return Array.from(this.itemRegistry.values());
  }

  // Get loaded mods
  getLoadedMods() {
    return Array.from(this.loadedMods);
  }
}

module.exports = { ModManager };