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

#[derive(Clone)]
pub struct PlatformRegistry {
    configs: Vec<PlatformConfig>,
}

impl PlatformRegistry {
    pub fn load() -> Result<Self> {
        let mut configs = Vec::new();

        for file in PLATFORM_DIR.files() {
            let name = file.path().file_stem().unwrap().to_str().unwrap();
            let yaml = file
                .contents_utf8()
                .ok_or_else(|| Error::PlatformConfig(format!("{name}: non-UTF-8 content")))?;

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

    pub fn repo_to_os(&self, repo: &str) -> Option<&os_info::Type> {
        let normalized = normalize_repo_name(repo);
        for config in &self.configs {
            if config.manager.to_lowercase() == repo.to_lowercase() {
                return config.targets.first();
            }
            for target in &config.targets {
                let target_str = format!("{:?}", target).to_lowercase();
                if normalized == target_str || normalized.starts_with(&target_str) {
                    return Some(target);
                }
            }
        }
        None
    }

    pub fn targets(&self) -> Vec<&os_info::Type> {
        let mut seen = Vec::new();
        for config in &self.configs {
            for target in &config.targets {
                if !seen.contains(&target) {
                    seen.push(target);
                }
            }
        }
        seen
    }
}

fn normalize_repo_name(repo: &str) -> String {
    let lower = repo.to_lowercase();
    let mut result = String::new();
    for part in lower.split('_') {
        if part.is_empty() {
            continue;
        }
        if part.bytes().all(|b| b.is_ascii_digit()) {
            break;
        }
        if !result.is_empty() {
            result.push('_');
        }
        result.push_str(part);
    }
    let prefixes = ["linux", "libre"];
    for prefix in &prefixes {
        if let Some(suffix) = result.strip_prefix(prefix) {
            if !suffix.is_empty() {
                return suffix.to_string();
            }
        }
    }
    result
}

pub fn detect() -> os_info::Type {
    os_info::get().os_type()
}
