-- Dashboard-only client metadata. Does not change Django or domain tables.
-- A baseline records the *server* Bauwerk attributes as last downloaded.
-- Comparing against this snapshot does not overwrite any local edits.
CREATE TABLE IF NOT EXISTS client_dashboard_bauwerk_baseline (
    bauwerk_id INTEGER PRIMARY KEY,
    server_snapshot_json TEXT NOT NULL CHECK(json_valid(server_snapshot_json)),
    downloaded_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_checked_at TEXT,
    server_update_available INTEGER NOT NULL DEFAULT 0 CHECK(server_update_available IN (0,1)),
    FOREIGN KEY (bauwerk_id) REFERENCES assetverwaltung_bauwerk(id)
);

-- Objects visible on the server but not downloaded. This is discovery metadata,
-- NOT an offline Bauwerk and cannot be opened until downloaded.
CREATE TABLE IF NOT EXISTS client_dashboard_available_bauwerk (
    bauwerk_id INTEGER PRIMARY KEY,
    bezeichnung TEXT NOT NULL,
    bauwerksnummer TEXT,
    archived INTEGER NOT NULL DEFAULT 0,
    projekte_json TEXT NOT NULL DEFAULT '[]' CHECK(json_valid(projekte_json)),
    last_seen_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS client_dashboard_meta (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);
