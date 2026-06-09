use std::collections::HashSet;

use crate::db::Database;
use crate::error::{Error, Result};
use crate::exec::Command;
use crate::fallback::FallbackSearcher;
use crate::os::{detect, PlatformRegistry};

/// A source that can resolve or search for packages for a given OS.
pub trait PackageSource {
    /// Resolve a package name to the OS-specific name, if known.
    fn resolve_package(&self, package: &str, os_type: &os_info::Type) -> Result<Option<String>>;

    /// Search for packages matching a query string.
    ///
    /// Default returns `Ok(None)` so sources that lack search skip it without boilerplate.
    fn search_packages(&self, query: &str, os_type: &os_info::Type) -> Result<Option<Vec<String>>> {
        let _ = (query, os_type);
        Ok(None)
    }
}

/// A package name found during search, with its provenance label.
#[derive(Debug, Clone)]
pub struct ResolveCandidate {
    /// The OS-specific package name.
    pub name: String,
    /// Where this candidate was found: `repology`, `database`, `fallback search`, etc.
    pub source: String,
}

/// Orchestrates package resolution across multiple sources: Repology, database, fallback, identity.
///
/// Tries network sources first, then local database, fallback search, and finally identity pass-through.
pub struct Resolver {
    registry: PlatformRegistry,
    db: Database,
    sources: Vec<Box<dyn PackageSource>>,
}

impl Resolver {
    /// Create a resolver with default settings and an empty source list.
    ///
    /// Uses the global `PlatformRegistry` and the default database path.
    pub fn new() -> Result<Self> {
        let registry = PlatformRegistry::global().clone();
        let db = Database::open()?;
        Ok(Self {
            registry,
            db,
            sources: Vec::new(),
        })
    }

    /// Create a resolver with a custom set of network/source providers.
    pub fn with_sources(sources: Vec<Box<dyn PackageSource>>) -> Result<Self> {
        let mut resolver = Self::new()?;
        resolver.sources = sources;
        Ok(resolver)
    }

    /// Create a resolver with a specific registry and source list (for testing / offline mode).
    pub fn with_registry_and_sources(
        registry: PlatformRegistry,
        sources: Vec<Box<dyn PackageSource>>,
    ) -> Result<Self> {
        let db = Database::open()?;
        Ok(Self {
            registry,
            db,
            sources,
        })
    }

    /// Resolve a package name to an install command for the detected OS.
    pub fn resolve(&self, package: &str) -> Result<Command> {
        let os_type = detect();
        log::debug!("detected OS: {os_type:?}");
        self.resolve_for_os(package, &os_type)
    }

    /// Resolve a package name to an install command for a specific OS.
    pub fn resolve_for_os(&self, package: &str, os_type: &os_info::Type) -> Result<Command> {
        let config = self
            .registry
            .for_type(os_type)
            .ok_or_else(|| Error::UnsupportedOs(format!("{os_type:?}")))?;

        log::debug!("using config: {} (sudo={})", config.manager, config.sudo);
        let (os_package, _source) = self.lookup_package_name(package, os_type)?;
        log::info!("resolved '{}' -> '{}'", package, os_package);

        Ok(Command::from_config(config, &os_package))
    }

    /// Resolve with full provenance: returns `(command, os_package, source_label, manager_name)`.
    pub fn resolve_detailed(
        &self,
        package: &str,
        os_type: &os_info::Type,
    ) -> Result<(Command, String, String, String)> {
        let config = self
            .registry
            .for_type(os_type)
            .ok_or_else(|| Error::UnsupportedOs(format!("{os_type:?}")))?;

        let (os_package, source) = self.lookup_package_name(package, os_type)?;
        let cmd = Command::from_config(config, &os_package);

        Ok((cmd, os_package, source, config.manager.clone()))
    }

    /// Resolve a package and return the command plus the database mapping, if any.
    ///
    /// Only checks the local database. Does not consult network sources or fallback.
    pub fn resolve_with_provenance(
        &self,
        package: &str,
    ) -> Result<(Command, Option<crate::db::Mapping>)> {
        let os_type = detect();
        self.resolve_with_provenance_for_os(package, &os_type)
    }

    /// Resolve for a specific OS and return the command plus the database mapping, if any.
    ///
    /// Only checks the local database. Does not consult network sources or fallback.
    pub fn resolve_with_provenance_for_os(
        &self,
        package: &str,
        os_type: &os_info::Type,
    ) -> Result<(Command, Option<crate::db::Mapping>)> {
        let config = self
            .registry
            .for_type(os_type)
            .ok_or_else(|| Error::UnsupportedOs(format!("{os_type:?}")))?;

        let mapping = self.db.lookup(package, os_type)?;
        let os_package = mapping
            .as_ref()
            .map(|m| m.os_package.clone())
            .unwrap_or_else(|| package.to_string());

        Ok((Command::from_config(config, &os_package), mapping))
    }

    /// Search for packages across all sources: Repology, database, fallback, and identity.
    ///
    /// Deduplicates by package name. Returns candidates ordered by source priority.
    pub fn search_candidates(
        &self,
        query: &str,
        os_type: &os_info::Type,
    ) -> Result<Vec<ResolveCandidate>> {
        let mut candidates = Vec::new();
        let mut seen = HashSet::new();

        for source in &self.sources {
            if let Some(names) = source.search_packages(query, os_type)? {
                for name in names {
                    if seen.insert(name.clone()) {
                        candidates.push(ResolveCandidate {
                            name,
                            source: "repology".into(),
                        });
                    }
                }
            }
        }

        for mapping in self.db.search(query, os_type)? {
            if seen.insert(mapping.os_package.clone()) {
                candidates.push(ResolveCandidate {
                    name: mapping.os_package,
                    source: format!("database (confidence={})", mapping.confidence),
                });
            }
        }

        if let Some(config) = self.registry.for_type(os_type) {
            if let Some(searcher) = FallbackSearcher::from_config(config) {
                if let Some(name) = searcher.search(query)? {
                    if seen.insert(name.clone()) {
                        candidates.push(ResolveCandidate {
                            name,
                            source: "fallback search".into(),
                        });
                    }
                }
            }
        }

        if seen.insert(query.to_string()) {
            candidates.push(ResolveCandidate {
                name: query.to_string(),
                source: "identity".into(),
            });
        }

        Ok(candidates)
    }

    fn lookup_package_name(
        &self,
        package: &str,
        os_type: &os_info::Type,
    ) -> Result<(String, String)> {
        for (i, source) in self.sources.iter().enumerate() {
            log::debug!("source[{i}]: trying to resolve '{package}'");
            if let Some(name) = source.resolve_package(package, os_type)? {
                log::debug!("source[{i}]: found '{name}'");
                return Ok((name, "repology".into()));
            }
        }

        log::debug!("db: looking up '{package}' on {os_type:?}");
        if let Some(mapping) = self.db.lookup(package, os_type)? {
            log::debug!(
                "db: hit '{}' (confidence={})",
                mapping.os_package,
                mapping.confidence
            );
            let source = format!("database (confidence={})", mapping.confidence);
            return Ok((mapping.os_package, source));
        }

        log::debug!("fallback: searching for '{package}'");
        if let Some(config) = self.registry.for_type(os_type) {
            if let Some(searcher) = FallbackSearcher::from_config(config) {
                if let Some(name) = searcher.search(package)? {
                    log::debug!("fallback: found '{name}'");
                    return Ok((name, "fallback search".into()));
                }
            }
        }

        log::debug!("identity: using '{package}' as-is");
        Ok((package.to_string(), "identity".into()))
    }
}
