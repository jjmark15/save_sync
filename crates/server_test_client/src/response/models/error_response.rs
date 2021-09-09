#[derive(Debug, serde::Deserialize)]
pub(crate) struct ErrorResponse {
    message: String,
}

impl ErrorResponse {
    pub(crate) fn message(&self) -> &String {
        &self.message
    }
}
