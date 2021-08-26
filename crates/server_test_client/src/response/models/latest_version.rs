#[derive(Debug, serde::Deserialize)]
pub(crate) struct LatestVersion {
    #[serde(rename(deserialize = "version"))]
    value: u32,
}

impl LatestVersion {
    pub(crate) fn value(&self) -> u32 {
        self.value
    }
}
