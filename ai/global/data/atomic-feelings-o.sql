-- Feelings of others (the one acted upon) after an event from ATOMIC-2020 (CC BY 4.0): head verb "PersonX <verb> …" + xReact → feeling category.
-- Run: duckdb < global/data/atomic-feelings.sql  (data: data/raw/atomic2020/atomic_2020/train.{source,target})
SET threads = 4;
CREATE TEMP TABLE src AS SELECT row_number() OVER () AS i, column0 AS s FROM read_csv('data/raw/atomic2020/atomic_2020/train.source', header=false, sep='\x01', quote='', columns={'column0':'VARCHAR'});
CREATE TEMP TABLE tgt AS SELECT row_number() OVER () AS i, lower(column0) AS t FROM read_csv('data/raw/atomic2020/atomic_2020/train.target', header=false, sep='\x01', quote='', columns={'column0':'VARCHAR'});
CREATE TEMP TABLE r AS
SELECT regexp_extract(s, '^PersonX (\w+)', 1) AS v, t FROM src JOIN tgt USING (i)
WHERE s LIKE 'PersonX %' AND s LIKE '% oReact [GEN]' AND t <> 'none';
-- verb lemma: …ies → …y, …es/…s → stripped, …ed → stripped (crude but deterministic)
CREATE TEMP TABLE r2 AS SELECT CASE
  WHEN v IN ('has') THEN 'have' WHEN v IN ('does') THEN 'do' WHEN v IN ('goes') THEN 'go' WHEN v IN ('is') THEN 'be'
  WHEN v LIKE '%oes' THEN substr(v, 1, length(v) - 2)
  WHEN v LIKE '%ies' AND length(v) <= 5 THEN substr(v, 1, length(v) - 1)
  WHEN v LIKE '%ies' THEN substr(v, 1, length(v) - 3) || 'y'
  WHEN v LIKE '%sses' OR v LIKE '%ches' OR v LIKE '%shes' OR v LIKE '%xes' THEN substr(v, 1, length(v) - 2)
  WHEN v LIKE '%s' AND v NOT LIKE '%ss' THEN substr(v, 1, length(v) - 1) ELSE v END AS verb, t FROM r WHERE v <> '';
CREATE TEMP TABLE emo(w VARCHAR, cat VARCHAR);
INSERT INTO emo VALUES
 ('happy','joy'),('glad','joy'),('joyful','joy'),('delighted','joy'),('pleased','joy'),('excited','joy'),('proud','joy'),('satisfied','joy'),('relieved','joy'),('content','joy'),('cheerful','joy'),('good','joy'),('accomplished','joy'),('thrilled','joy'),
 ('sad','sadness'),('unhappy','sadness'),('upset','sadness'),('depressed','sadness'),('disappointed','sadness'),('lonely','sadness'),('heartbroken','sadness'),('miserable','sadness'),('dejected','sadness'),('sorrowful','sadness'),('grieving','sadness'),
 ('afraid','fear'),('scared','fear'),('frightened','fear'),('nervous','fear'),('anxious','fear'),('worried','fear'),('terrified','fear'),('fearful','fear'),
 ('angry','anger'),('mad','anger'),('annoyed','anger'),('frustrated','anger'),('furious','anger'),('irritated','anger'),('upset ','anger'),
 ('surprised','surprise'),('shocked','surprise'),('amazed','surprise'),('astonished','surprise'),
 ('grateful','gratitude'),('thankful','gratitude'),('appreciated','gratitude'),('loved','gratitude'),
 ('ashamed','shame'),('embarrassed','shame'),('guilty','shame'),('humiliated','shame'),('sorry','shame');
CREATE TEMP TABLE hits AS
SELECT verb, cat, count(*) AS n FROM r2 JOIN emo ON r2.t = emo.w OR r2.t LIKE '% ' || emo.w OR r2.t LIKE emo.w || ' %' GROUP BY ALL;
CREATE TEMP TABLE tot AS SELECT verb, sum(n) AS tot FROM hits GROUP BY verb;
-- lift over the baseline: ATOMIC is positive overall, so a category wins if it is twice as frequent for the verb as on average
CREATE TEMP TABLE base AS SELECT cat, sum(n) * 1.0 / (SELECT sum(n) FROM hits) AS p FROM hits GROUP BY cat;
CREATE TEMP TABLE lift AS SELECT h.verb, h.cat, h.n, t.tot, (h.n * 1.0 / t.tot) / b.p AS lift FROM hits h JOIN tot t USING (verb) JOIN base b USING (cat) WHERE t.tot >= 6 AND h.n >= 3;
CREATE TEMP TABLE dom AS SELECT verb, arg_max(cat, lift) AS cat, max(lift) AS lift, max(tot) AS tot FROM lift GROUP BY verb HAVING max(lift) >= 2.0 AND verb NOT IN ('be','have','do','say','would','could','can','will','feel','become','get','go','make','take','alway','really','still','eventually','quickly','immediately','suddenly','soon','almost','never','accidentally','start','begin','need','want','try','act','remain','look','stand','arrive','cause','upset','satisfy','impress','change','turn','apply','head','depend','wish','dy','ap','file');
COPY (SELECT cat, count(*) n, string_agg(verb, ' ' ORDER BY tot DESC) words FROM dom GROUP BY cat ORDER BY cat) TO 'data/runs/atomic/feel-verbs-o.tsv' (HEADER, DELIMITER '\t');
SELECT cat, count(*) n, substr(string_agg(verb, ' ' ORDER BY tot DESC), 1, 160) AS ex FROM dom GROUP BY cat ORDER BY n DESC;
