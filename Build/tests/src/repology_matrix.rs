use upi_core::{OsType, PlatformRegistry};
use upi_net::{find_package_for_os, RepologyPackage};

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
