use reqwest::Response;

use crate::response::models::LatestVersion;
use crate::response::ResponseValueExtractor;

pub(crate) struct SaveVersionValueExtractor {}

impl SaveVersionValueExtractor {
    pub(crate) fn new() -> Self {
        SaveVersionValueExtractor {}
    }
}

#[async_trait::async_trait]
impl ResponseValueExtractor for SaveVersionValueExtractor {
    type Value = u32;

    async fn extract(&self, response: Response) -> Option<Self::Value> {
        match response.json::<LatestVersion>().await {
            Ok(version) => Some(version.value()),
            Err(_) => None,
        }
    }
}
