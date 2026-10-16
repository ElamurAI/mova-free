-- Paths are relative to the data directory ($MOVA_DATA): run duckdb from there.
-- HS22 (onto hs labels): labels of main item concepts in other languages from Wikidata (CC0), read-only database access.
-- Input: hs-q.txt (Q numbers, one per line) in the current directory; output: hs-labels.tsv (q, lang, label, is_alias).
-- Run: (cd <run> && ~/bin/duckdb -readonly $MOVA_DATA/db/wikidata.duckdb < <repo>/onto/sql/hs-labels.sql)
COPY (
  SELECT l.q, l.lang, l.label, l.is_alias
  FROM labels l
  JOIN read_csv('hs-q.txt', columns = {'q': 'INTEGER'}, header = false) h ON h.q = l.q
  WHERE l.lang IN ('uk', 'de', 'fr', 'es', 'pl', 'ru', 'it', 'nl')
  ORDER BY l.q, l.lang, l.is_alias, l.label
) TO 'hs-labels.tsv' (DELIMITER '\t', HEADER false);
