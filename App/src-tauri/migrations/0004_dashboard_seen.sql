-- Dashboard UI acknowledgement. Old baselines predate this field,
-- so classify them as already seen instead of incorrectly calling them new.
ALTER TABLE client_dashboard_bauwerk_baseline ADD COLUMN seen_at TEXT;
UPDATE client_dashboard_bauwerk_baseline SET seen_at=CURRENT_TIMESTAMP;
