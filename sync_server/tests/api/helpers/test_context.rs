use std::net::{IpAddr, Ipv6Addr, SocketAddr, TcpListener};

use tokio::task::JoinHandle;

use server_test_client::{HttpClient, ServerTestClient};
use sync_server::App;

pub(crate) struct TestContext {
    server_test_consumer_client: ServerTestClient,
}

impl TestContext {
    pub(crate) fn new() -> Self {
        let (address, _server) = Self::server();
        let server_test_client = ServerTestClient::new(address, HttpClient::new());

        TestContext {
            server_test_consumer_client: server_test_client,
        }
    }

    pub(crate) fn client(&self) -> &ServerTestClient {
        &self.server_test_consumer_client
    }

    fn server() -> (SocketAddr, JoinHandle<()>) {
        let listener =
            TcpListener::bind(SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 0)).unwrap();
        let address = listener.local_addr().unwrap();
        let server = App::new();

        let server_future = tokio::spawn(async move { server.run(listener).await });

        (address, server_future)
    }
}
