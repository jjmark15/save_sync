use http::header::HeaderName;
use http::{HeaderMap, HeaderValue, StatusCode};

pub struct HttpResponseDetails {
    status_code: StatusCode,
    headers: HeaderMap,
}

impl HttpResponseDetails {
    pub fn new(status_code: StatusCode, headers: HeaderMap) -> Self {
        HttpResponseDetails {
            status_code,
            headers,
        }
    }

    pub fn status(&self) -> StatusCode {
        self.status_code
    }

    pub fn header(&self, header_name: HeaderName) -> Option<&HeaderValue> {
        self.headers.get(header_name)
    }
}

impl From<&reqwest::Response> for HttpResponseDetails {
    fn from(response: &reqwest::Response) -> Self {
        HttpResponseDetails::new(response.status(), response.headers().clone())
    }
}
