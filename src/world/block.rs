/// Block types matching the JavaScript implementation
/// Covers all vanilla Minecraft blocks for 1.21.10 compatibility
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u16)]
pub enum BlockType {
    // Air and Technical
    Air = 0,
    CaveAir = 1,
    VoidAir = 2,

    // Natural Blocks
    GrassBlock = 3,
    Dirt = 4,
    CoarseDirt = 5,
    Podzol = 6,
    Mycelium = 7,
    RootedDirt = 8,
    Mud = 9,
    Clay = 10,
    Sand = 11,
    RedSand = 12,
    Gravel = 13,
    Stone = 14,
    Granite = 15,
    Diorite = 16,
    Andesite = 17,
    Deepslate = 18,
    Calcite = 19,
    Tuff = 20,
    DripstoneBlock = 21,

    // Ores
    CoalOre = 81,
    DeepslateCoalOre = 82,
    IronOre = 83,
    DeepslateIronOre = 84,
    CopperOre = 85,
    DeepslateCopperOre = 86,
    GoldOre = 87,
    DeepslateGoldOre = 88,
    RedstoneOre = 89,
    DeepslateRedstoneOre = 90,
    EmeraldOre = 91,
    DeepslateEmeraldOre = 92,
    LapisOre = 93,
    DeepslateLapisOre = 94,
    DiamondOre = 95,
    DeepslateDiamondOre = 96,

    // Manufactured Blocks
    Cobblestone = 100,
    MossyCobblestone = 101,
    StoneBricks = 102,
    SmoothStone = 106,
    Sandstone = 109,
    RedSandstone = 112,
    Bricks = 115,

    // Wood
    OakLog = 118,
    SpruceLog = 119,
    BirchLog = 120,
    JungleLog = 121,
    AcaciaLog = 122,
    DarkOakLog = 123,
    OakLeaves = 126,
    OakPlanks = 134,
    SprucePlanks = 135,
    BirchPlanks = 136,

    // Glass
    Glass = 182,
    WhiteStainedGlass = 183,

    // Nether
    Netherrack = 216,
    NetherBricks = 217,
    SoulSand = 222,
    Obsidian = 235,
    Bedrock = 236,

    // Liquids
    Water = 237,
    Lava = 238,

    // Functional
    CraftingTable = 239,
    Furnace = 240,
    Chest = 241,
}

impl Default for BlockType {
    fn default() -> Self {
        BlockType::Air
    }
}

impl BlockType {
    pub fn is_transparent(self) -> bool {
        matches!(
            self,
            BlockType::Air
                | BlockType::CaveAir
                | BlockType::VoidAir
                | BlockType::Glass
                | BlockType::WhiteStainedGlass
                | BlockType::Water
                | BlockType::Lava
                | BlockType::OakLeaves
        )
    }

    pub fn is_solid(self) -> bool {
        !matches!(
            self,
            BlockType::Air | BlockType::CaveAir | BlockType::VoidAir | BlockType::Water
        )
    }

    pub fn is_liquid(self) -> bool {
        matches!(self, BlockType::Water | BlockType::Lava)
    }

    pub fn hardness(self) -> f32 {
        match self {
            BlockType::Air | BlockType::CaveAir | BlockType::VoidAir => 0.0,
            BlockType::Bedrock => -1.0, // Unbreakable
            BlockType::GrassBlock => 0.6,
            BlockType::Dirt | BlockType::CoarseDirt | BlockType::Podzol => 0.5,
            BlockType::Sand | BlockType::RedSand => 0.5,
            BlockType::Gravel => 0.6,
            BlockType::Stone => 1.5,
            BlockType::Granite | BlockType::Diorite | BlockType::Andesite => 1.5,
            BlockType::Deepslate => 3.0,
            BlockType::CoalOre | BlockType::IronOre | BlockType::GoldOre => 3.0,
            BlockType::DiamondOre | BlockType::EmeraldOre => 3.0,
            BlockType::Cobblestone | BlockType::MossyCobblestone => 2.0,
            BlockType::StoneBricks => 1.5,
            BlockType::Sandstone | BlockType::RedSandstone => 0.8,
            BlockType::Bricks => 2.0,
            BlockType::OakLog
            | BlockType::SpruceLog
            | BlockType::BirchLog
            | BlockType::JungleLog
            | BlockType::AcaciaLog
            | BlockType::DarkOakLog => 2.0,
            BlockType::OakPlanks | BlockType::SprucePlanks | BlockType::BirchPlanks => 2.0,
            BlockType::OakLeaves => 0.2,
            BlockType::Glass | BlockType::WhiteStainedGlass => 0.3,
            BlockType::Netherrack => 0.4,
            BlockType::NetherBricks => 2.0,
            BlockType::SoulSand => 0.5,
            BlockType::Obsidian => 50.0,
            BlockType::CraftingTable => 2.5,
            BlockType::Furnace => 3.5,
            BlockType::Chest => 2.5,
            _ => 1.0,
        }
    }

