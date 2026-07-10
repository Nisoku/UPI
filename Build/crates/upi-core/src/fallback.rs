use crate::error::Result;
use crate::os::PlatformConfig;
use crate::resolver::PackageSource;
use strsim::{jaro_winkler, normalized_levenshtein};

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

        let (shell, flag) = if cfg!(windows) {
            ("cmd", "/C")
        } else {
            ("sh", "-c")
        };

        let output = std::process::Command::new(shell)
            .arg(flag)
            .arg(&cmd_str)
            .output()
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    crate::error::Error::ProgramNotFound(shell.into())
                } else {
                    crate::error::Error::from(e)
                }
            })?;

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
    let mut candidates: Vec<String> = Vec::new();

    for line in output.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some(name) = extract_name(trimmed, &query_lower) {
            candidates.push(name);
        }
    }

    best_candidate(&query_lower, candidates)
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

fn best_candidate(query: &str, candidates: Vec<String>) -> Option<String> {
    if candidates.is_empty() {
        return None;
    }

    let mut best: Option<(f64, String)> = None;
    for candidate in candidates {
        let score = score_candidate(query, &candidate);
        if score <= 0.0 {
            continue;
        }
        if let Some((best_score, _)) = &best {
            if score > *best_score {
                best = Some((score, candidate));
            }
        } else {
            best = Some((score, candidate));
        }
    }

    best.and_then(|(score, candidate)| {
        if score >= 220.0 {
            Some(candidate)
        } else {
            None
        }
    })
}

fn score_candidate(query: &str, candidate: &str) -> f64 {
    let c = candidate.to_lowercase();
    if c == query {
        return 1000.0;
    }

    // Very short queries like "rg" are too ambiguous for fuzzy matching.
    if query.len() <= 3 {
        return 0.0;
    }

    let mut score = 0.0;
    if c.starts_with(query) {
        score += 300.0;
    }
    if c.contains(query) {
        score += 120.0;
    }
    if c.split(|ch: char| !ch.is_ascii_alphanumeric())
        .any(|part| part == query)
    {
        score += 140.0;
    }

    score += jaro_winkler(&c, query) * 200.0;
    score += normalized_levenshtein(&c, query) * 200.0;

    let len_gap = (c.len() as i64 - query.len() as i64).abs() as f64;
    let separator_penalty = c
        .chars()
        .filter(|ch| *ch == '-' || *ch == '_' || *ch == '.')
        .count() as f64
        * 90.0;

    score - (len_gap * 18.0) - separator_penalty
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
    let name = if let Some(idx) = raw.find('/') {
        let before = &raw[..idx];
        let after = &raw[idx + 1..];
        if score_side(query, before) >= score_side(query, after) {
            before
        } else {
            after
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

fn score_side(query: &str, side: &str) -> f64 {
    let s = side.to_lowercase();
    if s == query {
        return 1000.0;
    }

    let mut score = 0.0;
    if s.starts_with(query) {
        score += 200.0;
    }
    if s.contains(query) {
        score += 120.0;
    }
    score += jaro_winkler(&s, query) * 100.0;
    score += normalized_levenshtein(&s, query) * 100.0;
    score
}
