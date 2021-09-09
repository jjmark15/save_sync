#[derive(derive_new::new)]
pub(crate) struct StoreNewSaveRequest {
    game_name: String,
    save_data: Vec<u8>,
    file_name: String,
}

impl StoreNewSaveRequest {
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
