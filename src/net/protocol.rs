use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ClientMessage {
    Connect { name: String },
    Position { x: f32, y: f32, z: f32, yaw: f32, pitch: f32 },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerMessage {
    WorldState { players: Vec<PlayerSnapshot> },
    Ack,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerSnapshot {
    pub id: u64,
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub yaw: f32,
    pub pitch: f32,
}
