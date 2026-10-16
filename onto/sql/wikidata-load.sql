-- Paths are relative to the data directory ($MOVA_DATA): run duckdb from there.
-- Wikidata → DuckDB: a separate database $MOVA_DATA/db/wikidata.duckdb (the main mova.duckdb is not touched; ATTACH it there).
-- Source — derived TSVs (zstd) from `onto wd-extract`, `onto wd-lex`, `onto wd-closure`, `onto wd-align`.
-- Run from the derivatives directory (paths are relative):
--   cd $MOVA_DATA/raw/wikidata-derived && ~/bin/duckdb $MOVA_DATA/db/wikidata.duckdb < <repo>/onto/sql/wikidata-load.sql
-- TSV fields without quotes and without escaping; an empty field is NULL. Q, P, L numbers are integers without the letter.
-- items, labels, sitelinks are not sorted: the files are already in dump order (pageid ≈ Q), and a full sort of 121 M
-- wide rows does not fit in 20 GB (Out of Memory 26.09) — the approximate order is enough for zone maps.

SET threads = 6;
SET memory_limit = '28GB';
SET preserve_insertion_order = false;
SET temp_directory = 'db/tmp-wikidata';

-- ── statements: an item as the value ─────────────────────────────────────────────────────────────
-- s — item, p — property (31 instance of, 279 subclass of, 171 parent taxon, …), o — value item;
-- rank: 0 deprecated, 1 normal, 2 preferred. Sorted by (p, s): selecting a property and an item uses zone maps.
CREATE OR REPLACE TABLE claims AS
SELECT * FROM read_csv('claims-w*.tsv.zst', delim = '\t', header = false, quote = '', escape = '', auto_detect = false,
    columns = {'s': 'INTEGER', 'p': 'SMALLINT', 'o': 'INTEGER', 'rank': 'UTINYINT'})
ORDER BY p, s, o;

-- ── items ───────────────────────────────────────────────────────────────────────────────────
-- flags: 1 has P279 (class as subject), 2 taxon, 4 wiki maintenance page, 8 labels in all languages in `labels`,
--        16 scholarly article, 32 has P31. is_class — takes part in P279 as subject or value.
CREATE OR REPLACE TABLE items AS
WITH obj AS (SELECT DISTINCT o AS q FROM claims WHERE p = 279 AND rank > 0)
-- Scholarly articles (flags & 16, P31 = Q13442814; 46 M): without titles and descriptions — the world model and translation do not
-- need them, and that is over 2 GB of the database (budget 15 GB); the texts stay in the derived items-w*.tsv.zst.
SELECT i.q,
       CASE WHEN i.flags & 16 = 0 THEN i.label_en END AS label_en,
       CASE WHEN i.flags & 16 = 0 THEN i.label_mul END AS label_mul,
       CASE WHEN i.flags & 16 = 0 THEN i.label_uk END AS label_uk,
       CASE WHEN i.flags & 16 = 0 THEN i.desc_en END AS desc_en,
       CASE WHEN i.flags & 16 = 0 THEN i.desc_uk END AS desc_uk,
       i.n_sitelinks, i.n_wiki, i.n_labels, i.flags,
       (i.flags & 1 > 0 OR obj.q IS NOT NULL) AS is_class,
       (i.flags & 2 > 0) AS is_taxon
FROM read_csv('items-w*.tsv.zst', delim = '\t', header = false, quote = '', escape = '', auto_detect = false,
    columns = {'q': 'INTEGER', 'label_en': 'VARCHAR', 'label_mul': 'VARCHAR', 'label_uk': 'VARCHAR',
               'desc_en': 'VARCHAR', 'desc_uk': 'VARCHAR', 'n_sitelinks': 'SMALLINT', 'n_wiki': 'SMALLINT',
               'n_labels': 'SMALLINT', 'flags': 'UTINYINT'}) i
LEFT JOIN obj USING (q);

-- ── labels and aliases in all languages (classes, items with sitelinks, taxa; without maintenance pages) ─
CREATE OR REPLACE TABLE labels AS
SELECT * FROM read_csv('labels-w*.tsv.zst', delim = '\t', header = false, quote = '', escape = '', auto_detect = false,
    columns = {'q': 'INTEGER', 'lang': 'VARCHAR', 'label': 'VARCHAR', 'is_alias': 'BOOLEAN'});

-- ── Wikipedia article titles in all languages (only *wiki Wikipedias; parenthesised disambiguations kept) ─────
CREATE OR REPLACE TABLE sitelinks AS
SELECT q, site, regexp_replace(site, 'wiki$', '') AS lang, title
FROM read_csv('sitelinks-w*.tsv.zst', delim = '\t', header = false, quote = '', escape = '', auto_detect = false,
    columns = {'q': 'INTEGER', 'site': 'VARCHAR', 'title': 'VARCHAR'});

