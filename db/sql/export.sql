-- db export: annotation sets by filter. Filter variables (file, run, source, tool, layer, dialect,
-- origin, doc, sent, sid, aid, gate) are set by `db export` via SET VARIABLE; unset — no filter.
-- Output — TSV in {q}: sets.tsv (blocks in order), rows.tsv (CoNLL-U rows), files.tsv (file boundaries).
CREATE TEMP TABLE sel AS
SELECT a.aid, a.file, a.pos, a.blank_after, a.comments, s.ord, a.doc
FROM annsets a
JOIN runs r ON r.run = a.run
JOIN docs d ON d.doc = a.doc
LEFT JOIN files f ON f.file = a.file
LEFT JOIN sentences s ON s.sid = a.sid
WHERE a.layer = coalesce(getvariable('layer'), 'ud')
  AND (getvariable('file') IS NULL OR f.path = getvariable('file') OR f.name = getvariable('file') OR CAST(f.file AS TEXT) = getvariable('file'))
  AND (getvariable('run') IS NULL OR r.name = getvariable('run'))
  AND (getvariable('source') IS NULL OR a.source = getvariable('source'))
  AND (getvariable('tool') IS NULL OR a.tool = getvariable('tool'))
  AND (getvariable('dialect') IS NULL OR a.dialect = getvariable('dialect'))
  AND (getvariable('origin') IS NULL OR a.origin = getvariable('origin'))
  AND (getvariable('doc') IS NULL OR d.key = getvariable('doc'))
  AND (getvariable('sent') IS NULL OR s.sent_id = getvariable('sent'))
  AND (getvariable('sid') IS NULL OR CAST(a.sid AS TEXT) = getvariable('sid'))
  AND (getvariable('aid') IS NULL OR CAST(a.aid AS TEXT) = getvariable('aid'))
  AND (getvariable('gate') IS NULL OR a.gate = getvariable('gate'));
COPY (SELECT aid, file, blank_after, comments FROM sel ORDER BY file NULLS LAST, pos, doc, ord, aid)
    TO '{q}/sets.tsv' (FORMAT csv, DELIMITER '\t', HEADER false, QUOTE '', ESCAPE '', NULLSTR '\N');
COPY (SELECT t.aid, t.k, t.id, t.form, t.lemma, t.upos, t.xpos, t.feats, t.head, t.deprel, t.deps, t.misc
      FROM tokens t WHERE t.aid IN (SELECT aid FROM sel) ORDER BY t.aid, t.k)
    TO '{q}/rows.tsv' (FORMAT csv, DELIMITER '\t', HEADER false, QUOTE '', ESCAPE '', NULLSTR '\N');
COPY (SELECT f.file, f.blank_before, f.final_newline, f.sha1 FROM files f WHERE f.file IN (SELECT file FROM sel) ORDER BY f.file)
    TO '{q}/files.tsv' (FORMAT csv, DELIMITER '\t', HEADER false, QUOTE '', ESCAPE '', NULLSTR '\N');
