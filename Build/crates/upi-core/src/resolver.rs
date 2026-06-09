use crate::db::Database;
use crate::error::{Error, Result};
use crate::exec::Command;
use crate::os::{detect, PlatformRegistry};

pub struct Resolver {
    registry: PlatformRegistry,
    db: Database,
}

impl Resolver {
    pub fn new() -> Result<Self> {
        let registry = PlatformRegistry::load()?;
        let db = Database::open()?;
        Ok(Self { registry, db })
    }

    pub fn resolve(&self, package: &str) -> Result<Command> {
        let os_type = detect();

        let config = self
            .registry
            .for_type(&os_type)
            .ok_or_else(|| Error::UnsupportedOs(format!("{os_type:?}")))?;

        let os_package = self
            .db
            .lookup(package, &os_type)?
            .map(|m| m.os_package)
            .unwrap_or_else(|| package.to_string());

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
}
