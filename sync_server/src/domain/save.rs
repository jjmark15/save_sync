use crate::domain::{GameName, SaveData, SaveFileName, SaveId, SaveVersion};

#[derive(derive_new::new)]
pub(crate) struct Save {
    id: SaveId,
    version: SaveVersion,
    game_name: GameName,
    file_name: SaveFileName,
    data: SaveData,
}

impl Save {
    pub(crate) fn id(&self) -> &SaveId {
        &self.id
    }

    pub(crate) fn version(&self) -> SaveVersion {
        self.version
    }

    pub(crate) fn game_name(&self) -> &GameName {
        &self.game_name
    }

    pub(crate) fn data(self) -> SaveData {
        self.data
    }

    pub(crate) fn file_name(&self) -> &SaveFileName {
        &self.file_name
    }
}
