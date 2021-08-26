use crate::response::HttpResponseDetails;

pub struct ResponseWrapper<T> {
    http_response_details: HttpResponseDetails,
    value: Option<T>,
}

impl<T> ResponseWrapper<T> {
    pub(crate) fn new(http_response_details: HttpResponseDetails, value: Option<T>) -> Self {
        ResponseWrapper {
            http_response_details,
            value,
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
}
