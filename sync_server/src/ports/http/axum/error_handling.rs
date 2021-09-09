use axum::response::IntoResponse;

use crate::ports::http::axum::handlers::AppError;

pub(crate) trait HandleAppResult<T, R: IntoResponse, SM: FnOnce(T) -> R> {
    fn handle_app_result(self, success_mapper: SM) -> Result<R, AppError>;
}

impl<T, E: Into<AppError>, R: IntoResponse, SM: FnOnce(T) -> R> HandleAppResult<T, R, SM>
    for Result<T, E>
{
    fn handle_app_result(self, success_mapper: SM) -> Result<R, AppError> {
        match self {
            Ok(value) => Ok(success_mapper(value)),
            Err(error) => Err(error.into()),
        }
    }
}
