-- GATE-MKP client database / schema snapshot from supplied Server/db.sqlite3
-- Mirrors ALL 35 Django application tables, including join tables, without copying any data.
-- Do not edit this migration after releasing it. Add a new versioned migration instead.
-- The Django-auth/system tables are intentionally not installed on devices.

-- 01. assetverwaltung_bauwerk
CREATE TABLE "assetverwaltung_bauwerk" ("id" integer NOT NULL PRIMARY KEY AUTOINCREMENT, "bezeichnung" varchar(100) NOT NULL UNIQUE, "archived" bool NOT NULL, "beschreibung" text NOT NULL, "strasse" varchar(100) NOT NULL, "bauweise" varchar(200) NOT NULL, "bauwerksnummer" varchar(100) NOT NULL, "bauwerkstyp" varchar(100) NULL, "bemerkung" text NULL);

-- 02. assetverwaltung_bauwerk_projekt
CREATE TABLE "assetverwaltung_bauwerk_projekt" ("id" integer NOT NULL PRIMARY KEY AUTOINCREMENT, "bauwerk_id" bigint NOT NULL REFERENCES "assetverwaltung_bauwerk" ("id") DEFERRABLE INITIALLY DEFERRED, "projekt_id" bigint NOT NULL REFERENCES "assetverwaltung_projekt" ("id") DEFERRABLE INITIALLY DEFERRED);

-- 03. assetverwaltung_firma
CREATE TABLE "assetverwaltung_firma" ("id" integer NOT NULL PRIMARY KEY AUTOINCREMENT, "bezeichnung" varchar(100) NOT NULL);

-- 04. assetverwaltung_person
CREATE TABLE "assetverwaltung_person" ("id" integer NOT NULL PRIMARY KEY AUTOINCREMENT, "name" varchar(100) NOT NULL, "vorname" varchar(100) NOT NULL, "stellenbezeichnung" varchar(100) NOT NULL, "email" varchar(254) NOT NULL, "telefon" varchar(100) NOT NULL, "firma_id" bigint NULL REFERENCES "assetverwaltung_firma" ("id") DEFERRABLE INITIALLY DEFERRED);

-- 05. assetverwaltung_projekt
CREATE TABLE "assetverwaltung_projekt" ("id" integer NOT NULL PRIMARY KEY AUTOINCREMENT, "projektnummer" varchar(100) NOT NULL, "name" varchar(100) NOT NULL, "bezeichnung" varchar(100) NOT NULL, "start" date NULL, "ende" date NULL);

-- 06. bauwerksdiagnostik_bohrkern
CREATE TABLE "bauwerksdiagnostik_bohrkern" ("id" integer NOT NULL PRIMARY KEY AUTOINCREMENT, "name" varchar(100) NOT NULL, "anzahlProben" integer NULL);

-- 07. bauwerksdiagnostik_bohrkern_fotos
CREATE TABLE "bauwerksdiagnostik_bohrkern_fotos" ("id" integer NOT NULL PRIMARY KEY AUTOINCREMENT, "bohrkern_id" bigint NOT NULL REFERENCES "bauwerksdiagnostik_bohrkern" ("id") DEFERRABLE INITIALLY DEFERRED, "media_id" bigint NOT NULL REFERENCES "bauwerksdiagnostik_media" ("id") DEFERRABLE INITIALLY DEFERRED);

-- 08. bauwerksdiagnostik_bohrkern_material
CREATE TABLE "bauwerksdiagnostik_bohrkern_material" ("id" integer NOT NULL PRIMARY KEY AUTOINCREMENT, "bohrkern_id" bigint NOT NULL REFERENCES "bauwerksdiagnostik_bohrkern" ("id") DEFERRABLE INITIALLY DEFERRED, "material_id" bigint NOT NULL REFERENCES "bauwerksdiagnostik_material" ("id") DEFERRABLE INITIALLY DEFERRED);

-- 09. bauwerksdiagnostik_chloridanalyse
CREATE TABLE "bauwerksdiagnostik_chloridanalyse" ("id" integer NOT NULL PRIMARY KEY AUTOINCREMENT, "datum" date NOT NULL, "zeit" time NULL, "bemerkung" text NOT NULL, "lufttemperatur" real NULL, "luftfeuchte" real NULL, "abgeschlossen" bool NULL, "sort_order" integer unsigned NOT NULL CHECK ("sort_order" >= 0), "pruefstelle" varchar(100) NOT NULL, "analyseverfahren" varchar(20) NOT NULL, "chloridgehaltZement" real NULL, "zementgehaltAnnahme" real NULL, "grenzwertUeberschritten" bool NULL, "bauwerk_id" bigint NULL REFERENCES "assetverwaltung_bauwerk" ("id") DEFERRABLE INITIALLY DEFERRED, "projekt_id" bigint NULL REFERENCES "assetverwaltung_projekt" ("id") DEFERRABLE INITIALLY DEFERRED, "titelBild_id" bigint NULL REFERENCES "bauwerksdiagnostik_media" ("id") DEFERRABLE INITIALLY DEFERRED, "probe_id" bigint NULL REFERENCES "bauwerksdiagnostik_probe" ("id") DEFERRABLE INITIALLY DEFERRED, "untersuchungsstelle_id" bigint NULL REFERENCES "bauwerksdiagnostik_untersuchungsstelle" ("id") DEFERRABLE INITIALLY DEFERRED);

