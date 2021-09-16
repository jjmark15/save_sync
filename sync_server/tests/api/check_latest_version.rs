use http::status::StatusCode;
use spectral::prelude::*;

use crate::helpers::default_values::default_save_id;
use crate::helpers::{store_default_new_save, TestContext};

#[tokio::test]
async fn check_latest_version() {
    let ctx = TestContext::new();
    let save_id = store_default_new_save(&ctx).await;

    let response = ctx.client().latest_version(&save_id).await;

    assert_that(&response.http_response_details().status()).is_equal_to(StatusCode::OK);
    assert_that(response.value()).is_equal_to(0);
}

#[tokio::test]
async fn fails_to_check_latest_version_when_id_does_not_exist() {
    let ctx = TestContext::new();
    let save_id = default_save_id();

    let response = ctx.client().latest_version(&save_id).await;

    assert_that(&response.http_response_details().status()).is_equal_to(&StatusCode::NOT_FOUND);
    assert_that(response.error().message())
        .is_equal_to(&format!("save with ID '{}' not found", save_id));
}
