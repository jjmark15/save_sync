use std::net::TcpListener;
use std::path::PathBuf;
use std::sync::Arc;

use crate::application::{ApplicationService, ApplicationServiceImpl};
use crate::domain::SaveServiceImpl;
use crate::ports::http::axum::AxumServer;
use crate::ports::persistence::kv::KvSaveRepositoryAdapter;

mod application;
mod domain;
mod ports;

pub(crate) type DynApplicationService = Arc<dyn ApplicationService + Send + Sync>;

pub struct App {
    axum_server: AxumServer,
}

impl App {
    pub fn new(persistence_directory: PathBuf) -> Self {
        App {
            axum_server: AxumServer::new(Self::application_service(persistence_directory)),
        }
    }

    pub async fn run(&self, tcp_listener: TcpListener) {
        self.axum_server.run(tcp_listener).await
    }

    fn application_service(persistence_directory: PathBuf) -> DynApplicationService {
        let save_repository = KvSaveRepositoryAdapter::new(persistence_directory);
        let save_service = SaveServiceImpl::new(save_repository);
        Arc::new(ApplicationServiceImpl::new(save_service))
    }
}