-- 10. bauwerksdiagnostik_chloridanalyse_bearbeiter
CREATE TABLE "bauwerksdiagnostik_chloridanalyse_bearbeiter" ("id" integer NOT NULL PRIMARY KEY AUTOINCREMENT, "chloridanalyse_id" bigint NOT NULL REFERENCES "bauwerksdiagnostik_chloridanalyse" ("id") DEFERRABLE INITIALLY DEFERRED, "person_id" bigint NOT NULL REFERENCES "assetverwaltung_person" ("id") DEFERRABLE INITIALLY DEFERRED);

-- 11. bauwerksdiagnostik_chloridanalyse_bilder
CREATE TABLE "bauwerksdiagnostik_chloridanalyse_bilder" ("id" integer NOT NULL PRIMARY KEY AUTOINCREMENT, "chloridanalyse_id" bigint NOT NULL REFERENCES "bauwerksdiagnostik_chloridanalyse" ("id") DEFERRABLE INITIALLY DEFERRED, "media_id" bigint NOT NULL REFERENCES "bauwerksdiagnostik_media" ("id") DEFERRABLE INITIALLY DEFERRED);

-- 12. bauwerksdiagnostik_chloridanalyse_pruefberichte
CREATE TABLE "bauwerksdiagnostik_chloridanalyse_pruefberichte" ("id" integer NOT NULL PRIMARY KEY AUTOINCREMENT, "chloridanalyse_id" bigint NOT NULL REFERENCES "bauwerksdiagnostik_chloridanalyse" ("id") DEFERRABLE INITIALLY DEFERRED, "media_id" bigint NOT NULL REFERENCES "bauwerksdiagnostik_media" ("id") DEFERRABLE INITIALLY DEFERRED);

-- 13. bauwerksdiagnostik_documentmedia
CREATE TABLE "bauwerksdiagnostik_documentmedia" ("media_ptr_id" bigint NOT NULL PRIMARY KEY REFERENCES "bauwerksdiagnostik_media" ("id") DEFERRABLE INITIALLY DEFERRED, "file_format" varchar(10) NOT NULL, "preview" text NOT NULL, "preview_image" varchar(100) NOT NULL);

-- 14. bauwerksdiagnostik_druckfestigkeitspruefung
CREATE TABLE "bauwerksdiagnostik_druckfestigkeitspruefung" ("id" integer NOT NULL PRIMARY KEY AUTOINCREMENT, "datum" date NOT NULL, "zeit" time NULL, "bemerkung" text NOT NULL, "lufttemperatur" real NULL, "luftfeuchte" real NULL, "abgeschlossen" bool NULL, "sort_order" integer unsigned NOT NULL CHECK ("sort_order" >= 0), "pruefalter" integer NULL, "lagerung" varchar(20) NOT NULL, "bruchlast" real NULL, "druckfestigkeit" real NULL, "bruchbild" varchar(20) NOT NULL, "bauwerk_id" bigint NULL REFERENCES "assetverwaltung_bauwerk" ("id") DEFERRABLE INITIALLY DEFERRED, "projekt_id" bigint NULL REFERENCES "assetverwaltung_projekt" ("id") DEFERRABLE INITIALLY DEFERRED, "pruefbericht_id" bigint NULL REFERENCES "bauwerksdiagnostik_media" ("id") DEFERRABLE INITIALLY DEFERRED, "titelBild_id" bigint NULL REFERENCES "bauwerksdiagnostik_media" ("id") DEFERRABLE INITIALLY DEFERRED, "probe_id" bigint NULL REFERENCES "bauwerksdiagnostik_probe" ("id") DEFERRABLE INITIALLY DEFERRED, "untersuchungsstelle_id" bigint NULL REFERENCES "bauwerksdiagnostik_untersuchungsstelle" ("id") DEFERRABLE INITIALLY DEFERRED);

-- 15. bauwerksdiagnostik_druckfestigkeitspruefung_bearbeiter
CREATE TABLE "bauwerksdiagnostik_druckfestigkeitspruefung_bearbeiter" ("id" integer NOT NULL PRIMARY KEY AUTOINCREMENT, "druckfestigkeitspruefung_id" bigint NOT NULL REFERENCES "bauwerksdiagnostik_druckfestigkeitspruefung" ("id") DEFERRABLE INITIALLY DEFERRED, "person_id" bigint NOT NULL REFERENCES "assetverwaltung_person" ("id") DEFERRABLE INITIALLY DEFERRED);

-- 16. bauwerksdiagnostik_druckfestigkeitspruefung_bilder
CREATE TABLE "bauwerksdiagnostik_druckfestigkeitspruefung_bilder" ("id" integer NOT NULL PRIMARY KEY AUTOINCREMENT, "druckfestigkeitspruefung_id" bigint NOT NULL REFERENCES "bauwerksdiagnostik_druckfestigkeitspruefung" ("id") DEFERRABLE INITIALLY DEFERRED, "media_id" bigint NOT NULL REFERENCES "bauwerksdiagnostik_media" ("id") DEFERRABLE INITIALLY DEFERRED);

-- 17. bauwerksdiagnostik_imagemedia
CREATE TABLE "bauwerksdiagnostik_imagemedia" ("media_ptr_id" bigint NOT NULL PRIMARY KEY REFERENCES "bauwerksdiagnostik_media" ("id") DEFERRABLE INITIALLY DEFERRED, "width" integer NULL, "height" integer NULL, "historie" text NULL CHECK ((JSON_VALID("historie") OR "historie" IS NULL)), "drawing" text NULL CHECK ((JSON_VALID("drawing") OR "drawing" IS NULL)), "exif" text NULL CHECK ((JSON_VALID("exif") OR "exif" IS NULL)));

