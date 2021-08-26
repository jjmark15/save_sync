use http::status::StatusCode;
use spectral::prelude::*;

use server_test_client::dto::SaveId;

use crate::helpers::TestContext;

#[tokio::test]
async fn check_latest_version() {
    let ctx = TestContext::new();
    let save_id = SaveId::from("save id");

    let response = ctx.client().latest_version(save_id).await;

    assert_that(&response.http_response_details().status()).is_equal_to(StatusCode::FOUND);
    assert_that(response.value()).is_equal_to(1);
}
