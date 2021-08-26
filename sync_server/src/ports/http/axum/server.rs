use std::net::TcpListener;

use axum::handler::get;
use axum::routing::BoxRoute;
use axum::Router;

use crate::ports::http::axum::handlers::{admin_status_handler, latest_version_handler};

pub(crate) struct AxumServer {}

impl AxumServer {
    pub(crate) fn new() -> Self {
        AxumServer {}
    }

    pub(crate) async fn run(&self, tcp_listener: TcpListener) {
        tracing::info!("listening on {}", tcp_listener.local_addr().unwrap());
        axum::Server::from_tcp(tcp_listener)
            .unwrap()
            .serve(self.api_routes().into_make_service())
            .await
            .unwrap();
    }

    fn api_routes(&self) -> Router<BoxRoute> {
        Router::new()
            .route("/admin/status", get(admin_status_handler))
            .route("/save/version/latest/:save_id", get(latest_version_handler))
            .boxed()
    }
}
