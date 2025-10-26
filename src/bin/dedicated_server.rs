use minecraft_clone_rust::server::{run_server, ServerConfig};
use tokio::runtime::Runtime;

fn main() {
    let rt = Runtime::new().expect("Failed to create tokio runtime");
    let config = ServerConfig { bind: "0.0.0.0".to_string(), port: 25565 };
    rt.block_on(async move {
        if let Err(e) = run_server(config).await {
            eprintln!("Server error: {}", e);
        }
    });
}
