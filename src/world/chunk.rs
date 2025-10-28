#![allow(dead_code)]
use super::block::BlockType;
use noise::{NoiseFn, Perlin};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

pub const CHUNK_SIZE: usize = 16;
pub const WORLD_HEIGHT: usize = 256;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub x: i32,
    pub z: i32,
    pub blocks: Vec<BlockType>,
}

impl Chunk {
    pub fn new(x: i32, z: i32) -> Self {
        Self {
            x,
            z,
            blocks: vec![BlockType::Air; CHUNK_SIZE * WORLD_HEIGHT * CHUNK_SIZE],
        }
    }

    pub fn get_block(&self, x: usize, y: usize, z: usize) -> BlockType {
        if x >= CHUNK_SIZE || y >= WORLD_HEIGHT || z >= CHUNK_SIZE {
            return BlockType::Air;
        }
        self.blocks[x + y * CHUNK_SIZE + z * CHUNK_SIZE * WORLD_HEIGHT]
    }

    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block: BlockType) {
        if x >= CHUNK_SIZE || y >= WORLD_HEIGHT || z >= CHUNK_SIZE {
            return;
        }
        self.blocks[x + y * CHUNK_SIZE + z * CHUNK_SIZE * WORLD_HEIGHT] = block;
    }
}

pub struct World {
    chunks: HashMap<(i32, i32), Chunk>,
    noise: Perlin,
    seed: u32,
    world_dir: Option<std::path::PathBuf>,
    dirty_chunks: HashSet<(i32, i32)>,
}

impl World {
    pub fn new(world_dir: Option<std::path::PathBuf>, seed: Option<u32>) -> Self {
        // If a world dir is provided, try to read seed from metadata; otherwise use provided seed or random
        let mut used_seed = seed.unwrap_or_else(|| rand::random());
        if let Some(ref dir) = world_dir {
            let mut meta_path = dir.clone();
            meta_path.push("world_meta.json");
            if let Ok(contents) = std::fs::read_to_string(&meta_path) {
                if let Ok(meta) = serde_json::from_str::<serde_json::Value>(&contents) {
                    if let Some(s) = meta.get("seed").and_then(|v| v.as_u64()) {
                        used_seed = s as u32;
                    }
                }
            } else {
                // Save metadata with chosen seed
                let meta = serde_json::json!({ "seed": used_seed });
                let _ = std::fs::create_dir_all(dir);
                let _ = std::fs::write(&meta_path, serde_json::to_string_pretty(&meta).unwrap());
            }
        }

        Self {
            chunks: HashMap::new(),
            noise: Perlin::new(used_seed),
            seed: used_seed,
            world_dir,
            dirty_chunks: HashSet::new(),
        }
    }

    pub fn get_chunk(&mut self, chunk_x: i32, chunk_z: i32) -> &Chunk {
        if !self.chunks.contains_key(&(chunk_x, chunk_z)) {
            let chunk = self
                .load_chunk_from_disk(chunk_x, chunk_z)
                .unwrap_or_else(|| {
                    let generated = self.generate_chunk(chunk_x, chunk_z);
                    self.dirty_chunks.insert((chunk_x, chunk_z));
                    generated
                });
            self.chunks.insert((chunk_x, chunk_z), chunk);
        }
        self.chunks.get(&(chunk_x, chunk_z)).unwrap()
    }

    pub fn get_chunk_mut(&mut self, chunk_x: i32, chunk_z: i32) -> &mut Chunk {
        if !self.chunks.contains_key(&(chunk_x, chunk_z)) {
            let chunk = self
                .load_chunk_from_disk(chunk_x, chunk_z)
                .unwrap_or_else(|| {
                    let generated = self.generate_chunk(chunk_x, chunk_z);
                    self.dirty_chunks.insert((chunk_x, chunk_z));
                    generated
                });
            self.chunks.insert((chunk_x, chunk_z), chunk);
        }
        self.chunks.get_mut(&(chunk_x, chunk_z)).unwrap()
    }

