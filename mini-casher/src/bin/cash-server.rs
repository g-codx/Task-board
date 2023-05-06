use mini_casher::server;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();
    server::run().await;
}