-- 18. bauwerksdiagnostik_kernbohrung
CREATE TABLE "bauwerksdiagnostik_kernbohrung" ("id" integer NOT NULL PRIMARY KEY AUTOINCREMENT, "datum" date NOT NULL, "zeit" time NULL, "bemerkung" text NOT NULL, "lufttemperatur" real NULL, "luftfeuchte" real NULL, "abgeschlossen" bool NULL, "sort_order" integer unsigned NOT NULL CHECK ("sort_order" >= 0), "durchmesser" real NULL, "bohrlaenge" real NULL, "neigungswinkel" real NULL, "kuehlung" varchar(20) NOT NULL, "verschlussmaterial" varchar(100) NOT NULL, "schichtaufbau" text NULL CHECK ((JSON_VALID("schichtaufbau") OR "schichtaufbau" IS NULL)), "bewehrungAngeschnitten" bool NULL, "kernVollstaendig" bool NULL, "bauwerk_id" bigint NULL REFERENCES "assetverwaltung_bauwerk" ("id") DEFERRABLE INITIALLY DEFERRED, "bohrkern_id" bigint NULL UNIQUE REFERENCES "bauwerksdiagnostik_bohrkern" ("id") DEFERRABLE INITIALLY DEFERRED, "projekt_id" bigint NULL REFERENCES "assetverwaltung_projekt" ("id") DEFERRABLE INITIALLY DEFERRED, "titelBild_id" bigint NULL REFERENCES "bauwerksdiagnostik_media" ("id") DEFERRABLE INITIALLY DEFERRED, "untersuchungsstelle_id" bigint NULL REFERENCES "bauwerksdiagnostik_untersuchungsstelle" ("id") DEFERRABLE INITIALLY DEFERRED);

-- 19. bauwerksdiagnostik_kernbohrung_bearbeiter
CREATE TABLE "bauwerksdiagnostik_kernbohrung_bearbeiter" ("id" integer NOT NULL PRIMARY KEY AUTOINCREMENT, "kernbohrung_id" bigint NOT NULL REFERENCES "bauwerksdiagnostik_kernbohrung" ("id") DEFERRABLE INITIALLY DEFERRED, "person_id" bigint NOT NULL REFERENCES "assetverwaltung_person" ("id") DEFERRABLE INITIALLY DEFERRED);

-- 20. bauwerksdiagnostik_kernbohrung_bilder
CREATE TABLE "bauwerksdiagnostik_kernbohrung_bilder" ("id" integer NOT NULL PRIMARY KEY AUTOINCREMENT, "kernbohrung_id" bigint NOT NULL REFERENCES "bauwerksdiagnostik_kernbohrung" ("id") DEFERRABLE INITIALLY DEFERRED, "media_id" bigint NOT NULL REFERENCES "bauwerksdiagnostik_media" ("id") DEFERRABLE INITIALLY DEFERRED);

-- 21. bauwerksdiagnostik_material
CREATE TABLE "bauwerksdiagnostik_material" ("id" integer NOT NULL PRIMARY KEY AUTOINCREMENT, "name" varchar(100) NOT NULL, "beschreibung" text NOT NULL);

-- 22. bauwerksdiagnostik_media
CREATE TABLE "bauwerksdiagnostik_media" ("id" integer NOT NULL PRIMARY KEY AUTOINCREMENT, "name" varchar(100) NOT NULL, "beschreibung" text NOT NULL, "local_id" integer NULL, "datei" varchar(100) NOT NULL, "hochgeladen_von" varchar(100) NOT NULL, "hochgeladen_am" datetime NOT NULL, "zuletzt_geaendert_am" datetime NOT NULL, "model_name" varchar(100) NOT NULL, "model_pk" integer NOT NULL, "bauwerk_id" bigint NULL REFERENCES "assetverwaltung_bauwerk" ("id") DEFERRABLE INITIALLY DEFERRED);

-- 23. bauwerksdiagnostik_messdateimedia
CREATE TABLE "bauwerksdiagnostik_messdateimedia" ("media_ptr_id" bigint NOT NULL PRIMARY KEY REFERENCES "bauwerksdiagnostik_media" ("id") DEFERRABLE INITIALLY DEFERRED, "measurement_data" text NULL CHECK ((JSON_VALID("measurement_data") OR "measurement_data" IS NULL)));

-- 24. bauwerksdiagnostik_probe
CREATE TABLE "bauwerksdiagnostik_probe" ("id" integer NOT NULL PRIMARY KEY AUTOINCREMENT, "probenbezeichnung" varchar(100) NOT NULL, "bemerkung" text NULL, "probendurchmesser" real NULL, "probenlaenge" real NULL, "entnahmetiefeVon" real NULL, "entnahmetiefeBis" real NULL, "bauwerk_id_id" bigint NULL REFERENCES "assetverwaltung_bauwerk" ("id") DEFERRABLE INITIALLY DEFERRED, "bohrkern_id" bigint NULL REFERENCES "bauwerksdiagnostik_bohrkern" ("id") DEFERRABLE INITIALLY DEFERRED, "projekt_id_id" bigint NULL REFERENCES "assetverwaltung_projekt" ("id") DEFERRABLE INITIALLY DEFERRED);

