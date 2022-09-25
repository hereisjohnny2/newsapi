#[cfg(feature = "async")]
use reqwest::Method;
use serde::Deserialize;
use thiserror::Error;
use url::Url;

const BASE_URL: &str = "https://newsapi.org/v2";

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

#[derive(Deserialize, Debug)]
pub struct NewAPIResponse {
    articles: Vec<Article>,
    status: String,
    code: Option<String>,
}

impl NewAPIResponse {
    pub fn articles(&self) -> &Vec<Article> {
        &self.articles
    }
}

#[derive(Deserialize, Debug)]
pub struct Article {
    title: String,
    url: String,
    description: Option<String>,
}

impl Article {
    pub fn title(&self) -> &str {
        self.title.as_str()
    }

    pub fn url(&self) -> &str {
        self.url.as_str()
    }

    pub fn desc(&self) -> Option<&String> {
        self.description.as_ref()
    }
}

pub enum Endpoint {
    TopHeadline,
}

impl ToString for Endpoint {
    fn to_string(&self) -> String {
        match self {
            Self::TopHeadline => "top-headlines".to_string(),
        }
    }
}

pub struct NewsAPI {
    api_key: String,
    endpoint: Endpoint,
    country: String,
}

impl NewsAPI {
    pub fn new(api_key: &str) -> NewsAPI {
        NewsAPI {
            api_key: api_key.to_string(),
            endpoint: Endpoint::TopHeadline,
            country: String::from("us"),
        }
    }

    pub fn endpoint(&mut self, endpoint: Endpoint) -> &mut NewsAPI {
        self.endpoint = endpoint;
        self
    }

    pub fn country(&mut self, country: &str) -> &mut NewsAPI {
        self.country = country.to_string();
        self
    }

    fn prepare_url(&self) -> Result<String, NewsAPIError> {
        let mut url = Url::parse(BASE_URL)?;
        url.path_segments_mut()
            .unwrap()
            .push(&self.endpoint.to_string());

        let country = format!("country={}", self.country.to_string());
        url.set_query(Some(&country));

        Ok(url.to_string())
    }

    pub fn fetch(&self) -> Result<NewAPIResponse, NewsAPIError> {
        let url = self.prepare_url()?;
        let req = ureq::get(&url).set("Authorization", &self.api_key);
        let response: NewAPIResponse = req.call()?.into_json()?;

        match response.status.as_str() {
            "ok" => Ok(response),
            _ => return Err(map_response_error(response.code)),
        }
    }

    #[cfg(feature = "async")]
    pub async fn fetch_async(&self) -> Result<NewAPIResponse, NewsAPIError> {
        let url = self.prepare_url()?;

        let client = reqwest::Client::new();

        let request = client
            .request(Method::GET, url)
            .header("Authorization", &self.api_key)
            .build()
            .map_err(|e| NewsAPIError::AsyncRequestFailed(e))?;

        let response = client.execute(request).await?;

        let response_json: NewAPIResponse = response
            .json()
            .await
            .map_err(|e| NewsAPIError::AsyncRequestFailed(e))?;

        match response_json.status.as_str() {
            "ok" => Ok(response_json),
            _ => return Err(map_response_error(response_json.code)),
        }
    }
}

fn map_response_error(code: Option<String>) -> NewsAPIError {
    if let Some(code) = code {
        match code.as_str() {
            "apiKeyDisable" => NewsAPIError::BadRequest("Your API key has been disables"),
            _ => NewsAPIError::BadRequest("Unknown error"),
        }
    } else {
        NewsAPIError::BadRequest("Unknown error")
    }
}
