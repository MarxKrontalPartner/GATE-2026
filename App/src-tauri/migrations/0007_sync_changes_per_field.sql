-- Offline changes, one row per changed field of a record (replaces the 0006 layout).
-- (table_name, column_name, row_pk) identifies the change: saving the same field
-- again only updates new_value; server_value keeps the value the server had.
-- Values are stored as JSON text, so null, booleans, numbers and lists keep their type.
-- column_name is the field name sent to the server (links without the "_id" suffix).
-- A row exists only while the change is not yet saved on the server.

DROP TABLE IF EXISTS client_sync_changes;

CREATE TABLE client_sync_changes (
    table_name TEXT NOT NULL,
    column_name TEXT NOT NULL,
    row_pk TEXT NOT NULL,
    server_value TEXT NOT NULL CHECK(json_valid(server_value)),
    new_value TEXT NOT NULL CHECK(json_valid(new_value)),
    PRIMARY KEY (table_name, column_name, row_pk)
);
