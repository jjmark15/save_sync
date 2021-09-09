use server_test_client::dto::SaveId;

use crate::helpers::default_values::{DEFAULT_GAME_NAME, DEFAULT_SAVE_FILE_NAME};
use crate::helpers::{default_file_contents, TestContext};

pub(crate) async fn store_default_new_save(ctx: &TestContext) -> SaveId {
    ctx.client()
        .store_new_save(
            DEFAULT_SAVE_FILE_NAME.to_string(),
            DEFAULT_GAME_NAME.to_string(),
            default_file_contents(),
        )
        .await
        .value()
        .clone()
}
