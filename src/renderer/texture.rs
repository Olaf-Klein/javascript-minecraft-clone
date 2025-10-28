#![allow(dead_code)]
use crate::world::BlockType;
use image::{ImageBuffer, Rgba, RgbaImage};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextureKey {
    GrassTop,
    GrassSide,
    Dirt,
    Stone,
    Deepslate,
    Bedrock,
    Sand,
    Cobblestone,
    OakPlanks,
    OakLog,
    OakLeaves,
    Glass,
    CoalOre,
    IronOre,
    GoldOre,
    DiamondOre,
    DeepslateCoalOre,
    DeepslateIronOre,
    DeepslateGoldOre,
    DeepslateDiamondOre,
    Water,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlockFace {
    Top,
    Bottom,
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy)]
pub struct AtlasUV {
    pub u_min: f32,
    pub v_min: f32,
    pub u_max: f32,
    pub v_max: f32,
}

#[derive(Debug)]
pub struct TextureResolver {
    tile_size: u32,
    uv_map: HashMap<TextureKey, AtlasUV>,
    fallback: AtlasUV,
}

impl TextureResolver {
    pub fn tile_size(&self) -> u32 {
        self.tile_size
    }

    pub fn uv(&self, key: TextureKey) -> AtlasUV {
        *self.uv_map.get(&key).unwrap_or(&self.fallback)
    }
}

pub struct AtlasBuildOutput {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
    pub resolver: Arc<TextureResolver>,
}

const TEXTURE_SEQUENCE: &[TextureKey] = &[
    TextureKey::GrassTop,
    TextureKey::GrassSide,
    TextureKey::Dirt,
    TextureKey::Stone,
    TextureKey::Deepslate,
    TextureKey::Bedrock,
    TextureKey::Sand,
    TextureKey::Cobblestone,
    TextureKey::OakPlanks,
    TextureKey::OakLog,
    TextureKey::OakLeaves,
    TextureKey::Glass,
    TextureKey::CoalOre,
    TextureKey::IronOre,
    TextureKey::GoldOre,
    TextureKey::DiamondOre,
    TextureKey::DeepslateCoalOre,
    TextureKey::DeepslateIronOre,
    TextureKey::DeepslateGoldOre,
    TextureKey::DeepslateDiamondOre,
    TextureKey::Water,
];

pub fn build_atlas(tile_size: u32) -> AtlasBuildOutput {
    let count = TEXTURE_SEQUENCE.len() as u32;
    let columns = (count.min(6)).max(1);
    let rows = (count + columns - 1) / columns;
    let width = columns * tile_size;
    let height = rows * tile_size;

    let mut atlas: RgbaImage = ImageBuffer::new(width, height);
    let mut uv_map = HashMap::new();

    for (index, key) in TEXTURE_SEQUENCE.iter().enumerate() {
        let tile = generate_tile(*key, tile_size);
        let idx = index as u32;
        let x = idx % columns;
        let y = idx / columns;
        blit_tile(&mut atlas, &tile, x * tile_size, y * tile_size);

        let texel_w = 0.5 / width as f32;
        let texel_h = 0.5 / height as f32;
        let u_min = (x * tile_size) as f32 / width as f32 + texel_w;
        let v_min = (y * tile_size) as f32 / height as f32 + texel_h;
        let u_max = ((x + 1) * tile_size) as f32 / width as f32 - texel_w;
        let v_max = ((y + 1) * tile_size) as f32 / height as f32 - texel_h;
        uv_map.insert(*key, AtlasUV {
            u_min,
            v_min,
            u_max,
            v_max,
        });
    }

    let fallback = *uv_map
        .get(&TextureKey::Stone)
        .unwrap_or(uv_map.values().next().unwrap());

    let resolver = TextureResolver {
        tile_size,
        uv_map,
        fallback,
    };

    AtlasBuildOutput {
        width,
        height,
        pixels: atlas.into_vec(),
        resolver: Arc::new(resolver),
    }
}

