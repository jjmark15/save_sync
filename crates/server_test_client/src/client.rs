use std::net::SocketAddr;

use http::Method;
use url::{ParseError, Url};

use crate::dto::SaveId;
use crate::response::{
    HttpResponseDetails, NoValueResponseValueExtractor, ResponseValueExtractor, ResponseWrapper,
    SaveVersionValueExtractor,
};

pub struct ServerTestClient {
    server_address: SocketAddr,
    http_client: reqwest::Client,
}

impl ServerTestClient {
    pub fn new(server_address: SocketAddr, http_client: reqwest::Client) -> Self {
        ServerTestClient {
            server_address,
            http_client,
        }
    }

    fn http_request_base_url(&self, url_path_segments: &[&str]) -> Result<Url, ParseError> {
        Url::parse(
            format!(
                "http://{}/{}",
                self.server_address.to_string(),
                url_path_segments.join("/")
            )
            .as_str(),
        )
    }

    async fn client_call<V>(
        &self,
        url_path_segments: &[&str],
        request_method: Method,
        response_value_extractor: impl ResponseValueExtractor<Value = V>,
    ) -> ResponseWrapper<V> {
        let request = self.http_client.request(
            request_method,
            self.http_request_base_url(url_path_segments).unwrap(),
        );

        let http_response = request.send().await.unwrap();

        ResponseWrapper::new(
            HttpResponseDetails::from(&http_response),
            response_value_extractor.extract(http_response).await,
        )
    }

    pub async fn status(&self) -> ResponseWrapper<()> {
        self.client_call(
            &["admin", "status"],
            Method::GET,
            NoValueResponseValueExtractor::new(),
        )
        .await
    }

    pub async fn latest_version(&self, save_id: SaveId) -> ResponseWrapper<u32> {
        self.client_call(
            &["save", "version", "latest", save_id.to_string().as_str()],
            Method::GET,
            SaveVersionValueExtractor::new(),
        )
        .await
    }
}
