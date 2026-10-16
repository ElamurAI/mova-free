-- Paths are relative to the data directory ($MOVA_DATA): run duckdb from there.
-- Closure of P279 + P171 (if `onto wd-closure` materialized it): q — node, anc — ancestor, depth — shortest
-- depth. Run from the derived-data directory after wikidata-load.sql.
SET threads = 6;
SET memory_limit = '28GB';
SET preserve_insertion_order = false;
SET temp_directory = 'db/tmp-wikidata';
CREATE OR REPLACE TABLE closure AS
SELECT * FROM read_csv('closure-t*.tsv.zst', delim = '\t', header = false, quote = '', escape = '', auto_detect = false,
    columns = {'q': 'INTEGER', 'anc': 'INTEGER', 'depth': 'UTINYINT'});
CHECKPOINT;
