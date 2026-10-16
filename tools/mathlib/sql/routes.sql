-- Paths are relative to the data directory ($MOVA_DATA): run duckdb from there.
-- Routes: sizes of math PDFs on the GCS mirror (a sample of months, listing from `mathlib gcs-list`)
-- versus source sizes from arXivRaw, to estimate how much the PDF route weighs for all of math.
-- Usage: mathlib sql $MOVA_DATA/db/math.duckdb sql/routes.sql gcs=<glob gcs-pdf-*.tsv> --mode markdown
CREATE OR REPLACE TABLE gcs_pdf AS
SELECT name, CAST(size AS BIGINT) AS size,
       regexp_extract(name, '/([0-9]{4}\.[0-9]{4,5})v([0-9]+)\.pdf$', 1) AS id,
       CAST(regexp_extract(name, '/([0-9]{4}\.[0-9]{4,5})v([0-9]+)\.pdf$', 2) AS INTEGER) AS v,
       regexp_extract(name, '/pdf/([0-9]{4})/', 1) AS yymm
FROM read_csv('{gcs}', delim = '\t', header = true, quote = '', escape = '', all_varchar = true);

.print '## GCS: sample of months — all objects and math'
WITH last_v AS (
    SELECT g.*, a.math_any, a.lic_class, a.size_kb, a.versions
    FROM gcs_pdf g LEFT JOIN arxiv a USING (id)
)
SELECT yymm,
       count(*) AS objects, count(DISTINCT id) AS papers, round(sum(size) / 1e9, 2) AS all_gb,
       count(DISTINCT id) FILTER (WHERE math_any) AS math_papers,
       round(sum(size) FILTER (WHERE math_any) / 1e9, 2) AS math_all_versions_gb,
       round(sum(size) FILTER (WHERE math_any AND v = versions) / 1e9, 2) AS math_last_gb,
       round(sum(size_kb) FILTER (WHERE math_any AND v = versions) * 1024 / 1e9, 2) AS math_src_last_gb,
       round(median(size) FILTER (WHERE math_any AND v = versions) / 1e6, 2) AS math_pdf_median_mb,
       round(sum(size) FILTER (WHERE math_any AND v = versions) / nullif(sum(size_kb * 1024) FILTER (WHERE math_any AND v = versions), 0), 2) AS pdf_to_src
FROM last_v GROUP BY yymm ORDER BY yymm;
