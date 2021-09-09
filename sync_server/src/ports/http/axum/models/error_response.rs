#[derive(serde::Serialize)]
pub(crate) struct ErrorResponse {
    message: String,
}

impl ErrorResponse {
    pub(crate) fn new(message: String) -> Self {
        ErrorResponse { message }
    }
}
