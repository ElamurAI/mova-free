-- db prag: the pragmatics layer from TSV (directory {dir}) — run, sets and `pragmatics` rows in one transaction.
-- An error in any COPY (key, foreign key, CHECK, missing column — a database without migration v2) stops
-- the script (-bail) without COMMIT. After COMMIT — the row count of the new run for reconciliation.
BEGIN TRANSACTION;
COPY runs (run, name, kind, tool, path, params, started, note)
    FROM '{dir}/runs.tsv' (DELIMITER '\t', HEADER, QUOTE '', ESCAPE '', NULLSTR '\N');
COPY annsets (aid, doc, sid, layer, source, tool, run, origin, dialect, created, method, gate)
    FROM '{dir}/annsets.tsv' (DELIMITER '\t', HEADER, QUOTE '', ESCAPE '', NULLSTR '\N');
COPY pragmatics (aid, sid, s_type, act, means, indirect, hedge, polarity, voice, trace)
    FROM '{dir}/pragmatics.tsv' (DELIMITER '\t', HEADER, QUOTE '', ESCAPE '', NULLSTR '\N');
COMMIT;
COPY (SELECT count(*) FROM pragmatics p JOIN annsets a ON a.aid = p.aid WHERE a.run = {run})
    TO '{dir}/post.tsv' (FORMAT csv, DELIMITER '\t', HEADER false, QUOTE '', ESCAPE '', NULLSTR '\N');
