use std::fmt;

#[derive(Debug)]
pub enum Error {
    UnsupportedOs(String),
    PlatformConfig(String),
    Resolve(String),
    Exec(String),
    Database(String),
    Network(String),
    Io(std::io::Error),
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

pub type Result<T> = std::result::Result<T, Error>;
