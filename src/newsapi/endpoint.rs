pub enum Endpoint {
    Everything,
    TopHeadline,
    Sources,
}

impl ToString for Endpoint {
    fn to_string(&self) -> String {
        match self {
            Self::TopHeadline => "top-headlines".to_string(),
            Self::Everything => "everything".to_string(),
            Self::Sources => "top-headlines/sources".to_string(),
        }
    }
}
