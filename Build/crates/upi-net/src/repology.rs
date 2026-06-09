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

        let mut response = match self
            .client
            .get(&url)
            .header("User-Agent", USER_AGENT)
            .call()
        {
            Ok(resp) => resp,
            Err(ureq::Error::StatusCode(404)) => return Ok(None),
            Err(e) => return Err(Error::Http(e.to_string())),
        };

        let body = response
            .body_mut()
            .read_to_string()
            .map_err(|e| Error::Http(format!("body read: {e}")))?;

        let data: RepologyResponse =
            serde_json::from_str(&body).map_err(|e| Error::Parse(format!("{e}")))?;

        Ok(find_package_for_os(&data, os_type, &self.registry))
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
