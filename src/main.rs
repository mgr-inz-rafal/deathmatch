use clap::Parser;
use futures::{StreamExt, TryStreamExt};
use tokio::net::TcpListener;
use tokio_util::codec::Framed;
use types::RequestCodec;

#[derive(Parser)]
struct Args {
    /// Server bind address.
    #[arg(short, long)]
    bind_addr: String,
}

async fn run_server(bind_addr: &str) {
    let listener = TcpListener::bind(bind_addr).await.unwrap();
    println!("Listening on: {}", bind_addr);

    loop {
        let (socket, remote_addr) = listener.accept().await.unwrap();
        println!("Connection from {remote_addr} established");

        let framed = Framed::new(socket, RequestCodec {});
        tokio::spawn(async move {
            let mut requests = framed.into_stream();
            while let Some(request) = requests.next().await {
                println!("got request from {remote_addr}: {}", request.unwrap());
            }
            println!("Connection to {remote_addr} closed");
        });
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    println!("Server running...");
    run_server(&args.bind_addr).await;
    println!("Server stopped");
}
