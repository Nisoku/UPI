use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{Duration, SystemTime};

use serde::Deserialize;
use upi_core::{repology_cache_dir, OsType, PackageSource, PlatformRegistry};

use crate::error::{Error, Result};

const USER_AGENT: &str = concat!(
    "upi/",
    env!("CARGO_PKG_VERSION"),
    " (+https://github.com/Nisoku/UPI)"
);
const CACHE_TTL_SECS: u64 = 86400; // 24 hours
const HTTP_TIMEOUT_SECS: u64 = 10;

fn disk_cache_path(project: &str) -> PathBuf {
    let encoded: String = url::form_urlencoded::byte_serialize(project.as_bytes()).collect();
    repology_cache_dir().join(format!("{encoded}.json"))
}

fn read_disk_cache(project: &str) -> Option<String> {
    let path = disk_cache_path(project);
    let meta = std::fs::metadata(&path).ok()?;

    let modified = meta.modified().ok()?;
    let age = SystemTime::now().duration_since(modified).ok()?;
    if age > Duration::from_secs(CACHE_TTL_SECS) {
        log::debug!(
            "repology disk cache expired for '{project}' (age: {:?})",
            age
        );
        return None;
    }

    let body = std::fs::read_to_string(&path).ok()?;
    log::debug!("repology disk cache hit: '{project}'");
    Some(body)
}

fn write_disk_cache(project: &str, body: &str) {
    let dir = repology_cache_dir();
    if std::fs::create_dir_all(&dir).is_err() {
        return;
    }
    let path = disk_cache_path(project);
    if std::fs::write(&path, body).is_err() {
        log::warn!("repology: failed to write disk cache for '{project}'");
    }
}

fn read_stale_disk_cache(project: &str) -> Option<String> {
    let path = disk_cache_path(project);
    let body = std::fs::read_to_string(&path).ok()?;
    log::debug!("repology stale disk cache fallback: '{project}'");
    Some(body)
}

/// A single entry from the Repology project API response.
#[derive(Debug, Clone, Deserialize)]
pub struct RepologyPackage {
    /// Repology repository name: `debian_13`, `homebrew`, `arch`, etc.
    pub repo: String,
    /// Source package name in the repository.
    pub srcname: Option<String>,
    /// Binary package name in the repository.
    pub binname: Option<String>,
    /// Human-visible name as displayed on repology.org.
    #[serde(rename = "visiblename")]
    pub visible_name: Option<String>,
    /// Package version string.
    pub version: Option<String>,
    /// Package status: `unique`, `newest`, `outdated`, etc.
    pub status: Option<String>,
}

/// Response from the single-project Repology API endpoint: a list of repo entries.
pub type RepologyResponse = Vec<RepologyPackage>;
/// Repology search endpoint response: project names mapped to their per-repo entries.
pub type RepologySearchResponse = HashMap<String, RepologyResponse>;

/// Caching HTTP client for the Repology API (project resolve + search).
pub struct RepologyClient {
    base_url: String,
    client: ureq::Agent,
    registry: PlatformRegistry,
    resolve_cache: Mutex<HashMap<String, RepologyResponse>>,
}

impl RepologyClient {
    /// Create a new Repology client with in-memory and on-disk caching.
    ///
    /// The registry is used to map Repology repo names to OS types.
    /// Timeout is set to `HTTP_TIMEOUT_SECS` seconds.
    pub fn new(registry: PlatformRegistry) -> Result<Self> {
        let config = ureq::Agent::config_builder()
            .timeout_global(Some(std::time::Duration::from_secs(HTTP_TIMEOUT_SECS)))
            .build();
        let client: ureq::Agent = config.into();
        Ok(Self {
            base_url: "https://repology.org/api/v1".into(),
            client,
            registry,
            resolve_cache: Mutex::new(HashMap::new()),
        })
    }

