CREATE TABLE IF NOT EXISTS packages (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS mappings (
    package_id INTEGER NOT NULL REFERENCES packages(id),
    os TEXT NOT NULL,
    os_package TEXT NOT NULL,
    source TEXT NOT NULL DEFAULT 'manual',
    confidence REAL NOT NULL DEFAULT 1.0,
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    version TEXT,
    notes TEXT,
    PRIMARY KEY (package_id, os)
);

CREATE INDEX IF NOT EXISTS idx_mappings_os ON mappings(os);
CREATE INDEX IF NOT EXISTS idx_mappings_os_package ON mappings(os, os_package);

CREATE TABLE IF NOT EXISTS meta (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS aliases (
    alias TEXT NOT NULL PRIMARY KEY,
    canonical TEXT NOT NULL,
    source TEXT NOT NULL DEFAULT 'manual',
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    notes TEXT
);

INSERT INTO meta (key, value) VALUES ('version', '1');
INSERT INTO meta (key, value) VALUES ('created_at', (datetime('now')));
INSERT INTO meta (key, value) VALUES ('schema_version', '1');
