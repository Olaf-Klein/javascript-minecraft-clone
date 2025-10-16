// Shared constants and utilities for Minecraft clone

const BLOCK_TYPES = {
  AIR: 0,
  GRASS: 1,
  DIRT: 2,
  STONE: 3,
  COBBLESTONE: 4,
  WOOD: 5,
  LEAVES: 6,
  SAND: 7,
  GRAVEL: 8,
  GOLD_ORE: 9,
  IRON_ORE: 10,
  COAL_ORE: 11,
  LOG: 12,
  PLANKS: 13,
  WOOL: 14,
  BEDROCK: 15,
  WATER: 16,
  LAVA: 17,
  // TODO: Add all vanilla blocks (hundreds more)
};

const CHUNK_SIZE = 16;
const WORLD_HEIGHT = 256;

module.exports = {
  BLOCK_TYPES,
  CHUNK_SIZE,
  WORLD_HEIGHT,
};