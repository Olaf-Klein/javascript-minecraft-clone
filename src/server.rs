use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerConfig {
    pub bind: String,
    pub port: u16,
}

pub async fn run_server(config: ServerConfig) -> Result<()> {
    let addr: SocketAddr = format!("{}:{}", config.bind, config.port).parse()?;
    let listener = TcpListener::bind(addr).await?;
    println!("Dedicated server listening on {}:{}", config.bind, config.port);

    loop {
        let (mut socket, peer) = listener.accept().await?;
        println!("Accepted connection: {}", peer);
        tokio::spawn(async move {
            let mut buf = vec![0u8; 1024];
            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => break,
                    Ok(n) => {
                        // Echo for now, placeholder for protocol
                        if let Err(e) = socket.write_all(&buf[..n]).await {
                            eprintln!("write error: {}", e);
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("read error: {}", e);
                        break;
                    }
                }
            }
            println!("Connection closed: {}", peer);
        });
    }
}