-- ── other values: strings (P225 taxon name, P2888/P1709 URL, P5063 ILI, P8814 synset), texts with a language
--    (P1843 taxon common name), quantities (mass, length, height, speed, life expectancy) ─────
CREATE OR REPLACE TABLE strs AS
SELECT * FROM read_csv('strs-w*.tsv.zst', delim = '\t', header = false, quote = '', escape = '', auto_detect = false,
    columns = {'q': 'INTEGER', 'p': 'SMALLINT', 'value': 'VARCHAR', 'rank': 'UTINYINT'})
ORDER BY p, q;

CREATE OR REPLACE TABLE mono AS
SELECT * FROM read_csv('mono-w*.tsv.zst', delim = '\t', header = false, quote = '', escape = '', auto_detect = false,
    columns = {'q': 'INTEGER', 'p': 'SMALLINT', 'lang': 'VARCHAR', 'text': 'VARCHAR', 'rank': 'UTINYINT'})
ORDER BY p, q, lang;

-- amount/lower/upper — in units of unit (the unit's Q; 0 — dimensionless); quals — qualifiers `P21=Q…;P3831=Q…`
CREATE OR REPLACE TABLE quant AS
SELECT * FROM read_csv('quant-w*.tsv.zst', delim = '\t', header = false, quote = '', escape = '', auto_detect = false,
    columns = {'q': 'INTEGER', 'p': 'SMALLINT', 'amount': 'DOUBLE', 'unit': 'INTEGER', 'lower': 'DOUBLE',
               'upper': 'DOUBLE', 'rank': 'UTINYINT', 'quals': 'VARCHAR'})
ORDER BY p, q;

CREATE OR REPLACE TABLE props AS
SELECT * FROM read_csv('props-w*.tsv.zst', delim = '\t', header = false, quote = '', escape = '', auto_detect = false,
    columns = {'p': 'INTEGER', 'datatype': 'VARCHAR', 'label_en': 'VARCHAR', 'label_uk': 'VARCHAR', 'desc_en': 'VARCHAR'})
ORDER BY p;

-- ── "is a kind of" hierarchy: P279 + P171 without cycle edges (gate `onto wd-closure`), cycles separately ─────────
CREATE OR REPLACE TABLE isa_edges AS
SELECT * FROM read_csv('isa_edges.tsv.zst', delim = '\t', header = false, quote = '', escape = '', auto_detect = false,
    columns = {'s': 'INTEGER', 'o': 'INTEGER', 'p': 'SMALLINT'})
ORDER BY s, o;

CREATE OR REPLACE TABLE isa_cycles AS
SELECT * FROM read_csv('isa_cycles.tsv', delim = '\t', header = false, quote = '', escape = '', auto_detect = false, skip = 1,
    columns = {'cycle': 'INTEGER', 'size': 'INTEGER', 'q': 'INTEGER', 'label': 'VARCHAR'});

CREATE OR REPLACE TABLE isa_cycle_edges AS
SELECT * FROM read_csv('isa_cycle_edges.tsv', delim = '\t', header = false, quote = '', escape = '', auto_detect = false, skip = 1,
    columns = {'cycle': 'INTEGER', 's': 'INTEGER', 'o': 'INTEGER', 'p': 'SMALLINT'});

-- ── lexemes (dump 2026-09-23) ──────────────────────────────────────────────────────────────────
CREATE OR REPLACE TABLE lexemes AS
SELECT * FROM read_csv('lex-lexemes.tsv.zst', delim = '\t', header = false, quote = '', escape = '', auto_detect = false,
    columns = {'l': 'INTEGER', 'lang': 'INTEGER', 'cat': 'INTEGER', 'lemma': 'VARCHAR', 'lemma_code': 'VARCHAR',
               'n_forms': 'SMALLINT', 'n_senses': 'SMALLINT'})
ORDER BY l;

CREATE OR REPLACE TABLE lemmas AS
SELECT * FROM read_csv('lex-lemmas.tsv.zst', delim = '\t', header = false, quote = '', escape = '', auto_detect = false,
    columns = {'l': 'INTEGER', 'code': 'VARCHAR', 'lemma': 'VARCHAR'})
ORDER BY l, code;

-- feats — grammatical features of a form (Q: case, number, tense…)
CREATE OR REPLACE TABLE forms AS
SELECT l, f, code, rep,
       CASE WHEN feats IS NULL OR feats = '' THEN []::INTEGER[] ELSE list_transform(string_split(feats, ','), x -> x::INTEGER) END AS feats
FROM read_csv('lex-forms.tsv.zst', delim = '\t', header = false, quote = '', escape = '', auto_detect = false,
    columns = {'l': 'INTEGER', 'f': 'INTEGER', 'code': 'VARCHAR', 'rep': 'VARCHAR', 'feats': 'VARCHAR'})
ORDER BY l, f;

CREATE OR REPLACE TABLE glosses AS
SELECT * FROM read_csv('lex-glosses.tsv.zst', delim = '\t', header = false, quote = '', escape = '', auto_detect = false,
    columns = {'l': 'INTEGER', 's': 'INTEGER', 'code': 'VARCHAR', 'gloss': 'VARCHAR'})
ORDER BY l, s, code;

-- sense → item: P5137 "item for this sense" and other sense properties with an item
CREATE OR REPLACE TABLE sense_items AS
SELECT * FROM read_csv('lex-sense_items.tsv.zst', delim = '\t', header = false, quote = '', escape = '', auto_detect = false,
    columns = {'l': 'INTEGER', 's': 'INTEGER', 'p': 'INTEGER', 'q': 'INTEGER', 'rank': 'UTINYINT'})
ORDER BY p, l, s;

-- sense → sense: P5972 translation, P5973 synonym, P5974 antonym etc.
CREATE OR REPLACE TABLE sense_links AS
SELECT * FROM read_csv('lex-sense_links.tsv.zst', delim = '\t', header = false, quote = '', escape = '', auto_detect = false,
    columns = {'l': 'INTEGER', 's': 'INTEGER', 'p': 'INTEGER', 'l2': 'INTEGER', 's2': 'INTEGER', 'rank': 'UTINYINT'})
ORDER BY p, l, s;

CREATE OR REPLACE TABLE sense_strs AS
SELECT * FROM read_csv('lex-sense_strs.tsv.zst', delim = '\t', header = false, quote = '', escape = '', auto_detect = false,
    columns = {'l': 'INTEGER', 's': 'INTEGER', 'p': 'INTEGER', 'value': 'VARCHAR', 'rank': 'UTINYINT'})
ORDER BY p, l, s;

CREATE OR REPLACE TABLE lex_claims AS
SELECT * FROM read_csv('lex-lex_claims.tsv.zst', delim = '\t', header = false, quote = '', escape = '', auto_detect = false,
    columns = {'l': 'INTEGER', 'p': 'INTEGER', 'q': 'INTEGER', 'rank': 'UTINYINT'})
ORDER BY p, l;

-- ── OEWN synset ↔ Q alignment (`onto wd-align`) ───────────────────────────────────────────────
-- status: accepted (the synset's best Q with ancestor support), also (also supported), unsupported (no support)
CREATE OR REPLACE TABLE synset_q AS
SELECT * FROM read_csv('synset_q.tsv', delim = '\t', header = false, quote = '', escape = '', auto_detect = false,
    columns = {'synset': 'VARCHAR', 'q': 'INTEGER', 'status': 'VARCHAR', 'sources': 'VARCHAR',
               'support_synset': 'VARCHAR', 'support_q': 'INTEGER', 'ds': 'UTINYINT', 'dq': 'UTINYINT'})
ORDER BY synset, status, q;

-- ── macros ────────────────────────────────────────────────────────────────────────────────────
-- label in a language with fallbacks: language → mul → en
CREATE OR REPLACE MACRO wd_label(x, lng) AS coalesce(
    (SELECT min(label) FROM labels WHERE q = x AND lang = lng AND NOT is_alias),
    (SELECT min(label) FROM labels WHERE q = x AND lang = 'mul' AND NOT is_alias),
    (SELECT label_en FROM items WHERE q = x),
    (SELECT label_mul FROM items WHERE q = x));

-- ancestors via P279 + P171 (shortest depth); up to 64 steps
CREATE OR REPLACE MACRO wd_ancestors(x) AS TABLE
    WITH RECURSIVE a(q, depth) AS (
        SELECT x::INTEGER, 0
        UNION
        SELECT e.o, a.depth + 1 FROM a JOIN isa_edges e ON e.s = a.q WHERE a.depth < 64
    )
    SELECT q AS anc, min(depth) AS depth FROM a WHERE q <> x GROUP BY q;

-- descendants via P279 + P171 (subclasses and subtaxa)
CREATE OR REPLACE MACRO wd_descendants(x) AS TABLE
    WITH RECURSIVE d(q, depth) AS (
        SELECT x::INTEGER, 0
        UNION
        SELECT e.s, d.depth + 1 FROM d JOIN isa_edges e ON e.o = d.q WHERE d.depth < 64
    )
    SELECT q AS descendant, min(depth) AS depth FROM d WHERE q <> x GROUP BY q;

-- whether x is a kind of y (P279/P171, any depth)
CREATE OR REPLACE MACRO wd_isa(x, y) AS EXISTS (SELECT 1 FROM wd_ancestors(x) WHERE anc = y);

CHECKPOINT;
