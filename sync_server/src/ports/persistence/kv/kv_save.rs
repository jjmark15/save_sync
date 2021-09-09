#[derive(serde::Serialize, serde::Deserialize, PartialEq)]
pub(crate) struct KvSave {
    save_version: u32,
    game_name: String,
    file_name: String,
    save_data: Vec<u8>,
}

impl KvSave {
    pub(crate) fn new(
        save_version: u32,
        game_name: String,
        file_name: String,
        save_data: Vec<u8>,
    ) -> Self {
        KvSave {
            save_version,
            game_name,
            save_data,
            file_name,
        }
    }

    pub(crate) fn save_version(&self) -> u32 {
        self.save_version
    }

    pub(crate) fn game_name(&self) -> &String {
        &self.game_name
    }

    pub(crate) fn save_data(self) -> Vec<u8> {
        self.save_data
    }

    pub(crate) fn file_name(&self) -> &String {
        &self.file_name
    }
}
