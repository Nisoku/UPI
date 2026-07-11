use std::fmt;

/// Errors that can occur during UPI operations.
#[derive(Debug)]
pub enum Error {
    /// The detected or specified OS has no matching platform config.
    UnsupportedOs(String),
    /// A YAML platform definition is malformed or missing required fields.
    PlatformConfig(String),
    /// Package resolution failed across all available sources.
    Resolve(String),
    /// The install command or a subprocess failed.
    Exec(String),
    /// Database open, query, or rehydration failed.
    Database(String),
    /// A network request failed (HTTP, DNS, timeout).
    Network(String),
    /// An underlying I/O operation failed.
    Io(std::io::Error),
    /// A required command or shell was not found on PATH.
    ProgramNotFound(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::UnsupportedOs(msg) => write!(f, "unsupported OS: {msg}"),
            Error::PlatformConfig(msg) => write!(f, "platform config error: {msg}"),
            Error::Resolve(msg) => write!(f, "resolve error: {msg}"),
            Error::Exec(msg) => write!(f, "execution error: {msg}"),
            Error::Database(msg) => write!(f, "database error: {msg}"),
            Error::Network(msg) => write!(f, "network error: {msg}"),
            Error::Io(err) => write!(f, "I/O error: {err}"),
            Error::ProgramNotFound(program) => write!(f, "program not found: {program}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

/// Convenience alias for `Result<T, upi_core::Error>`.
pub type Result<T> = std::result::Result<T, Error>;