pub fn texture_key_for(block: BlockType, face: BlockFace) -> TextureKey {
    match block {
        BlockType::GrassBlock => match face {
            BlockFace::Top => TextureKey::GrassTop,
            BlockFace::Bottom => TextureKey::Dirt,
            _ => TextureKey::GrassSide,
        },
        BlockType::Dirt | BlockType::CoarseDirt | BlockType::RootedDirt | BlockType::Podzol =>
            TextureKey::Dirt,
        BlockType::Stone | BlockType::Granite | BlockType::Diorite | BlockType::Andesite =>
            TextureKey::Stone,
        BlockType::Deepslate | BlockType::Calcite | BlockType::Tuff => TextureKey::Deepslate,
        BlockType::Bedrock => TextureKey::Bedrock,
        BlockType::Sand | BlockType::RedSand => TextureKey::Sand,
        BlockType::Cobblestone | BlockType::MossyCobblestone | BlockType::StoneBricks
        | BlockType::SmoothStone | BlockType::Sandstone | BlockType::RedSandstone
        | BlockType::Bricks => TextureKey::Cobblestone,
        BlockType::OakPlanks | BlockType::SprucePlanks | BlockType::BirchPlanks =>
            TextureKey::OakPlanks,
        BlockType::OakLog | BlockType::SpruceLog | BlockType::BirchLog | BlockType::JungleLog
        | BlockType::AcaciaLog | BlockType::DarkOakLog => TextureKey::OakLog,
        BlockType::OakLeaves => TextureKey::OakLeaves,
        BlockType::Glass | BlockType::WhiteStainedGlass => TextureKey::Glass,
        BlockType::Water => TextureKey::Water,
        BlockType::CoalOre => TextureKey::CoalOre,
        BlockType::IronOre => TextureKey::IronOre,
        BlockType::GoldOre => TextureKey::GoldOre,
        BlockType::DiamondOre => TextureKey::DiamondOre,
        BlockType::DeepslateCoalOre => TextureKey::DeepslateCoalOre,
        BlockType::DeepslateIronOre => TextureKey::DeepslateIronOre,
        BlockType::DeepslateGoldOre => TextureKey::DeepslateGoldOre,
        BlockType::DeepslateDiamondOre => TextureKey::DeepslateDiamondOre,
        _ => TextureKey::Stone,
    }
}

fn blit_tile(atlas: &mut RgbaImage, tile: &RgbaImage, offset_x: u32, offset_y: u32) {
    for y in 0..tile.height() {
        for x in 0..tile.width() {
            let pixel = tile.get_pixel(x, y);
            atlas.put_pixel(offset_x + x, offset_y + y, *pixel);
        }
    }
}

fn generate_tile(key: TextureKey, tile_size: u32) -> RgbaImage {
    match key {
        TextureKey::GrassTop => generate_grass_top(tile_size),
        TextureKey::GrassSide => generate_grass_side(tile_size),
        TextureKey::Dirt => generate_noise_tile(tile_size, [110, 78, 48], 18, 1),
        TextureKey::Stone => generate_noise_tile(tile_size, [110, 110, 110], 12, 2),
        TextureKey::Deepslate => generate_noise_tile(tile_size, [70, 70, 78], 10, 3),
        TextureKey::Bedrock => generate_noise_tile(tile_size, [32, 32, 32], 6, 4),
        TextureKey::Sand => generate_noise_tile(tile_size, [220, 214, 170], 10, 5),
        TextureKey::Cobblestone => generate_cobblestone(tile_size),
        TextureKey::OakPlanks => generate_planks(tile_size),
        TextureKey::OakLog => generate_log(tile_size),
        TextureKey::OakLeaves => generate_leaves(tile_size),
        TextureKey::Glass => generate_glass(tile_size),
        TextureKey::CoalOre => generate_ore(tile_size, [60, 60, 60], [30, 30, 30], 10),
        TextureKey::IronOre => generate_ore(tile_size, [190, 140, 110], [110, 110, 110], 11),
        TextureKey::GoldOre => generate_ore(tile_size, [223, 195, 51], [110, 110, 110], 12),
        TextureKey::DiamondOre => generate_ore(tile_size, [80, 220, 225], [110, 110, 110], 13),
        TextureKey::DeepslateCoalOre => generate_ore(tile_size, [60, 60, 60], [70, 70, 78], 14),
        TextureKey::DeepslateIronOre => generate_ore(tile_size, [190, 140, 110], [70, 70, 78], 15),
        TextureKey::DeepslateGoldOre => generate_ore(tile_size, [223, 195, 51], [70, 70, 78], 16),
        TextureKey::DeepslateDiamondOre => generate_ore(tile_size, [80, 220, 225], [70, 70, 78], 17),
        TextureKey::Water => generate_water(tile_size),
    }
}

