use crate::error::{Error, Result};
use crate::exec::Command;
use crate::os::{detect, PlatformRegistry};

pub struct Resolver {
    registry: PlatformRegistry,
}

impl Resolver {
    pub fn new() -> Result<Self> {
        let registry = PlatformRegistry::load()?;
        Ok(Self { registry })
    }

    pub fn resolve(&self, package: &str) -> Result<Command> {
        let os_type = detect();
        let config = self
            .registry
            .for_type(&os_type)
            .ok_or_else(|| Error::UnsupportedOs(format!("{os_type:?}")))?;

        Ok(Command::from_config(config, package))
    }
}
