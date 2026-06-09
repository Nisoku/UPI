use crate::error::Result;

pub struct RepologyClient {
    #[allow(dead_code)]
    base_url: String,
}

impl Default for RepologyClient {
    fn default() -> Self {
        Self {
            base_url: "https://repology.org/api/v1".into(),
        }
    }
}

impl RepologyClient {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn resolve(&self, _project: &str) -> Result<Option<String>> {
        Err(crate::error::Error::NotFound(
            "repology not implemented".into(),
        ))
    }
}
