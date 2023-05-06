use std::net::SocketAddr;

use hyper::server::conn::http1;
use hyper::service::service_fn;
use tokio::net::TcpListener;


extern crate pretty_env_logger;


mod json;
mod error;
mod api;
mod service;
mod cash_client;


#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();

    let addr: SocketAddr = ([127, 0, 0, 1], 8080).into();
    let listener = TcpListener::bind(addr).await?;
    log::info!("Listening on http://{}", addr);

    loop {
        let (stream, _) = listener.accept().await?;

        tokio::task::spawn(async move {

            let service = service_fn(move |req| {
                log::info!("{:?}", &req);
                api::router(req)
            });

            if let Err(err) = http1::Builder::new()
                .serve_connection(stream, service)
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}

