use include_dir::{include_dir, Dir};
use serde::Deserialize;

use crate::error::{Error, Result};

static PLATFORM_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/../../platform");

#[derive(Debug, Clone, Deserialize)]
pub struct PlatformConfig {
    pub targets: Vec<os_info::Type>,
    pub manager: String,
    pub sudo: bool,
    pub install: String,
    pub search: Option<String>,
    pub provides: Option<String>,
    pub provides_parse: Option<String>,
    pub binary_paths: Vec<String>,
}

pub struct PlatformRegistry {
    configs: Vec<PlatformConfig>,
}

impl PlatformRegistry {
    pub fn load() -> Result<Self> {
        let mut configs = Vec::new();

        for file in PLATFORM_DIR.files() {
            let name = file.path().file_stem().unwrap().to_str().unwrap();
            let yaml = file.contents_utf8().ok_or_else(|| {
                Error::PlatformConfig(format!("{name}: non-UTF-8 content"))
            })?;

            let config: PlatformConfig = noyalib::from_str(yaml)
                .map_err(|e| Error::PlatformConfig(format!("{name}: {e}")))?;

            configs.push(config);
        }

        Ok(Self { configs })
    }

    pub fn for_type(&self, os_type: &os_info::Type) -> Option<&PlatformConfig> {
        self.configs.iter().find(|c| c.targets.contains(os_type))
    }

    pub fn all(&self) -> &[PlatformConfig] {
        &self.configs
    }
}

pub fn detect() -> os_info::Type {
    os_info::get().os_type()
}
