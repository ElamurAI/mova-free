-- qagen: fairy-tale sentences (Project Gutenberg, public domain) with paragraphs and the paragraph's block kind
-- (p — prose, lg — verse, head — heading; from `newpar_block` of the paragraph's first sentence), in text order.
-- Output — TSV into {out}: key title author license para block ord sent_id text. Read by `qagen select`.
COPY (
    SELECT d.key, coalesce(d.title, ''), coalesce(d.author, ''), coalesce(d.license, ''), s.para,
           coalesce(split_part(first_value(json_extract_string(s.comments, '$.newpar_block'))
                               OVER (PARTITION BY s.doc, s.para ORDER BY s.ord), ' ', 1), ''),
           s.ord, s.sent_id, s.text
    FROM sentences s
    JOIN docs d ON d.doc = s.doc
    WHERE d.key LIKE 'gutenberg:%'
    ORDER BY d.key, s.ord
) TO '{out}' (FORMAT csv, DELIMITER '\t', HEADER false, QUOTE '', ESCAPE '', NULLSTR '\N');
