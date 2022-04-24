use std::convert::From;

#[derive(Default)]
pub struct Header {
    pub key: String,
    pub value: String,
}

#[derive(Default)]
pub struct Request {
    pub method: String,
    pub url: String,
    pub headers: Vec<Header>,
}

#[derive(Default)]
pub struct Content {
    pub mime_type: Option<String>,
    pub text: Option<String>,
}

#[derive(Default)]
pub struct Response {
    pub status: i64,
    pub status_text: String,
    pub http_version: String,
    pub headers: Vec<Header>,
    pub content: Content,
}

#[derive(Default)]
pub struct Entry {
    pub request: Request,
    pub response: Response,
}

impl From<&har::v1_2::Request> for Request {
    fn from(item: &har::v1_2::Request) -> Self {
        Self {
            method: item.method.clone(),
            url: item.url.clone(),
            headers: Vec::new(),
        }
    }
}

impl From<&har::v1_2::Entries> for Entry {
    fn from(item: &har::v1_2::Entries) -> Self {
        let mut result = Self::default();
        result.request = Request::from(&item.request);
        result.response = Response::from(&item.response);

        result
    }
}

impl From<&har::v1_2::Response> for Response {
    fn from(item: &har::v1_2::Response) -> Self {
        Self {
            status: item.status,
            status_text: item.status_text.clone(),
            http_version: item.http_version.clone(),
            content: Content::from(&item.content),
            headers: Vec::new(),
        }
    }
}

impl From<&har::v1_2::Content> for Content {
    fn from(item: &har::v1_2::Content) -> Self {
        Self {
            mime_type: item.mime_type.clone(),
            text: item.text.clone(),
        }
    }
}
