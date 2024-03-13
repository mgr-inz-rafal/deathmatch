use std::net::SocketAddr;

use futures::sink::SinkExt;
use tokio::net::TcpStream;
use tokio_util::codec::Framed;
use types::RequestCodec;

#[tokio::main]
async fn main() {
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let mut framed_stream = Framed::new(TcpStream::connect(addr).await.unwrap(), RequestCodec {});
    println!("Connected to server");

    loop {
        println!("Which request?");
        println!("L) Move left");
        println!("R) Move right");
        println!("Q) Quit");

        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input).unwrap();

        for c in input.trim_end().chars() {
            let (request, should_quit) = match c {
                'L' | 'l' => (types::Request::left(), false),
                'R' | 'r' => (types::Request::right(), false),
                'Q' | 'q' => (types::Request::quit(), true),
                _ => panic!("unknown request ({})", c as u8),
            };
            let request_desc = request.to_string();
            framed_stream.send(request).await.unwrap();
            println!("Request sent: {}", request_desc);
            if should_quit {
                println!("bye bye...");
                return;
            }
        }
    }
}
