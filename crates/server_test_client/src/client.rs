use std::net::SocketAddr;
use std::option::Option::Some;

use http::{Method, StatusCode};
use reqwest::multipart::{Form, Part};
use url::{ParseError, Url};

use crate::dto::{Save, SaveId, SaveMetadata};
use crate::response::{
    ApiErrorValueExtractor, HttpResponseDetails, NoValueResponseValueExtractor,
    ResponseValueExtractor, ResponseWrapper, SaveIDValueExtractor, SaveMetadataValueExtractor,
    SaveValueExtractor, SaveVersionValueExtractor,
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
        expected_status_code: StatusCode,
        response_value_extractor: impl ResponseValueExtractor<Value = V>,
    ) -> ResponseWrapper<V> {
        self.client_form_call(
            url_path_segments,
            request_method,
            None,
            expected_status_code,
            response_value_extractor,
        )
        .await
    }

    async fn client_form_call<V>(
        &self,
        url_path_segments: &[&str],
        request_method: Method,
        form: Option<Form>,
        expected_status_code: StatusCode,
        response_value_extractor: impl ResponseValueExtractor<Value = V>,
    ) -> ResponseWrapper<V> {
        let mut request = self.http_client.request(
            request_method,
            self.http_request_base_url(url_path_segments).unwrap(),
        );

        if let Some(form) = form {
            request = request.multipart(form);
        }

        let http_response = request.send().await.unwrap();
        let http_response_details = HttpResponseDetails::from(&http_response);

        if http_response.status() == expected_status_code {
            ResponseWrapper::new(
                http_response_details,
                response_value_extractor.extract(http_response).await,
                None,
            )
        } else {
            ResponseWrapper::new(
                http_response_details,
                None,
                ApiErrorValueExtractor::new().extract(http_response).await,
            )
        }
    }

    pub async fn status(&self) -> ResponseWrapper<()> {
        self.client_call(
            &["admin", "status"],
            Method::GET,
            StatusCode::OK,
            NoValueResponseValueExtractor::new(),
        )
        .await
    }

    pub async fn latest_version(&self, save_id: &SaveId) -> ResponseWrapper<u32> {
        self.client_call(
            &["save", "version", "latest", save_id.to_string().as_str()],
            Method::GET,
            StatusCode::OK,
            SaveVersionValueExtractor::new(),
        )
        .await
    }

    pub async fn store_new_save(
        &self,
        file_name: impl Into<Option<String>>,
        game_name: impl Into<Option<String>>,
        file_bytes: impl Into<Option<Vec<u8>>>,
    ) -> ResponseWrapper<SaveId> {
        let mut form = Form::new();

        if let Some(game_name) = game_name.into() {
            form = form.text("game_name", game_name);
        }

        if let Some(file_bytes) = file_bytes.into() {
            let mut file_part = Part::bytes(file_bytes);
            if let Some(file_name) = file_name.into() {
                file_part = file_part.file_name(file_name);
            }
            form = form.part("contents", file_part);
        }

        self.client_form_call(
            &["save"],
            Method::POST,
            Some(form),
            StatusCode::CREATED,
            SaveIDValueExtractor::new(),
        )
        .await
    }

    pub async fn get_save_info(&self, save_id: &SaveId) -> ResponseWrapper<SaveMetadata> {
        self.client_call(
            &["save", "info", save_id.to_string().as_str()],
            Method::GET,
            StatusCode::OK,
            SaveMetadataValueExtractor::new(),
        )
        .await
    }

    pub async fn get_save(&self, save_id: &SaveId) -> ResponseWrapper<Save> {
        self.client_call(
            &["save", save_id.to_string().as_str()],
            Method::GET,
            StatusCode::OK,
            SaveValueExtractor::new(),
        )
        .await
    }
}
