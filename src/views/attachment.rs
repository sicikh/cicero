use axum::response::IntoResponse;
use http::{header, HeaderMap, HeaderValue};
use tracing::error;

// TODO: use axum-extra 0.9.4, once it releases
#[derive(Debug)]
pub struct Attachment<T> {
    inner: T,
    filename: Option<HeaderValue>,
    content_type: Option<HeaderValue>,
}

impl<T: IntoResponse> Attachment<T> {
    /// Creates a new [`Attachment`].
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            filename: None,
            content_type: None,
        }
    }

    /// Sets the filename of the [`Attachment`].
    ///
    /// This updates the `Content-Disposition` header to add a filename.
    pub fn filename<H: TryInto<HeaderValue>>(mut self, value: H) -> Self {
        self.filename = if let Ok(filename) = value.try_into() {
            Some(filename)
        } else {
            error!("Attachment filename contains invalid characters");
            None
        };
        self
    }

    /// Sets the content-type of the [`Attachment`]
    pub fn content_type<H: TryInto<HeaderValue>>(mut self, value: H) -> Self {
        if let Ok(content_type) = value.try_into() {
            self.content_type = Some(content_type);
        } else {
            error!("Attachment content-type contains invalid characters");
        }
        self
    }
}

impl<T> IntoResponse for Attachment<T>
where
    T: IntoResponse,
{
    fn into_response(self) -> axum::response::Response {
        let mut headers = HeaderMap::new();

        if let Some(content_type) = self.content_type {
            headers.append(header::CONTENT_TYPE, content_type);
        }

        let content_disposition = if let Some(filename) = self.filename {
            let mut bytes = b"attachment; filename=\"".to_vec();
            bytes.extend_from_slice(filename.as_bytes());
            bytes.push(b'\"');

            HeaderValue::from_bytes(&bytes).expect("This was a HeaderValue so this can not fail")
        } else {
            HeaderValue::from_static("attachment")
        };

        headers.append(header::CONTENT_DISPOSITION, content_disposition);

        (headers, self.inner).into_response()
    }
}