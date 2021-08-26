use reqwest::Response;

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
