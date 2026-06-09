use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use rusqlite::Connection;
use serde::Deserialize;
use upi_core::{OsType, PlatformRegistry};

const USER_AGENT: &str = concat!(
    "upi-db-update/",
    env!("CARGO_PKG_VERSION"),
    " (+https://github.com/Nisoku/UPI)"
);
const REPOLOGY_BASE: &str = "https://repology.org/api/v1";
const RATE_LIMIT_MS: u64 = 1100;
const PAGE_SIZE: usize = 200;

#[derive(Debug, Deserialize)]
struct RepologyPackage {
    repo: String,
    srcname: Option<String>,
    binname: Option<String>,
    #[serde(rename = "visiblename")]
    visible_name: Option<String>,
}

type RepologyPage = HashMap<String, Vec<RepologyPackage>>;

#[derive(Debug, Deserialize)]
struct WingetIndexItem {
    #[serde(rename = "PackageId")]
    package_id: String,
    #[serde(rename = "Name")]
    name: String,
}

const WINGET_INDEX_URL: &str =
    "https://raw.githubusercontent.com/svrooij/winget-pkgs-index/main/index.json";

fn is_runtime_or_library(name: &str) -> bool {
    let lower = name.to_lowercase();
    if lower.contains(':') {
        return true;
    }
    let prefixes = [
        "py-", "python-", "p5-", "php-", "perl-", "rubygem-", "gem-", "node-", "lib",
    ];
    let suffixes = ["-el", "-devel", "-dev", "-dbg", "-docs", "-common"];
    for p in &prefixes {
        if lower.starts_with(p) {
            return true;
        }
    }
    for s in &suffixes {
        if lower.ends_with(s) {
            return true;
        }
    }
    false
}

fn pillar_defs(registry: &PlatformRegistry) -> HashMap<OsType, Vec<OsType>> {
    let wanted: HashSet<OsType> = [
        OsType::Debian,
        OsType::Fedora,
        OsType::Arch,
        OsType::Macos,
        OsType::Windows,
    ]
    .into_iter()
    .collect();
    let mut map = HashMap::new();
    for config in registry.all() {
        if let Some(first) = config.targets.first() {
            if wanted.contains(first) {
                map.insert(*first, config.targets.clone());
            }
        }
    }
    map
}

fn fetch_page(client: &ureq::Agent, cursor: &str) -> Result<RepologyPage, String> {
    let cursor_enc: String = url::form_urlencoded::byte_serialize(cursor.as_bytes()).collect();
    let url = if cursor.is_empty() {
        format!("{}/projects/?families=3-", REPOLOGY_BASE)
    } else {
        format!("{}/projects/{}/?families=3-", REPOLOGY_BASE, cursor_enc)
    };

    log::info!("fetching: {url}");
    let mut response = client
        .get(&url)
        .header("User-Agent", USER_AGENT)
        .call()
        .map_err(|e| format!("HTTP request failed: {e}"))?;

    let body = response
        .body_mut()
        .read_to_string()
        .map_err(|e| format!("body read: {e}"))?;

    serde_json::from_str(&body).map_err(|e| format!("JSON parse: {e}"))
}

fn db_path() -> PathBuf {
    let cargo_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    cargo_dir.join("../../data/").canonicalize().unwrap()
}

fn build_db_schema(conn: &Connection) -> Result<(), String> {
    conn.execute_batch(include_str!("../../../data/schema.sql"))
        .map_err(|e| format!("schema: {e}"))
}

