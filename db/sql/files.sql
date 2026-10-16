-- db verify: loaded files with counters from the file (at load time) and from the database (now) and
-- the run's treebank card. Output — {q}/files.tsv.
COPY (
    SELECT f.file, f.path, f.name, f.sha1, f.sents, f.rows, f.words, f.mwt, f.empty,
           count(DISTINCT a.aid) AS db_sents,
           count(t.k) AS db_rows,
           count(t.k) FILTER (WHERE t.id NOT LIKE '%-%' AND t.id NOT LIKE '%.%') AS db_words,
           count(t.k) FILTER (WHERE t.id LIKE '%-%') AS db_mwt,
           count(t.k) FILTER (WHERE t.id LIKE '%.%') AS db_empty,
           r.kind, json_extract_string(r.params, '$.card') AS card
    FROM files f
    JOIN runs r ON r.run = f.run
    LEFT JOIN annsets a ON a.file = f.file
    LEFT JOIN tokens t ON t.aid = a.aid
    GROUP BY ALL
    ORDER BY f.file
) TO '{q}/files.tsv' (FORMAT csv, DELIMITER '\t', HEADER false, QUOTE '', ESCAPE '', NULLSTR '\N');
