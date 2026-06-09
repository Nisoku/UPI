pub mod error;
pub mod repology;

pub use repology::{
    find_package_for_os, RepologyClient, RepologyPackage, RepologyResponse, RepologySearchResponse,
};
