use uuid::Uuid;

use server_test_client::dto::SaveId;

pub(crate) const DEFAULT_GAME_NAME: &str = "game name";
pub(crate) const DEFAULT_SAVE_FILE_NAME: &str = "file_name.ext";

pub(crate) fn default_file_contents() -> Vec<u8> {
    vec![b'X'; 249 * 1024]
}

pub(crate) fn oversized_file_contents() -> Vec<u8> {
    vec![b'X'; 250 * 1024]
}

pub(crate) fn default_save_id() -> SaveId {
    SaveId::new(Uuid::nil().to_string())
}
