-- Allow sync_status 'new': a record created offline that is not yet on the server.
-- It gets a temporary negative id locally; after the sync it gets the server id
-- and the status 'synced'. SQLite cannot change a CHECK constraint in place, so
-- the table is rebuilt with all existing rows.

CREATE TABLE client_record_state_rebuilt (
    entity_table TEXT NOT NULL,
    local_pk TEXT NOT NULL,
    client_uuid TEXT NOT NULL UNIQUE,
    server_pk TEXT,
    base_server_revision TEXT,
    local_revision INTEGER NOT NULL DEFAULT 0 CHECK(local_revision >= 0),
    sync_status TEXT NOT NULL DEFAULT 'synced'
        CHECK(sync_status IN ('new', 'synced', 'pending', 'syncing', 'conflict', 'failed', 'deleted')),
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_synced_at TEXT,
    deleted_at TEXT,
    PRIMARY KEY(entity_table, local_pk)
);

INSERT INTO client_record_state_rebuilt (
    entity_table, local_pk, client_uuid, server_pk, base_server_revision, local_revision,
    sync_status, updated_at, last_synced_at, deleted_at)
SELECT entity_table, local_pk, client_uuid, server_pk, base_server_revision, local_revision,
    sync_status, updated_at, last_synced_at, deleted_at
FROM client_record_state;

DROP TABLE client_record_state;
ALTER TABLE client_record_state_rebuilt RENAME TO client_record_state;

CREATE INDEX IF NOT EXISTS idx_client_record_state_status
    ON client_record_state(sync_status, entity_table);
CREATE UNIQUE INDEX IF NOT EXISTS idx_client_record_state_server_pk
    ON client_record_state(entity_table, server_pk) WHERE server_pk IS NOT NULL;
