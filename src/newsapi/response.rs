use serde::Deserialize;

use super::article::Article;

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

  pub fn status(&self) -> &String {
    &self.status
  }

  pub fn code(&self) -> &Option<String>{
    &self.code
  }
}