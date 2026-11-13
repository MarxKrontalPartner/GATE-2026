"""Validate the client SQL migrations in a disposable in-memory SQLite database."""
from pathlib import Path
import sqlite3

base = Path(__file__).resolve().parent.parent
conn = sqlite3.connect(':memory:')
conn.execute('PRAGMA foreign_keys=ON')
for path in sorted((base/'migrations').glob('*.sql')):
    conn.executescript(path.read_text(encoding='utf-8'))
    print('Applied', path.name)
tables = [r[0] for r in conn.execute("SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'")]
app = [t for t in tables if not t.startswith('client_')]
meta = [t for t in tables if t.startswith('client_')]
assert len(app) == 35, f'Expected 35 domain tables, got {len(app)}'
assert len(meta) == 9, f'Expected 9 client tables, got {len(meta)}'
for table in tables:
    for fk in conn.execute(f'PRAGMA foreign_key_list("{table}")'):
        assert fk[2] in tables, f'{table} references missing {fk[2]}'
assert conn.execute('PRAGMA foreign_key_check').fetchall() == []
print(f'PASS: {len(app)} domain tables, {len(meta)} client tables; FK checks OK')
