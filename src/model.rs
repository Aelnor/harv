use std::convert::From;

#[derive(Default)]
pub struct Header {
    pub name: String,
    pub value: String,
}

#[derive(Default)]
pub struct QueryString {
    pub name: String,
    pub value: String,
}

#[derive(Default)]
pub struct PostData {
    pub mime_type: String,
    pub text: Option<String>,
}

#[derive(Default)]
pub struct Request {
    pub method: String,
    pub url: String,
    pub headers: Vec<Header>,
    pub query_string: Vec<QueryString>,
    pub post_data: Option<PostData>,
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
            headers: item.headers.iter().map(|e| Header::from(e)).collect(),
            query_string: item
                .query_string
                .iter()
                .map(|e| QueryString {
                    name: e.name.clone(),
                    value: e.value.clone(),
                })
                .collect(),

            post_data: if item.post_data.is_none() {
                None
            } else {
                Some(PostData {
                    mime_type: item.post_data.clone().unwrap().mime_type,
                    text: item.post_data.clone().unwrap().text,
                })
            },
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
            headers: item.headers.iter().map(|e| Header::from(e)).collect(),
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

impl From<&har::v1_2::Headers> for Header {
    fn from(item: &har::v1_2::Headers) -> Self {
        Self {
            name: item.name.clone(),
            value: item.value.clone(),
        }
    }
}
