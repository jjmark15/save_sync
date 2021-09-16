use http::status::StatusCode;
use spectral::prelude::*;

use crate::helpers::default_values::{
    default_file_contents, default_save_id, DEFAULT_GAME_NAME, DEFAULT_SAVE_FILE_NAME,
};
use crate::helpers::{store_default_new_save, TestContext};

#[tokio::test]
async fn gets_save() {
    let ctx = TestContext::new();
    let save_id = store_default_new_save(&ctx).await;

    let response = ctx.client().get_save(&save_id).await;

    assert_that(&response.http_response_details().status()).is_equal_to(&StatusCode::OK);

    let save = response.value().clone();

    assert_that(save.file_name()).is_equal_to(&DEFAULT_SAVE_FILE_NAME.to_string());
    assert_that(save.game_name()).is_equal_to(&DEFAULT_GAME_NAME.to_string());
    assert_that(&save.version()).is_equal_to(0);
    assert_that(save.data()).is_equal_to(&default_file_contents());
}

#[tokio::test]
async fn fails_to_get_save_when_id_does_not_exist() {
    let ctx = TestContext::new();
    let save_id = default_save_id();

    let response = ctx.client().get_save(&save_id).await;

    assert_that(&response.http_response_details().status()).is_equal_to(&StatusCode::NOT_FOUND);
    assert_that(response.error().message())
        .is_equal_to(&format!("save with ID '{}' not found", save_id));
}
