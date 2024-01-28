use std::net::{SocketAddr};
use hyper::server::conn::http1;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use crate::forwarder::Forwarder;
mod forwarder;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    let forwarder = Forwarder{};

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        let clone_forwarder = forwarder.clone();
        tokio::task::spawn(async move {
            // Finally, we bind the incoming connection to our `hello` service
            if let Err(err) = http1::Builder::new()
                // `service_fn` converts our function in a `Service`
                .serve_connection(io, clone_forwarder)
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}
