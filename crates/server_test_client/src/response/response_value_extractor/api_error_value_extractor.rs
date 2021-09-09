use reqwest::Response;

use crate::dto::ApiError;
use crate::response::ResponseValueExtractor;

pub(crate) struct ApiErrorValueExtractor {}

impl ApiErrorValueExtractor {
    pub(crate) fn new() -> Self {
        ApiErrorValueExtractor {}
    }
}

#[async_trait::async_trait]
impl ResponseValueExtractor for ApiErrorValueExtractor {
    type Value = ApiError;

    async fn extract(&self, response: Response) -> Option<Self::Value> {
        match response
            .json::<crate::response::models::ErrorResponse>()
            .await
        {
            Ok(error_response) => Some(ApiError::new(error_response.message().to_string())),
            Err(_) => None,
        }
    }
}
