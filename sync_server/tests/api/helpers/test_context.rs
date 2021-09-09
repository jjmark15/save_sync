use std::net::{IpAddr, Ipv6Addr, SocketAddr, TcpListener};
use std::path::PathBuf;

use tokio::task::JoinHandle;

use server_test_client::{HttpClient, ServerTestClient};
use sync_server::App;

use crate::helpers::test_directory_manager::TestDirectoryManager;

pub(crate) struct TestContext {
    server_test_client: ServerTestClient,
    test_directory_manager: TestDirectoryManager,
}

impl TestContext {
    pub(crate) fn new() -> Self {
        let tcp_listener = Self::tcp_listener();
        let socket_address = tcp_listener.local_addr().unwrap();
        let server_test_client = ServerTestClient::new(socket_address, HttpClient::new());

        let test_context = TestContext {
            server_test_client,
            test_directory_manager: TestDirectoryManager::new(),
        };

        let _server = Self::server(
            tcp_listener,
            test_context
                .test_directory_manager
                .persistence_directory()
                .to_path_buf(),
        );

        test_context
    }

    pub(crate) fn client(&self) -> &ServerTestClient {
        &self.server_test_client
    }

    fn server(listener: TcpListener, persistence_directory: PathBuf) -> JoinHandle<()> {
        let server = App::new(persistence_directory);
        tokio::spawn(async move { server.run(listener).await })
    }

    fn tcp_listener() -> TcpListener {
        TcpListener::bind(SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 0)).unwrap()
    }
}
