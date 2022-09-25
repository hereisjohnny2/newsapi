use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Article {
    source: Source,
    author: String,
    title: String,
    description: Option<String>,
    url: String,
    url_to_image: String,
    published_at: String,
    content: String,
}

impl Article {
    pub fn source(&self) -> &Source {
        &self.source
    }

    pub fn author(&self) -> &str {
        &self.author
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn url(&self) -> &str {
        &self.url
    }
    
    pub fn url_to_image(&self) -> &str {
        &self.url_to_image
    }
    
    pub fn published_at(&self) -> &str {
        &self.published_at
    }
    
    pub fn content(&self) -> &str {
        &self.content
    }
}

#[derive(Deserialize, Debug)]
pub struct Source {
    id: String,
    name: String,
    description: String, 
    url: String,
    category: String,
    language: String,
    country: String,
}

impl Source {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn category(&self) -> &str {
        &self.category
    }

    pub fn language(&self) -> &str {
        &self.language
    }

    pub fn country(&self) -> &str {
        &self.country
    }
}
