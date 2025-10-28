use std::error::Error;
use std::time::Duration;
use std::sync::mpsc::RecvTimeoutError;

use minecraft_clone_rust::net::start_client;
use minecraft_clone_rust::net::protocol::{ClientMessage, ServerMessage};

fn main() -> Result<(), Box<dyn Error>> {
    let addr = std::env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:25565".to_string());
    println!("Test client connecting to {}", addr);

    let handle = start_client(addr)?;

    // send a Connect message
    handle.send.send(ClientMessage::Connect { name: "testbot".to_string() })?;

    // send periodic position updates and print any incoming ServerMessage
    for i in 0..6 {
        std::thread::sleep(Duration::from_secs(1));
        let x = i as f32;
        let _ = handle.send.send(ClientMessage::Position { x, y: 0.0, z: 0.0, yaw: 0.0, pitch: 0.0 });

        match handle.recv.recv_timeout(Duration::from_secs(1)) {
            Ok(msg) => println!("Received from server: {:?}", msg),
            Err(RecvTimeoutError::Timeout) => println!("No message received this tick"),
            Err(e) => { println!("recv error: {:?}", e); break; }
        }
    }

    println!("Test client finished");
    Ok(())
}