    pub fn get_color(self) -> [f32; 3] {
        match self {
            BlockType::Air | BlockType::CaveAir | BlockType::VoidAir => [0.0, 0.0, 0.0],
            BlockType::GrassBlock => [0.35, 0.65, 0.25],
            BlockType::Dirt | BlockType::CoarseDirt => [0.55, 0.35, 0.2],
            BlockType::Sand => [0.9, 0.85, 0.6],
            BlockType::RedSand => [0.8, 0.5, 0.3],
            BlockType::Stone => [0.5, 0.5, 0.5],
            BlockType::Granite => [0.6, 0.4, 0.35],
            BlockType::Diorite => [0.85, 0.85, 0.85],
            BlockType::Andesite => [0.55, 0.55, 0.55],
            BlockType::Deepslate => [0.3, 0.3, 0.35],
            BlockType::CoalOre => [0.4, 0.4, 0.4],
            BlockType::IronOre => [0.65, 0.6, 0.55],
            BlockType::GoldOre => [0.9, 0.8, 0.3],
            BlockType::DiamondOre => [0.4, 0.7, 0.8],
            BlockType::EmeraldOre => [0.3, 0.8, 0.4],
            BlockType::Cobblestone => [0.45, 0.45, 0.45],
            BlockType::StoneBricks => [0.5, 0.5, 0.5],
            BlockType::Sandstone => [0.85, 0.8, 0.6],
            BlockType::RedSandstone => [0.75, 0.45, 0.3],
            BlockType::Bricks => [0.6, 0.3, 0.2],
            BlockType::OakLog => [0.4, 0.3, 0.2],
            BlockType::SpruceLog => [0.3, 0.25, 0.2],
            BlockType::BirchLog => [0.85, 0.85, 0.8],
            BlockType::OakPlanks => [0.65, 0.5, 0.3],
            BlockType::SprucePlanks => [0.45, 0.35, 0.25],
            BlockType::BirchPlanks => [0.75, 0.7, 0.55],
            BlockType::OakLeaves => [0.2, 0.6, 0.2],
            BlockType::Glass => [0.85, 0.95, 1.0],
            BlockType::WhiteStainedGlass => [0.95, 0.95, 1.0],
            BlockType::Netherrack => [0.6, 0.25, 0.25],
            BlockType::NetherBricks => [0.3, 0.15, 0.2],
            BlockType::SoulSand => [0.35, 0.3, 0.25],
            BlockType::Obsidian => [0.05, 0.05, 0.15],
            BlockType::Bedrock => [0.2, 0.2, 0.2],
            BlockType::Water => [0.1, 0.3, 0.8],
            BlockType::Lava => [0.9, 0.4, 0.1],
            BlockType::CraftingTable => [0.6, 0.45, 0.3],
            BlockType::Furnace => [0.4, 0.4, 0.4],
            BlockType::Chest => [0.55, 0.4, 0.25],
            _ => [0.7, 0.7, 0.7],
        }
    }

        /// Construct a BlockType from its numeric id. Unknown ids map to Air.
        pub fn from_id(id: u16) -> Self {
            match id {
                0 => BlockType::Air,
                1 => BlockType::CaveAir,
                2 => BlockType::VoidAir,
                3 => BlockType::GrassBlock,
                4 => BlockType::Dirt,
                5 => BlockType::CoarseDirt,
                6 => BlockType::Podzol,
                7 => BlockType::Mycelium,
                8 => BlockType::RootedDirt,
                9 => BlockType::Mud,
                10 => BlockType::Clay,
                11 => BlockType::Sand,
                12 => BlockType::RedSand,
                13 => BlockType::Gravel,
                14 => BlockType::Stone,
                15 => BlockType::Granite,
                16 => BlockType::Diorite,
                17 => BlockType::Andesite,
                18 => BlockType::Deepslate,
                19 => BlockType::Calcite,
                20 => BlockType::Tuff,
                21 => BlockType::DripstoneBlock,
                81 => BlockType::CoalOre,
                82 => BlockType::DeepslateCoalOre,
                83 => BlockType::IronOre,
                84 => BlockType::DeepslateIronOre,
                85 => BlockType::CopperOre,
                86 => BlockType::DeepslateCopperOre,
                87 => BlockType::GoldOre,
                88 => BlockType::DeepslateGoldOre,
                89 => BlockType::RedstoneOre,
                90 => BlockType::DeepslateRedstoneOre,
                91 => BlockType::EmeraldOre,
                92 => BlockType::DeepslateEmeraldOre,
                93 => BlockType::LapisOre,
                94 => BlockType::DeepslateLapisOre,
                95 => BlockType::DiamondOre,
                96 => BlockType::DeepslateDiamondOre,
                100 => BlockType::Cobblestone,
                101 => BlockType::MossyCobblestone,
                102 => BlockType::StoneBricks,
                106 => BlockType::SmoothStone,
                109 => BlockType::Sandstone,
                112 => BlockType::RedSandstone,
                115 => BlockType::Bricks,
                118 => BlockType::OakLog,
                119 => BlockType::SpruceLog,
                120 => BlockType::BirchLog,
                121 => BlockType::JungleLog,
                122 => BlockType::AcaciaLog,
                123 => BlockType::DarkOakLog,
                126 => BlockType::OakLeaves,
                134 => BlockType::OakPlanks,
                135 => BlockType::SprucePlanks,
                136 => BlockType::BirchPlanks,
                182 => BlockType::Glass,
                183 => BlockType::WhiteStainedGlass,
                216 => BlockType::Netherrack,
                217 => BlockType::NetherBricks,
                222 => BlockType::SoulSand,
                235 => BlockType::Obsidian,
                236 => BlockType::Bedrock,
                237 => BlockType::Water,
                238 => BlockType::Lava,
                239 => BlockType::CraftingTable,
                240 => BlockType::Furnace,
                241 => BlockType::Chest,
                _ => BlockType::Air,
            }
        }
}
