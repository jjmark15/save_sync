use http::status::StatusCode;
use spectral::prelude::*;

use crate::helpers::{store_default_new_save, TestContext};

#[tokio::test]
async fn check_latest_version() {
    let ctx = TestContext::new();
    let save_id = store_default_new_save(&ctx).await;

    let response = ctx.client().latest_version(save_id).await;

    assert_that(&response.http_response_details().status()).is_equal_to(StatusCode::OK);
    assert_that(response.value()).is_equal_to(0);
}
