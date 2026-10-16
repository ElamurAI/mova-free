-- Paths are relative to the data directory ($MOVA_DATA): run duckdb from there.
-- All annotation sets of one sentence: who, how and when annotated it; then the words side by side —
-- "head:relation UPOS" in each set. The sentence is the variable sent (`# sent_id`).
--   duckdb -readonly $MOVA_DATA/db/mova.duckdb < db/queries/sentence-sets.sql
SET VARIABLE sent = 'weblog-blogspot.com_tacitusproject_20040715092419_ENG_20040715_092419-0007';

SELECT a.aid, a.source AS "source", a.tool AS "who", a.origin AS "run or treebank", a.dialect AS "dialect",
       a.gate AS "gate", CAST(a.created AS DATE) AS "when", regexp_replace(a.method, '^annot calib · ', '') AS "how annotated"
FROM annsets a JOIN sentences s ON s.sid = a.sid
WHERE s.sent_id = getvariable('sent')
ORDER BY a.aid;

PIVOT (
    SELECT w.i, w.form, regexp_replace(a.origin, '^(calib-ud-|UD_English-)', '') AS set,
           w.h || ':' || w.deprel || ' ' || w.upos AS ann
    FROM annsets a
    JOIN sentences s ON s.sid = a.sid
    JOIN words w ON w.aid = a.aid
    WHERE s.sent_id = getvariable('sent')
) ON set USING first(ann) GROUP BY i, form ORDER BY i;
