use minecraft_clone_rust::server::{run_server, ServerConfig};
use tokio::runtime::Runtime;
use std::env;

fn main() {
    let rt = Runtime::new().expect("Failed to create tokio runtime");

    // Allow container / Pterodactyl provided env vars to configure bind/port
    let bind = env::var("BIND").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").ok().and_then(|s| s.parse::<u16>().ok()).unwrap_or(25565);

    let config = ServerConfig { bind, port };
    rt.block_on(async move {
        if let Err(e) = run_server(config).await {
            eprintln!("Server error: {}", e);
        }
    });
}
