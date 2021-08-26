use axum::http::StatusCode;

pub(crate) async fn admin_status_handler() -> StatusCode {
    StatusCode::OK
}
