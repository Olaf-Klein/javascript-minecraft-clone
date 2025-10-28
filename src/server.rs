#![allow(dead_code)]
use anyhow::Result;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::{broadcast, Mutex};
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use futures::{SinkExt, StreamExt};
use bytes::Bytes;

use crate::net::protocol::{ClientMessage, PlayerSnapshot, ServerMessage};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ServerConfig {
    pub bind: String,
    pub port: u16,
}

pub async fn run_server(config: ServerConfig) -> Result<()> {
    let addr: SocketAddr = format!("{}:{}", config.bind, config.port).parse()?;
    let listener = TcpListener::bind(addr).await?;
    println!("Dedicated server listening on {}:{}", config.bind, config.port);

    // broadcast channel for sending ServerMessage to connected clients
    let (tx, _rx) = broadcast::channel::<Vec<u8>>(128);

    // simple in-memory player registry (id -> snapshot)
    let players = Arc::new(Mutex::new(HashMap::<u64, PlayerSnapshot>::new()));
    let mut next_id: u64 = 1;

    loop {
        let (socket, peer) = listener.accept().await?;
        println!("Accepted connection from {}", peer);

        let tx = tx.clone();
        let mut rx = tx.subscribe();
        let players = players.clone();
        let client_id = next_id;
        next_id += 1;

        tokio::spawn(async move {
            if let Err(e) = handle_client(socket, peer, client_id, tx, rx, players).await {
                eprintln!("client {} error: {}", peer, e);
            }
        });
    }
}

async fn handle_client(
    socket: tokio::net::TcpStream,
    peer: SocketAddr,
    client_id: u64,
    tx: broadcast::Sender<Vec<u8>>,
    mut rx: broadcast::Receiver<Vec<u8>>,
    players: Arc<Mutex<HashMap<u64, PlayerSnapshot>>>,
) -> Result<()> {
    let mut framed = Framed::new(socket, LengthDelimitedCodec::new());

    // Split framed into sink (for sending) and stream (for receiving)
    let (mut sink, mut stream) = framed.split();

    let send_task = tokio::spawn(async move {
        while let Ok(msg_bytes) = rx.recv().await {
            let bytes = Bytes::from(msg_bytes);
            if let Err(e) = sink.send(bytes).await {
                eprintln!("failed to send to {}: {}", peer, e);
                break;
            }
        }
    });

    // Read loop: decode incoming messages and update player state
    while let Some(frame_res) = stream.next().await {
        let frame = frame_res?; // bytes::BytesMut
        let msg: ClientMessage = bincode::deserialize(&frame[..])?;
        match msg {
            ClientMessage::Connect { name } => {
                println!("{} connected as '{}' (id={})", peer, name, client_id);
                let snapshot = PlayerSnapshot { id: client_id, name, x: 0.0, y: 0.0, z: 0.0, yaw: 0.0, pitch: 0.0 };
                players.lock().await.insert(client_id, snapshot.clone());

                // broadcast a world state immediately
                let state = ServerMessage::WorldState { players: players.lock().await.values().cloned().collect() };
                let bytes = bincode::serialize(&state)?;
                let _ = tx.send(bytes);
            }
            ClientMessage::Position { x, y, z, yaw, pitch } => {
                if let Some(p) = players.lock().await.get_mut(&client_id) {
                    p.x = x; p.y = y; p.z = z; p.yaw = yaw; p.pitch = pitch;
                }
                // broadcast updated world state
                let state = ServerMessage::WorldState { players: players.lock().await.values().cloned().collect() };
                let bytes = bincode::serialize(&state)?;
                let _ = tx.send(bytes);
            }
        }
    }

    // Ensure send_task is aborted
    send_task.abort();

    println!("Client {} disconnected", peer);
    players.lock().await.remove(&client_id);
    Ok(())
}
