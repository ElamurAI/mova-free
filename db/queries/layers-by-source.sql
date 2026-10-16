-- Paths are relative to the data directory ($MOVA_DATA): run duckdb from there.
-- Layer statistics by source: sets, sentences, documents, rows and words; the share of sets that
-- passed the `en` gates (tree, closed lists, UD 2.18 registry).
--   duckdb -readonly $MOVA_DATA/db/mova.duckdb < db/queries/layers-by-source.sql
SELECT a.layer AS "layer", a.source AS "source", a.tool AS "who", a.dialect AS "dialect",
       count(*) AS "sets", count(DISTINCT a.sid) AS "sentences", count(DISTINCT a.doc) AS "documents",
       sum(t.rows) AS "rows", sum(t.words) AS "words",
       round(100.0 * count(*) FILTER (WHERE a.gate = 'ok') / count(*), 1) AS "gates ok, %"
FROM annsets a
LEFT JOIN (SELECT aid, count(*) AS rows, count(*) FILTER (WHERE id NOT LIKE '%-%' AND id NOT LIKE '%.%') AS words
           FROM tokens GROUP BY aid) t ON t.aid = a.aid
GROUP BY ALL
ORDER BY "layer", "sets" DESC;
