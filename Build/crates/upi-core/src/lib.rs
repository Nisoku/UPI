pub mod db;
pub mod error;
pub mod exec;
pub mod os;
pub mod resolver;

pub use error::{Error, Result};
pub use exec::Command;
pub use os::{detect, PlatformConfig, PlatformRegistry};
pub use resolver::Resolver;
pub use os_info::Type as OsType;