-- 25. bauwerksdiagnostik_probe_foto
CREATE TABLE "bauwerksdiagnostik_probe_foto" ("id" integer NOT NULL PRIMARY KEY AUTOINCREMENT, "probe_id" bigint NOT NULL REFERENCES "bauwerksdiagnostik_probe" ("id") DEFERRABLE INITIALLY DEFERRED, "media_id" bigint NOT NULL REFERENCES "bauwerksdiagnostik_media" ("id") DEFERRABLE INITIALLY DEFERRED);

-- 26. bauwerksdiagnostik_probe_material
CREATE TABLE "bauwerksdiagnostik_probe_material" ("id" integer NOT NULL PRIMARY KEY AUTOINCREMENT, "probe_id" bigint NOT NULL REFERENCES "bauwerksdiagnostik_probe" ("id") DEFERRABLE INITIALLY DEFERRED, "material_id" bigint NOT NULL REFERENCES "bauwerksdiagnostik_material" ("id") DEFERRABLE INITIALLY DEFERRED);

-- 27. bauwerksdiagnostik_rissbreitenmessung
CREATE TABLE "bauwerksdiagnostik_rissbreitenmessung" ("id" integer NOT NULL PRIMARY KEY AUTOINCREMENT, "datum" date NOT NULL, "zeit" time NULL, "bemerkung" text NOT NULL, "lufttemperatur" real NULL, "luftfeuchte" real NULL, "abgeschlossen" bool NULL, "sort_order" integer unsigned NOT NULL CHECK ("sort_order" >= 0), "messmittel" varchar(20) NOT NULL, "risse" text NULL CHECK ((JSON_VALID("risse") OR "risse" IS NULL)), "maxRissbreite" real NULL, "monitoringEmpfohlen" bool NULL, "bauwerk_id" bigint NULL REFERENCES "assetverwaltung_bauwerk" ("id") DEFERRABLE INITIALLY DEFERRED, "projekt_id" bigint NULL REFERENCES "assetverwaltung_projekt" ("id") DEFERRABLE INITIALLY DEFERRED, "titelBild_id" bigint NULL REFERENCES "bauwerksdiagnostik_media" ("id") DEFERRABLE INITIALLY DEFERRED, "untersuchungsstelle_id" bigint NULL REFERENCES "bauwerksdiagnostik_untersuchungsstelle" ("id") DEFERRABLE INITIALLY DEFERRED);

-- 28. bauwerksdiagnostik_rissbreitenmessung_bearbeiter
CREATE TABLE "bauwerksdiagnostik_rissbreitenmessung_bearbeiter" ("id" integer NOT NULL PRIMARY KEY AUTOINCREMENT, "rissbreitenmessung_id" bigint NOT NULL REFERENCES "bauwerksdiagnostik_rissbreitenmessung" ("id") DEFERRABLE INITIALLY DEFERRED, "person_id" bigint NOT NULL REFERENCES "assetverwaltung_person" ("id") DEFERRABLE INITIALLY DEFERRED);

-- 29. bauwerksdiagnostik_rissbreitenmessung_bilder
CREATE TABLE "bauwerksdiagnostik_rissbreitenmessung_bilder" ("id" integer NOT NULL PRIMARY KEY AUTOINCREMENT, "rissbreitenmessung_id" bigint NOT NULL REFERENCES "bauwerksdiagnostik_rissbreitenmessung" ("id") DEFERRABLE INITIALLY DEFERRED, "media_id" bigint NOT NULL REFERENCES "bauwerksdiagnostik_media" ("id") DEFERRABLE INITIALLY DEFERRED);

-- 30. bauwerksdiagnostik_rueckprallhammer
CREATE TABLE "bauwerksdiagnostik_rueckprallhammer" ("id" integer NOT NULL PRIMARY KEY AUTOINCREMENT, "datum" date NOT NULL, "zeit" time NULL, "bemerkung" text NOT NULL, "lufttemperatur" real NULL, "luftfeuchte" real NULL, "abgeschlossen" bool NULL, "sort_order" integer unsigned NOT NULL CHECK ("sort_order" >= 0), "hammertyp" varchar(20) NOT NULL, "schlagrichtung" varchar(20) NOT NULL, "oberflaecheGeschliffen" bool NULL, "kalibriertAm" date NULL, "messreihe" text NULL CHECK ((JSON_VALID("messreihe") OR "messreihe" IS NULL)), "medianRueckprallwert" real NULL, "bauwerk_id" bigint NULL REFERENCES "assetverwaltung_bauwerk" ("id") DEFERRABLE INITIALLY DEFERRED, "projekt_id" bigint NULL REFERENCES "assetverwaltung_projekt" ("id") DEFERRABLE INITIALLY DEFERRED, "titelBild_id" bigint NULL REFERENCES "bauwerksdiagnostik_media" ("id") DEFERRABLE INITIALLY DEFERRED, "untersuchungsstelle_id" bigint NULL REFERENCES "bauwerksdiagnostik_untersuchungsstelle" ("id") DEFERRABLE INITIALLY DEFERRED);

-- 31. bauwerksdiagnostik_rueckprallhammer_bearbeiter
CREATE TABLE "bauwerksdiagnostik_rueckprallhammer_bearbeiter" ("id" integer NOT NULL PRIMARY KEY AUTOINCREMENT, "rueckprallhammer_id" bigint NOT NULL REFERENCES "bauwerksdiagnostik_rueckprallhammer" ("id") DEFERRABLE INITIALLY DEFERRED, "person_id" bigint NOT NULL REFERENCES "assetverwaltung_person" ("id") DEFERRABLE INITIALLY DEFERRED);

