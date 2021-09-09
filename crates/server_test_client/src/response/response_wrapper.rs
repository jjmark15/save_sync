use crate::dto::ApiError;
use crate::response::HttpResponseDetails;

#[derive(Debug)]
pub struct ResponseWrapper<T> {
    http_response_details: HttpResponseDetails,
    value: Option<T>,
    api_error: Option<ApiError>,
}

impl<T> ResponseWrapper<T> {
    pub(crate) fn new(
        http_response_details: HttpResponseDetails,
        value: Option<T>,
        api_error: Option<ApiError>,
    ) -> Self {
        ResponseWrapper {
            http_response_details,
            value,
            api_error,
        }
    }

    pub fn http_response_details(&self) -> &HttpResponseDetails {
        &self.http_response_details
    }

    pub fn value(&self) -> &T {
        self.value
            .as_ref()
            .expect("expected test response to have a value")
    }

    pub fn error(&self) -> &ApiError {
        self.api_error
            .as_ref()
            .expect("expected test response to have an error")
    }
}
