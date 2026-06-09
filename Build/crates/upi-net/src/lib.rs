//! Network-backed package resolution: Repology client, types, and error handling.

pub mod error;
pub mod repology;

pub use repology::{
    find_package_for_os, RepologyClient, RepologyPackage, RepologyResponse, RepologySearchResponse,
};
