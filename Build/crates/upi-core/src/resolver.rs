use std::collections::HashSet;
use strsim::{jaro_winkler, normalized_levenshtein};

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
/// Identity is **disabled by default**, use `allow_identity(true)` or pass `--allow-identity` to enable.
pub struct Resolver {
    registry: PlatformRegistry,
    db: Database,
    sources: Vec<Box<dyn PackageSource>>,
    allow_identity: bool,
}

const MAX_SUGGESTIONS: usize = 3;

/// Minimum suggestion score for a candidate to be presented as a "did you mean" option.
const SUGGESTION_THRESHOLD: f64 = 220.0;

/// Short queries (<= this length) require exact or alias match only and no fuzzy fallback.
const SHORT_QUERY_LIMIT: usize = 3;

impl Resolver {
    /// Create a resolver with default settings and an empty source list.
    ///
    /// Uses the global `PlatformRegistry` and the default database path.
    /// Identity is **disabled** by default, call `.allow_identity(true)` to enable.
    pub fn new() -> Result<Self> {
        let registry = PlatformRegistry::global().clone();
        let db = Database::open()?;
        Ok(Self {
            registry,
            db,
            sources: Vec::new(),
            allow_identity: false,
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
            allow_identity: false,
        })
    }

    /// Enable or disable identity fallback.
    ///
    /// When enabled, if no other source matches, the raw query is used as the package name.
    /// When disabled (default), unresolved queries produce an error with suggestions.
    pub fn allow_identity(mut self, allow: bool) -> Self {
        self.allow_identity = allow;
        self
    }

    /// Resolve a package name to an install command for the detected OS.
    pub fn resolve(&self, package: &str) -> Result<Command> {
        let os_type = detect();
        log::debug!("detected OS: {os_type:?}");
        self.resolve_for_os(package, &os_type)
    }

    /// Resolve a package name to an install command for a specific OS.
    pub fn resolve_for_os(&self, package: &str, os_type: &os_info::Type) -> Result<Command> {
        let commands =
            self.resolve_commands_for_os_with_options(package, os_type, self.allow_identity)?;
        commands
            .into_iter()
            .next()
            .ok_or_else(|| Error::Resolve(format!("no install command available for {os_type:?}")))
    }

    /// Resolve a package name to all matching install commands for a specific OS.
    ///
    /// Available managers are ordered first so callers can try them before falling back.
    pub fn resolve_commands_for_os(
        &self,
        package: &str,
        os_type: &os_info::Type,
    ) -> Result<Vec<Command>> {
        self.resolve_commands_for_os_with_options(package, os_type, self.allow_identity)
    }

