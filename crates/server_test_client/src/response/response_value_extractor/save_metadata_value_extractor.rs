use reqwest::Response;

use crate::dto::{SaveId, SaveMetadata};
use crate::response::models::SaveMetadataResponse;
use crate::response::ResponseValueExtractor;

pub(crate) struct SaveMetadataValueExtractor {}

impl SaveMetadataValueExtractor {
    pub(crate) fn new() -> Self {
        SaveMetadataValueExtractor {}
    }
}

#[async_trait::async_trait]
impl ResponseValueExtractor for SaveMetadataValueExtractor {
    type Value = SaveMetadata;

    async fn extract(&self, response: Response) -> Option<Self::Value> {
        match response.json::<SaveMetadataResponse>().await {
            Ok(save_metadata_response) => Some(SaveMetadata::new(
                SaveId::new(save_metadata_response.id().to_string()),
                save_metadata_response.version(),
                save_metadata_response.file_name().to_string(),
                save_metadata_response.game_name().to_string(),
            )),
            Err(_) => None,
        }
    }
}
