//! A simple JSON-based echo server over TCP using tokio-jsoncodec.
//!
//! This example demonstrates how to use `tokio-jsoncodec` to create a simple echo server that
//! listens on TCP port 7777 and echoes back any JSON object it receives.
//!
//! You can run this example with:
//! ```bash
//! cargo run --example echo
//! ```
//!
//! You can use `nc` to test this server:
//! ```bash
//! echo '{"hello": "world"}' | nc -N localhost 7777
//! ```

use futures::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_jsoncodec::Codec;
use tokio_util::codec::{Decoder, Framed};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("[::]:7777").await?;
    eprintln!("Listening on {}", listener.local_addr()?);

    if let Ok((tcp_stream, peer_addr)) = listener.accept().await {
        eprintln!("Accepted connection from {}", peer_addr);
        let frames: Framed<TcpStream, Codec<serde_json::Value>> =
            Codec::default().framed(tcp_stream);
        let (mut sink, mut stream) = frames.split::<serde_json::Value>();
        while let Some(frame) = stream.next().await {
            let frame = frame?;
            eprintln!("echoing: {:?}", frame);
            sink.send(frame).await?;
        }
    }
    eprintln!("Connection closed");
    Ok(())
}
