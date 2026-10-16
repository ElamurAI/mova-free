-- Paths are relative to the data directory ($MOVA_DATA): run duckdb from there.
-- Statistics of the arXiv math catalog (for the report). size_kb is the size of the latest version from arXivRaw.
-- Usage: mathlib sql $MOVA_DATA/db/math.duckdb sql/stats.sql --mode markdown
.print '## Overall'
SELECT count(*) AS records,
       count(*) FILTER (WHERE math_any) AS math_any,
       count(*) FILTER (WHERE math_primary) AS math_primary,
       count(*) FILTER (WHERE primary_cat = 'math-ph') AS mathph_primary,
       count(*) FILTER (WHERE math_any AND NOT math_primary) AS math_crosslist_only,
       count(*) FILTER (WHERE list_contains(cat_list, 'cs.SC')) AS cs_sc,
       count(*) FILTER (WHERE list_contains(cat_list, 'cs.SC') AND NOT math_any) AS cs_sc_not_math
FROM arxiv;

.print '## Licenses (math_any)'
SELECT license, lic_class, count(*) AS n, round(100.0 * count(*) / sum(count(*)) OVER (), 2) AS pct,
       round(sum(size_kb) * 1024 / 1e9, 2) AS src_gb
FROM arxiv WHERE math_any GROUP BY ALL ORDER BY n DESC;

.print '## License classes (math_any; open = CC BY / BY-SA / CC0 / PD)'
SELECT coalesce(lic_class, 'total') AS lic_class, count(*) AS n, round(200.0 * count(*) / sum(count(*)) OVER (), 2) AS pct,
       round(sum(size_kb) * 1024 / 1e9, 2) AS src_gb, round(sum(size_kb_all) * 1024 / 1e9, 2) AS all_versions_gb,
       round(median(size_kb)) AS median_kb, round(avg(size_kb)) AS avg_kb
FROM arxiv WHERE math_any GROUP BY ROLLUP (lic_class) ORDER BY n DESC;

.print '## Primary category (math_primary)'
SELECT primary_cat, count(*) AS n, count(*) FILTER (WHERE lic_class = 'open') AS open_n,
       round(sum(size_kb) * 1024 / 1e9, 2) AS src_gb, round(sum(size_kb) FILTER (WHERE lic_class = 'open') * 1024 / 1e9, 2) AS open_gb
FROM arxiv WHERE math_primary GROUP BY ALL ORDER BY n DESC;

.print '## Years (math_any)'
SELECT year, count(*) AS n, count(*) FILTER (WHERE lic_class = 'open') AS open_n,
       round(100.0 * count(*) FILTER (WHERE lic_class = 'open') / count(*), 1) AS open_pct,
       round(sum(size_kb) * 1024 / 1e9, 2) AS src_gb
FROM arxiv WHERE math_any GROUP BY ALL ORDER BY year;

.print '## Adjacent: math-ph, stat.TH, cs.SC, cs.LO, cs.DM — how many are already in math and how many not'
SELECT c AS cat, count(*) AS n, count(*) FILTER (WHERE math_any) AS in_math, count(*) FILTER (WHERE NOT math_any) AS not_in_math
FROM (SELECT unnest(cat_list) AS c, math_any FROM arxiv)
WHERE c IN ('math-ph', 'math.MP', 'stat.TH', 'math.ST', 'cs.SC', 'cs.LO', 'cs.DM', 'cs.IT', 'math.IT', 'cs.NA', 'math.NA')
GROUP BY ALL ORDER BY n DESC;
