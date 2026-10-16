-- Paths are relative to the data directory ($MOVA_DATA): run duckdb from there.
-- Sentences where Rust and Opus disagree. Pairs of ud sets on the same sentence: Rust (tool = 'rust':
-- rust-b of calibration runs, bronze) versus Opus's final annotation from the same run (final)
-- or silver versus bronze. Only pairs with the same words.
--   duckdb -readonly $MOVA_DATA/db/mova.duckdb < db/queries/rust-vs-opus.sql
WITH r AS (SELECT aid, sid, run FROM annsets WHERE layer = 'ud' AND tool = 'rust'),
     o AS (SELECT aid, sid, run, source, origin FROM annsets WHERE layer = 'ud' AND tool = 'opus' AND origin NOT LIKE '%/opus-a'),
     pairs AS (SELECT r.sid, r.aid AS ra, o.aid AS oa, o.origin FROM r JOIN o ON o.sid = r.sid AND (o.run = r.run OR o.source = 'silver')),
     cmp AS (
         SELECT p.sid, p.origin, count(*) AS words,
                count(*) FILTER (WHERE wr.h IS DISTINCT FROM wo.h OR wr.deprel <> wo.deprel) AS tree_diff,
                count(*) FILTER (WHERE wr.upos <> wo.upos OR wr.xpos <> wo.xpos OR wr.feats <> wo.feats OR lower(wr.lemma) <> lower(wo.lemma)) AS tag_diff,
                bool_and(wr.form = wo.form) AS same_forms,
                string_agg(CASE WHEN wr.h IS DISTINCT FROM wo.h OR wr.deprel <> wo.deprel
                                THEN wr.form || ' ' || wr.h || ':' || wr.deprel || '→' || wo.h || ':' || wo.deprel END, '; ' ORDER BY wr.i) AS tree
         FROM pairs p
         JOIN words wr ON wr.aid = p.ra
         JOIN words wo ON wo.aid = p.oa AND wo.i = wr.i
         GROUP BY p.sid, p.origin
     )
SELECT origin AS "Opus (final)", count(*) AS "pairs", count(*) FILTER (WHERE tree_diff > 0 OR tag_diff > 0) AS "disagree",
       round(100.0 * sum(words - tree_diff) / sum(words), 1) AS "LAS agreement, %",
       round(100.0 * sum(words - tag_diff) / sum(words), 1) AS "tags and lemmas, %"
FROM cmp WHERE same_forms
GROUP BY origin ORDER BY "pairs" DESC, origin;

-- largest tree disagreements: Rust head:relation → Opus
WITH r AS (SELECT aid, sid, run FROM annsets WHERE layer = 'ud' AND tool = 'rust'),
     o AS (SELECT aid, sid, run, source, origin FROM annsets WHERE layer = 'ud' AND tool = 'opus' AND origin NOT LIKE '%/opus-a'),
     pairs AS (SELECT r.sid, r.aid AS ra, o.aid AS oa, o.origin FROM r JOIN o ON o.sid = r.sid AND (o.run = r.run OR o.source = 'silver')),
     cmp AS (
         SELECT p.sid, p.origin, count(*) AS words,
                count(*) FILTER (WHERE wr.h IS DISTINCT FROM wo.h OR wr.deprel <> wo.deprel) AS tree_diff,
                bool_and(wr.form = wo.form) AS same_forms,
                string_agg(CASE WHEN wr.h IS DISTINCT FROM wo.h OR wr.deprel <> wo.deprel
                                THEN wr.form || ' ' || wr.h || ':' || wr.deprel || '→' || wo.h || ':' || wo.deprel END, '; ' ORDER BY wr.i) AS tree
         FROM pairs p
         JOIN words wr ON wr.aid = p.ra
         JOIN words wo ON wo.aid = p.oa AND wo.i = wr.i
         GROUP BY p.sid, p.origin
     )
SELECT s.sent_id, c.origin AS "Opus", c.words AS "words", c.tree_diff AS "tree ≠", left(s.text, 60) AS "text", left(c.tree, 80) AS "Rust → Opus"
FROM cmp c JOIN sentences s ON s.sid = c.sid
WHERE c.same_forms AND c.tree_diff > 0 AND c.origin = 'calib-ud-patch300-2026-09-26/final'
ORDER BY c.tree_diff DESC, c.words
LIMIT 5;