-- 32. bauwerksdiagnostik_rueckprallhammer_bilder
CREATE TABLE "bauwerksdiagnostik_rueckprallhammer_bilder" ("id" integer NOT NULL PRIMARY KEY AUTOINCREMENT, "rueckprallhammer_id" bigint NOT NULL REFERENCES "bauwerksdiagnostik_rueckprallhammer" ("id") DEFERRABLE INITIALLY DEFERRED, "media_id" bigint NOT NULL REFERENCES "bauwerksdiagnostik_media" ("id") DEFERRABLE INITIALLY DEFERRED);

-- 33. bauwerksdiagnostik_rueckprallhammer_messprotokolle
CREATE TABLE "bauwerksdiagnostik_rueckprallhammer_messprotokolle" ("id" integer NOT NULL PRIMARY KEY AUTOINCREMENT, "rueckprallhammer_id" bigint NOT NULL REFERENCES "bauwerksdiagnostik_rueckprallhammer" ("id") DEFERRABLE INITIALLY DEFERRED, "media_id" bigint NOT NULL REFERENCES "bauwerksdiagnostik_media" ("id") DEFERRABLE INITIALLY DEFERRED);

-- 34. bauwerksdiagnostik_untersuchungsstelle
CREATE TABLE "bauwerksdiagnostik_untersuchungsstelle" ("id" integer NOT NULL PRIMARY KEY AUTOINCREMENT, "name" varchar(100) NOT NULL, "bemerkung" text NOT NULL, "lagenbezugHoehe" varchar(100) NULL, "hoehe" real NULL, "lagenbezugAbstand1" varchar(100) NULL, "abstand1" real NULL, "lagenbezugAbstand2" varchar(100) NULL, "abstand2" real NULL, "bauwerk_id" bigint NOT NULL REFERENCES "assetverwaltung_bauwerk" ("id") DEFERRABLE INITIALLY DEFERRED, "projekt_id" bigint NULL REFERENCES "assetverwaltung_projekt" ("id") DEFERRABLE INITIALLY DEFERRED);

-- 35. bauwerksdiagnostik_untersuchungsstelle_media
CREATE TABLE "bauwerksdiagnostik_untersuchungsstelle_media" ("id" integer NOT NULL PRIMARY KEY AUTOINCREMENT, "untersuchungsstelle_id" bigint NOT NULL REFERENCES "bauwerksdiagnostik_untersuchungsstelle" ("id") DEFERRABLE INITIALLY DEFERRED, "media_id" bigint NOT NULL REFERENCES "bauwerksdiagnostik_media" ("id") DEFERRABLE INITIALLY DEFERRED);

-- Preserve all 89 explicitly declared Django indexes and unique indexes.

