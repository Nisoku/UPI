use std::fmt;

/// Errors from the Repology HTTP client and response parsing.
#[derive(Debug)]
pub enum Error {
    /// HTTP request failed (connection, timeout, non-2xx status other than 404).
    Http(String),
    /// Response body could not be parsed as the expected JSON shape.
    Parse(String),
    /// The requested resource was not found (HTTP 404).
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

/// Convenience alias for `Result<T, upi_net::Error>`.
pub type Result<T> = std::result::Result<T, Error>;
