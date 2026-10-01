-- Common sense from ATOMIC-2020 (CC BY 4.0): "thing → what for" (ObjectUse) and "who → what can do" (CapableOf) for nouns
-- of our corpus (Wikidata core words). The first word of the answer is the action; frequency ≥ 2.
SET threads = 4;
CREATE TEMP TABLE src AS SELECT row_number() OVER () AS i, column0 AS s FROM read_csv('data/raw/atomic2020/atomic_2020/train.source', header=false, sep='\x01', quote='', columns={'column0':'VARCHAR'});
CREATE TEMP TABLE tgt AS SELECT row_number() OVER () AS i, lower(column0) AS t FROM read_csv('data/raw/atomic2020/atomic_2020/train.target', header=false, sep='\x01', quote='', columns={'column0':'VARCHAR'});
CREATE TEMP TABLE corpus AS SELECT DISTINCT lemma FROM read_csv('data/runs/wd-core/core.tsv', header=true, sep='\t');
CREATE TEMP TABLE rel AS
SELECT lower(regexp_extract(s, '^([a-zA-Z]+) (ObjectUse|CapableOf) ', 1)) AS head, regexp_extract(s, ' (ObjectUse|CapableOf) ', 1) AS r,
       regexp_extract(t, '^([a-z]+)', 1) AS act
FROM src JOIN tgt USING (i) WHERE regexp_matches(s, '^[a-zA-Z]+ (ObjectUse|CapableOf) ') AND t <> 'none';
CREATE TEMP TABLE agg AS
SELECT r, act, head, count(*) AS n FROM rel JOIN corpus ON corpus.lemma = rel.head
WHERE length(act) >= 3 AND act NOT IN ('the','and','for','with','you','can','get','make','use','have','put','take','give','keep','being','not')
GROUP BY ALL HAVING count(*) >= 2;
-- action → things (each thing gets up to its 6 most frequent actions)
CREATE TEMP TABLE top AS SELECT * FROM (SELECT *, row_number() OVER (PARTITION BY r, head ORDER BY n DESC) AS k FROM agg) WHERE k <= 6;
COPY (SELECT CASE r WHEN 'ObjectUse' THEN 'use_' ELSE 'can_' END || act AS name, count(*) AS n, string_agg(head, ' ' ORDER BY head) AS words
      FROM top GROUP BY 1 HAVING count(*) >= 2 ORDER BY 1) TO 'data/runs/atomic/uses.tsv' (HEADER, DELIMITER '\t');
SELECT r, count(DISTINCT head) heads, count(DISTINCT act) acts FROM top GROUP BY r;
