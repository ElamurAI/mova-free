-- Paths are relative to the data directory ($MOVA_DATA): run duckdb from there.
-- Control queries against $MOVA_DATA/db/wikidata.duckdb (read-only):
--   ~/bin/duckdb -readonly $MOVA_DATA/db/wikidata.duckdb < onto/sql/wikidata-checks.sql
.mode markdown

-- 0. What is in the database
SELECT 'items' AS what, count(*) AS how_many FROM items
UNION ALL SELECT 'classes (take part in P279)', count(*) FILTER (WHERE is_class) FROM items
UNION ALL SELECT 'taxa', count(*) FILTER (WHERE is_taxon) FROM items
UNION ALL SELECT 'items with a Wikipedia article', count(*) FILTER (WHERE n_wiki > 0) FROM items
UNION ALL SELECT 'labels and aliases', count(*) FROM labels
UNION ALL SELECT 'label languages', count(DISTINCT lang) FROM labels
UNION ALL SELECT 'Wikipedia article titles', count(*) FROM sitelinks
UNION ALL SELECT 'Wikipedias', count(DISTINCT site) FROM sitelinks
UNION ALL SELECT 'statements with an item', count(*) FROM claims
UNION ALL SELECT '«is a kind of» edges (P279+P171, no cycles)', count(*) FROM isa_edges
UNION ALL SELECT 'P279/P171 cycles', count(DISTINCT cycle) FROM isa_cycles
UNION ALL SELECT 'lexemes', count(*) FROM lexemes
UNION ALL SELECT 'forms (representations)', count(*) FROM forms
UNION ALL SELECT 'lexeme senses', count(*) FROM (SELECT DISTINCT l, s FROM glosses)
UNION ALL SELECT 'sense translations (P5972)', count(*) FROM sense_links WHERE p = 5972
UNION ALL SELECT 'OEWN synsets ↔ Q (accepted)', count(*) FROM synset_q WHERE status = 'accepted';

-- 1. apple (Q89) — a subclass of fruit?
SELECT 'Q89 apple ⊂' AS query, a.anc AS q, i.label_en, a.depth
FROM wd_ancestors(89) a JOIN items i ON i.q = a.anc
WHERE a.anc IN (1364, 3314483) OR i.label_en IN ('fruit', 'food', 'pome', 'edible fruit')
ORDER BY a.depth;

-- 2. whale — mammals, not fish: Q7377 (Mammalia) and Q152 (fish)
SELECT q, label_en, desc_en, wd_isa(q, 7377) AS mammal, wd_isa(q, 152) AS fish
FROM items WHERE label_en = 'whale' AND is_class ORDER BY n_wiki DESC;

-- 3. Corvus corone — a taxon, common names in several languages (P1843)
SELECT s.q, i.label_en, i.is_taxon, count(m.text) AS names, count(DISTINCT m.lang) AS langs,
       string_agg(m.lang || ': ' || m.text, '; ' ORDER BY m.lang) FILTER (WHERE m.lang IN ('en', 'uk', 'de', 'fr', 'ru', 'pl', 'ja', 'nl')) AS examples
FROM strs s JOIN items i USING (q) LEFT JOIN mono m ON m.q = s.q AND m.p = 1843
WHERE s.p = 225 AND s.value = 'Corvus corone' GROUP BY ALL;

-- 4. the uk label for Q89 (Ukrainian data); and what Q89 is called in several languages
SELECT q, lang, label, is_alias FROM labels WHERE q = 89 AND lang IN ('uk', 'en', 'de', 'fr', 'ja', 'mul') ORDER BY lang, is_alias, label;

-- 5. Wikipedia article titles for Q89 (en / uk / de / fr / ja)
SELECT q, site, title FROM sitelinks WHERE q = 89 AND lang IN ('en', 'uk', 'de', 'fr', 'ja') ORDER BY site;

-- 6. produced sound (P4733): how many and examples — birds (descendants of Q5113)
SELECT count(*) AS statements_P4733, count(DISTINCT s) AS items FROM claims WHERE p = 4733 AND rank > 0;
SELECT c.s AS q, wd_label(c.s, 'en') AS who, wd_label(c.s, 'uk') AS who_uk, c.o AS sound, wd_label(c.o, 'en') AS sound_en, wd_label(c.o, 'uk') AS sound_uk
FROM claims c WHERE c.p = 4733 AND c.rank > 0 AND wd_isa(c.s, 5113) ORDER BY c.s LIMIT 15;

-- 7. mass, length, height — lion and mouse (P2067, P2043, P2048), units are Q
SELECT q.q, wd_label(q.q, 'en') AS who, q.p, q.amount, q.unit, wd_label(q.unit, 'en') AS unit_label, q.quals
FROM quant q WHERE q.q IN (140, 83310) ORDER BY q.q, q.p;

-- 8. the English lexeme «apple» (en, Q1860): forms with features, sense → item, translations
SELECT x.l, x.lemma, f.rep, f.feats FROM lexemes x JOIN forms f USING (l)
WHERE x.lemma = 'apple' AND x.lang = 1860 ORDER BY f.f LIMIT 12;
SELECT si.l, si.s, si.q, wd_label(si.q, 'en') AS item FROM sense_items si JOIN lexemes x USING (l)
WHERE x.lemma = 'apple' AND x.lang = 1860 AND si.p = 5137;
SELECT x.lemma, t.l2, x2.lemma AS translation, x2.lemma_code FROM lexemes x JOIN sense_links t ON t.l = x.l AND t.p = 5972
JOIN lexemes x2 ON x2.l = t.l2 WHERE x.lemma = 'apple' AND x.lang = 1860 ORDER BY x2.lemma_code LIMIT 20;

-- 9. synset ↔ Q for fable keywords
SELECT synset, q, wd_label(q, 'en') AS en, wd_label(q, 'uk') AS uk, sources, support_synset, support_q, ds, dq
FROM synset_q WHERE status = 'accepted' AND q IN (89, 5113, 43365, 1865281, 140, 10943, 144, 146, 8331, 10884, 12870);

-- 10. property numbers of the pass ↔ their names in the dump itself (check that the number is right)
SELECT p, label_en, label_uk, datatype FROM props
WHERE p IN (31, 279, 361, 527, 186, 171, 1034, 4733, 1582, 105, 1672, 2670, 3113, 462, 1419, 2974, 366, 1552, 3094,
            225, 2888, 1709, 5063, 8814, 1843, 2067, 2043, 2048, 2052, 2250, 5137, 5972, 5973, 5974, 5185)
ORDER BY p;
