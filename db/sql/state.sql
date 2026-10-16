-- db load: database state before loading — largest ids, runs, files, documents, candidate sentences
-- for matching (by `sent_id` from {q}/want.tsv). Output — TSV in directory {q}; read by Rust.
COPY (SELECT (SELECT coalesce(max(run), 0) FROM runs), (SELECT coalesce(max(file), 0) FROM files),
             (SELECT coalesce(max(doc), 0) FROM docs), (SELECT coalesce(max(sid), 0) FROM sentences),
             (SELECT coalesce(max(aid), 0) FROM annsets))
    TO '{q}/max.tsv' (FORMAT csv, DELIMITER '\t', HEADER false, QUOTE '', ESCAPE '', NULLSTR '\N');
COPY (SELECT run, name FROM runs)
    TO '{q}/runs.tsv' (FORMAT csv, DELIMITER '\t', HEADER false, QUOTE '', ESCAPE '', NULLSTR '\N');
COPY (SELECT path, sha1 FROM files)
    TO '{q}/files.tsv' (FORMAT csv, DELIMITER '\t', HEADER false, QUOTE '', ESCAPE '', NULLSTR '\N');
COPY (SELECT d.doc, d.key, coalesce(max(s.ord), 0), coalesce(max(s.para), 0)
      FROM docs d LEFT JOIN sentences s ON s.doc = d.doc GROUP BY d.doc, d.key)
    TO '{q}/docs.tsv' (FORMAT csv, DELIMITER '\t', HEADER false, QUOTE '', ESCAPE '', NULLSTR '\N');
COPY (SELECT s.sid, s.doc, s.sent_id, s.text FROM sentences s
      WHERE s.sent_id IN (SELECT sent_id FROM read_csv('{q}/want.tsv', delim = '\t', header = true, quote = '',
                                                        escape = '', nullstr = '\N', columns = {'sent_id': 'VARCHAR'})))
    TO '{q}/sents.tsv' (FORMAT csv, DELIMITER '\t', HEADER false, QUOTE '', ESCAPE '', NULLSTR '\N');