    /// Resolve a project name to an OS-specific package name via Repology.
    ///
    /// Checks in-memory cache first, then on-disk cache under 24h old, then network, then stale on-disk fallback on error.
    pub fn resolve(&self, project: &str, os_type: &OsType) -> Result<Option<String>> {
        let project_key = project.to_string();

        // In-memory cache
        {
            let cache = self
                .resolve_cache
                .lock()
                .expect("resolve_cache mutex poisoned");
            if let Some(data) = cache.get(&project_key) {
                log::debug!("repology mem cache hit: '{project}'");
                let r = find_package_for_os(data, os_type, &self.registry);
                log::debug!("repology mem cache: '{project}' on {os_type:?} = {r:?}");
                return Ok(r);
            }
        }

        // On-disk cache (fresh, <24h)
        if let Some(body) = read_disk_cache(project) {
            if let Ok(data) = serde_json::from_str::<RepologyResponse>(&body) {
                self.resolve_cache
                    .lock()
                    .expect("resolve_cache mutex poisoned")
                    .insert(project_key.clone(), data.clone());
                let r = find_package_for_os(&data, os_type, &self.registry);
                log::debug!("repology disk cache: '{project}' on {os_type:?} = {r:?}");
                return Ok(r);
            }
        }

        // Network fetch
        let encoded: String = url::form_urlencoded::byte_serialize(project.as_bytes()).collect();
        let url = format!("{}/project/{}", self.base_url, encoded);

        log::debug!("repology: GET {url}");

        let result = match self
            .client
            .get(&url)
            .header("User-Agent", USER_AGENT)
            .call()
        {
            Ok(mut resp) => {
                let body = resp
                    .body_mut()
                    .read_to_string()
                    .map_err(|e| Error::Http(format!("body read: {e}")))?;

                write_disk_cache(project, &body);

                let data: RepologyResponse =
                    serde_json::from_str(&body).map_err(|e| Error::Parse(format!("{e}")))?;

                log::debug!("repology: {} entries for '{project}'", data.len());
                self.resolve_cache
                    .lock()
                    .expect("resolve_cache mutex poisoned")
                    .insert(project_key, data.clone());
                find_package_for_os(&data, os_type, &self.registry)
            }
            Err(ureq::Error::StatusCode(404)) => {
                log::debug!("repology: 404 for '{project}'");
                write_disk_cache(project, "[]");
                self.resolve_cache
                    .lock()
                    .expect("resolve_cache mutex poisoned")
                    .insert(project_key, Vec::new());
                return Ok(None);
            }
            Err(e) => {
                // Network failure -> stale disk cache fallback
                log::warn!("repology: HTTP error for '{project}': {e}");
                if let Some(body) = read_stale_disk_cache(project) {
                    log::info!("repology: using stale cached data for '{project}'");
                    if let Ok(data) = serde_json::from_str::<RepologyResponse>(&body) {
                        self.resolve_cache
                            .lock()
                            .unwrap()
                            .insert(project_key, data.clone());
                        let r = find_package_for_os(&data, os_type, &self.registry);
                        return Ok(r);
                    }
                }
                return Err(Error::Http(e.to_string()));
            }
        };

        log::debug!("repology: result for '{project}' on {os_type:?} = {result:?}");
        Ok(result)
    }

    /// Search Repology for projects matching `query`, filtered to a specific OS.
    pub fn search(&self, query: &str, os_type: &OsType) -> Result<Vec<String>> {
        let encoded: String = url::form_urlencoded::byte_serialize(query.as_bytes()).collect();
        let url = format!("{}/projects/?search={}", self.base_url, encoded);

        log::debug!("repology search: GET {url}");

        let mut response = match self
            .client
            .get(&url)
            .header("User-Agent", USER_AGENT)
            .call()
        {
            Ok(resp) => resp,
            Err(e) => return Err(Error::Http(e.to_string())),
        };

        let body = response
            .body_mut()
            .read_to_string()
            .map_err(|e| Error::Http(format!("body read: {e}")))?;

        let data: RepologySearchResponse =
            serde_json::from_str(&body).map_err(|e| Error::Parse(format!("{e}")))?;

        log::debug!("repology search: {} projects for '{query}'", data.len());

        let mut results = Vec::new();
        for (project, packages) in &data {
            if let Some(os_name) = find_package_for_os(packages, os_type, &self.registry) {
                log::debug!("repology search: '{query}' -> project={project}, os_name={os_name}");
                results.push(os_name);
            }
        }
        results.sort();
        log::debug!(
            "repology search: {} results for '{query}' on {os_type:?}",
            results.len()
        );
        Ok(results)
    }
}

impl PackageSource for RepologyClient {
    fn resolve_package(
        &self,
        package: &str,
        os_type: &OsType,
    ) -> std::result::Result<Option<String>, upi_core::Error> {
        self.resolve(package, os_type)
            .map_err(|e| upi_core::Error::Network(e.to_string()))
    }

    fn search_packages(
        &self,
        query: &str,
        os_type: &OsType,
    ) -> std::result::Result<Option<Vec<String>>, upi_core::Error> {
        self.search(query, os_type)
            .map(Some)
            .map_err(|e| upi_core::Error::Network(e.to_string()))
    }
}

/// Find the first package in a Repology response matching the given OS type.
///
/// Returns the `binname`, falling back to `srcname`, then `visiblename`.
pub fn find_package_for_os(
    data: &RepologyResponse,
    os_type: &OsType,
    registry: &PlatformRegistry,
) -> Option<String> {
    for pkg in data {
        if let Some(mapped_os) = registry.repo_to_os(&pkg.repo) {
            if mapped_os == os_type {
                let name = pkg
                    .binname
                    .as_deref()
                    .or(pkg.srcname.as_deref())
                    .or(pkg.visible_name.as_deref())?;
                return Some(name.to_string());
            }
        }
    }
    None
}
