use tokio::{io::AsyncWriteExt, net::TcpSocket};
use types::Encode;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080".parse().unwrap();
    let socket = TcpSocket::new_v4().unwrap();
    let mut stream = socket.connect(addr).await.unwrap();
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
            let encoded = request.encode();
            stream.write_all(&encoded).await.unwrap();
            println!("Request sent: {}", request);
            if should_quit {
                println!("bye bye...");
                return;
            }
        }
    }
}
