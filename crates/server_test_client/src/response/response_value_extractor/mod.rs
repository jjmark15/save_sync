use reqwest::Response;

pub(crate) use save_version_value_extractor::SaveVersionValueExtractor;

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
