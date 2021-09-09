#[derive(Debug, serde::Serialize)]
pub(crate) struct LatestVersionResponse {
    #[serde(rename(serialize = "version"))]
    value: u32,
}

impl LatestVersionResponse {
    pub(crate) fn new(value: u32) -> Self {
        LatestVersionResponse { value }
    }
}
