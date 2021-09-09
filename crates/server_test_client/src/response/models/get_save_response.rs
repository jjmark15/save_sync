#[derive(Debug, serde::Deserialize)]
pub(crate) struct GetSaveResponse {
    id: String,
    version: u32,
    file_name: String,
    game_name: String,
    data: Vec<u8>,
}

impl GetSaveResponse {
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

    pub(crate) fn data(self) -> Vec<u8> {
        self.data
    }
}
