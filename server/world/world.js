/**
 * World management - handles chunks, persistence, and world state
 */

const WorldGenerator = require('./generator');
const Chunk = require('./chunk');
const Database = require('better-sqlite3');
const path = require('path');
const fs = require('fs');

class World {
  constructor(worldName, seed) {
    this.worldName = worldName;
    this.seed = seed;
    this.generator = new WorldGenerator(seed);
    this.chunks = new Map();
    this.worldAge = 0;
    this.timeOfDay = 0;

    // Initialize database
    this.initDatabase();
  }

  /**
   * Initialize SQLite database for world persistence
   */
  initDatabase() {
    const worldDir = path.join(__dirname, '../../worlds', this.worldName);
    if (!fs.existsSync(worldDir)) {
      fs.mkdirSync(worldDir, { recursive: true });
    }

    const dbPath = path.join(worldDir, 'world.db');
    this.db = new Database(dbPath);

    // Create tables
    this.db.exec(`
      CREATE TABLE IF NOT EXISTS chunks (
        x INTEGER,
        z INTEGER,
        data BLOB,
        modified INTEGER,
        PRIMARY KEY (x, z)
      );

      CREATE TABLE IF NOT EXISTS metadata (
        key TEXT PRIMARY KEY,
        value TEXT
      );
    `);

    // Load or save metadata
    const metaStmt = this.db.prepare('SELECT value FROM metadata WHERE key = ?');
    const seedRow = metaStmt.get('seed');
    if (!seedRow) {
      const insertStmt = this.db.prepare('INSERT INTO metadata (key, value) VALUES (?, ?)');
      insertStmt.run('seed', this.seed.toString());
      insertStmt.run('worldAge', '0');
      insertStmt.run('timeOfDay', '0');
    } else {
      this.worldAge = parseInt(metaStmt.get('worldAge').value) || 0;
      this.timeOfDay = parseInt(metaStmt.get('timeOfDay').value) || 0;
    }
  }

  /**
   * Get chunk key for Map storage
   */
  getChunkKey(x, z) {
    return `${x},${z}`;
  }

  /**
   * Get or generate a chunk
   */
  getChunk(x, z) {
    const key = this.getChunkKey(x, z);
    
    if (this.chunks.has(key)) {
      return this.chunks.get(key);
    }

    // Try to load from database
    const chunk = this.loadChunk(x, z);
    if (chunk) {
      this.chunks.set(key, chunk);
      return chunk;
    }

    // Generate new chunk
    const newChunk = this.generator.generateChunk(x, z);
    this.chunks.set(key, newChunk);
    this.saveChunk(newChunk);
    return newChunk;
  }

  /**
   * Load chunk from database
   */
  loadChunk(x, z) {
    const stmt = this.db.prepare('SELECT data FROM chunks WHERE x = ? AND z = ?');
    const row = stmt.get(x, z);
    
    if (row) {
      const data = JSON.parse(row.data);
      return Chunk.deserialize(data);
    }
    
    return null;
  }

  /**
   * Save chunk to database
   */
  saveChunk(chunk) {
    const stmt = this.db.prepare(`
      INSERT OR REPLACE INTO chunks (x, z, data, modified)
      VALUES (?, ?, ?, ?)
    `);
    
    stmt.run(
      chunk.x,
      chunk.z,
      JSON.stringify(chunk.serialize()),
      chunk.modified ? 1 : 0
    );
  }

  /**
   * Get block at world coordinates
   */
  getBlock(x, y, z) {
    const chunkX = Math.floor(x / 16);
    const chunkZ = Math.floor(z / 16);
    const chunk = this.getChunk(chunkX, chunkZ);
    
    const localX = ((x % 16) + 16) % 16;
    const localZ = ((z % 16) + 16) % 16;
    
    return chunk.getBlock(localX, y, localZ);
  }

  /**
   * Set block at world coordinates
   */
  setBlock(x, y, z, blockId) {
    const chunkX = Math.floor(x / 16);
    const chunkZ = Math.floor(z / 16);
    const chunk = this.getChunk(chunkX, chunkZ);
    
    const localX = ((x % 16) + 16) % 16;
    const localZ = ((z % 16) + 16) % 16;
    
    const result = chunk.setBlock(localX, y, localZ, blockId);
    if (result) {
      this.saveChunk(chunk);
    }
    return result;
  }

  /**
   * Unload chunks that are far from all players
   */
  unloadDistantChunks(playerPositions, renderDistance) {
    const keysToRemove = [];
    
    for (const [key, chunk] of this.chunks.entries()) {
      let shouldUnload = true;
      
      for (const pos of playerPositions) {
        const playerChunkX = Math.floor(pos.x / 16);
        const playerChunkZ = Math.floor(pos.z / 16);
        const distance = Math.max(
          Math.abs(chunk.x - playerChunkX),
          Math.abs(chunk.z - playerChunkZ)
        );
        
        if (distance <= renderDistance + 2) {
          shouldUnload = false;
          break;
        }
      }
      
      if (shouldUnload) {
        if (chunk.modified) {
          this.saveChunk(chunk);
        }
        keysToRemove.push(key);
      }
    }
    
    keysToRemove.forEach(key => this.chunks.delete(key));
  }

  /**
   * Update world time
   */
  tick() {
    this.worldAge++;
    this.timeOfDay = (this.timeOfDay + 1) % 24000;
    
    // Save time periodically
    if (this.worldAge % 100 === 0) {
      const updateStmt = this.db.prepare('UPDATE metadata SET value = ? WHERE key = ?');
      updateStmt.run(this.worldAge.toString(), 'worldAge');
      updateStmt.run(this.timeOfDay.toString(), 'timeOfDay');
    }
  }

  /**
   * Save all modified chunks
   */
  saveAll() {
    for (const chunk of this.chunks.values()) {
      if (chunk.modified) {
        this.saveChunk(chunk);
        chunk.modified = false;
      }
    }
  }

  /**
   * Close database connection
   */
  close() {
    this.saveAll();
    this.db.close();
  }
}

module.exports = World;
