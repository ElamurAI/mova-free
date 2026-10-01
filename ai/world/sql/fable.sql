-- world: sentences of one document with paragraphs, in text order ({doc} is a number, substituted by `world`).
-- Output is TSV in {out}: title license para ord text.
COPY (
    SELECT coalesce(d.title, ''), coalesce(d.license, ''), s.para, s.ord, s.text
    FROM sentences s
    JOIN docs d ON d.doc = s.doc
    WHERE d.doc = {doc}
    ORDER BY s.ord
) TO '{out}' (FORMAT csv, DELIMITER '\t', HEADER false, QUOTE '', ESCAPE '', NULLSTR '\N');
