-- Pragmatics in the database: LLM silver vs SLM (cross-validated) predictions — match rate per field for each
-- model, and how many sentences are in each set. `db sql data/db/mova.duckdb prag/sql/silver-vs-rust.sql`
WITH s AS (SELECT * FROM prag_labels WHERE source = 'silver'),
     r AS (SELECT * FROM prag_labels WHERE source = 'rust')
SELECT r.origin AS model, count(*) AS n,
       round(100 * avg((s.form = r.form)::INT), 1) AS form,
       round(100 * avg((s.act = r.act)::INT), 1) AS act,
       round(100 * avg((s.indirect = r.indirect)::INT), 1) AS indirect,
       round(100 * avg((s.hedge = r.hedge)::INT), 1) AS hedge,
       round(100 * avg((s.polarity = r.polarity)::INT), 1) AS polarity,
       round(100 * avg((s.voice = r.voice)::INT), 1) AS voice
FROM s JOIN r USING (sid)
GROUP BY r.origin
ORDER BY r.origin;

-- Indirect requests from silver and what the perceptron says about them (with the most weighty act feature).
SELECT s.sent_id, left(s.text, 70) AS text, s.act AS silver_act, r.act AS rust_act, r.indirect AS rust_indirect,
       r.trace->'act'->'why'->0 AS top_feature, s.means
FROM prag_labels s JOIN prag_labels r USING (sid)
WHERE s.source = 'silver' AND r.origin = 'prag-2026-09-26/perceptron' AND s.act = 'request' AND s.indirect
ORDER BY s.sent_id
LIMIT 8;
