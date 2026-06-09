use crate::error::Result;

pub struct Database;

impl Database {
    pub fn new() -> Result<Self> {
        Err(crate::error::Error::Database(
            "db not implemented".into(),
        ))
    }

    pub fn lookup(&self, _package: &str) -> Result<Option<String>> {
        Err(crate::error::Error::Database(
            "db not implemented".into(),
        ))
    }
}
