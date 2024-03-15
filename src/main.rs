use std::sync::Arc;

use clap::Parser;
use futures::{StreamExt, TryStreamExt};
use tokio::{net::TcpListener, sync::Semaphore};
use tokio_util::codec::Framed;
use types::RequestCodec;

#[derive(Parser)]
struct Args {
    /// Server bind address.
    #[arg(short, long)]
    bind_addr: String,
    /// Maximum number of simultaneous connections.
    #[arg(short, long)]
    max_conn: u8,
}

async fn run_server(bind_addr: &str, max_conn: u8) {
    let listener = TcpListener::bind(bind_addr).await.unwrap();
    println!(
        "Listening on {} with a limit of {max_conn} connections",
        bind_addr
    );

    let semaphore = Arc::new(Semaphore::new(max_conn as usize));
    loop {
        println!(
            "Available permits: {}/{max_conn}",
            semaphore.available_permits()
        );
        let (socket, remote_addr) = listener.accept().await.unwrap();
        println!("Connection from {remote_addr} established");
        let permit = match semaphore.clone().try_acquire_owned() {
            Ok(permit) => permit,
            Err(_) => {
                println!("No more connection slots available, sorry");
                drop(socket);
                continue;
            }
        };

        let framed = Framed::new(socket, RequestCodec {});
        tokio::spawn(async move {
            let mut requests = framed.into_stream();
            while let Some(request) = requests.next().await {
                println!("got request from {remote_addr}: {}", request.unwrap());
            }
            println!("Connection to {remote_addr} closed");
            drop(permit)
        });
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    println!("Server running...");
    run_server(&args.bind_addr, args.max_conn).await;
    println!("Server stopped");
}
