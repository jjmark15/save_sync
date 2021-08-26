use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

use crate::ports::http::axum::models::LatestVersion;

pub(crate) async fn latest_version_handler(Path(_save_id): Path<String>) -> impl IntoResponse {
    (StatusCode::FOUND, Json(LatestVersion::new(1)))
}
