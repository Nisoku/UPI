use std::collections::HashMap;

use serde::Deserialize;
use upi_core::{OsType, PackageSource, PlatformRegistry};

use crate::error::{Error, Result};

const USER_AGENT: &str = concat!(
    "upi/",
    env!("CARGO_PKG_VERSION"),
    " (+https://github.com/Nisoku/UPI)"
);

#[derive(Debug, Deserialize)]
pub struct RepologyPackage {
    pub repo: String,
    pub srcname: Option<String>,
    pub binname: Option<String>,
    #[serde(rename = "visiblename")]
    pub visible_name: Option<String>,
    pub version: Option<String>,
    pub status: Option<String>,
}

pub type RepologyResponse = Vec<RepologyPackage>;
pub type RepologySearchResponse = HashMap<String, RepologyResponse>;

pub struct RepologyClient {
    base_url: String,
    client: ureq::Agent,
    registry: PlatformRegistry,
}

impl RepologyClient {
    pub fn new(registry: PlatformRegistry) -> Result<Self> {
        let config = ureq::Agent::config_builder()
            .timeout_global(Some(std::time::Duration::from_secs(10)))
            .build();
        let client: ureq::Agent = config.into();
        Ok(Self {
            base_url: "https://repology.org/api/v1".into(),
            client,
            registry,
        })
    }

    pub fn resolve(&self, project: &str, os_type: &OsType) -> Result<Option<String>> {
        let encoded: String = url::form_urlencoded::byte_serialize(project.as_bytes()).collect();
        let url = format!("{}/project/{}", self.base_url, encoded);

        log::debug!("repology: GET {url}");

        let mut response = match self
            .client
            .get(&url)
            .header("User-Agent", USER_AGENT)
            .call()
        {
            Ok(resp) => resp,
            Err(ureq::Error::StatusCode(404)) => {
                log::debug!("repology: 404 for '{project}'");
                return Ok(None);
            }
            Err(e) => return Err(Error::Http(e.to_string())),
        };

        let body = response
            .body_mut()
            .read_to_string()
            .map_err(|e| Error::Http(format!("body read: {e}")))?;

        let data: RepologyResponse =
            serde_json::from_str(&body).map_err(|e| Error::Parse(format!("{e}")))?;

        log::debug!("repology: {} entries for '{project}'", data.len());
        let result = find_package_for_os(&data, os_type, &self.registry);
        log::debug!("repology: result for '{project}' on {os_type:?} = {result:?}");
        Ok(result)
    }

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
