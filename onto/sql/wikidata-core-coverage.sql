-- Paths are relative to the data directory ($MOVA_DATA): run duckdb from there.
-- Coverage of the "compiled core" by Wikidata data: fable and tale classes (bridge `onto bridge` → OEWN synset)
-- → Q (synset_q, accepted) → which core fields Wikidata already provides. Input — core-classes.tsv (class, type, entities,
-- synset) from a run; read-only on the database:
--   ~/bin/duckdb -readonly $MOVA_DATA/db/wikidata.duckdb < onto/sql/wikidata-core-coverage.sql
.mode markdown
CREATE TEMP TABLE core AS
SELECT * FROM read_csv('runs/wikidata-2026-09-26/core-classes.tsv', delim = '\t', header = false,
    auto_detect = false, quote = '', escape = '', columns = {'cls': 'VARCHAR', 'typ': 'VARCHAR', 'n': 'INTEGER', 'synset': 'VARCHAR'});

CREATE TEMP TABLE m AS
SELECT c.cls, split_part(c.typ, ' ', 2) AS typ, c.synset, s.q, s.sources
FROM core c LEFT JOIN synset_q s ON s.synset = c.synset AND s.status = 'accepted';

CREATE TEMP TABLE f AS
SELECT m.*,
    EXISTS (SELECT 1 FROM labels l WHERE l.q = m.q AND l.lang = 'uk' AND NOT l.is_alias) AS uk,
    (SELECT count(DISTINCT lang) FROM labels l WHERE l.q = m.q) AS langs,
    EXISTS (SELECT 1 FROM claims c WHERE c.p = 4733 AND c.s = m.q AND c.rank > 0) AS sound,
    EXISTS (SELECT 1 FROM claims c WHERE c.p = 1034 AND c.s = m.q AND c.rank > 0) AS food,
    EXISTS (SELECT 1 FROM quant x WHERE x.q = m.q AND x.p IN (2067, 2043, 2048)) AS size,
    EXISTS (SELECT 1 FROM claims c WHERE c.p = 2974 AND c.s = m.q AND c.rank > 0) AS habitat,
    EXISTS (SELECT 1 FROM claims c WHERE c.p = 462 AND c.s = m.q AND c.rank > 0) AS color,
    EXISTS (SELECT 1 FROM claims c WHERE c.p IN (527, 2670) AND c.s = m.q AND c.rank > 0) AS parts,
    EXISTS (SELECT 1 FROM claims c WHERE c.p = 366 AND c.s = m.q AND c.rank > 0) AS use_,
    EXISTS (SELECT 1 FROM claims c WHERE c.p = 186 AND c.s = m.q AND c.rank > 0) AS material,
    EXISTS (SELECT 1 FROM sense_items si JOIN lexemes x USING (l) WHERE si.p = 5137 AND si.q = m.q AND x.lang = 8798) AS uk_lexeme
FROM m;

SELECT typ AS type, count(*) AS n_classes, count(q) AS with_q, count(*) FILTER (WHERE uk) AS label_uk,
       count(*) FILTER (WHERE uk_lexeme) AS lexeme_uk, count(*) FILTER (WHERE sound) AS n_sound,
       count(*) FILTER (WHERE food) AS n_food, count(*) FILTER (WHERE size) AS n_size, count(*) FILTER (WHERE habitat) AS n_habitat,
       count(*) FILTER (WHERE color) AS n_color, count(*) FILTER (WHERE parts) AS n_parts, count(*) FILTER (WHERE use_) AS n_use,
       count(*) FILTER (WHERE material) AS n_material
FROM f GROUP BY ROLLUP (typ) ORDER BY n_classes DESC;

-- animals: what is already there
SELECT cls, synset, q, wd_label(q, 'en') AS en, wd_label(q, 'uk') AS uk, sources, sound, food, size, habitat, color
FROM f WHERE typ = 'animal' ORDER BY cls;
