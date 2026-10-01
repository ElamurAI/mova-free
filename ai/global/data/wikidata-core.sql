-- Level-1 core from Wikidata: categories for nouns that actually occur in our
-- tales (42 Gutenberg books) and problems (SVAMP, GSM8K). Run:
--   duckdb -readonly data/db/wikidata.duckdb < global/data/wikidata-core.sql
-- Output: data/runs/wd-core/core.tsv (word, Q, label, category, depth) and categories.tsv (category → words).
SET threads = 6;
-- 1) corpus words: frequency ≥ 3, length ≥ 3, singular (…ies → …y, …s → …)
CREATE TEMP TABLE raw_words AS
SELECT lower(w) AS w FROM (
  SELECT unnest(regexp_split_to_array(content, '[^A-Za-z]+')) AS w FROM read_text('data/raw/tales-pg-*/*.txt')
  UNION ALL
  SELECT unnest(regexp_split_to_array(Body || ' ' || Question, '[^A-Za-z]+')) FROM read_json_auto('data/raw/mwpdata-svamp/svamp-train.json')
  UNION ALL
  SELECT unnest(regexp_split_to_array(question, '[^A-Za-z]+')) FROM read_json_auto('data/raw/mwpdata-gsm8k-socratic/gsm8k-socratic-train.json')
) WHERE length(w) >= 3;
CREATE TEMP TABLE words AS
SELECT CASE WHEN w LIKE '%ies' AND length(w) > 4 THEN substr(w, 1, length(w) - 3) || 'y'
            WHEN w LIKE '%s' AND w NOT LIKE '%ss' AND length(w) > 3 THEN substr(w, 1, length(w) - 1)
            ELSE w END AS lemma, count(*) AS n
FROM raw_words GROUP BY 1 HAVING count(*) >= 3;
-- 2) meaning: an entity with that English label and ≥ 5 Wikipedias; the best-known one
CREATE TEMP TABLE sense AS
SELECT lemma, arg_max(q, n_wiki) AS q, max(n_wiki) AS n_wiki
FROM words JOIN items ON lower(items.label_en) = words.lemma
WHERE n_wiki >= 5
GROUP BY lemma;
-- 3) categories via the closure "subclass of / instance of / parent taxon"
CREATE TEMP TABLE cats(q INTEGER, cat VARCHAR);
INSERT INTO cats VALUES (729,'animal'),(756,'plant'),(2095,'food'),(1364,'fruit'),(11004,'vegetable'),(39546,'tool'),(728,'weapon'),
  (42889,'vehicle'),(41176,'building'),(11460,'clothing'),(14745,'furniture'),(5,'human'),(987767,'container'),(34379,'musical_instrument'),
  (11422,'toy'),(47574,'measure_unit'),(1790144,'time_unit'),(8142,'currency'),(11435,'liquid'),(11426,'metal'),(83437,'gemstone'),
  (1075,'color'),(2239243,'mythical_creature'),(6999,'celestial_body'),(4936952,'body_part'),(12737077,'occupation');
CREATE TEMP TABLE core AS
SELECT s.lemma, s.q, i.label_en, c.cat, min(cl.depth) AS depth, s.n_wiki
FROM sense s JOIN items i ON i.q = s.q
JOIN (SELECT q, anc, depth FROM closure UNION ALL SELECT q, q, 0 FROM sense) cl ON cl.q = s.q
JOIN cats c ON c.q = cl.anc
GROUP BY ALL;
-- 4) gate priorities against Wikidata noise (a human is not an "animal"; a fruit is not a "body part"; something concrete is not a "tool")
CREATE TEMP TABLE has AS SELECT lemma, list(cat) AS cs FROM core GROUP BY lemma;
CREATE TEMP TABLE core_clean AS
SELECT c.* FROM core c JOIN has h USING (lemma)
WHERE NOT (c.cat = 'animal' AND (list_contains(h.cs, 'human') OR list_contains(h.cs, 'occupation')))
  AND NOT (c.cat = 'body_part' AND (list_contains(h.cs, 'fruit') OR list_contains(h.cs, 'plant') OR list_contains(h.cs, 'food') OR list_contains(h.cs, 'vegetable')))
  AND NOT (c.cat = 'tool' AND list_has_any(h.cs, ['clothing','container','vehicle','weapon','musical_instrument','furniture','toy','building','food']))
  AND NOT (c.cat IN ('color', 'liquid') AND list_has_any(h.cs, ['animal','human','plant','occupation','mythical_creature']))
  AND NOT (c.cat = 'human' AND list_contains(h.cs, 'mythical_creature'));
CREATE OR REPLACE TEMP TABLE core AS SELECT * FROM core_clean;
COPY (SELECT * FROM core ORDER BY cat, lemma) TO 'data/runs/wd-core/core.tsv' (HEADER, DELIMITER '\t');
COPY (SELECT cat, count(*) AS n, string_agg(lemma, ' ' ORDER BY lemma) AS words FROM core GROUP BY cat ORDER BY cat) TO 'data/runs/wd-core/categories.tsv' (HEADER, DELIMITER '\t');
SELECT count(*) AS words_corpus FROM words;
SELECT count(*) AS with_sense FROM sense;
SELECT cat, count(*) n FROM core GROUP BY cat ORDER BY n DESC;
