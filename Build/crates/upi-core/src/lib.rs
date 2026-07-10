//! Core types and logic for UPI: database, resolution, execution, OS config, and fallback search.

pub mod db;
pub mod error;
pub mod exec;
pub mod fallback;
pub mod os;
pub mod resolver;

pub use db::{repology_cache_dir, Database, Mapping};
pub use error::{Error, Result};
pub use exec::Command;
pub use fallback::{parse_search_output, FallbackSearcher};
pub use os::{detect, expand_env, PlatformConfig, PlatformRegistry};
pub use os_info::Type as OsType;
pub use resolver::{PackageSource, ResolveCandidate, ResolveResult, Resolver};
