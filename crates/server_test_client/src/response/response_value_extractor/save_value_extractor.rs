use reqwest::Response;

use crate::dto::{Save, SaveId};
use crate::response::models::GetSaveResponse;
use crate::response::ResponseValueExtractor;

pub(crate) struct SaveValueExtractor {}

impl SaveValueExtractor {
    pub(crate) fn new() -> Self {
        SaveValueExtractor {}
    }
}

#[async_trait::async_trait]
impl ResponseValueExtractor for SaveValueExtractor {
    type Value = Save;

    async fn extract(&self, response: Response) -> Option<Self::Value> {
        match response.json::<GetSaveResponse>().await {
            Ok(save_response) => Some(Save::new(
                SaveId::new(save_response.id().to_string()),
                save_response.version(),
                save_response.file_name().to_string(),
                save_response.game_name().to_string(),
                save_response.data(),
            )),
            Err(_) => None,
        }
    }
}
