use crate::error::Result;

pub struct Resolver;

impl Default for Resolver {
    fn default() -> Self {
        Self
    }
}

impl Resolver {
    pub fn new() -> Self {
        Self
    }

    pub fn resolve(&self, _package: &str) -> Result<String> {
        Err(crate::error::Error::Resolve(
            "resolution not implemented".into(),
        ))
    }
}
