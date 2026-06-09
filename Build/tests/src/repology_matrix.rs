use upi_core::{OsType, PlatformRegistry};
use upi_net::{find_package_for_os, RepologyPackage, RepologySearchResponse};

fn registry() -> PlatformRegistry {
    PlatformRegistry::load().unwrap()
}

fn make_pkg(
    repo: &str,
    binname: Option<&str>,
    srcname: Option<&str>,
    visiblename: Option<&str>,
) -> RepologyPackage {
    RepologyPackage {
        repo: repo.to_string(),
        binname: binname.map(String::from),
        srcname: srcname.map(String::from),
        visible_name: visiblename.map(String::from),
        version: None,
        status: None,
    }
}

fn make_pkg_simple(repo: &str, binname: &str) -> RepologyPackage {
    make_pkg(repo, Some(binname), None, None)
}

#[test]
fn finds_package_for_current_os() {
    let data = vec![
        make_pkg_simple("debian_13", "ffmpeg-debian"),
        make_pkg_simple("arch", "ffmpeg-arch"),
    ];
    let result = find_package_for_os(&data, &OsType::Debian, &registry());
    assert_eq!(result, Some("ffmpeg-debian".into()));
}

#[test]
fn prefers_binname_over_visiblename() {
    let data = vec![make_pkg(
        "homebrew",
        Some("ffmpeg-binary"),
        None,
        Some("ffmpeg-visible"),
    )];
    let result = find_package_for_os(&data, &OsType::Macos, &registry());
    assert_eq!(result, Some("ffmpeg-binary".into()));
}

#[test]
fn falls_back_to_srcname_when_no_binname() {
    let data = vec![make_pkg(
        "homebrew",
        None,
        Some("ffmpeg-source"),
        Some("ffmpeg-visible"),
    )];
    let result = find_package_for_os(&data, &OsType::Macos, &registry());
    assert_eq!(result, Some("ffmpeg-source".into()));
}

#[test]
fn falls_back_to_visiblename_when_no_binname_or_srcname() {
    let data = vec![make_pkg("homebrew", None, None, Some("ffmpeg-visible"))];
    let result = find_package_for_os(&data, &OsType::Macos, &registry());
    assert_eq!(result, Some("ffmpeg-visible".into()));
}

#[test]
fn skips_repos_not_matching_os() {
    let data = vec![
        make_pkg_simple("debian_13", "ffmpeg-deb"),
        make_pkg_simple("arch", "ffmpeg-arch"),
    ];
    let result = find_package_for_os(&data, &OsType::Macos, &registry());
    assert_eq!(result, None);
}

#[test]
fn returns_none_when_no_repos_match() {
    let data = vec![make_pkg_simple("freebsd", "ffmpeg")];
    let result = find_package_for_os(&data, &OsType::Debian, &registry());
    assert_eq!(result, None);
}

#[test]
fn returns_none_on_empty_data() {
    let data = vec![];
    let result = find_package_for_os(&data, &OsType::Debian, &registry());
    assert_eq!(result, None);
}

#[test]
fn search_response_deserializes_from_json() {
    let json = r#"{
        "python": [
            {"repo": "homebrew", "binname": "python@3.14", "version": "3.14.0"},
            {"repo": "debian_13", "binname": "python3", "version": "3.13.0"}
        ],
        "python2": [
            {"repo": "homebrew", "binname": "python@2.7", "version": "2.7.18"}
        ]
    }"#;

    let data: RepologySearchResponse = serde_json::from_str(json).unwrap();
    assert_eq!(data.len(), 2);
    assert!(data.contains_key("python"));
    assert!(data.contains_key("python2"));

    let python_packages = &data["python"];
    assert_eq!(python_packages.len(), 2);

    let homebrew = &python_packages[0];
    assert_eq!(homebrew.repo, "homebrew");
    assert_eq!(homebrew.binname.as_deref(), Some("python@3.14"));

    let debian = &python_packages[1];
    assert_eq!(debian.repo, "debian_13");
    assert_eq!(debian.binname.as_deref(), Some("python3"));
}

#[test]
fn search_response_filters_by_os() {
    let json = r#"{
        "python": [
            {"repo": "homebrew", "binname": "python@3.14", "version": "3.14.0"},
            {"repo": "arch", "binname": "python", "version": "3.13.0"}
        ],
        "python2": [
            {"repo": "homebrew", "binname": "python@2.7", "version": "2.7.18"}
        ]
    }"#;

    let data: RepologySearchResponse = serde_json::from_str(json).unwrap();

    let mut results = Vec::new();
    for (_project, packages) in &data {
        if let Some(os_name) = find_package_for_os(packages, &OsType::Macos, &registry()) {
            results.push(os_name);
        }
    }
    results.sort();
    assert_eq!(results, vec!["python@2.7", "python@3.14"]);
}
