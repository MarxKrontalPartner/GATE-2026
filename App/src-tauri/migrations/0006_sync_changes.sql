-- Offline changes of synchronized records (for now: Rückprallhammer).
-- One row per local save made without server connection.
-- payload_json is exactly one item of the sync request sent to Django:
--   {"id": 1, "patch": [{"field": "bemerkung", "base": "alt", "new": "neu"}]}
-- Conflicts returned by the server are stored in client_sync_conflicts.

CREATE TABLE IF NOT EXISTS client_sync_changes (
    change_id TEXT PRIMARY KEY NOT NULL,
    entity_table TEXT NOT NULL,
    local_pk TEXT NOT NULL,
    base_server_revision TEXT,
    payload_json TEXT NOT NULL CHECK(json_valid(payload_json)),
    status TEXT NOT NULL DEFAULT 'pending'
        CHECK(status IN ('pending', 'sending', 'acknowledged', 'failed', 'conflict')),
    attempt_count INTEGER NOT NULL DEFAULT 0 CHECK(attempt_count >= 0),
    last_error TEXT,
    created_at TEXT NOT NULL,
    last_attempt_at TEXT,
    acknowledged_at TEXT
);
CREATE INDEX IF NOT EXISTS idx_client_sync_changes_status
    ON client_sync_changes(entity_table, status, created_at);
CREATE INDEX IF NOT EXISTS idx_client_sync_changes_record
    ON client_sync_changes(entity_table, local_pk, created_at);
