use crate::http_client::{error::FetchError, http_timeout_secs};
use bytes::Bytes;

#[derive(Debug, Clone)]
pub struct HttpClient(reqwest::Client);

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpClient {
    #[must_use]
    pub fn new() -> Self {
        Self(
            reqwest::Client::builder()
                .user_agent("tombi-language-server")
                .timeout(std::time::Duration::from_secs(http_timeout_secs()))
                .build()
                .expect("Failed to create reqwest client"),
        )
    }

    /// Fetch raw bytes from the given URL.
    ///
    /// # Errors
    ///
    /// Returns a `FetchError` if the HTTP request fails, the response status is not successful, or reading the response body fails.
    pub async fn get_bytes(&self, url: &str) -> Result<Bytes, FetchError> {
        let response = self
            .0
            .get(url)
            .send()
            .await
            .map_err(|err| FetchError::FetchFailed {
                reason: err.to_string(),
            })?;

        if !response.status().is_success() {
            return Err(FetchError::StatusNotOk {
                status: response.status().as_u16(),
            });
        }

        response
            .bytes()
            .await
            .map_err(|err| FetchError::BodyReadFailed {
                reason: err.to_string(),
            })
    }
}