fn generate_grass_top(tile_size: u32) -> RgbaImage {
    let mut img = ImageBuffer::new(tile_size, tile_size);
    for y in 0..tile_size {
        for x in 0..tile_size {
            let noise = jitter(x, y, 21) as i32 - 128;
            let base = [70i32, 140, 40];
            let color = [
                (base[0] + noise / 10).clamp(30, 200) as u8,
                (base[1] + noise / 8).clamp(60, 255) as u8,
                (base[2] + noise / 12).clamp(30, 200) as u8,
            ];
            img.put_pixel(x, y, Rgba([color[0], color[1], color[2], 255]));
        }
    }
    img
}

fn generate_grass_side(tile_size: u32) -> RgbaImage {
    let mut img = ImageBuffer::new(tile_size, tile_size);
    let grass_height = (tile_size / 5).max(2);
    let blend_band = grass_height.min(3);
    for y in 0..tile_size {
        for x in 0..tile_size {
            let dirt = generate_noise_color([110, 78, 48], 18, x, y, 32);

            if y < grass_height {
                let noise = jitter(x, y, 31) as i32 - 128;
                let base = [70i32, 130, 45];
                let color = [
                    (base[0] + noise / 10).clamp(40, 180) as u8,
                    (base[1] + noise / 8).clamp(60, 220) as u8,
                    (base[2] + noise / 10).clamp(30, 180) as u8,
                ];
                img.put_pixel(x, y, Rgba([color[0], color[1], color[2], 255]));
            } else if y < grass_height + blend_band {
                // Blend a thin strip so the transition isn't harsh when scaled.
                let t = ((y - grass_height + 1).max(0) as f32) / (blend_band as f32 + 1.0);
                let noise = jitter(x, y, 31) as i32 - 128;
                let grass = [
                    (70 + noise / 10).clamp(40, 180) as u8,
                    (130 + noise / 8).clamp(60, 220) as u8,
                    (45 + noise / 10).clamp(30, 180) as u8,
                ];
                let blended = [
                    (grass[0] as f32 * (1.0 - t) + dirt[0] as f32 * t).round() as u8,
                    (grass[1] as f32 * (1.0 - t) + dirt[1] as f32 * t).round() as u8,
                    (grass[2] as f32 * (1.0 - t) + dirt[2] as f32 * t).round() as u8,
                ];
                img.put_pixel(x, y, Rgba([blended[0], blended[1], blended[2], 255]));
            } else {
                img.put_pixel(x, y, Rgba([dirt[0], dirt[1], dirt[2], 255]));
            }
        }
    }
    img
}

fn generate_noise_tile(tile_size: u32, base: [u8; 3], variation: u8, seed: u32) -> RgbaImage {
    let mut img = ImageBuffer::new(tile_size, tile_size);
    for y in 0..tile_size {
        for x in 0..tile_size {
            let color = generate_noise_color(base, variation, x, y, seed);
            img.put_pixel(x, y, Rgba([color[0], color[1], color[2], 255]));
        }
    }
    img
}

fn generate_noise_color(base: [u8; 3], variation: u8, x: u32, y: u32, seed: u32) -> [u8; 3] {
    let noise = jitter(x, y, seed) % (variation as u32);
    let offsets = [noise as i32 - variation as i32 / 2; 3];
    [
        (base[0] as i32 + offsets[0]).clamp(0, 255) as u8,
        (base[1] as i32 + offsets[1]).clamp(0, 255) as u8,
        (base[2] as i32 + offsets[2]).clamp(0, 255) as u8,
    ]
}

fn generate_cobblestone(tile_size: u32) -> RgbaImage {
    let mut img = ImageBuffer::new(tile_size, tile_size);
    let mortar_color = [90, 90, 95];
    for y in 0..tile_size {
        for x in 0..tile_size {
            let noise = jitter(x, y, 41) as i32 - 128;
            let base = [110i32, 110, 115];
            let mut color = [
                (base[0] + noise / 12).clamp(60, 180) as u8,
                (base[1] + noise / 16).clamp(60, 180) as u8,
                (base[2] + noise / 12).clamp(60, 180) as u8,
            ];
            if x % (tile_size / 4).max(2) == 0 || y % (tile_size / 4).max(2) == 0 {
                color = mortar_color;
            }
            img.put_pixel(x, y, Rgba([color[0], color[1], color[2], 255]));
        }
    }
    img
}

fn generate_planks(tile_size: u32) -> RgbaImage {
    let mut img = ImageBuffer::new(tile_size, tile_size);
    for y in 0..tile_size {
        for x in 0..tile_size {
            let noise = jitter(x, y, 51) as i32 - 128;
            let base = [160i32, 120, 70];
            let mut color = [
                (base[0] + noise / 16).clamp(60, 220) as u8,
                (base[1] + noise / 18).clamp(60, 200) as u8,
                (base[2] + noise / 20).clamp(40, 180) as u8,
            ];
            if y % (tile_size / 4).max(2) == 0 {
                color = [120, 90, 60];
            }
            img.put_pixel(x, y, Rgba([color[0], color[1], color[2], 255]));
        }
    }
    img
}

