use crate::net::protocol::{ClientMessage, ServerMessage};
use bincode;

use bytes::Bytes;
use futures::{SinkExt, StreamExt};

use std::error::Error;
use std::net::SocketAddr;

use std::sync::mpsc::{self, Receiver, Sender};
use tokio::net::TcpStream;

use tokio::runtime::Runtime;
use tokio::sync::mpsc::UnboundedSender;

use tokio_util::codec::{Framed, LengthDelimitedCodec};
pub struct ClientHandle {
    pub send: UnboundedSender<ClientMessage>,
    pub recv: Receiver<ServerMessage>,
}
pub fn start_client(addr: String) -> Result<ClientHandle, Box<dyn Error>> {
    // outgoing from main -> client runtime
    let (out_tx, mut out_rx) = tokio::sync::mpsc::unbounded_channel::<ClientMessage>();
    // incoming from runtime -> main
    let (in_tx, in_rx) = mpsc::channel::<ServerMessage>();

    std::thread::spawn(move || {
        let rt = Runtime::new().expect("failed to create runtime");
        rt.block_on(async move {
            loop {
                match TcpStream::connect(&addr).await {
                    Ok(stream) => {
                        if let Err(e) = run_connection(stream, &mut out_rx, in_tx.clone()).await {
                            eprintln!("connection error: {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("failed to connect to {}: {}", addr, e);
                    }
                // reconnect delay
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }
        pub use crate::net::client_impl::*;
    });


