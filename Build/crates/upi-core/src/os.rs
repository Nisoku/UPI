use crate::error::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Os {
    MacOs,
    Debian,
    Fedora,
    Arch,
    Windows,
}

impl Os {
    pub fn name(&self) -> &'static str {
        match self {
            Os::MacOs => "macos",
            Os::Debian => "debian",
            Os::Fedora => "fedora",
            Os::Arch => "arch",
            Os::Windows => "windows",
        }
    }
}

pub fn detect() -> Result<Os> {
    Err(crate::error::Error::UnsupportedOs(
        "detection not implemented".into(),
    ))
}

#[derive(Debug, Clone)]
pub struct OsConfig {
    pub name: String,
}

pub fn load_config(_os: &Os) -> Result<OsConfig> {
    Err(crate::error::Error::PlatformConfig(
        "YAML loading not implemented".into(),
    ))
}
