#[derive(Debug, serde::Deserialize)]
pub(crate) struct NewSaveResponse {
    save_id: String,
}

impl NewSaveResponse {
    pub(crate) fn save_id(&self) -> &String {
        &self.save_id
    }
}
