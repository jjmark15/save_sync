use http::status::StatusCode;
use spectral::prelude::*;

use crate::helpers::TestContext;

#[tokio::test]
async fn reports_ok_server_status() {
    let ctx = TestContext::new();

    let response = ctx.client().status().await;

    assert_that(&response.http_response_details().status()).is_equal_to(StatusCode::OK);
}
