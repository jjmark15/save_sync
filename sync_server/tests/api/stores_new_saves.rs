use http::status::StatusCode;
use spectral::prelude::*;
use uuid::Uuid;

use server_test_client::dto::SaveId;
use server_test_client::ResponseWrapper;

use crate::helpers::default_values::{DEFAULT_GAME_NAME, DEFAULT_SAVE_FILE_NAME};
use crate::helpers::{default_file_contents, oversized_file_contents, TestContext};

#[tokio::test]
async fn stores_new_save() {
    let ctx = TestContext::new();

    let response: ResponseWrapper<SaveId> = ctx
        .client()
        .store_new_save(
            DEFAULT_SAVE_FILE_NAME.to_string(),
            DEFAULT_GAME_NAME.to_string(),
            default_file_contents(),
        )
        .await;

    assert_that(&response.http_response_details().status()).is_equal_to(StatusCode::CREATED);
    assert_that(&Uuid::parse_str(response.value().to_string().as_ref())).is_ok();

    let save = ctx
        .client()
        .get_save(response.value())
        .await
        .value()
        .clone();

    assert_that(save.file_name()).is_equal_to(&DEFAULT_SAVE_FILE_NAME.to_string());
    assert_that(save.game_name()).is_equal_to(&DEFAULT_GAME_NAME.to_string());
    assert_that(&save.version()).is_equal_to(0);
    assert_that(save.data()).is_equal_to(&default_file_contents());
}

#[tokio::test]
async fn save_file_name_must_not_be_empty() {
    let ctx = TestContext::new();

    let response: ResponseWrapper<SaveId> = ctx
        .client()
        .store_new_save(
            String::new(),
            DEFAULT_GAME_NAME.to_string(),
            default_file_contents(),
        )
        .await;

    assert_that(&response.http_response_details().status()).is_equal_to(StatusCode::NOT_ACCEPTABLE);
    assert_that(response.error().message()).is_equal_to(
        &"failed to store new save: invalid save metadata: save file name cannot be empty"
            .to_string(),
    );
}

#[tokio::test]
async fn save_file_name_must_not_be_empty_with_whitespace() {
    let ctx = TestContext::new();

    let response: ResponseWrapper<SaveId> = ctx
        .client()
        .store_new_save(
            String::from("  "),
            DEFAULT_GAME_NAME.to_string(),
            default_file_contents(),
        )
        .await;

    assert_that(&response.http_response_details().status()).is_equal_to(StatusCode::NOT_ACCEPTABLE);
    assert_that(response.error().message()).is_equal_to(
        &"failed to store new save: invalid save metadata: save file name cannot be empty"
            .to_string(),
    );
}

#[tokio::test]
async fn save_file_name_is_mandatory() {
    let ctx = TestContext::new();

    let response: ResponseWrapper<SaveId> = ctx
        .client()
        .store_new_save(None, DEFAULT_GAME_NAME.to_string(), default_file_contents())
        .await;

    assert_that(&response.http_response_details().status()).is_equal_to(StatusCode::NOT_ACCEPTABLE);
    assert_that(response.error().message()).is_equal_to(&"missing mandatory data".to_string());
}

#[tokio::test]
async fn save_data_is_mandatory() {
    let ctx = TestContext::new();

    let response: ResponseWrapper<SaveId> = ctx
        .client()
        .store_new_save(
            DEFAULT_SAVE_FILE_NAME.to_string(),
            DEFAULT_GAME_NAME.to_string(),
            None,
        )
        .await;

    assert_that(&response.http_response_details().status()).is_equal_to(StatusCode::NOT_ACCEPTABLE);
    assert_that(response.error().message()).is_equal_to(&"missing mandatory data".to_string());
}

#[tokio::test]
async fn game_name_is_mandatory() {
    let ctx = TestContext::new();

    let response: ResponseWrapper<SaveId> = ctx
        .client()
        .store_new_save(
            DEFAULT_SAVE_FILE_NAME.to_string(),
            None,
            default_file_contents(),
        )
        .await;

    assert_that(&response.http_response_details().status()).is_equal_to(StatusCode::NOT_ACCEPTABLE);
    assert_that(response.error().message()).is_equal_to(&"missing mandatory data".to_string());
}

#[tokio::test]
async fn too_large_save_data_is_not_accepted() {
    let ctx = TestContext::new();

    let response: ResponseWrapper<SaveId> = ctx
        .client()
        .store_new_save(
            DEFAULT_SAVE_FILE_NAME.to_string(),
            DEFAULT_GAME_NAME.to_string(),
            oversized_file_contents(),
        )
        .await;

    assert_that(&response.http_response_details().status())
        .is_equal_to(StatusCode::PAYLOAD_TOO_LARGE);
}
