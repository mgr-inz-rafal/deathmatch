use futures::{StreamExt, TryStreamExt};
use tokio::net::TcpListener;
use tokio_util::codec::Framed;
use types::RequestCodec;

async fn run_server() {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("Listening on: {}", addr);

    loop {
        let (socket, remote_addr) = listener.accept().await.unwrap();
        println!("Connection from {remote_addr} established");

        let framed = Framed::new(socket, RequestCodec {});
        tokio::spawn(async move {
            let mut requests = framed.into_stream();
            while let Some(request) = requests.next().await {
                println!("got request from {remote_addr}: {}", request.unwrap());
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