fn insert_candidate(
    conn: &Connection,
    project_name: &str,
    os_mappings: &HashMap<OsType, String>,
    pillar_fanout: &HashMap<OsType, Vec<OsType>>,
) -> Result<(), String> {
    let tx = conn
        .unchecked_transaction()
        .map_err(|e| format!("tx start: {e}"))?;

    tx.execute(
        "INSERT OR IGNORE INTO packages (name) VALUES (?1)",
        rusqlite::params![project_name],
    )
    .map_err(|e| format!("insert package: {e}"))?;

    let package_id: i64 = tx
        .query_row(
            "SELECT id FROM packages WHERE name = ?1",
            rusqlite::params![project_name],
            |row| row.get(0),
        )
        .map_err(|e| format!("get package id: {e}"))?;

    for (os_type, os_package) in os_mappings {
        let os_str = format!("{:?}", os_type);
        tx.execute(
            "INSERT OR IGNORE INTO mappings (package_id, os, os_package, source, confidence)
             VALUES (?1, ?2, ?3, 'repology_auto', 1.0)",
            rusqlite::params![package_id, os_str, os_package],
        )
        .map_err(|e| format!("insert mapping: {e}"))?;

        if let Some(derivatives) = pillar_fanout.get(os_type) {
            for derivative in derivatives {
                if derivative == os_type {
                    continue;
                }
                let deriv_str = format!("{:?}", derivative);
                tx.execute(
                    "INSERT OR IGNORE INTO mappings (package_id, os, os_package, source, confidence)
                     VALUES (?1, ?2, ?3, 'derived', 0.9)",
                    rusqlite::params![package_id, deriv_str, os_package],
                )
                .map_err(|e| format!("insert derived mapping: {e}"))?;
            }
        }
    }

    tx.commit().map_err(|e| format!("tx commit: {e}"))?;
    Ok(())
}

fn fetch_winget_index(client: &ureq::Agent) -> Result<Vec<WingetIndexItem>, String> {
    log::info!("fetching winget index: {WINGET_INDEX_URL}");
    let mut response = client
        .get(WINGET_INDEX_URL)
        .header("User-Agent", USER_AGENT)
        .call()
        .map_err(|e| format!("winget index HTTP request failed: {e}"))?;

    let body = response
        .body_mut()
        .read_to_string()
        .map_err(|e| format!("winget index body read: {e}"))?;

    serde_json::from_str(&body).map_err(|e| format!("winget index JSON parse: {e}"))
}

