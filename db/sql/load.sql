-- db load: TSVs prepared by Rust (directory {dir}) go into tables in one transaction. An error in any
-- COPY (key, foreign key, NOT NULL, CHECK) stops the script (-bail) without COMMIT — nothing in the database
-- changes. After COMMIT — counters of the new file {file} for reconciliation with the file.
BEGIN TRANSACTION;
COPY runs (run, name, kind, tool, path, params, started, note)
    FROM '{dir}/runs.tsv' (DELIMITER '\t', HEADER, QUOTE '', ESCAPE '', NULLSTR '\N');
COPY files (file, run, path, name, bytes, sha1, blank_before, final_newline, sents, rows, words, mwt, empty)
    FROM '{dir}/files.tsv' (DELIMITER '\t', HEADER, QUOTE '', ESCAPE '', NULLSTR '\N');
COPY docs (doc, key, source, title, author, lang, license, genre, meta)
    FROM '{dir}/docs.tsv' (DELIMITER '\t', HEADER, QUOTE '', ESCAPE '', NULLSTR '\N');
COPY sentences (sid, doc, para, ord, sent_id, text, comments)
    FROM '{dir}/sentences.tsv' (DELIMITER '\t', HEADER, QUOTE '', ESCAPE '', NULLSTR '\N');
COPY annsets (aid, doc, sid, layer, source, tool, run, origin, dialect, created, method, gate, file, pos, blank_after, comments)
    FROM '{dir}/annsets.tsv' (DELIMITER '\t', HEADER, QUOTE '', ESCAPE '', NULLSTR '\N');
COPY tokens (aid, k, id, form, lemma, upos, xpos, feats, head, deprel, deps, misc)
    FROM '{dir}/tokens.tsv' (DELIMITER '\t', HEADER, QUOTE '', ESCAPE '', NULLSTR '\N');
COMMIT;
COPY (SELECT (SELECT count(*) FROM annsets WHERE file = {file}),
             (SELECT count(*) FROM tokens WHERE aid IN (SELECT aid FROM annsets WHERE file = {file})),
             (SELECT count(*) FROM words WHERE aid IN (SELECT aid FROM annsets WHERE file = {file})))
    TO '{dir}/post.tsv' (FORMAT csv, DELIMITER '\t', HEADER false, QUOTE '', ESCAPE '', NULLSTR '\N');
