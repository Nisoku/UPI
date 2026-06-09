use std::fmt;

#[derive(Debug)]
pub enum Error {
    Http(String),
    Parse(String),
    NotFound(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Http(msg) => write!(f, "HTTP error: {msg}"),
            Error::Parse(msg) => write!(f, "parse error: {msg}"),
            Error::NotFound(msg) => write!(f, "not found: {msg}"),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
