use std::sync::OnceLock;

use include_dir::{include_dir, Dir};
use noyalib;
use serde::Deserialize;

use crate::error::{Error, Result};

static PLATFORM_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/platform");

/// YAML-defined configuration for a package manager on one or more OS targets.
#[derive(Debug, Clone, Deserialize)]
pub struct PlatformConfig {
    /// OS types this config applies to: `Debian`, `Macos`, etc.
    pub targets: Vec<os_info::Type>,
    /// Human-readable name of the package manager: `apt`, `brew`, etc.
    pub manager: String,
    /// Whether install commands need `sudo`.
    pub sudo: bool,
    /// Install command template with `{package}` placeholder.
    pub install: String,
    /// Optional search command template with `{query}` placeholder.
    pub search: Option<String>,
    /// Optional shell glob or regex to discover installed packages.
    pub provides: Option<String>,
    /// Parse strategy override field, unused because the generic parser handles all output formats.
    pub provides_parse: Option<String>,
    /// Paths to manager binaries (supports `%VAR%`, `$VAR`, `${VAR}` expansion).
    pub binary_paths: Vec<String>,
}

/// Registry of all platform configurations, loaded from embedded YAML files.
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
    /// Load the global registry, caching it for the process lifetime.
    pub fn global() -> &'static Self {
        static REGISTRY: OnceLock<PlatformRegistry> = OnceLock::new();
        REGISTRY.get_or_init(|| Self::load().expect("failed to load platform registry"))
    }

    /// Parse all YAML files from the embedded `Build/platform/` directory.
    ///
    /// Skips empty files; returns `Err` on malformed YAML.
    pub fn load() -> Result<Self> {
        let mut configs = Vec::new();

        for file in PLATFORM_DIR.files() {
            let name = file
                .path()
                .file_stem()
                .and_then(std::ffi::OsStr::to_str)
                .expect("platform config filename must have a stem and be valid UTF-8");
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

    /// Find the platform config for a given OS type.
    ///
    /// Returns `None` for unsupported OS types that have no matching YAML definition.
    pub fn for_type(&self, os_type: &os_info::Type) -> Option<&PlatformConfig> {
        self.configs.iter().find(|c| c.targets.contains(os_type))
    }

    /// All platform configs loaded from YAML.
    pub fn all(&self) -> &[PlatformConfig] {
        &self.configs
    }

    /// Map a Repology repository name to an OS type.
    ///
    /// Normalizes the repo name (strips version suffixes, `linux`/`libre` prefixes)
    /// and matches against YAML-defined targets and manager names.
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

    /// Parse a human-readable OS name string into an OS type.
    ///
    /// Accepts partial name matching. Used by `--os` CLI flag.
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

    /// All unique OS types referenced across all platform configs.
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

/// Detect the current operating system using `os_info`.
pub fn detect() -> os_info::Type {
    os_info::get().os_type()
}