    /// Resolve a package name with an explicit identity fallback policy.
    ///
    /// When `allow_identity` is false and no confident match is found, returns an error with suggestions.
    pub fn resolve_commands_for_os_with_options(
        &self,
        package: &str,
        os_type: &os_info::Type,
        allow_identity: bool,
    ) -> Result<Vec<Command>> {
        let config = self
            .registry
            .for_type(os_type)
            .ok_or_else(|| Error::UnsupportedOs(format!("{os_type:?}")))?;

        log::debug!("using config: {} (sudo={})", config.manager, config.sudo);
        let mut candidates = Vec::new();
        let (os_package, _source) =
            self.lookup_package_name(package, os_type, allow_identity, &mut candidates)?;
        log::info!("resolved '{}' -> '{}'", package, os_package);

        let mut configs = self.registry.configs_for_type(os_type);
        configs.sort_by_key(|config| !config.is_available());

        Ok(configs
            .into_iter()
            .map(|config| Command::from_config(config, &os_package))
            .collect())
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

        let mut candidates = Vec::new();
        let (os_package, source) =
            self.lookup_package_name(package, os_type, self.allow_identity, &mut candidates)?;
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

        let mut configs = self.registry.configs_for_type(os_type);
        configs.sort_by_key(|config| !config.is_available());
        if query.chars().count() > SHORT_QUERY_LIMIT {
            for config in configs {
                if let Some(searcher) = FallbackSearcher::from_config(config) {
                    if let Some(name) = searcher.search(query)? {
                        if seen.insert(name.clone()) {
                            candidates.push(ResolveCandidate {
                                name,
                                source: format!("fallback search ({})", config.manager),
                            });
                        }
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
        allow_identity: bool,
        candidates: &mut Vec<ResolveCandidate>,
    ) -> Result<(String, String)> {
        if let Some(alias) = resolve_alias(package) {
            log::debug!("alias: '{}' -> '{}'", package, alias);
            candidates.push(ResolveCandidate {
                name: alias.to_string(),
                source: "alias".into(),
            });
            return Ok((alias.to_string(), "alias".into()));
        }

        for (i, source) in self.sources.iter().enumerate() {
            log::debug!("source[{i}]: trying to resolve '{package}'");
            if let Some(name) = source.resolve_package(package, os_type)? {
                log::debug!("source[{i}]: found '{name}'");
                candidates.push(ResolveCandidate {
                    name: name.clone(),
                    source: "repology".into(),
                });
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
            candidates.push(ResolveCandidate {
                name: mapping.os_package.clone(),
                source: source.clone(),
            });
            return Ok((mapping.os_package, source));
        }

        // Short queries (<=3 chars) require exact or alias match only and no fuzzy fallback.
        if package.chars().count() <= SHORT_QUERY_LIMIT {
            log::debug!(
                "short query '{package}' (chars={}): skipping fuzzy fallback",
                package.chars().count()
            );
        } else {
            log::debug!("fallback: searching for '{package}'");
            let mut configs = self.registry.configs_for_type(os_type);
            configs.sort_by_key(|config| !config.is_available());
            for config in configs {
                if let Some(searcher) = FallbackSearcher::from_config(config) {
                    if let Some(name) = searcher.search(package)? {
                        log::debug!("fallback: found '{name}'");
                        candidates.push(ResolveCandidate {
                            name: name.clone(),
                            source: format!("fallback search ({})", config.manager),
                        });
                        return Ok((name, format!("fallback search ({})", config.manager)));
                    }
                }
            }
        }

        if allow_identity {
            log::debug!("identity: using '{package}' as-is");
            candidates.push(ResolveCandidate {
                name: package.to_string(),
                source: "identity".into(),
            });
            return Ok((package.to_string(), "identity".into()));
        }

        let suggestions = self.best_suggestions(package, candidates);
        if suggestions.is_empty() {
            Err(Error::Resolve(format!(
                "no confident match for '{package}'. Re-run with --allow-identity to install the exact input"
            )))
        } else {
            let suggested = suggestions.join(", ");
            Err(Error::Resolve(format!(
                "no confident match for '{package}'. Did you mean: {suggested}? Re-run with --allow-identity to install the exact input"
            )))
        }
    }

    fn best_suggestions(&self, query: &str, candidates: &[ResolveCandidate]) -> Vec<String> {
        let mut ranked: Vec<(f64, String)> = candidates
            .iter()
            .filter(|c| c.source != "identity")
            .map(|c| (suggestion_score(query, &c.name), c.name.clone()))
            .collect();

        // When search results are sparse, also score against well-known package names
        // so typos like "ripgrepp" still get "ripgrep" as a suggestion.
        if ranked.iter().all(|(s, _)| *s < SUGGESTION_THRESHOLD) {
            for &name in KNOWN_PACKAGES {
                let score = suggestion_score(query, name);
                if score >= SUGGESTION_THRESHOLD {
                    ranked.push((score, name.to_string()));
                }
            }
        }

        ranked.sort_by(|a, b| b.0.total_cmp(&a.0));
        ranked.retain(|(score, _)| *score >= SUGGESTION_THRESHOLD);

        let mut seen = HashSet::new();
        let mut out = Vec::new();
        for (_score, name) in ranked {
            if seen.insert(name.clone()) {
                out.push(name);
            }
            if out.len() >= MAX_SUGGESTIONS {
                break;
            }
        }

        out
    }
}

/// Well-known package names used for suggestion scoring when search results are sparse.
const KNOWN_PACKAGES: &[&str] = &[
    "ripgrep",
    "python",
    "python3",
    "nodejs",
    "nodejs-lts",
    "neovim",
    "vim",
    "ffmpeg",
    "yt-dlp",
    "git",
    "curl",
    "wget",
    "htop",
    "bat",
    "fd",
    "fzf",
    "jq",
    "yq",
    "exa",
    "lsd",
    "delta",
    "ag",
    "ghostscript",
    "tree",
    "tmux",
    "nano",
    "gcc",
    "make",
    "cmake",
    "rust",
    "go",
    "deno",
    "bun",
    "docker",
    "podman",
    "gh",
    "zig",
    "ruby",
    "perl",
    "lua",
    "php",
    "swift",
    "rustup",
    "cargo",
    "gradle",
    "maven",
    "ansible",
    "terraform",
    "kubectl",
    "helm",
    "lazygit",
    "lazydocker",
    "bottom",
    "tokei",
    "hyperfine",
    "bandwhich",
    "difftastic",
    "grex",
    "sd",
    "procs",
    "duf",
    "dust",
    "navi",
    "zoxide",
    "starship",
    "alacritty",
    "kitty",
    "wezterm",
    "ranger",
    "nnn",
    "lf",
    "broot",
    "fish",
    "zsh",
    "bash",
    "nushell",
    "elvish",
];

fn resolve_alias(package: &str) -> Option<&'static str> {
    match package.to_ascii_lowercase().as_str() {
        "rg" => Some("ripgrep"),
        "py" => Some("python"),
        "python3" => Some("python"),
        "node" => Some("nodejs"),
        "nodejs-lts" => Some("nodejs-lts"),
        "yt" => Some("yt-dlp"),
        "vim" => Some("vim"),
        "nvim" => Some("neovim"),
        "gs" => Some("ghostscript"),
        "ff" => Some("ffmpeg"),
        "grep" => Some("ripgrep"),
        "ag" => Some("the_silver_searcher"),
        "fd" => Some("fd-find"),
        "bat" => Some("bat"),
        "exa" => Some("exa"),
        "lsd" => Some("lsd"),
        "delta" => Some("delta"),
        "jq" => Some("jq"),
        "yq" => Some("yq"),
        "fzf" => Some("fzf"),
        "ripgrep" => Some("ripgrep"),
        "neovim" => Some("neovim"),
        "ffmpeg" => Some("ffmpeg"),
        "yt-dlp" => Some("yt-dlp"),
        _ => None,
    }
}

fn suggestion_score(query: &str, candidate: &str) -> f64 {
    let q = query.to_ascii_lowercase();
    let c = candidate.to_ascii_lowercase();
    if q == c {
        return 1000.0;
    }

    let mut score = 0.0;
    if c.starts_with(&q) {
        score += 220.0;
    }
    if c.contains(&q) {
        score += 90.0;
    }
    score += jaro_winkler(&q, &c) * 220.0;
    score += normalized_levenshtein(&q, &c) * 220.0;
    let len_gap = (q.len() as i64 - c.len() as i64).abs() as f64;
    score - (len_gap * 8.0)
}