fn generate_log(tile_size: u32) -> RgbaImage {
    let mut img = ImageBuffer::new(tile_size, tile_size);
    let radius = tile_size as f32 / 2.0;
    let center = radius - 0.5;
    for y in 0..tile_size {
        for x in 0..tile_size {
            let dx = x as f32 - center;
            let dy = y as f32 - center;
            let dist = (dx * dx + dy * dy).sqrt();
            let base = if dist < radius * 0.8 {
                [160, 110, 70]
            } else {
                [90, 60, 40]
            };
            let noise = jitter(x, y, 61) as i32 - 128;
            let color = [
                (base[0] as i32 + noise / 20).clamp(40, 200) as u8,
                (base[1] as i32 + noise / 20).clamp(40, 180) as u8,
                (base[2] as i32 + noise / 25).clamp(20, 160) as u8,
            ];
            img.put_pixel(x, y, Rgba([color[0], color[1], color[2], 255]));
        }
    }
    img
}

fn generate_leaves(tile_size: u32) -> RgbaImage {
    let mut img = ImageBuffer::new(tile_size, tile_size);
    for y in 0..tile_size {
        for x in 0..tile_size {
            let noise = jitter(x, y, 71) as i32 - 128;
            let base = [50i32, 120, 50];
            let alpha = (200 + (noise / 8).clamp(-40, 40)) as u8;
            let color = [
                (base[0] + noise / 15).clamp(10, 140) as u8,
                (base[1] + noise / 10).clamp(50, 200) as u8,
                (base[2] + noise / 15).clamp(10, 140) as u8,
            ];
            img.put_pixel(x, y, Rgba([color[0], color[1], color[2], alpha]));
        }
    }
    img
}

fn generate_glass(tile_size: u32) -> RgbaImage {
    let mut img = ImageBuffer::new(tile_size, tile_size);
    for y in 0..tile_size {
        for x in 0..tile_size {
            let base = [180, 200, 220];
            let noise = jitter(x, y, 81) as i32 - 128;
            let color = [
                (base[0] as i32 + noise / 18).clamp(120, 240) as u8,
                (base[1] as i32 + noise / 20).clamp(120, 240) as u8,
                (base[2] as i32 + noise / 18).clamp(120, 255) as u8,
            ];
            let alpha = if x % (tile_size / 4).max(2) == 0 || y % (tile_size / 4).max(2) == 0 {
                180
            } else {
                150
            };
            img.put_pixel(x, y, Rgba([color[0], color[1], color[2], alpha]));
        }
    }
    img
}

fn generate_ore(tile_size: u32, ore_color: [u8; 3], stone_color: [u8; 3], seed: u32) -> RgbaImage {
    let mut img = ImageBuffer::new(tile_size, tile_size);
    for y in 0..tile_size {
        for x in 0..tile_size {
            let base = generate_noise_color(stone_color, 10, x, y, seed + 100);
            let sparkle = jitter(x, y, seed + 200) % 255;
            let ratio = sparkle > 210;
            let color = if ratio {
                ore_color
            } else {
                base
            };
            img.put_pixel(x, y, Rgba([color[0], color[1], color[2], 255]));
        }
    }
    img
}

fn generate_water(tile_size: u32) -> RgbaImage {
    let mut img = ImageBuffer::new(tile_size, tile_size);
    for y in 0..tile_size {
        for x in 0..tile_size {
            let noise = jitter(x, y, 91) as i32 - 128;
            let base = [30i32, 90, 180];
            let color = [
                (base[0] + noise / 20).clamp(10, 120) as u8,
                (base[1] + noise / 15).clamp(40, 180) as u8,
                (base[2] + noise / 10).clamp(100, 255) as u8,
            ];
            img.put_pixel(x, y, Rgba([color[0], color[1], color[2], 160]));
        }
    }
    img
}

fn jitter(x: u32, y: u32, seed: u32) -> u32 {
    let mut v = x.wrapping_mul(374761393)
        ^ y.wrapping_mul(668265263)
        ^ seed.wrapping_mul(362437);
    v = (v ^ (v >> 13)).wrapping_mul(1274126177);
    v ^ (v >> 16)
}
