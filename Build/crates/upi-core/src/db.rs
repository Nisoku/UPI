use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::Connection;

use crate::error::{Error, Result};

const SEED_DB: &[u8] = include_bytes!("../../../data/seed.db.zst");
const SEED_VERSION: &str = include_str!("../../../data/seed-version.txt");
const META_FILE: &str = "meta.json";
const DB_FILE: &str = "seed.db";

#[derive(Debug, Clone)]
pub struct Mapping {
    pub os_package: String,
    pub source: String,
    pub confidence: f64,
    pub notes: Option<String>,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn open() -> Result<Self> {
        Self::open_at(&cache_dir_path())
    }

    pub fn open_at(cache_dir: &Path) -> Result<Self> {
        std::fs::create_dir_all(cache_dir).map_err(Error::Io)?;

        let db_path = cache_dir.join(DB_FILE);
        let meta_path = cache_dir.join(META_FILE);

        if should_rehydrate(&meta_path, &db_path)? {
            log::info!("rehydrating seed database");
            let decompressed = decompress_seed()?;
            std::fs::write(&db_path, &decompressed).map_err(Error::Io)?;
            write_meta(&meta_path)?;
        } else {
            log::debug!("seed database is current");
        }

        let conn = Connection::open(&db_path).map_err(|e| Error::Database(e.to_string()))?;
        log::debug!("database opened at {}", db_path.display());

        Ok(Self { conn })
    }

    pub fn lookup(&self, package: &str, os_type: &os_info::Type) -> Result<Option<Mapping>> {
        let canonical = self.resolve_alias(package)?;
        let os_str = format!("{:?}", os_type);
        log::debug!(
            "db lookup: '{}' -> canonical='{}' for {}",
            package,
            canonical,
            os_str
        );

        let mut stmt = self
            .conn
            .prepare(
                "SELECT m.os_package, m.source, m.confidence, m.notes
                 FROM mappings m
                 JOIN packages p ON p.id = m.package_id
                 WHERE p.name = ? AND m.os = ?
                 ORDER BY m.confidence DESC
                 LIMIT 1",
            )
            .map_err(|e| Error::Database(e.to_string()))?;

        let result = stmt.query_row(rusqlite::params![canonical, os_str], |row| {
            Ok(Mapping {
                os_package: row.get(0)?,
                source: row.get(1)?,
                confidence: row.get(2)?,
                notes: row.get(3)?,
            })
        });

        match &result {
            Ok(mapping) => log::debug!(
                "db hit: {} (confidence={}, source={})",
                mapping.os_package,
                mapping.confidence,
                mapping.source
            ),
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                log::debug!("db miss: '{}' on {}", package, os_str)
            }
            _ => {}
        }
        match result {
            Ok(mapping) => Ok(Some(mapping)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(Error::Database(e.to_string())),
        }
    }

    pub fn search(&self, query: &str, os_type: &os_info::Type) -> Result<Vec<Mapping>> {
        let os_str = format!("{:?}", os_type);
        let pattern = format!("%{}%", query.to_lowercase());
        log::debug!("db search: '{query}' on {os_str}");

        let mut stmt = self
            .conn
            .prepare(
                "SELECT m.os_package, m.source, m.confidence, m.notes
                 FROM mappings m
                 JOIN packages p ON p.id = m.package_id
                 WHERE LOWER(p.name) LIKE ?1 AND m.os = ?2
                 ORDER BY m.confidence DESC
                 LIMIT 20",
            )
            .map_err(|e| Error::Database(e.to_string()))?;

        let rows = stmt
            .query_map(rusqlite::params![pattern, os_str], |row| {
                Ok(Mapping {
                    os_package: row.get(0)?,
                    source: row.get(1)?,
                    confidence: row.get(2)?,
                    notes: row.get(3)?,
                })
            })
            .map_err(|e| Error::Database(e.to_string()))?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row.map_err(|e| Error::Database(e.to_string()))?);
        }
        log::debug!("db search: {} results", results.len());
        Ok(results)
    }

    fn resolve_alias(&self, name: &str) -> Result<String> {
        let mut stmt = self
            .conn
            .prepare("SELECT canonical FROM aliases WHERE alias = ?")
            .map_err(|e| Error::Database(e.to_string()))?;

        let result = stmt.query_row(rusqlite::params![name], |row| row.get::<_, String>(0));

        match result {
            Ok(canonical) => Ok(canonical),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(name.to_string()),
            Err(e) => Err(Error::Database(e.to_string())),
        }
    }

    pub fn seed_version(&self) -> Result<String> {
        self.conn
            .query_row("SELECT value FROM meta WHERE key = 'version'", [], |row| {
                row.get::<_, String>(0)
            })
            .map_err(|e| Error::Database(e.to_string()))
    }
}

fn cache_dir_path() -> PathBuf {
    let base = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    base.join(".upi").join("db")
}

fn should_rehydrate(meta_path: &Path, db_path: &Path) -> Result<bool> {
    if !meta_path.exists() || !db_path.exists() {
        return Ok(true);
    }

    let content =
        std::fs::read_to_string(meta_path).map_err(|e| Error::Database(format!("{e}")))?;

    let meta: serde_json::Value =
        serde_json::from_str(&content).map_err(|e| Error::Database(format!("{e}")))?;

    let cached_version = meta.get("version").and_then(|v| v.as_str()).unwrap_or("");

    Ok(cached_version != SEED_VERSION)
}

fn write_meta(meta_path: &Path) -> Result<()> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let meta = serde_json::json!({
        "version": SEED_VERSION,
        "updated_at": now,
    });

    let content = serde_json::to_string_pretty(&meta).unwrap();
    std::fs::write(meta_path, content).map_err(|e| Error::Database(format!("{e}")))
}

fn decompress_seed() -> Result<Vec<u8>> {
    use std::io::Read;

    let mut decoder = ruzstd::decoding::StreamingDecoder::new(SEED_DB)
        .map_err(|e| Error::Database(format!("zstd decoder init: {e}")))?;
    let mut output = Vec::new();
    decoder
        .read_to_end(&mut output)
        .map_err(|e| Error::Database(format!("zstd decode: {e}")))?;
    Ok(output)
}
