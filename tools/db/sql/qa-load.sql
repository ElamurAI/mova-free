-- db qa: questions and answers from TSV (directory {dir}) — run and `qa` rows in one transaction. An error
-- in any COPY (key, foreign key, CHECK on type, explicitness, answer or source) stops the script (-bail)
-- without COMMIT. Replacement (`--replaces`): the new row carries `replaces` — the old qid, and the old row in the same
-- transaction gets `replaced_by` — the new qid; without replacement `replaces` is empty and UPDATE touches nothing.
-- After COMMIT — counters for reconciliation: rows of the run, rows with a replacement, old rows marked.
BEGIN TRANSACTION;
COPY runs (run, name, kind, tool, path, params, started, note)
    FROM '{dir}/runs.tsv' (DELIMITER '\t', HEADER, QUOTE '', ESCAPE '', NULLSTR '\N');
COPY qa (qid, run, doc, para, k, question, qtype, ex_or_im, answer, anchors, source, replaces)
    FROM '{dir}/qa.tsv' (DELIMITER '\t', HEADER, QUOTE '', ESCAPE '', NULLSTR '\N');
UPDATE qa SET replaced_by = n.qid FROM qa AS n WHERE n.run = {run} AND n.replaces = qa.qid;
COMMIT;
COPY (SELECT (SELECT count(*) FROM qa WHERE run = {run}),
             (SELECT count(*) FROM qa WHERE run = {run} AND replaces IS NOT NULL),
             (SELECT count(*) FROM qa o JOIN qa n ON n.qid = o.replaced_by AND n.replaces = o.qid WHERE n.run = {run}))
    TO '{dir}/post.tsv' (FORMAT csv, DELIMITER '\t', HEADER false, QUOTE '', ESCAPE '', NULLSTR '\N');
