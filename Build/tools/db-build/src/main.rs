use std::path::PathBuf;

const MIN_DIVISOR: f64 = 1.0; // prevent division by zero in ratio calculation

fn data_dir() -> PathBuf {
    let cargo_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    cargo_dir
        .join("../../data/")
        .canonicalize()
        .expect("../../data/ must exist relative to CARGO_MANIFEST_DIR")
}

fn main() {
    let data = data_dir();
    let schema_path = data.join("schema.sql");
    let seed_path = data.join("seed-data.sql");
    let output_path = data.join("seed.db.zst");

    let schema = std::fs::read_to_string(&schema_path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", schema_path.display()));
    let seed = std::fs::read_to_string(&seed_path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", seed_path.display()));

    let tmp = std::env::temp_dir().join(format!("upi-db-build-{}.db", std::process::id()));
    let conn = rusqlite::Connection::open(&tmp)
        .unwrap_or_else(|e| panic!("failed to create temp DB: {e}"));

    conn.execute_batch(&schema)
        .unwrap_or_else(|e| panic!("failed to execute schema: {e}"));
    conn.execute_batch(&seed)
        .unwrap_or_else(|e| panic!("failed to execute seed: {e}"));

    drop(conn);

    let db_bytes = std::fs::read(&tmp).unwrap_or_else(|e| panic!("failed to read temp DB: {e}"));

    std::fs::remove_file(&tmp).ok();

    let compressed = ruzstd::encoding::compress_to_vec(
        db_bytes.as_slice(),
        ruzstd::encoding::CompressionLevel::Fastest,
    );

    std::fs::write(&output_path, &compressed)
        .unwrap_or_else(|e| panic!("failed to write {}: {e}", output_path.display()));

    let original_kb = db_bytes.len() as f64 / 1024.0;
    let compressed_kb = compressed.len() as f64 / 1024.0;
    println!(
        "seed.db.zst generated: {:.1} KB -> {:.1} KB (ratio: {:.2}x)",
        original_kb,
        compressed_kb,
        original_kb / compressed_kb.max(MIN_DIVISOR)
    );
}
