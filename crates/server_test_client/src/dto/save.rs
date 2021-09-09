use crate::dto::SaveId;

#[derive(Clone, Debug)]
pub struct Save {
    id: SaveId,
    version: u32,
    file_name: String,
    game_name: String,
    data: Vec<u8>,
}

impl Save {
    pub(crate) fn new(
        id: SaveId,
        version: u32,
        file_name: String,
        game_name: String,
        data: Vec<u8>,
    ) -> Self {
        Save {
            id,
            version,
            file_name,
            game_name,
            data,
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

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }
}
