use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
use types::{Decode, Request};

async fn run_server() {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("Listening on: {}", addr);

    loop {
        let (mut socket, remote_addr) = listener.accept().await.unwrap();
        println!("Connection from {remote_addr} established");
        tokio::spawn(async move {
            let mut message = vec![];
            let mut byte = [0_u8];
            loop {
                if 0 == socket.read(&mut byte).await.unwrap() {
                    println!("Connection from {remote_addr} closed");
                    break;
                } else if byte[0] == types::PACKET_END {
                    // Entire message read
                    let request = Request::decode(&message);
                    println!("got request from {remote_addr}: {request}");
                    message.clear();
                } else {
                    message.push(byte[0])
                }
            }
        });
    }
}

#[tokio::main]
async fn main() {
    println!("Server running...");
    run_server().await;
    println!("Server stopped");
}
