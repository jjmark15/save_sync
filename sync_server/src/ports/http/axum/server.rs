use std::net::TcpListener;

use axum::handler::{get, post};
use axum::routing::BoxRoute;
use axum::{AddExtensionLayer, Router};

use crate::DynApplicationService;

use super::handlers::{
    admin_status_handler, get_save_handler, latest_version_handler, store_new_save_handler,
};

pub(crate) struct AxumServer {
    application_service: DynApplicationService,
}

impl AxumServer {
    pub(crate) fn new(application_service: DynApplicationService) -> Self {
        AxumServer {
            application_service,
        }
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
            .nest(
                "/save",
                Router::new()
                    .route("/version/latest/:save_id", get(latest_version_handler))
                    .route("/", post(store_new_save_handler))
                    .route("/:save_id", get(get_save_handler))
                    .layer(AddExtensionLayer::new(self.application_service.clone())),
            )
            .boxed()
    }
}
