"""SQLite migration and material-reference tests using Python stdlib only.

Run from the repository root:
    python3 -m unittest discover -s App/tests/release -p 'test_*.py' -v
No existing user database is opened or modified.
"""
import sqlite3
import unittest
from pathlib import Path


MIGRATIONS = Path(__file__).resolve().parents[2] / 'src-tauri' / 'migrations'


def new_db():
    db = sqlite3.connect(':memory:')
    db.execute('PRAGMA foreign_keys=ON')
    for migration in sorted(MIGRATIONS.glob('*.sql')):
        db.executescript(migration.read_text(encoding='utf-8'))
    return db


class MaterialReferences(unittest.TestCase):
    def setUp(self):
        self.db = new_db()

    def tearDown(self):
        self.db.close()

    def test_all_nine_migrations_apply_and_foreign_key_check_is_clean(self):
        self.assertEqual(len(list(MIGRATIONS.glob('*.sql'))), 9)
        self.assertEqual(self.db.execute('PRAGMA foreign_key_check').fetchall(), [])

    def test_valid_material_relation_is_stored(self):
        self.db.execute('INSERT INTO bauwerksdiagnostik_bohrkern(id,name) VALUES (10,?)', ('Kern A',))
        self.db.execute('INSERT INTO bauwerksdiagnostik_material(id,name,beschreibung) VALUES (2,?,?)', ('Beton',''))
        self.db.execute('INSERT INTO bauwerksdiagnostik_bohrkern_material(bohrkern_id,material_id) VALUES (10,2)')
        self.db.commit()
        self.assertEqual(self.db.execute('SELECT material_id FROM bauwerksdiagnostik_bohrkern_material').fetchone()[0], 2)

    def test_invalid_material_fk_cannot_be_committed(self):
        self.db.execute('INSERT INTO bauwerksdiagnostik_bohrkern(id,name) VALUES (10,?)', ('Kern A',))
        self.db.execute('INSERT INTO bauwerksdiagnostik_bohrkern_material(bohrkern_id,material_id) VALUES (10,2)')
        with self.assertRaises(sqlite3.IntegrityError):
            self.db.commit()
        self.db.rollback()

    def test_same_id_can_exist_locally_but_be_missing_on_server(self):
        # Two independent databases: local has Material #2; server has only #1.
        local, server = self.db, new_db()
        try:
            local.execute('INSERT INTO bauwerksdiagnostik_material(id,name,beschreibung) VALUES (2,?,?)', ('Beton',''))
            local.commit()
            server.execute('INSERT INTO bauwerksdiagnostik_material(id,name,beschreibung) VALUES (1,?,?)', ('Beton',''))
            server.execute('INSERT INTO bauwerksdiagnostik_bohrkern(id,name) VALUES (10,?)', ('Kern A',))
            server.execute('INSERT INTO bauwerksdiagnostik_bohrkern_material(bohrkern_id,material_id) VALUES (10,2)')
            with self.assertRaises(sqlite3.IntegrityError):
                server.commit()
            server.rollback()
            self.assertTrue(local.execute('SELECT id FROM bauwerksdiagnostik_material WHERE id=2').fetchone())
            self.assertFalse(server.execute('SELECT id FROM bauwerksdiagnostik_material WHERE id=2').fetchone())
        finally:
            server.close()


if __name__ == '__main__':
    unittest.main()
