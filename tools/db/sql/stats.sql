-- db stats: database overview — tables, runs, layers by source, document licenses, fairy tales.

SELECT 'runs' AS "table", count(*) AS "rows" FROM runs
UNION ALL SELECT 'files', count(*) FROM files
UNION ALL SELECT 'docs', count(*) FROM docs
UNION ALL SELECT 'sentences', count(*) FROM sentences
UNION ALL SELECT 'annsets', count(*) FROM annsets
UNION ALL SELECT 'tokens', count(*) FROM tokens
UNION ALL SELECT 'entities', count(*) FROM entities
UNION ALL SELECT 'mentions', count(*) FROM mentions
UNION ALL SELECT 'events', count(*) FROM events
UNION ALL SELECT 'pragmatics', count(*) FROM pragmatics
UNION ALL SELECT 'passport', count(*) FROM passport
UNION ALL SELECT 'summaries', count(*) FROM summaries;

-- runs: files, sets, sentences
SELECT r.name AS "run", r.kind AS "kind", count(DISTINCT f.file) AS "files",
       count(DISTINCT a.aid) AS "sets", count(DISTINCT a.sid) AS "sentences"
FROM runs r
LEFT JOIN files f ON f.run = r.run
LEFT JOIN annsets a ON a.run = r.run
GROUP BY r.run, r.name, r.kind
ORDER BY r.run;

-- layers by source
SELECT a.layer AS "layer", a.source AS "source", a.tool AS "who", a.dialect AS "dialect",
       count(*) AS "sets", count(DISTINCT a.sid) AS "sentences", count(DISTINCT a.doc) AS "documents",
       sum(t.rows) AS "rows", sum(t.words) AS "words",
       round(100.0 * count(*) FILTER (WHERE a.gate = 'ok') / count(*), 1) AS "gates ok, %"
FROM annsets a
LEFT JOIN (SELECT aid, count(*) AS rows, count(*) FILTER (WHERE id NOT LIKE '%-%' AND id NOT LIKE '%.%') AS words
           FROM tokens GROUP BY aid) t ON t.aid = a.aid
GROUP BY ALL
ORDER BY "layer", "sets" DESC;

-- documents by license
SELECT coalesce(d.license, '—') AS "license", count(*) AS "documents", sum(s.n) AS "sentences"
FROM docs d
LEFT JOIN (SELECT doc, count(*) AS n FROM sentences GROUP BY doc) s ON s.doc = d.doc
GROUP BY ALL
ORDER BY "sentences" DESC;

-- fairy tales: book, tales, paragraphs, sentences, words
SELECT d.meta->>'book' AS "book", count(DISTINCT d.doc) AS "tales",
       count(DISTINCT s.doc || ':' || s.para) AS "paragraphs", count(DISTINCT s.sid) AS "sentences", count(w.i) AS "words"
FROM docs d
JOIN sentences s ON s.doc = d.doc
JOIN annsets a ON a.sid = s.sid AND a.layer = 'ud'
JOIN words w ON w.aid = a.aid
WHERE d.source LIKE 'Project Gutenberg%'
GROUP BY ALL
ORDER BY "sentences" DESC;
