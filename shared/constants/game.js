/**
 * Game constants shared between client and server
 */

// Chunk dimensions
export const CHUNK_SIZE = 16;
export const CHUNK_HEIGHT = 256;
export const RENDER_DISTANCE = 8;

// World generation
export const WORLD_SEED = 12345;
export const SEA_LEVEL = 64;
export const MAX_HEIGHT = 320;
export const MIN_HEIGHT = -64;

// Network
export const PROTOCOL_VERSION = "1.21.10";
export const MAX_PACKET_SIZE = 1024 * 1024; // 1MB
export const TICK_RATE = 20; // ticks per second

// Player
export const PLAYER_HEIGHT = 1.8;
export const PLAYER_WIDTH = 0.6;
export const PLAYER_SPEED = 4.317; // blocks per second
export const PLAYER_JUMP_VELOCITY = 8.0;
export const GRAVITY = 32.0;

// Game modes
export const GAME_MODES = {
  SURVIVAL: 0,
  CREATIVE: 1,
  ADVENTURE: 2,
  SPECTATOR: 3
};

// Block faces
export const FACES = {
  TOP: 0,
  BOTTOM: 1,
  NORTH: 2,
  SOUTH: 3,
  EAST: 4,
  WEST: 5
};

module.exports = {
  CHUNK_SIZE,
  CHUNK_HEIGHT,
  RENDER_DISTANCE,
  WORLD_SEED,
  SEA_LEVEL,
  MAX_HEIGHT,
  MIN_HEIGHT,
  PROTOCOL_VERSION,
  MAX_PACKET_SIZE,
  TICK_RATE,
  PLAYER_HEIGHT,
  PLAYER_WIDTH,
  PLAYER_SPEED,
  PLAYER_JUMP_VELOCITY,
  GRAVITY,
  GAME_MODES,
  FACES
};
