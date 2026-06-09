use include_dir::{include_dir, Dir};
use noyalib;
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

impl PlatformConfig {
    /// Return binary paths with environment variables expanded.
    pub fn expanded_binary_paths(&self) -> Vec<String> {
        self.binary_paths.iter().map(|p| expand_env(p)).collect()
    }
}

impl PlatformRegistry {
    pub fn load() -> Result<Self> {
        let mut configs = Vec::new();

        for file in PLATFORM_DIR.files() {
            let name = file.path().file_stem().unwrap().to_str().unwrap();
            let yaml_str = file
                .contents_utf8()
                .ok_or_else(|| Error::PlatformConfig(format!("{name}: non-UTF-8 content")))?;

            match noyalib::from_str::<PlatformConfig>(yaml_str) {
                Ok(config) => configs.push(config),
                Err(e) => {
                    if yaml_str.trim().is_empty() {
                        continue;
                    }
                    return Err(Error::PlatformConfig(format!("{name}: {e}")));
                }
            }
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

    pub fn parse_os(&self, name: &str) -> Option<&os_info::Type> {
        let normalized = name.to_lowercase().replace(['-', '_'], "");
        for config in &self.configs {
            for target in &config.targets {
                let t = format!("{:?}", target).to_lowercase();
                if t == normalized || t.starts_with(&normalized) || normalized.starts_with(&t) {
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

/// Expand environment variables in a string.
///
/// Supports Windows-style `%VAR%` and Unix-style `$VAR` / `${VAR}`.
/// Unset variables are left as-is to allow fallback resolution.
pub fn expand_env(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '%' => {
                let mut var = String::new();
                for ch in chars.by_ref() {
                    if ch == '%' {
                        break;
                    }
                    var.push(ch);
                }
                match std::env::var(&var) {
                    Ok(val) => result.push_str(&val),
                    Err(_) => {
                        result.push('%');
                        result.push_str(&var);
                        result.push('%');
                    }
                }
            }
            '$' => {
                if chars.peek() == Some(&'{') {
                    chars.next();
                    let mut var = String::new();
                    for ch in chars.by_ref() {
                        if ch == '}' {
                            break;
                        }
                        var.push(ch);
                    }
                    match std::env::var(&var) {
                        Ok(val) => result.push_str(&val),
                        Err(_) => {
                            result.push_str(&format!("${{{var}}}"));
                        }
                    }
                } else {
                    let mut var = String::new();
                    while let Some(&ch) = chars.peek() {
                        if ch.is_alphanumeric() || ch == '_' {
                            var.push(ch);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    match std::env::var(&var) {
                        Ok(val) => result.push_str(&val),
                        Err(_) => {
                            result.push('$');
                            result.push_str(&var);
                        }
                    }
                }
            }
            other => result.push(other),
        }
    }

    result
}

pub fn detect() -> os_info::Type {
    os_info::get().os_type()
}
