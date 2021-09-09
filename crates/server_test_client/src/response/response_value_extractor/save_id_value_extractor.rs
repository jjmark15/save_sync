use reqwest::Response;

use crate::dto::SaveId;
use crate::response::ResponseValueExtractor;

pub(crate) struct SaveIDValueExtractor {}

impl SaveIDValueExtractor {
    pub(crate) fn new() -> Self {
        SaveIDValueExtractor {}
    }
}

#[async_trait::async_trait]
impl ResponseValueExtractor for SaveIDValueExtractor {
    type Value = SaveId;

    async fn extract(&self, response: Response) -> Option<Self::Value> {
        match response
            .json::<crate::response::models::NewSaveResponse>()
            .await
        {
            Ok(response) => Some(SaveId::new(response.save_id().to_string())),
            Err(_) => None,
        }
    }
}