CREATE INDEX "assetverwaltung_bauwerk_projekt_bauwerk_id_3486013b" ON "assetverwaltung_bauwerk_projekt" ("bauwerk_id");
CREATE UNIQUE INDEX "assetverwaltung_bauwerk_projekt_bauwerk_id_projekt_id_7c8b1d96_uniq" ON "assetverwaltung_bauwerk_projekt" ("bauwerk_id", "projekt_id");
CREATE INDEX "assetverwaltung_bauwerk_projekt_projekt_id_7814f433" ON "assetverwaltung_bauwerk_projekt" ("projekt_id");
CREATE INDEX "assetverwaltung_person_firma_id_d67ac6d8" ON "assetverwaltung_person" ("firma_id");
CREATE INDEX "bauwerksdiagnostik_bohrkern_fotos_bohrkern_id_f8a09a5f" ON "bauwerksdiagnostik_bohrkern_fotos" ("bohrkern_id");
CREATE UNIQUE INDEX "bauwerksdiagnostik_bohrkern_fotos_bohrkern_id_media_id_5f40bce7_uniq" ON "bauwerksdiagnostik_bohrkern_fotos" ("bohrkern_id", "media_id");
CREATE INDEX "bauwerksdiagnostik_bohrkern_fotos_media_id_407aac9c" ON "bauwerksdiagnostik_bohrkern_fotos" ("media_id");
CREATE INDEX "bauwerksdiagnostik_bohrkern_material_bohrkern_id_d5c9d3de" ON "bauwerksdiagnostik_bohrkern_material" ("bohrkern_id");
CREATE UNIQUE INDEX "bauwerksdiagnostik_bohrkern_material_bohrkern_id_material_id_83767607_uniq" ON "bauwerksdiagnostik_bohrkern_material" ("bohrkern_id", "material_id");
CREATE INDEX "bauwerksdiagnostik_bohrkern_material_material_id_bbb7e685" ON "bauwerksdiagnostik_bohrkern_material" ("material_id");
CREATE INDEX "bauwerksdiagnostik_chloridanalyse_bauwerk_id_719b2797" ON "bauwerksdiagnostik_chloridanalyse" ("bauwerk_id");
CREATE INDEX "bauwerksdiagnostik_chloridanalyse_probe_id_e1cc7f71" ON "bauwerksdiagnostik_chloridanalyse" ("probe_id");
CREATE INDEX "bauwerksdiagnostik_chloridanalyse_projekt_id_fb857096" ON "bauwerksdiagnostik_chloridanalyse" ("projekt_id");
CREATE INDEX "bauwerksdiagnostik_chloridanalyse_sort_order_acd06495" ON "bauwerksdiagnostik_chloridanalyse" ("sort_order");
CREATE INDEX "bauwerksdiagnostik_chloridanalyse_titelBild_id_33832737" ON "bauwerksdiagnostik_chloridanalyse" ("titelBild_id");
CREATE INDEX "bauwerksdiagnostik_chloridanalyse_untersuchungsstelle_id_4f35b2e8" ON "bauwerksdiagnostik_chloridanalyse" ("untersuchungsstelle_id");
CREATE INDEX "bauwerksdiagnostik_chloridanalyse_bearbeiter_chloridanalyse_id_bc5723f7" ON "bauwerksdiagnostik_chloridanalyse_bearbeiter" ("chloridanalyse_id");
CREATE UNIQUE INDEX "bauwerksdiagnostik_chloridanalyse_bearbeiter_chloridanalyse_id_person_id_d72b0008_uniq" ON "bauwerksdiagnostik_chloridanalyse_bearbeiter" ("chloridanalyse_id", "person_id");
CREATE INDEX "bauwerksdiagnostik_chloridanalyse_bearbeiter_person_id_cb3ed08d" ON "bauwerksdiagnostik_chloridanalyse_bearbeiter" ("person_id");
CREATE INDEX "bauwerksdiagnostik_chloridanalyse_bilder_chloridanalyse_id_9329735d" ON "bauwerksdiagnostik_chloridanalyse_bilder" ("chloridanalyse_id");
CREATE UNIQUE INDEX "bauwerksdiagnostik_chloridanalyse_bilder_chloridanalyse_id_media_id_796ddd2d_uniq" ON "bauwerksdiagnostik_chloridanalyse_bilder" ("chloridanalyse_id", "media_id");
CREATE INDEX "bauwerksdiagnostik_chloridanalyse_bilder_media_id_838680a0" ON "bauwerksdiagnostik_chloridanalyse_bilder" ("media_id");
CREATE INDEX "bauwerksdiagnostik_chloridanalyse_pruefberichte_chloridanalyse_id_0f40c6b2" ON "bauwerksdiagnostik_chloridanalyse_pruefberichte" ("chloridanalyse_id");
CREATE UNIQUE INDEX "bauwerksdiagnostik_chloridanalyse_pruefberichte_chloridanalyse_id_media_id_6d5bcb85_uniq" ON "bauwerksdiagnostik_chloridanalyse_pruefberichte" ("chloridanalyse_id", "media_id");
CREATE INDEX "bauwerksdiagnostik_chloridanalyse_pruefberichte_media_id_2f3e8c85" ON "bauwerksdiagnostik_chloridanalyse_pruefberichte" ("media_id");
CREATE INDEX "bauwerksdiagnostik_druckfestigkeitspruefung_bauwerk_id_310c7d2d" ON "bauwerksdiagnostik_druckfestigkeitspruefung" ("bauwerk_id");
CREATE INDEX "bauwerksdiagnostik_druckfestigkeitspruefung_probe_id_4e31ae4e" ON "bauwerksdiagnostik_druckfestigkeitspruefung" ("probe_id");
CREATE INDEX "bauwerksdiagnostik_druckfestigkeitspruefung_projekt_id_3a214586" ON "bauwerksdiagnostik_druckfestigkeitspruefung" ("projekt_id");
CREATE INDEX "bauwerksdiagnostik_druckfestigkeitspruefung_pruefbericht_id_1f7282a9" ON "bauwerksdiagnostik_druckfestigkeitspruefung" ("pruefbericht_id");
CREATE INDEX "bauwerksdiagnostik_druckfestigkeitspruefung_sort_order_2ac17770" ON "bauwerksdiagnostik_druckfestigkeitspruefung" ("sort_order");
CREATE INDEX "bauwerksdiagnostik_druckfestigkeitspruefung_titelBild_id_16280904" ON "bauwerksdiagnostik_druckfestigkeitspruefung" ("titelBild_id");
CREATE INDEX "bauwerksdiagnostik_druckfestigkeitspruefung_untersuchungsstelle_id_0188cf87" ON "bauwerksdiagnostik_druckfestigkeitspruefung" ("untersuchungsstelle_id");
CREATE INDEX "bauwerksdiagnostik_druckfestigkeitspruefung_bearbeiter_druckfestigkeitspruefung_id_7c81de4b" ON "bauwerksdiagnostik_druckfestigkeitspruefung_bearbeiter" ("druckfestigkeitspruefung_id");
CREATE UNIQUE INDEX "bauwerksdiagnostik_druckfestigkeitspruefung_bearbeiter_druckfestigkeitspruefung_id_person_id_e9768331_uniq" ON "bauwerksdiagnostik_druckfestigkeitspruefung_bearbeiter" ("druckfestigkeitspruefung_id", "person_id");
CREATE INDEX "bauwerksdiagnostik_druckfestigkeitspruefung_bearbeiter_person_id_042690c6" ON "bauwerksdiagnostik_druckfestigkeitspruefung_bearbeiter" ("person_id");
CREATE INDEX "bauwerksdiagnostik_druckfestigkeitspruefung_bilder_druckfestigkeitspruefung_id_7892b34d" ON "bauwerksdiagnostik_druckfestigkeitspruefung_bilder" ("druckfestigkeitspruefung_id");
CREATE UNIQUE INDEX "bauwerksdiagnostik_druckfestigkeitspruefung_bilder_druckfestigkeitspruefung_id_media_id_e2e83518_uniq" ON "bauwerksdiagnostik_druckfestigkeitspruefung_bilder" ("druckfestigkeitspruefung_id", "media_id");
CREATE INDEX "bauwerksdiagnostik_druckfestigkeitspruefung_bilder_media_id_e89522b6" ON "bauwerksdiagnostik_druckfestigkeitspruefung_bilder" ("media_id");
CREATE INDEX "bauwerksdiagnostik_kernbohrung_bauwerk_id_40150ecb" ON "bauwerksdiagnostik_kernbohrung" ("bauwerk_id");
CREATE INDEX "bauwerksdiagnostik_kernbohrung_projekt_id_6ed1c7c8" ON "bauwerksdiagnostik_kernbohrung" ("projekt_id");
CREATE INDEX "bauwerksdiagnostik_kernbohrung_sort_order_eae2db6d" ON "bauwerksdiagnostik_kernbohrung" ("sort_order");
CREATE INDEX "bauwerksdiagnostik_kernbohrung_titelBild_id_917396cd" ON "bauwerksdiagnostik_kernbohrung" ("titelBild_id");
CREATE INDEX "bauwerksdiagnostik_kernbohrung_untersuchungsstelle_id_7bfae308" ON "bauwerksdiagnostik_kernbohrung" ("untersuchungsstelle_id");
CREATE INDEX "bauwerksdiagnostik_kernbohrung_bearbeiter_kernbohrung_id_64009941" ON "bauwerksdiagnostik_kernbohrung_bearbeiter" ("kernbohrung_id");
CREATE UNIQUE INDEX "bauwerksdiagnostik_kernbohrung_bearbeiter_kernbohrung_id_person_id_c2d8fbaf_uniq" ON "bauwerksdiagnostik_kernbohrung_bearbeiter" ("kernbohrung_id", "person_id");
CREATE INDEX "bauwerksdiagnostik_kernbohrung_bearbeiter_person_id_8af3e131" ON "bauwerksdiagnostik_kernbohrung_bearbeiter" ("person_id");
CREATE INDEX "bauwerksdiagnostik_kernbohrung_bilder_kernbohrung_id_e51ead93" ON "bauwerksdiagnostik_kernbohrung_bilder" ("kernbohrung_id");
CREATE UNIQUE INDEX "bauwerksdiagnostik_kernbohrung_bilder_kernbohrung_id_media_id_e57b6d37_uniq" ON "bauwerksdiagnostik_kernbohrung_bilder" ("kernbohrung_id", "media_id");
CREATE INDEX "bauwerksdiagnostik_kernbohrung_bilder_media_id_595d9535" ON "bauwerksdiagnostik_kernbohrung_bilder" ("media_id");
CREATE INDEX "bauwerksdiagnostik_media_bauwerk_id_8d174688" ON "bauwerksdiagnostik_media" ("bauwerk_id");
CREATE INDEX "bauwerksdiagnostik_probe_bauwerk_id_id_10509232" ON "bauwerksdiagnostik_probe" ("bauwerk_id_id");
CREATE INDEX "bauwerksdiagnostik_probe_bohrkern_id_605416e7" ON "bauwerksdiagnostik_probe" ("bohrkern_id");
CREATE INDEX "bauwerksdiagnostik_probe_projekt_id_id_9172b5b5" ON "bauwerksdiagnostik_probe" ("projekt_id_id");
CREATE INDEX "bauwerksdiagnostik_probe_foto_media_id_f59dc954" ON "bauwerksdiagnostik_probe_foto" ("media_id");
CREATE INDEX "bauwerksdiagnostik_probe_foto_probe_id_8cc4307e" ON "bauwerksdiagnostik_probe_foto" ("probe_id");
CREATE UNIQUE INDEX "bauwerksdiagnostik_probe_foto_probe_id_media_id_e0742de5_uniq" ON "bauwerksdiagnostik_probe_foto" ("probe_id", "media_id");
CREATE INDEX "bauwerksdiagnostik_probe_material_material_id_49fef604" ON "bauwerksdiagnostik_probe_material" ("material_id");
CREATE INDEX "bauwerksdiagnostik_probe_material_probe_id_1a22cbe7" ON "bauwerksdiagnostik_probe_material" ("probe_id");
CREATE UNIQUE INDEX "bauwerksdiagnostik_probe_material_probe_id_material_id_7b10be36_uniq" ON "bauwerksdiagnostik_probe_material" ("probe_id", "material_id");
CREATE INDEX "bauwerksdiagnostik_rissbreitenmessung_bauwerk_id_7a4d880f" ON "bauwerksdiagnostik_rissbreitenmessung" ("bauwerk_id");
CREATE INDEX "bauwerksdiagnostik_rissbreitenmessung_projekt_id_f49263f1" ON "bauwerksdiagnostik_rissbreitenmessung" ("projekt_id");
CREATE INDEX "bauwerksdiagnostik_rissbreitenmessung_sort_order_18569149" ON "bauwerksdiagnostik_rissbreitenmessung" ("sort_order");
CREATE INDEX "bauwerksdiagnostik_rissbreitenmessung_titelBild_id_10438d7c" ON "bauwerksdiagnostik_rissbreitenmessung" ("titelBild_id");
CREATE INDEX "bauwerksdiagnostik_rissbreitenmessung_untersuchungsstelle_id_29011fa7" ON "bauwerksdiagnostik_rissbreitenmessung" ("untersuchungsstelle_id");
CREATE INDEX "bauwerksdiagnostik_rissbreitenmessung_bearbeiter_person_id_fd5d6e11" ON "bauwerksdiagnostik_rissbreitenmessung_bearbeiter" ("person_id");
CREATE INDEX "bauwerksdiagnostik_rissbreitenmessung_bearbeiter_rissbreitenmessung_id_1d4623e6" ON "bauwerksdiagnostik_rissbreitenmessung_bearbeiter" ("rissbreitenmessung_id");
CREATE UNIQUE INDEX "bauwerksdiagnostik_rissbreitenmessung_bearbeiter_rissbreitenmessung_id_person_id_4e31a271_uniq" ON "bauwerksdiagnostik_rissbreitenmessung_bearbeiter" ("rissbreitenmessung_id", "person_id");
CREATE INDEX "bauwerksdiagnostik_rissbreitenmessung_bilder_media_id_1523d732" ON "bauwerksdiagnostik_rissbreitenmessung_bilder" ("media_id");
CREATE INDEX "bauwerksdiagnostik_rissbreitenmessung_bilder_rissbreitenmessung_id_bc2c7559" ON "bauwerksdiagnostik_rissbreitenmessung_bilder" ("rissbreitenmessung_id");
CREATE UNIQUE INDEX "bauwerksdiagnostik_rissbreitenmessung_bilder_rissbreitenmessung_id_media_id_5244c622_uniq" ON "bauwerksdiagnostik_rissbreitenmessung_bilder" ("rissbreitenmessung_id", "media_id");
CREATE INDEX "bauwerksdiagnostik_rueckprallhammer_bauwerk_id_cc92f2e7" ON "bauwerksdiagnostik_rueckprallhammer" ("bauwerk_id");
CREATE INDEX "bauwerksdiagnostik_rueckprallhammer_projekt_id_da6b8104" ON "bauwerksdiagnostik_rueckprallhammer" ("projekt_id");
CREATE INDEX "bauwerksdiagnostik_rueckprallhammer_sort_order_a028f9d1" ON "bauwerksdiagnostik_rueckprallhammer" ("sort_order");
CREATE INDEX "bauwerksdiagnostik_rueckprallhammer_titelBild_id_f8435c7c" ON "bauwerksdiagnostik_rueckprallhammer" ("titelBild_id");
CREATE INDEX "bauwerksdiagnostik_rueckprallhammer_untersuchungsstelle_id_6c7f5c4a" ON "bauwerksdiagnostik_rueckprallhammer" ("untersuchungsstelle_id");
CREATE INDEX "bauwerksdiagnostik_rueckprallhammer_bearbeiter_person_id_b5aeb600" ON "bauwerksdiagnostik_rueckprallhammer_bearbeiter" ("person_id");
CREATE INDEX "bauwerksdiagnostik_rueckprallhammer_bearbeiter_rueckprallhammer_id_1e4edd98" ON "bauwerksdiagnostik_rueckprallhammer_bearbeiter" ("rueckprallhammer_id");
CREATE UNIQUE INDEX "bauwerksdiagnostik_rueckprallhammer_bearbeiter_rueckprallhammer_id_person_id_d0c7bac9_uniq" ON "bauwerksdiagnostik_rueckprallhammer_bearbeiter" ("rueckprallhammer_id", "person_id");
CREATE INDEX "bauwerksdiagnostik_rueckprallhammer_bilder_media_id_8b6e004f" ON "bauwerksdiagnostik_rueckprallhammer_bilder" ("media_id");
CREATE INDEX "bauwerksdiagnostik_rueckprallhammer_bilder_rueckprallhammer_id_096ae3e4" ON "bauwerksdiagnostik_rueckprallhammer_bilder" ("rueckprallhammer_id");
CREATE UNIQUE INDEX "bauwerksdiagnostik_rueckprallhammer_bilder_rueckprallhammer_id_media_id_3a677e27_uniq" ON "bauwerksdiagnostik_rueckprallhammer_bilder" ("rueckprallhammer_id", "media_id");
CREATE INDEX "bauwerksdiagnostik_rueckprallhammer_messprotokolle_media_id_94a7a28a" ON "bauwerksdiagnostik_rueckprallhammer_messprotokolle" ("media_id");
CREATE INDEX "bauwerksdiagnostik_rueckprallhammer_messprotokolle_rueckprallhammer_id_b98708db" ON "bauwerksdiagnostik_rueckprallhammer_messprotokolle" ("rueckprallhammer_id");
CREATE UNIQUE INDEX "bauwerksdiagnostik_rueckprallhammer_messprotokolle_rueckprallhammer_id_media_id_b5aa93ba_uniq" ON "bauwerksdiagnostik_rueckprallhammer_messprotokolle" ("rueckprallhammer_id", "media_id");
CREATE INDEX "bauwerksdiagnostik_untersuchungsstelle_bauwerk_id_0be187e7" ON "bauwerksdiagnostik_untersuchungsstelle" ("bauwerk_id");
CREATE INDEX "bauwerksdiagnostik_untersuchungsstelle_projekt_id_5297f9a7" ON "bauwerksdiagnostik_untersuchungsstelle" ("projekt_id");
CREATE INDEX "bauwerksdiagnostik_untersuchungsstelle_media_media_id_0c7f2697" ON "bauwerksdiagnostik_untersuchungsstelle_media" ("media_id");
CREATE INDEX "bauwerksdiagnostik_untersuchungsstelle_media_untersuchungsstelle_id_05574a1d" ON "bauwerksdiagnostik_untersuchungsstelle_media" ("untersuchungsstelle_id");
CREATE UNIQUE INDEX "bauwerksdiagnostik_untersuchungsstelle_media_untersuchungsstelle_id_media_id_6671329a_uniq" ON "bauwerksdiagnostik_untersuchungsstelle_media" ("untersuchungsstelle_id", "media_id");
