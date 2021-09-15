use std::str::FromStr;

use axum::extract::{Extension, Path};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use uuid::Uuid;

use crate::ports::http::axum::handlers::app_error::AppError;
use crate::ports::http::axum::models::LatestVersionResponse;
use crate::DynApplicationService;

pub(crate) async fn latest_version_handler(
    Path(save_id): Path<String>,
    Extension(application_service): Extension<DynApplicationService>,
) -> Result<impl IntoResponse, AppError> {
    application_service
        .get_latest_version(Uuid::from_str(save_id.as_str()).unwrap())
        .await
        .map_err(AppError::from)
        .map(|version| (StatusCode::OK, Json(LatestVersionResponse::new(version))))
}
