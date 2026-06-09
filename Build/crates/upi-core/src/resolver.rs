use crate::db::Database;
use crate::error::{Error, Result};
use crate::exec::Command;
use crate::os::{detect, PlatformRegistry};

pub trait PackageSource {
    fn resolve_package(
        &self,
        package: &str,
        os_type: &os_info::Type,
    ) -> Result<Option<String>>;
}

pub struct Resolver {
    registry: PlatformRegistry,
    db: Database,
    sources: Vec<Box<dyn PackageSource>>,
}

impl Resolver {
    pub fn new() -> Result<Self> {
        let registry = PlatformRegistry::load()?;
        let db = Database::open()?;
        Ok(Self {
            registry,
            db,
            sources: Vec::new(),
        })
    }

    pub fn with_sources(sources: Vec<Box<dyn PackageSource>>) -> Result<Self> {
        let mut resolver = Self::new()?;
        resolver.sources = sources;
        Ok(resolver)
    }

    pub fn with_registry_and_sources(
        registry: PlatformRegistry,
        sources: Vec<Box<dyn PackageSource>>,
    ) -> Result<Self> {
        let db = Database::open()?;
        Ok(Self { registry, db, sources })
    }

    pub fn resolve(&self, package: &str) -> Result<Command> {
        let os_type = detect();

        let config = self
            .registry
            .for_type(&os_type)
            .ok_or_else(|| Error::UnsupportedOs(format!("{os_type:?}")))?;

        let os_package = self.lookup_package_name(package, &os_type)?;

        Ok(Command::from_config(config, &os_package))
    }

    pub fn resolve_with_provenance(
        &self,
        package: &str,
    ) -> Result<(Command, Option<crate::db::Mapping>)> {
        let os_type = detect();

        let config = self
            .registry
            .for_type(&os_type)
            .ok_or_else(|| Error::UnsupportedOs(format!("{os_type:?}")))?;

        let mapping = self.db.lookup(package, &os_type)?;
        let os_package = mapping
            .as_ref()
            .map(|m| m.os_package.clone())
            .unwrap_or_else(|| package.to_string());

        Ok((Command::from_config(config, &os_package), mapping))
    }

    fn lookup_package_name(&self, package: &str, os_type: &os_info::Type) -> Result<String> {
        for source in &self.sources {
            if let Some(name) = source.resolve_package(package, os_type)? {
                return Ok(name);
            }
        }

        if let Some(mapping) = self.db.lookup(package, os_type)? {
            return Ok(mapping.os_package);
        }

        Ok(package.to_string())
    }
}
