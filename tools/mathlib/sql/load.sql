-- Paths are relative to the data directory ($MOVA_DATA): run duckdb from there.
-- Math catalog: TSV from `mathlib parse` / `mathlib books` / `mathlib fetch` manifests → tables.
-- Usage: mathlib sql $MOVA_DATA/db/math.duckdb sql/load.sql arxiv=<arxiv.tsv> books=<books.tsv> files="<raw>/*/manifest*.tsv"
CREATE OR REPLACE TABLE arxiv AS
SELECT *,
       string_split(cats, ' ') AS cat_list,
       primary_cat LIKE 'math.%' AS math_primary,
       len(list_filter(string_split(cats, ' '), lambda c: c LIKE 'math.%' OR c = 'math-ph')) > 0 AS math_any
FROM read_csv('{arxiv}', delim = '\t', header = true, quote = '', escape = '', auto_detect = false,
  columns = {'id': 'VARCHAR', 'primary_cat': 'VARCHAR', 'cats': 'VARCHAR', 'n_cats': 'INTEGER',
             'license': 'VARCHAR', 'lic_class': 'VARCHAR', 'license_url': 'VARCHAR', 'year': 'INTEGER',
             'first_date': 'DATE', 'last_date': 'DATE', 'versions': 'INTEGER', 'size_kb': 'BIGINT',
             'size_kb_all': 'BIGINT', 'pages': 'INTEGER', 'msc': 'VARCHAR', 'acm': 'VARCHAR', 'doi': 'VARCHAR',
             'journal_ref': 'VARCHAR', 'title': 'VARCHAR', 'authors': 'VARCHAR', 'abstract': 'VARCHAR',
             'comments': 'VARCHAR', 'submitter': 'VARCHAR', 'source_type': 'VARCHAR', 'datestamp': 'DATE',
             'sets': 'VARCHAR', 'harvest': 'VARCHAR'});

CREATE OR REPLACE TABLE books AS
SELECT * FROM read_csv('{books}', delim = '\t', header = true, quote = '', escape = '', all_varchar = true);

-- union_by_name: the same glob may also match logs (`manifest.log.tsv` from old runs) —
-- they have other columns; their rows are dropped by `sha256 IS NOT NULL`
CREATE OR REPLACE TABLE files AS
SELECT * REPLACE (CAST(bytes AS BIGINT) AS bytes)
FROM read_csv('{files}', delim = '\t', header = true, quote = '', escape = '', all_varchar = true, filename = true, union_by_name = true)
WHERE sha256 IS NOT NULL;

SELECT 'arxiv' AS t, count(*) AS n FROM arxiv
UNION ALL SELECT 'books', count(*) FROM books
UNION ALL SELECT 'files', count(*) FROM files;
