#[derive(Debug, serde::Serialize)]
pub(crate) struct LatestVersion {
    #[serde(rename(serialize = "version"))]
    value: u32,
}

impl LatestVersion {
    pub(crate) fn new(value: u32) -> Self {
        LatestVersion { value }
    }
}
