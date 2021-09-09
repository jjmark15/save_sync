use reqwest::Response;

pub(crate) use api_error_value_extractor::ApiErrorValueExtractor;
pub(crate) use save_id_value_extractor::SaveIDValueExtractor;
pub(crate) use save_metadata_value_extractor::SaveMetadataValueExtractor;
pub(crate) use save_value_extractor::SaveValueExtractor;
pub(crate) use save_version_value_extractor::SaveVersionValueExtractor;

mod api_error_value_extractor;
mod save_id_value_extractor;
mod save_metadata_value_extractor;
mod save_value_extractor;
mod save_version_value_extractor;

#[async_trait::async_trait]
pub(crate) trait ResponseValueExtractor {
    type Value;

    async fn extract(&self, response: reqwest::Response) -> Option<Self::Value>;
}

pub(crate) struct NoValueResponseValueExtractor {}

impl NoValueResponseValueExtractor {
    pub(crate) fn new() -> Self {
        NoValueResponseValueExtractor {}
    }
}

#[async_trait::async_trait]
impl ResponseValueExtractor for NoValueResponseValueExtractor {
    type Value = ();

    async fn extract(&self, _response: Response) -> Option<Self::Value> {
        None
    }
}
