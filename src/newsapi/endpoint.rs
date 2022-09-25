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