use std::path::PathBuf;

use upi_core::{Database, OsType};

fn tmp_cache_dir() -> PathBuf {
    let id = std::process::id();
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    std::env::temp_dir().join(format!("upi-test-{id}-{ts}"))
}

#[test]
fn opens_successfully() {
    let dir = tmp_cache_dir();
    let _db = Database::open_at(&dir).unwrap();
    assert!(dir.join("seed.db").exists());
    assert!(dir.join("meta.json").exists());
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn version_matches_seed() {
    let dir = tmp_cache_dir();
    let db = Database::open_at(&dir).unwrap();
    let version = db.seed_version().unwrap();
    assert_eq!(version, "1");
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn lookup_known_package_on_macos() {
    let dir = tmp_cache_dir();
    let db = Database::open_at(&dir).unwrap();

    let result = db.lookup("ffmpeg", &OsType::Macos).unwrap();
    assert!(result.is_some());
    let mapping = result.unwrap();
    assert_eq!(mapping.os_package, "ffmpeg");
    assert_eq!(mapping.source, "repology_auto");
    assert!(mapping.confidence >= 0.9);

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn lookup_known_package_on_debian() {
    let dir = tmp_cache_dir();
    let db = Database::open_at(&dir).unwrap();

    let result = db.lookup("ffmpeg", &OsType::Debian).unwrap();
    assert!(result.is_some());
    let mapping = result.unwrap();
    assert_eq!(mapping.os_package, "ffmpeg");

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn lookup_python_returns_python2_7_on_debian() {
    let dir = tmp_cache_dir();
    let db = Database::open_at(&dir).unwrap();

    let result = db.lookup("python", &OsType::Debian).unwrap();
    assert!(result.is_some());
    let mapping = result.unwrap();
    assert_eq!(mapping.os_package, "python2.7");

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn lookup_wget_returns_wget2_on_debian() {
    let dir = tmp_cache_dir();
    let db = Database::open_at(&dir).unwrap();

    let result = db.lookup("wget", &OsType::Debian).unwrap();
    assert!(result.is_some());
    let mapping = result.unwrap();
    assert_eq!(mapping.os_package, "wget2");

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn lookup_wget_returns_wget2_on_fedora() {
    let dir = tmp_cache_dir();
    let db = Database::open_at(&dir).unwrap();

    let result = db.lookup("wget", &OsType::Fedora).unwrap();
    assert!(result.is_some());
    let mapping = result.unwrap();
    assert_eq!(mapping.os_package, "wget2");

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn lookup_unknown_package_returns_none() {
    let dir = tmp_cache_dir();
    let db = Database::open_at(&dir).unwrap();

    let result = db
        .lookup("nonexistent-package-12345", &OsType::Macos)
        .unwrap();
    assert!(result.is_none());

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn alias_nodejs_resolves_to_node() {
    let dir = tmp_cache_dir();
    let db = Database::open_at(&dir).unwrap();

    let result = db.lookup("nodejs", &OsType::Macos).unwrap();
    assert!(result.is_some());
    let mapping = result.unwrap();
    assert_eq!(mapping.os_package, "node");

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn every_known_os_has_some_mappings() {
    let dir = tmp_cache_dir();
    let db = Database::open_at(&dir).unwrap();

    let oss = vec![
        OsType::Macos,
        OsType::Debian,
        OsType::Ubuntu,
        OsType::Fedora,
        OsType::Arch,
        OsType::Windows,
    ];

    let packages = vec!["ffmpeg", "vim", "curl", "nodejs", "wget", "python"];

    for os in &oss {
        for pkg in &packages {
            let result = db.lookup(pkg, os).unwrap();
            assert!(result.is_some(), "no mapping for {pkg} on {os:?}");
        }
    }

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn rehydrate_on_version_mismatch() {
    let dir = tmp_cache_dir();
    std::fs::create_dir_all(&dir).unwrap();

    let meta_path = dir.join("meta.json");
    std::fs::write(&meta_path, br#"{"version":"0","updated_at":0}"#).unwrap();

    let db_path = dir.join("seed.db");
    std::fs::write(&db_path, vec![0u8; 64]).unwrap();

    let db = Database::open_at(&dir).unwrap();
    let version = db.seed_version().unwrap();
    assert_eq!(version, "1");

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn mapping_has_provenance() {
    let dir = tmp_cache_dir();
    let db = Database::open_at(&dir).unwrap();

    let result = db.lookup("ffmpeg", &OsType::Debian).unwrap().unwrap();
    assert_eq!(result.source, "repology_auto");
    assert!(result.confidence > 0.0);

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn opens_reuses_cache() {
    let dir = tmp_cache_dir();
    let db = Database::open_at(&dir).unwrap();
    drop(db);

    let db2 = Database::open_at(&dir).unwrap();
    let result = db2.lookup("ffmpeg", &OsType::Macos).unwrap();
    assert!(result.is_some());

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn search_partial_match_finds_package() {
    let dir = tmp_cache_dir();
    let db = Database::open_at(&dir).unwrap();

    let results = db.search("pyt", &OsType::Macos).unwrap();
    assert!(
        !results.is_empty(),
        "expected at least one result for 'pyt'"
    );
    let names: Vec<&str> = results.iter().map(|m| m.os_package.as_str()).collect();
    assert!(
        names.contains(&"python@3.12"),
        "expected 'python@3.12' in search results for 'pyt'"
    );

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn search_exact_match() {
    let dir = tmp_cache_dir();
    let db = Database::open_at(&dir).unwrap();

    let results = db.search("python", &OsType::Macos).unwrap();
    assert!(!results.is_empty());
    assert!(results.iter().any(|m| m.os_package == "python@3.12"));

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn search_no_match_returns_empty() {
    let dir = tmp_cache_dir();
    let db = Database::open_at(&dir).unwrap();

    let results = db.search("xyznonexistent12345", &OsType::Macos).unwrap();
    assert!(results.is_empty());

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn search_case_insensitive() {
    let dir = tmp_cache_dir();
    let db = Database::open_at(&dir).unwrap();

    let results = db.search("PYTHON", &OsType::Macos).unwrap();
    assert!(
        !results.is_empty(),
        "expected results for 'PYTHON' (uppercase)"
    );
    assert!(results.iter().any(|m| m.os_package == "python@3.12"));

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn search_multiple_results() {
    let dir = tmp_cache_dir();
    let db = Database::open_at(&dir).unwrap();

    let results = db.search("lib", &OsType::Debian).unwrap();
    assert!(
        results.len() >= 2,
        "expected at least 2 results for 'lib', got {}",
        results.len()
    );

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn search_os_specific() {
    let dir = tmp_cache_dir();
    let db = Database::open_at(&dir).unwrap();

    let macos = db.search("python", &OsType::Macos).unwrap();
    let debian = db.search("python", &OsType::Debian).unwrap();

    let macos_names: Vec<&str> = macos.iter().map(|m| m.os_package.as_str()).collect();
    let debian_names: Vec<&str> = debian.iter().map(|m| m.os_package.as_str()).collect();

    assert!(
        macos_names.contains(&"python@3.12"),
        "macos should have 'python@3.12'"
    );
    assert!(
        debian_names.contains(&"python2.7"),
        "debian should have 'python2.7'"
    );

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn distro_variants_have_entries() {
    let dir = tmp_cache_dir();
    let db = Database::open_at(&dir).unwrap();

    let debian_result = db.lookup("ffmpeg", &OsType::Debian).unwrap().unwrap();
    let ubuntu_result = db.lookup("ffmpeg", &OsType::Ubuntu).unwrap().unwrap();
    let mint_result = db.lookup("ffmpeg", &OsType::Mint).unwrap().unwrap();
    let pop_result = db.lookup("ffmpeg", &OsType::Pop).unwrap().unwrap();

    assert_eq!(debian_result.os_package, "ffmpeg");
    assert_eq!(ubuntu_result.os_package, "ffmpeg");
    assert_eq!(mint_result.os_package, "ffmpeg");
    assert_eq!(pop_result.os_package, "ffmpeg");

    std::fs::remove_dir_all(&dir).ok();
}
