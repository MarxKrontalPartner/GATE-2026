-- UI-only recent activity, not a synchronization or domain revision.
-- Persists which Bauwerk the engineer most recently opened on this device.
CREATE TABLE IF NOT EXISTS client_dashboard_recent_bauwerk (
    bauwerk_id INTEGER PRIMARY KEY NOT NULL,
    last_opened_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (bauwerk_id) REFERENCES assetverwaltung_bauwerk(id) ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS idx_client_dashboard_recent_opened
    ON client_dashboard_recent_bauwerk(last_opened_at DESC);
