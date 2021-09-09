#[derive(Debug)]
pub struct ApiError {
    message: String,
}

impl ApiError {
    pub(crate) fn new(message: String) -> Self {
        ApiError { message }
    }

    pub fn message(&self) -> &String {
        &self.message
    }
}
