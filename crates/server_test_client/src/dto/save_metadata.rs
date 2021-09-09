use crate::dto::SaveId;

#[derive(Clone, Debug)]
pub struct SaveMetadata {
    id: SaveId,
    version: u32,
    file_name: String,
    game_name: String,
}

impl SaveMetadata {
    pub(crate) fn new(id: SaveId, version: u32, file_name: String, game_name: String) -> Self {
        SaveMetadata {
            id,
            version,
            file_name,
            game_name,
        }
    }

    pub fn id(&self) -> &SaveId {
        &self.id
    }

    pub fn file_name(&self) -> &String {
        &self.file_name
    }

    pub fn game_name(&self) -> &String {
        &self.game_name
    }

    pub fn version(&self) -> u32 {
        self.version
    }
}
