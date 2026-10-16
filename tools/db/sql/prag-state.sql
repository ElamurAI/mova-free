-- db prag: database state before loading the pragmatics layer — largest run and set numbers, existing
-- runs, candidate sentences by `sent_id` from {q}/want.tsv. Output — TSV in directory {q}; read by Rust.
-- (Internal SQL of the pragmatics layer; its place is next to db/sql/ once it is moved.)
COPY (SELECT (SELECT coalesce(max(run), 0) FROM runs), (SELECT coalesce(max(aid), 0) FROM annsets))
    TO '{q}/max.tsv' (FORMAT csv, DELIMITER '\t', HEADER false, QUOTE '', ESCAPE '', NULLSTR '\N');
COPY (SELECT run, name FROM runs)
    TO '{q}/runs.tsv' (FORMAT csv, DELIMITER '\t', HEADER false, QUOTE '', ESCAPE '', NULLSTR '\N');
COPY (SELECT s.sid, s.doc, s.sent_id FROM sentences s
      WHERE s.sent_id IN (SELECT sent_id FROM read_csv('{q}/want.tsv', delim = '\t', header = true, quote = '',
                                                        escape = '', nullstr = '\N', columns = {'sent_id': 'VARCHAR'})))
    TO '{q}/sents.tsv' (FORMAT csv, DELIMITER '\t', HEADER false, QUOTE '', ESCAPE '', NULLSTR '\N');
