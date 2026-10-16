-- db qa: database state before loading questions — largest run and question numbers, existing runs,
-- documents by keys from {q}/want-docs.tsv, anchor sentences by `sent_id` from {q}/want-sents.tsv and existing
-- pairs of these documents (for `--replaces`). Output — TSV in directory {q}; read by Rust. A database without
-- migrations v3–v4 (no `qa` or `replaced_by`) fails here, before any write.
COPY (SELECT (SELECT coalesce(max(run), 0) FROM runs), (SELECT coalesce(max(qid), 0) FROM qa))
    TO '{q}/max.tsv' (FORMAT csv, DELIMITER '\t', HEADER false, QUOTE '', ESCAPE '', NULLSTR '\N');
COPY (SELECT run, name FROM runs)
    TO '{q}/runs.tsv' (FORMAT csv, DELIMITER '\t', HEADER false, QUOTE '', ESCAPE '', NULLSTR '\N');
COPY (SELECT doc, key FROM docs
      WHERE key IN (SELECT doc_key FROM read_csv('{q}/want-docs.tsv', delim = '\t', header = true, quote = '',
                                                  escape = '', nullstr = '\N', columns = {'doc_key': 'VARCHAR'})))
    TO '{q}/docs.tsv' (FORMAT csv, DELIMITER '\t', HEADER false, QUOTE '', ESCAPE '', NULLSTR '\N');
COPY (SELECT s.sid, s.doc, s.para, s.sent_id FROM sentences s
      WHERE s.sent_id IN (SELECT sent_id FROM read_csv('{q}/want-sents.tsv', delim = '\t', header = true, quote = '',
                                                        escape = '', nullstr = '\N', columns = {'sent_id': 'VARCHAR'})))
    TO '{q}/sents.tsv' (FORMAT csv, DELIMITER '\t', HEADER false, QUOTE '', ESCAPE '', NULLSTR '\N');
COPY (SELECT q.qid, q.run, q.doc, q.para, q.k, q.replaced_by FROM qa q JOIN docs d ON d.doc = q.doc
      WHERE d.key IN (SELECT doc_key FROM read_csv('{q}/want-docs.tsv', delim = '\t', header = true, quote = '',
                                                   escape = '', nullstr = '\N', columns = {'doc_key': 'VARCHAR'})))
    TO '{q}/oldqa.tsv' (FORMAT csv, DELIMITER '\t', HEADER false, QUOTE '', ESCAPE '', NULLSTR '\N');
