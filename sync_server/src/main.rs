use std::net::{IpAddr, Ipv6Addr, SocketAddr, TcpListener};

use sync_server::App;

#[tokio::main]
async fn main() {
    initialise_logging();
    let address = SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 3030);
    let listener = TcpListener::bind(address).unwrap();

    App::new().run(listener).await
}

fn initialise_logging() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "monitor_server=info")
    }
    tracing_subscriber::fmt::init();
}
