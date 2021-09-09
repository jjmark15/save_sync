#[derive(Debug, serde::Serialize)]
pub(crate) struct StoreNewSaveResponse {
    save_id: String,
}

impl StoreNewSaveResponse {
    pub(crate) fn new(save_id: String) -> Self {
        StoreNewSaveResponse { save_id }
    }
}