    fn generate_chunk(&self, chunk_x: i32, chunk_z: i32) -> Chunk {
        let mut chunk = Chunk::new(chunk_x, chunk_z);
        let chunk_world_x = chunk_x * CHUNK_SIZE as i32;
        let chunk_world_z = chunk_z * CHUNK_SIZE as i32;

        // Generate heightmap
        let mut height_map = [[0usize; CHUNK_SIZE]; CHUNK_SIZE];
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let world_x = chunk_world_x + x as i32;
                let world_z = chunk_world_z + z as i32;

                // Base terrain height (40-120)
                let base_height =
                    40.0 + self.sample_noise(world_x as f64, world_z as f64, 6, 0.5, 0.005) * 80.0;

                // Add hills and valleys
                let hill_noise =
                    self.sample_noise(world_x as f64, world_z as f64, 3, 0.7, 0.02) * 20.0;
                let valley_noise =
                    self.sample_noise(world_x as f64, world_z as f64, 2, 0.8, 0.01) * 15.0;

                height_map[x][z] = (base_height + hill_noise - valley_noise)
                    .max(1.0)
                    .min(WORLD_HEIGHT as f64 - 1.0) as usize;
            }
        }

        // Generate blocks
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let height = height_map[x][z];

                for y in 0..WORLD_HEIGHT {
                    let block = if y == 0 {
                        // Bedrock layer
                        BlockType::Bedrock
                    } else if y < height.saturating_sub(3) {
                        // Deep stone
                        if y < 16 {
                            BlockType::Deepslate
                        } else {
                            BlockType::Stone
                        }
                    } else if y < height.saturating_sub(1) {
                        // Dirt layer
                        BlockType::Dirt
                    } else if y < height {
                        // Surface block
                        if height > 90 {
                            BlockType::Stone // Mountain peaks
                        } else if height < 65 {
                            BlockType::Sand // Beaches
                        } else {
                            BlockType::GrassBlock
                        }
                    } else if y < 63 {
                        // Water below sea level
                        BlockType::Water
                    } else {
                        BlockType::Air
                    };

                    chunk.set_block(x, y, z, block);
                }

                // Add ores
                self.generate_ores(&mut chunk, x, z, height);
            }
        }

        chunk
    }

    fn generate_ores(&self, chunk: &mut Chunk, x: usize, z: usize, surface_height: usize) {
        let chunk_world_x = chunk.x * CHUNK_SIZE as i32;
        let chunk_world_z = chunk.z * CHUNK_SIZE as i32;
        let world_x = chunk_world_x + x as i32;
        let world_z = chunk_world_z + z as i32;

        for y in 1..surface_height.min(WORLD_HEIGHT - 1) {
            let ore_noise =
                self.noise
                    .get([world_x as f64 * 0.1, y as f64 * 0.1, world_z as f64 * 0.1]);

            // Coal (common, all heights)
            if ore_noise > 0.6 && y > 0 && y < 128 {
                chunk.set_block(
                    x,
                    y,
                    z,
                    if y < 16 {
                        BlockType::DeepslateCoalOre
                    } else {
                        BlockType::CoalOre
                    },
                );
            }
            // Iron (medium depth)
            else if ore_noise > 0.7 && y > 0 && y < 64 {
                chunk.set_block(
                    x,
                    y,
                    z,
                    if y < 16 {
                        BlockType::DeepslateIronOre
                    } else {
                        BlockType::IronOre
                    },
                );
            }
            // Gold (deeper)
            else if ore_noise > 0.75 && y > 0 && y < 32 {
                chunk.set_block(
                    x,
                    y,
                    z,
                    if y < 16 {
                        BlockType::DeepslateGoldOre
                    } else {
                        BlockType::GoldOre
                    },
                );
            }
            // Diamond (very deep, rare)
            else if ore_noise > 0.8 && y > 1 && y < 20 {
                chunk.set_block(
                    x,
                    y,
                    z,
                    if y < 16 {
                        BlockType::DeepslateDiamondOre
                    } else {
                        BlockType::DiamondOre
                    },
                );
            }
        }
    }

    fn sample_noise(&self, x: f64, z: f64, octaves: i32, persistence: f64, scale: f64) -> f64 {
        let mut value = 0.0;
        let mut amplitude = 1.0;
        let mut frequency = scale;
        let mut max_value = 0.0;

        for _ in 0..octaves {
            value += self.noise.get([x * frequency, z * frequency]) * amplitude;
            max_value += amplitude;
            amplitude *= persistence;
            frequency *= 2.0;
        }

        (value / max_value + 1.0) / 2.0 // Normalize to 0-1
    }

    pub fn get_loaded_chunks(&self) -> Vec<(i32, i32)> {
        self.chunks.keys().copied().collect()
    }

    pub fn is_chunk_loaded(&self, chunk_x: i32, chunk_z: i32) -> bool {
        self.chunks.contains_key(&(chunk_x, chunk_z))
    }

    pub fn has_dirty_chunks(&self) -> bool {
        !self.dirty_chunks.is_empty()
    }

    pub fn seed(&self) -> u32 {
        self.seed
    }

    pub fn save_meta(&self) {
        if let Some(ref dir) = self.world_dir {
            let mut meta_path = dir.clone();
            meta_path.push("world_meta.json");
            let meta = serde_json::json!({ "seed": self.seed });
            let _ = std::fs::create_dir_all(dir);
            let _ = std::fs::write(&meta_path, serde_json::to_string_pretty(&meta).unwrap());
        }
    }

    pub fn save_dirty_chunks(&mut self) {
        if self.dirty_chunks.is_empty() {
            return;
        }

        if self.world_dir.is_none() {
            self.dirty_chunks.clear();
            return;
        }

        let to_save: Vec<(i32, i32)> = self.dirty_chunks.drain().collect();
        for (chunk_x, chunk_z) in to_save {
            if let Some(chunk) = self.chunks.get(&(chunk_x, chunk_z)) {
                if let Err(err) = self.save_chunk_to_disk(chunk) {
                    log::warn!(
                        "Failed to save chunk ({}, {}): {}",
                        chunk_x, chunk_z, err
                    );
                    // Keep chunk marked dirty so we can retry later
                    self.dirty_chunks.insert((chunk_x, chunk_z));
                }
            }
        }
    }

    fn split_world_coord(coord: i32) -> (i32, usize) {
        let size = CHUNK_SIZE as i32;
        let chunk = coord.div_euclid(size);
        let local = coord.rem_euclid(size) as usize;
        (chunk, local)
    }

    pub fn get_block_at(&mut self, world_x: i32, world_y: i32, world_z: i32) -> BlockType {
        if world_y < 0 || world_y >= WORLD_HEIGHT as i32 {
            return BlockType::Air;
        }

        let (chunk_x, local_x) = Self::split_world_coord(world_x);
        let (chunk_z, local_z) = Self::split_world_coord(world_z);
        let chunk = self.get_chunk(chunk_x, chunk_z);
        chunk.get_block(local_x, world_y as usize, local_z)
    }

    pub fn set_block_at(
        &mut self,
        world_x: i32,
        world_y: i32,
        world_z: i32,
        block: BlockType,
    ) -> Option<(i32, i32)> {
        if world_y < 0 || world_y >= WORLD_HEIGHT as i32 {
            return None;
        }

        let (chunk_x, local_x) = Self::split_world_coord(world_x);
        let (chunk_z, local_z) = Self::split_world_coord(world_z);
        let chunk = self.get_chunk_mut(chunk_x, chunk_z);
        let current = chunk.get_block(local_x, world_y as usize, local_z);
        if current == block {
            return None;
        }
        chunk.set_block(local_x, world_y as usize, local_z, block);
        self.dirty_chunks.insert((chunk_x, chunk_z));
        Some((chunk_x, chunk_z))
    }

    fn chunk_directory(&self) -> Option<PathBuf> {
        let mut dir = self.world_dir.clone()?;
        dir.push("chunks");
        Some(dir)
    }

    fn chunk_path(&self, chunk_x: i32, chunk_z: i32) -> Option<PathBuf> {
        let mut dir = self.chunk_directory()?;
        dir.push(format!("chunk_{}_{}.bin", chunk_x, chunk_z));
        Some(dir)
    }

    fn load_chunk_from_disk(&self, chunk_x: i32, chunk_z: i32) -> Option<Chunk> {
        let path = self.chunk_path(chunk_x, chunk_z)?;
        let data = std::fs::read(path).ok()?;
        bincode::deserialize(&data).ok()
    }

    fn save_chunk_to_disk(&self, chunk: &Chunk) -> anyhow::Result<()> {
        let Some(dir) = self.chunk_directory() else {
            return Ok(());
        };
        std::fs::create_dir_all(&dir)?;
        let mut path = dir;
        path.push(format!("chunk_{}_{}.bin", chunk.x, chunk.z));
        let data = bincode::serialize(chunk)?;
        std::fs::write(path, data)?;
        Ok(())
    }
}
