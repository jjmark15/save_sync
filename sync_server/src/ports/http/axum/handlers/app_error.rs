use std::convert::Infallible;

use axum::body::{Bytes, Full};
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use axum::Json;

use crate::domain::GetSaveError;
use crate::ports::http::axum::error::StoreNewSaveError;
use crate::ports::http::axum::models::ErrorResponse;

#[derive(Debug, thiserror::Error)]
pub(crate) enum AppError {
    #[error(transparent)]
    GetSave(#[from] GetSaveError),
    #[error(transparent)]
    StoreNewSave(#[from] StoreNewSaveError),
}

impl IntoResponse for AppError {
    type Body = Full<Bytes>;
    type BodyError = Infallible;

    fn into_response(self) -> Response<Self::Body> {
        let (status, error_message) = match self {
            AppError::GetSave(error) => (StatusCode::NOT_FOUND, error.to_string()),
            AppError::StoreNewSave(error) => (StatusCode::NOT_ACCEPTABLE, error.to_string()),
        };

        let body = Json(ErrorResponse::new(error_message));

        (status, body).into_response()
    }
}
