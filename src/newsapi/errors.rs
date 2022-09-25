use thiserror::Error;

#[derive(Error, Debug)]
pub enum NewsAPIError {
    #[error("Failed fetching articles")]
    RequestFailed(#[from] ureq::Error),
    
    #[error("Failed converting response to string")]
    FailedResponseToString(#[from] std::io::Error),
    
    #[error("Failed parsing articles data")]
    ArticleParseFailed(serde_json::Error),
    
    #[error("Failed parsing url")]
    UrlParsing(#[from] url::ParseError),

    #[error("Request failed: {0}")]
    BadRequest(&'static str),

    #[error("Async request failed: {0}")]
    #[cfg(feature = "async")]
    AsyncRequestFailed(#[from] reqwest::Error),
}

pub fn map_response_error(code: &Option<String>) -> NewsAPIError {
  if let Some(code) = code {
      match code.as_str() {
          "apiKeyDisable" => NewsAPIError::BadRequest("Your API key has been disables"),
          _ => NewsAPIError::BadRequest("Unknown error"),
      }
  } else {
      NewsAPIError::BadRequest("Unknown error")
  }
}