#[derive(Debug, serde::Deserialize)]
pub(crate) struct SaveMetadataResponse {
    id: String,
    version: u32,
    file_name: String,
    game_name: String,
}

impl SaveMetadataResponse {
    pub(crate) fn id(&self) -> &String {
        &self.id
    }

    pub(crate) fn file_name(&self) -> &String {
        &self.file_name
    }

    pub(crate) fn game_name(&self) -> &String {
        &self.game_name
    }

    pub(crate) fn version(&self) -> u32 {
        self.version
    }
}
