#[derive(derive_new::new, serde::Serialize, Debug)]
pub(crate) struct GetSaveResponse {
    id: String,
    version: u32,
    game_name: String,
    file_name: String,
    data: Vec<u8>,
}
