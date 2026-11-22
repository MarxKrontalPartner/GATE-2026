-- Conflicts reported by the server, one row per conflicting field
-- (replaces the 0002 layout with JSON snapshots).
-- server_value: the value the server has; new_value: the value the app sent.
-- Values are stored as JSON text, so null, booleans, numbers and lists keep their type.
-- column_name is the field name used in the sync request (links without "_id").

DROP TABLE IF EXISTS client_sync_conflicts;

CREATE TABLE client_sync_conflicts (
    conflict_id TEXT PRIMARY KEY NOT NULL,
    entity_table TEXT NOT NULL,
    local_pk TEXT NOT NULL,
    column_name TEXT NOT NULL,
    server_value TEXT NOT NULL CHECK(json_valid(server_value)),
    new_value TEXT NOT NULL CHECK(json_valid(new_value)),
    server_revision TEXT,
    status TEXT NOT NULL DEFAULT 'unresolved'
        CHECK(status IN ('unresolved', 'resolved_local', 'resolved_server')),
    created_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_client_conflicts_status
    ON client_sync_conflicts(status, created_at);
CREATE INDEX IF NOT EXISTS idx_client_conflicts_record
    ON client_sync_conflicts(entity_table, local_pk);