fn inject_winget_mappings(
    conn: &Connection,
    items: &[WingetIndexItem],
) -> Result<usize, String> {
    let mut stmt = conn
        .prepare("SELECT id, name FROM packages")
        .map_err(|e| format!("select packages: {e}"))?;

    let existing: Vec<(i64, String)> = stmt
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
        .map_err(|e| format!("query packages: {e}"))?
        .filter_map(|r| r.ok())
        .collect();

    let os_str = "Windows";
    let mut matched = 0usize;
    let tx = conn
        .unchecked_transaction()
        .map_err(|e| format!("tx start: {e}"))?;

    {
        let mut insert = tx
            .prepare(
                "INSERT OR IGNORE INTO mappings (package_id, os, os_package, source, confidence)
                 VALUES (?1, ?2, ?3, 'winget_direct', 1.0)",
            )
            .map_err(|e| format!("prepare insert mapping: {e}"))?;

        for item in items {
            let last_seg = item
                .package_id
                .rsplit('.')
                .next()
                .unwrap_or(&item.package_id)
                .to_lowercase();
            let name_lower = item.name.to_lowercase();

            for (pkg_id, canonical_name) in &existing {
                let canon_lower = canonical_name.to_lowercase();
                if canon_lower == last_seg || canon_lower == name_lower {
                    insert
                        .execute(rusqlite::params![
                            pkg_id,
                            os_str,
                            item.package_id,
                        ])
                        .map_err(|e| format!("insert mapping: {e}"))?;
                    matched += 1;
                    break;
                }
            }
        }
    }

    tx.commit().map_err(|e| format!("tx commit: {e}"))?;
    Ok(matched)
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    if std::env::args().any(|a| a == "--help" || a == "-h") {
        println!("db-update: Build seed.db.zst from Repology cross-platform data");
        println!();
        println!("Usage: db-update [--dry-run] [--verbose]");
        println!("  --dry-run   Only log candidates, don't write DB");
        println!("  --verbose   Show detailed progress");
        return;
    }

    let dry_run = std::env::args().any(|a| a == "--dry-run");
    let verbose = std::env::args().any(|a| a == "--verbose");

    let client = ureq::Agent::new_with_defaults();
    let registry = PlatformRegistry::load().expect("failed to load platform registry");
    let pillar_fanout = pillar_defs(&registry);

    let pillar_set: HashSet<OsType> = pillar_fanout.keys().copied().collect();
    log::info!("checking {} pillars: {:?}", pillar_set.len(), pillar_set);

    let mut cursor = String::new();
    let mut candidates: Vec<(String, HashMap<OsType, String>)> = Vec::new();
    let mut page_count = 0u32;

    loop {
        let page = match fetch_page(&client, &cursor) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("error: {e}");
                break;
            }
        };

        if page.is_empty() {
            log::info!("empty page, done");
            break;
        }

        page_count += 1;
        let keys: Vec<&String> = page.keys().collect();

        for project_name in &keys {
            let packages = &page[project_name.as_str()];

            if is_runtime_or_library(project_name) {
                if verbose {
                    log::info!("  skip (lib)   : {project_name}");
                }
                continue;
            }

            let mut covered: HashMap<OsType, String> = HashMap::new();
            for pkg in packages {
                if let Some(os_type) = registry.repo_to_os(&pkg.repo) {
                    if pillar_set.contains(os_type) {
                        if let Some(name) = pkg
                            .binname
                            .as_deref()
                            .or(pkg.srcname.as_deref())
                            .or(pkg.visible_name.as_deref())
                        {
                            covered.entry(*os_type).or_insert_with(|| name.to_string());
                        }
                    }
                }
            }

            if covered.len() == pillar_set.len() {
                if verbose {
                    log::info!("  candidate    : {project_name}");
                    for (os, pkg) in &covered {
                        log::info!("    {os:?} -> {pkg}");
                    }
                }
                candidates.push((project_name.to_string(), covered));
            } else {
                if verbose {
                    log::info!(
                        "  skip ({}/{})  : {project_name}",
                        covered.len(),
                        pillar_set.len()
                    );
                }
            }
        }

        cursor = keys.last().map(|s| (*s).clone()).unwrap_or_default();

        if keys.len() < PAGE_SIZE {
            log::info!("last page (< {PAGE_SIZE} projects), done");
            break;
        }

        std::thread::sleep(Duration::from_millis(RATE_LIMIT_MS));
    }

    log::info!(
        "scanned {page_count} pages, found {} candidates",
        candidates.len()
    );

    if dry_run {
        log::info!(
            "dry-run: would insert {} packages into DB",
            candidates.len()
        );
        return;
    }

    if candidates.is_empty() {
        log::warn!("no candidates found, DB not updated");
        return;
    }

    let data_dir = db_path();
    let tmp = std::env::temp_dir().join(format!("upi-db-build-{}.db", std::process::id()));
    let conn = Connection::open(&tmp).expect("failed to create temp DB");

    build_db_schema(&conn).expect("failed to build schema");

    for (project_name, os_mappings) in &candidates {
        insert_candidate(&conn, project_name, os_mappings, &pillar_fanout)
            .unwrap_or_else(|e| panic!("failed to insert '{project_name}': {e}"));
    }

    log::info!("inserted {} candidates from Repology", candidates.len());

    match fetch_winget_index(&client) {
        Ok(winget_items) => {
            log::info!("fetched {} winget packages", winget_items.len());
            match inject_winget_mappings(&conn, &winget_items) {
                Ok(matched) => {
                    log::info!(
                        "matched {matched} winget packages to existing DB entries"
                    );
                }
                Err(e) => {
                    log::warn!("error injecting winget mappings: {e}");
                }
            }
        }
        Err(e) => {
            log::warn!("error fetching winget index: {e}");
        }
    }

    drop(conn);

    let db_bytes = std::fs::read(&tmp).unwrap_or_else(|e| panic!("failed to read temp DB: {e}"));
    std::fs::remove_file(&tmp).ok();

    let compressed = ruzstd::encoding::compress_to_vec(
        db_bytes.as_slice(),
        ruzstd::encoding::CompressionLevel::Fastest,
    );

    let output_path = data_dir.join("seed.db.zst");
    std::fs::write(&output_path, &compressed)
        .unwrap_or_else(|e| panic!("failed to write {}: {e}", output_path.display()));

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let version_path = data_dir.join("seed-version.txt");
    std::fs::write(&version_path, now.to_string())
        .unwrap_or_else(|e| panic!("failed to write seed-version.txt: {e}"));

    let original_kb = db_bytes.len() as f64 / 1024.0;
    let compressed_kb = compressed.len() as f64 / 1024.0;
    let package_count = candidates.len();
    log::info!(
        "seed.db.zst generated: {:.1} KB -> {:.1} KB (ratio: {:.2}x, {package_count} packages)",
        original_kb,
        compressed_kb,
        original_kb / compressed_kb.max(1.0)
    );
}
