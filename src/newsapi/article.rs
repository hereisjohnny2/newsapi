use serde::Deserialize;

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