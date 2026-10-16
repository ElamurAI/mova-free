-- Paths are relative to the data directory ($MOVA_DATA): run duckdb from there.
-- Textbooks to download: allowed licenses only (CC BY, CC BY-SA, CC0, public domain, GFDL),
-- not GitHub, direct PDF, university level (no "pre"). Duplicates by title and by URL — keep one
-- (AIM > OTL > OpenStax). Order: core courses first, because the 2 GB cap cuts the tail of the list.
-- Usage: mathlib sql $MOVA_DATA/db/math.duckdb sql/books-pick.sql out=<books-list.tsv>
CREATE OR REPLACE TEMP TABLE books_pick AS
WITH b AS (
    SELECT *, regexp_replace(lower(title), '[^a-z0-9]+', '', 'g') AS nt,
        CASE source WHEN 'aim' THEN 1 WHEN 'otl' THEN 2 ELSE 3 END AS src_rank,
        CASE topic WHEN 'calculus' THEN 1 WHEN 'linear-algebra' THEN 2 WHEN 'diff-eq' THEN 3
                   WHEN 'prob-stat' THEN 4 WHEN 'discrete' THEN 5 WHEN 'algebra' THEN 6 WHEN 'analysis' THEN 7
                   WHEN 'proofs-logic' THEN 8 WHEN 'number-theory' THEN 9 WHEN 'geometry-topology' THEN 10
                   WHEN 'applied' THEN 11 WHEN 'precalc' THEN 12 ELSE 13 END AS topic_rank
    FROM books
    -- LibreTexts is mostly NC and builds PDFs itself — catalog only (per assignment)
    WHERE allowed = 'true' AND github = 'false' AND level <> 'pre' AND topic NOT IN ('not-math')
      AND pdf_host NOT LIKE '%libretexts%'
      AND regexp_matches(lower(pdf_url), '^https?://[^ ]+(\.pdf($|[?#])|viewcontent\.cgi\?)')
), b2 AS (
    -- an OpenStax book that OTL lists under the same title from another host — keep the publisher
    SELECT * FROM b
    WHERE NOT (source = 'otl' AND nt IN (SELECT regexp_replace(lower(title), '[^a-z0-9]+', '', 'g') FROM books WHERE source = 'openstax'))
), d AS (
    SELECT *, row_number() OVER (PARTITION BY nt, akey ORDER BY src_rank) AS dup_t,
              row_number() OVER (PARTITION BY pdf_url ORDER BY src_rank) AS dup_u
    FROM b2
)
SELECT *, left(source || '-' || trim(regexp_replace(lower(title), '[^a-z0-9]+', '-', 'g'), '-'), 70) AS slug
FROM d WHERE dup_t = 1 AND dup_u = 1;

COPY (
    SELECT slug, key, pdf_url AS url, slug || '.pdf' AS save_as, license, license_url,
           replace(title, '"', '''') AS title, url AS source_page, topic, level, source, site
    FROM books_pick ORDER BY topic_rank, src_rank, title
) TO '{out}' (FORMAT csv, DELIMITER '\t', HEADER true, QUOTE '');

.print '## Textbooks to download'
SELECT topic, level, count(*) AS n, string_agg(DISTINCT license, ', ') AS licenses
FROM books_pick GROUP BY ALL ORDER BY min(topic_rank), level;
