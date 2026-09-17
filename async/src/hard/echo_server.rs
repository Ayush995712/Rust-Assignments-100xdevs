/*
  Problem 99: Async TCP Echo Server (Simplified)

  Write an async function that starts a mock TCP echo server using
  tokio::net::TcpListener on a given port. It should accept one connection,
  read exactly 5 bytes, and write them back. Return the bytes read.

  Run the tests for this problem with:
    cargo test --test echo_server_test
*/

use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn run_echo_server(port: u16) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
  let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
  let (mut stream, _) = listener.accept().await?;
  let mut buffer = [0u8; 1024];

  let n = stream.read(&mut buffer).await?;
  stream.write_all(&buffer[..n]).await?;

  Ok(buffer[..n].to_vec())
}
