-- GATE-MKP local-only metadata for sync, device state, media, and conflicts.
-- All fields live outside the domain tables, to prevent schema drift with Django.

CREATE TABLE IF NOT EXISTS client_record_state (
    entity_table TEXT NOT NULL,
    local_pk TEXT NOT NULL,
    client_uuid TEXT NOT NULL UNIQUE,
    server_pk TEXT,
    base_server_revision TEXT,
    local_revision INTEGER NOT NULL DEFAULT 0 CHECK(local_revision >= 0),
    sync_status TEXT NOT NULL DEFAULT 'synced'
        CHECK(sync_status IN ('synced', 'pending', 'syncing', 'conflict', 'failed', 'deleted')),
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_synced_at TEXT,
    deleted_at TEXT,
    PRIMARY KEY(entity_table, local_pk)
);
CREATE INDEX IF NOT EXISTS idx_client_record_state_status
    ON client_record_state(sync_status, entity_table);
CREATE UNIQUE INDEX IF NOT EXISTS idx_client_record_state_server_pk
    ON client_record_state(entity_table, server_pk) WHERE server_pk IS NOT NULL;

-- One idempotent event per local operation; multiple events may exist per record.
CREATE TABLE IF NOT EXISTS client_sync_outbox (
    operation_id TEXT PRIMARY KEY NOT NULL,
    entity_table TEXT NOT NULL,
    local_pk TEXT NOT NULL,
    operation TEXT NOT NULL CHECK(operation IN ('create', 'update', 'delete', 'link', 'unlink')),
    base_server_revision TEXT,
    payload_json TEXT CHECK(payload_json IS NULL OR json_valid(payload_json)),
    status TEXT NOT NULL DEFAULT 'pending'
        CHECK(status IN ('pending', 'sending', 'acknowledged', 'failed', 'conflict')),
    attempt_count INTEGER NOT NULL DEFAULT 0 CHECK(attempt_count >= 0),
    last_error TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_attempt_at TEXT,
    acknowledged_at TEXT
);
CREATE INDEX IF NOT EXISTS idx_client_sync_outbox_status
    ON client_sync_outbox(status, created_at);
CREATE INDEX IF NOT EXISTS idx_client_sync_outbox_record
    ON client_sync_outbox(entity_table, local_pk, created_at);

CREATE TABLE IF NOT EXISTS client_sync_conflicts (
    conflict_id TEXT PRIMARY KEY NOT NULL,
    entity_table TEXT NOT NULL,
    local_pk TEXT NOT NULL,
    operation_id TEXT,
    reason TEXT NOT NULL,
    base_snapshot_json TEXT CHECK(base_snapshot_json IS NULL OR json_valid(base_snapshot_json)),
    local_snapshot_json TEXT NOT NULL CHECK(json_valid(local_snapshot_json)),
    server_snapshot_json TEXT CHECK(server_snapshot_json IS NULL OR json_valid(server_snapshot_json)),
    server_revision TEXT,
    status TEXT NOT NULL DEFAULT 'unresolved'
        CHECK(status IN ('unresolved', 'resolved_local', 'resolved_server', 'resolved_merged')),
    resolution_json TEXT CHECK(resolution_json IS NULL OR json_valid(resolution_json)),
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    resolved_at TEXT,
    resolved_by TEXT,
    FOREIGN KEY(operation_id) REFERENCES client_sync_outbox(operation_id)
);
CREATE INDEX IF NOT EXISTS idx_client_conflicts_status
    ON client_sync_conflicts(status, created_at);

-- Cursor is scoped to a downloadable dataset/project and resource category.
CREATE TABLE IF NOT EXISTS client_sync_cursors (
    dataset_key TEXT NOT NULL,
    resource_key TEXT NOT NULL,
    cursor TEXT,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY(dataset_key, resource_key)
);

-- Offline delete protection; keep tombstones until server acknowledgement.
CREATE TABLE IF NOT EXISTS client_sync_tombstones (
    entity_table TEXT NOT NULL,
    local_pk TEXT NOT NULL,
    client_uuid TEXT NOT NULL,
    server_pk TEXT,
    base_server_revision TEXT,
    deleted_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    acknowledged_at TEXT,
    PRIMARY KEY(entity_table, local_pk)
);

CREATE TABLE IF NOT EXISTS client_cached_projects (
    project_id INTEGER NOT NULL PRIMARY KEY,
    download_status TEXT NOT NULL DEFAULT 'not_downloaded'
        CHECK(download_status IN ('not_downloaded', 'downloading', 'ready', 'failed')),
    last_full_download_at TEXT,
    last_sync_at TEXT,
    last_error TEXT,
    FOREIGN KEY(project_id) REFERENCES assetverwaltung_projekt(id)
);

-- Binary files are kept in the app-private file system, NOT in SQLite text columns.
CREATE TABLE IF NOT EXISTS client_media_transfers (
    transfer_id TEXT PRIMARY KEY NOT NULL,
    media_local_pk TEXT,
    local_file_path TEXT NOT NULL,
    content_sha256 TEXT,
    mime_type TEXT,
    byte_size INTEGER CHECK(byte_size IS NULL OR byte_size >= 0),
    remote_file_key TEXT,
    direction TEXT NOT NULL CHECK(direction IN ('upload', 'download')),
    status TEXT NOT NULL DEFAULT 'pending'
        CHECK(status IN ('pending', 'transferring', 'complete', 'failed')),
    retries INTEGER NOT NULL DEFAULT 0 CHECK(retries >= 0),
    last_error TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    completed_at TEXT
);
CREATE INDEX IF NOT EXISTS idx_client_media_transfers_status
    ON client_media_transfers(status, created_at);

-- Store only offline-safe user identity/roles; never passwords, JWTs or session records here.
CREATE TABLE IF NOT EXISTS client_user_context (
    user_id TEXT PRIMARY KEY NOT NULL,
    display_name TEXT,
    permissions_json TEXT CHECK(permissions_json IS NULL OR json_valid(permissions_json)),
    last_online_auth_at TEXT,
    offline_access_expires_at TEXT
);

CREATE TABLE IF NOT EXISTS client_device_context (
    device_id TEXT PRIMARY KEY NOT NULL,
    registered_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    server_registered_at TEXT,
    last_successful_sync_at TEXT
);
