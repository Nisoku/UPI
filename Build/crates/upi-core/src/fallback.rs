use crate::error::Result;
use crate::os::PlatformConfig;
use crate::resolver::PackageSource;

/// Searches for packages using the OS package manager's own search command.
///
/// Used as a last-resort source when Repology and the local database have no match.
pub struct FallbackSearcher {
    search_template: String,
}

impl FallbackSearcher {
    /// Create a searcher from a platform config's `search` template, if defined.
    pub fn from_config(config: &PlatformConfig) -> Option<Self> {
        let template = config.search.as_ref()?;
        Some(Self {
            search_template: template.clone(),
        })
    }

    /// Run the search command and parse its output for a matching package name.
    pub fn search(&self, query: &str) -> Result<Option<String>> {
        let cmd_str = self.search_template.replace("{query}", query);
        log::debug!("fallback: running {cmd_str}");

        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg(&cmd_str)
            .output()?;

        if !output.status.success() {
            log::debug!("fallback: non-zero exit ({:?})", output.status.code());
            return Ok(None);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let result = parse_search_output(&stdout, query);
        log::debug!("fallback: result={result:?}");
        Ok(result)
    }
}

impl PackageSource for FallbackSearcher {
    fn resolve_package(&self, package: &str, _os_type: &os_info::Type) -> Result<Option<String>> {
        self.search(package)
    }
}

/// Parse the stdout of a package manager search command and return the first relevant package name.
///
/// Scans lines for the query (case-insensitive), skips headers and separators,
/// extracts the package name from formats like `name --> description` or `name  version`.
pub fn parse_search_output(output: &str, query: &str) -> Option<String> {
    let query_lower = query.to_lowercase();

    for line in output.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let lower = trimmed.to_lowercase();
        if !lower.contains(&query_lower) {
            continue;
        }
        if let Some(name) = extract_name(trimmed, &query_lower) {
            return Some(name);
        }
    }

    None
}

fn extract_name(line: &str, query: &str) -> Option<String> {
    let trimmed = line.trim();
    if should_skip(trimmed) {
        return None;
    }

    let raw = if let Some(idx) = trimmed.find("-->") {
        trimmed[idx + 3..].split_whitespace().next().unwrap_or("")
    } else if let Some(idx) = trimmed.find("  ") {
        trimmed[..idx].trim()
    } else {
        trimmed.split_whitespace().next().unwrap_or(trimmed)
    };
    if raw.is_empty() {
        return None;
    }

    let name = clean_package_name(raw, query)?;
    if name.is_empty() {
        None
    } else {
        Some(name.to_string())
    }
}

fn should_skip(line: &str) -> bool {
    line.starts_with("==")
        || line.starts_with("Sorting")
        || line.starts_with("Full Text")
        || line.starts_with("Name") && line.contains("Id") && line.contains("Version")
        || line.starts_with("---")
        || line.contains("suggests")
        || line.contains("Results from")
}

fn clean_package_name<'a>(raw: &'a str, query: &str) -> Option<&'a str> {
    let query_lower = query.to_lowercase();

    let name = if let Some(idx) = raw.find('/') {
        let before = &raw[..idx];
        let after = &raw[idx + 1..];
        if before.to_lowercase().contains(&query_lower) {
            before
        } else if after.to_lowercase().contains(&query_lower) {
            after
        } else {
            raw
        }
    } else {
        raw
    };

    let name = name.split('.').next().unwrap_or(name);
    let name = name.split('@').next().unwrap_or(name);
    let name = name.split(':').next().unwrap_or(name);
    if name.is_empty() {
        None
    } else {
        Some(name)
    }
}
