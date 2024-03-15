use std::net::SocketAddr;

use clap::Parser;
use futures::sink::SinkExt;
use rand::Rng;
use tokio::net::TcpStream;
use tokio_util::codec::Framed;
use types::{Request, RequestCodec};

#[derive(Parser)]
struct Args {
    /// Server address.
    #[arg(short, long)]
    addr: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let mut rng = rand::thread_rng();

    let addr: SocketAddr = match args.addr.parse() {
        Ok(addr) => addr,
        Err(err) => {
            println!("{err}");
            return;
        }
    };
    println!("Connecting to {addr}...");
    let mut framed_stream = Framed::new(TcpStream::connect(addr).await.unwrap(), RequestCodec {});
    println!("Connected!");

    loop {
        println!("Which request?");
        println!("L) Move left");
        println!("R) Move right");
        println!("B) Send a bunch of random bytes");
        println!("Q) Quit");

        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input).unwrap();

        for c in input.trim_end().chars() {
            let (request, should_quit) = match c {
                'L' | 'l' => (Request::left(), false),
                'R' | 'r' => (Request::right(), false),
                'Q' | 'q' => (Request::quit(), true),
                'B' | 'b' => {
                    let count = rng.gen_range(0..1024 * 1024 * 4);
                    (
                        Request::bytes(std::iter::repeat_with(|| rng.gen()).take(count)),
                        false,
                    )
                }
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
