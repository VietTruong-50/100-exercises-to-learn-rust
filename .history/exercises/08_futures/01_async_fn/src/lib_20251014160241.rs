use tokio::{io, net::TcpListener};

// TODO: write an echo server that accepts incoming TCP connections and
//  echoes the received data back to the client.
//  `echo` should not return when it finishes processing a connection, but should
//  continue to accept new connections.
//
// Hint: you should rely on `tokio`'s structs and methods to implement the echo server.
// In particular:
// - `tokio::net::TcpListener::accept` to process the next incoming connection
// - `tokio::net::TcpStream::split` to obtain a reader and a writer from the socket
// - `tokio::io::copy` to copy data from the reader to the writer
use tokio::io::{self, AsyncWriteExt};

pub async fn echo(listener: TcpListener) -> Result<(), anyhow::Error> {
    loop {
        // Chờ một kết nối mới
        let (mut socket, _) = listener.accept().await?;
        // Tách ra reader & writer
        let (mut reader, mut writer) = socket.split();

        // Spawn một task riêng để xử lý kết nối đó
        tokio::spawn(async move {
            if let Err(e) = io::copy(&mut reader, &mut writer).await {
                eprintln!("connection error: {:?}", e);
            }

            // Đảm bảo flush dữ liệu (ghi ra hết)
            let _ = writer.shutdown().await;
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    #[tokio::test]
    async fn test_echo() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(echo(listener));

        let requests = vec!["hello", "world", "foo", "bar"];

        for request in requests {
            let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();
            let (mut reader, mut writer) = socket.split();

            // Send the request
            writer.write_all(request.as_bytes()).await.unwrap();
            // Close the write side of the socket
            writer.shutdown().await.unwrap();

            // Read the response
            let mut buf = Vec::with_capacity(request.len());
            reader.read_to_end(&mut buf).await.unwrap();
            assert_eq!(&buf, request.as_bytes());
        }
    }
}
