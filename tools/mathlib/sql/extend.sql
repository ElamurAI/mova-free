-- Paths are relative to the data directory ($MOVA_DATA): run duckdb from there.
-- Wave 0+: while everyone else is busy, top up the math collection — up to ~10 GB including what exists;
-- LaTeX and compressed text; priority is material that teaches solving high-level problems.
-- Only CC BY / CC BY-SA / CC0 / public domain. Tiers (prio) give the download order:
--   1 olympiad and competition math, problem solving;
--   2 math.HO;
--   3 cs.SC; student-level methods (integration, equations, linear algebra) — the former pilot;
--   4 math.GM;
--   5 NT, CO, algebra, inequalities, geometry, analysis — expository/elementary/short;
--   6 the remaining short ones (≤ 25 pp.) from NT, CO, MG, CA, RA, GR, AC.
-- Shard = year (slug y2024 …): folders of a few thousand files. fetch never re-downloads existing files.
-- Usage: mathlib sql $MOVA_DATA/db/math.duckdb sql/extend.sql out=<export-list.tsv> gcs_out=<gcs-list.tsv>
CREATE OR REPLACE TEMP TABLE ext AS
WITH base AS (
    SELECT *, lower(title) AS lt, lower(title || ' ' || abstract) AS lta
    FROM arxiv
    WHERE license IN ('cc-by-4.0', 'cc-by-3.0', 'cc-by-sa-4.0', 'cc0-1.0', 'pd-cc-old')
      AND size_kb BETWEEN 1 AND 30720
      AND (math_any OR list_contains(cat_list, 'cs.SC'))
), f AS (
    SELECT *,
        regexp_matches(lta, '(olympiad|\bimo\b|putnam|competition problems?|contest problems?|mathematical competitions?|math(ematics)? competitions?|problem[- ]solving|problems and solutions|challenging problems|\busamo\b|\baime\b|\bamc ?1[02]\b)') AS k_olymp,
        regexp_matches(lt, '(inequalit)') AS k_ineq,
        regexp_matches(lt, '(triangle|euclidean geometry|incircle|circumcircle|cevian|ceva|menelaus|ptolemy|conic|polygon|tetrahedr|quadrilateral)') AS k_geom,
        regexp_matches(lt, '(\b(elementary|expository|survey|introduction|introductory|primer|tutorial|lecture notes)\b|notes on|a note on|simple proof|short proof|new proof|another proof|elementary proof|students?|undergraduate|teaching|classroom)') AS k_student,
        regexp_matches(lt, '\b(integration|integrals|antiderivatives?|(in)?definite integral|quadrature|risch)\b')
          AND NOT regexp_matches(lt, '\bintegral (points?|closures?|domains?|triangles?|bas(is|es)|lattices?|solutions?|forms?|(co)?homology|extensions?)\b') AS k_int,
        regexp_matches(lt, '(solv(e|es|ing)\b.*\b(equations?|systems?)\b|\bequations?\b.*\bsolv|polynomial systems?|gr(ö|oe|o)bner|resultants?|\b(quadratic|cubic|quartic|quintic) equations?|linear algebra|matri(x|ces)|determinants?|eigenvalue)') AS k_eqla
    FROM base
), t AS (
    SELECT *,
        CASE
            WHEN k_olymp THEN 1
            WHEN list_contains(cat_list, 'math.HO') THEN 2
            WHEN list_contains(cat_list, 'cs.SC') OR ((k_int OR k_eqla) AND k_student) THEN 3
            WHEN primary_cat = 'math.GM' THEN 4
            WHEN primary_cat IN ('math.NT', 'math.CO', 'math.RA', 'math.AC', 'math.GR', 'math.CA', 'math.MG', 'math.CV', 'math.FA')
                 AND (k_student OR k_ineq OR k_geom) AND coalesce(pages, 99) <= 40 THEN 5
            WHEN primary_cat IN ('math.NT', 'math.CO', 'math.MG', 'math.CA', 'math.RA', 'math.GR', 'math.AC')
                 AND coalesce(pages, 99) <= 25 THEN 6
        END AS prio,
        k_olymp::INT * 3 + k_student::INT * 2 + (k_ineq OR k_geom)::INT * 2 + (k_int OR k_eqla)::INT
          + coalesce(pages BETWEEN 1 AND 30, false)::INT AS score
    FROM f
)
SELECT * FROM t WHERE prio IS NOT NULL;

COPY (
    SELECT 'y' || year AS slug, id || 'v' || versions AS key,
           'https://export.arxiv.org/src/' || id || 'v' || versions AS url,
           replace(id, '/', '_') || 'v' || versions || '.*' AS save_as,
           license, license_url, replace(title, '"', '''') AS title, 'https://arxiv.org/abs/' || id AS source_page,
           prio, score, size_kb, primary_cat, pages
    FROM ext ORDER BY prio, score DESC, year DESC, id
) TO '{out}' (FORMAT csv, DELIMITER '\t', HEADER true, QUOTE '');

-- The same list for the GCS mirror (Kaggle, `gs://arxiv-dataset`): PDF of the latest version.
-- A route for when export.arxiv.org is closed (26.09 at 16:53 — HTTP 429): PDF only "where
-- nothing else exists", and LaTeX sources are currently reachable only via S3 (paid, needs an AWS account).
COPY (
    SELECT 'y' || year AS slug, id || 'v' || versions AS key,
           'https://storage.googleapis.com/arxiv-dataset/arxiv/' ||
             CASE WHEN contains(id, '/')
                  THEN split_part(id, '/', 1) || '/pdf/' || substr(split_part(id, '/', 2), 1, 4) || '/' || split_part(id, '/', 2)
                  ELSE 'arxiv/pdf/' || substr(id, 1, 4) || '/' || id END || 'v' || versions || '.pdf' AS url,
           replace(id, '/', '_') || 'v' || versions || '.pdf' AS save_as,
           license, license_url, replace(title, '"', '''') AS title, 'https://arxiv.org/abs/' || id AS source_page,
           prio, score, size_kb, primary_cat, pages
    FROM ext ORDER BY prio, score DESC, year DESC, id
) TO '{gcs_out}' (FORMAT csv, DELIMITER '\t', HEADER true, QUOTE '');

.print '## Top-up tiers (open licenses only)'
SELECT prio, count(*) AS papers, round(sum(size_kb) * 1024 / 1e9, 2) AS src_gb,
       count(*) FILTER (WHERE license LIKE 'cc-by-sa%') AS by_sa, count(*) FILTER (WHERE license IN ('cc0-1.0', 'pd-cc-old')) AS cc0_pd,
       round(sum(count(*)) OVER (ORDER BY prio) / 900.0, 1) AS cum_hours_at_900_per_h
FROM ext GROUP BY prio ORDER BY prio;
