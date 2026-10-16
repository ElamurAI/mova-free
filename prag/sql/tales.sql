-- prag: all tale sentences (Project Gutenberg, public domain) with paragraphs, in text order.
-- Output — TSV in {out}: key source title license para ord sent_id text. Read by `prag select`.
COPY (
    SELECT d.key, d.source, coalesce(d.title, ''), coalesce(d.license, ''), s.para, s.ord, s.sent_id, s.text
    FROM sentences s
    JOIN docs d ON d.doc = s.doc
    WHERE d.key LIKE 'gutenberg:%'
    ORDER BY d.key, s.ord
) TO '{out}' (FORMAT csv, DELIMITER '\t', HEADER false, QUOTE '', ESCAPE '', NULLSTR '\N');
