use std::net::TcpListener;

use crate::ports::http::axum::AxumServer;

mod ports;

pub struct App {
    axum_server: AxumServer,
}

impl App {
    pub fn new() -> Self {
        App {
            axum_server: AxumServer::new(),
        }
    }

    pub async fn run(&self, tcp_listener: TcpListener) {
        self.axum_server.run(tcp_listener).await
    }
}

impl Default for App {
    fn default() -> Self {
        App::new()
    }
}
