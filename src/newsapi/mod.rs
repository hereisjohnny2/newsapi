pub mod article;
pub mod endpoint;
pub mod errors;
pub mod response;

#[cfg(feature = "async")]
use reqwest::Method;
use url::Url;

use self::{
    endpoint::Endpoint,
    errors::{map_response_error, NewsAPIError},
    response::NewAPIResponse,
};

const BASE_URL: &str = "https://newsapi.org/v2";

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

    pub fn fetch(&self) -> Result<NewAPIResponse, NewsAPIError> {
        let url = self.prepare_url()?;
        let req = ureq::get(&url).set("Authorization", &self.api_key);
        let response: NewAPIResponse = req.call()?.into_json()?;

        match response.status().as_str() {
            "ok" => Ok(response),
            _ => return Err(map_response_error(response.code())),
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

    fn prepare_url(&self) -> Result<String, NewsAPIError> {
        let mut url = Url::parse(BASE_URL)?;
        url.path_segments_mut()
            .unwrap()
            .push(&self.endpoint.to_string());

        let country = format!("country={}", self.country.to_string());
        url.set_query(Some(&country));

        Ok(url.to_string())
    }
}
