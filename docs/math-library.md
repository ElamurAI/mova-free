# Math library — wave 0

Original request, 26.09.2026:

> "download hundreds of thousands of math research papers"

The goal: an SLM (small language model) that computes at the level of a university math student. The same day the scope was narrowed:

> "No need for many math papers right now."

- arXiv math metadata stays: the catalog is small and will be needed to pick what is useful.
- Student-level textbooks are the priority. Open licenses only, 2 GB cap.
- Full-text pilot: up to ~200 papers and up to 1 GB, only what is useful to the SLM:
  - cs.SC (symbolic computation);
  - math.HO;
  - student-level methods: integration, equation solving, linear algebra.
- Disk space is tight. Before every download, run `df -h ~`. Less than 150 GB free means stop (until 16:50 the threshold was 200 GB).
- zbMATH Open: skip. Wave 1: only a short note on what to take once space frees up.

That evening:

> "While everyone is busy, keep downloading math too"

- Volume: up to ~10 GB including what is already downloaded.
- Format: LaTeX and compressed text. PDF only where nothing else exists.
- Priority: material that teaches solving advanced problems:
  - olympiad and competition math;
  - math.HO and papers on problem solving;
  - number theory, combinatorics, algebra, inequalities, geometry, analysis.
- Same licenses.
- Forbidden: shadow libraries and datasets taken down via DMCA (hendrycks/competition_math).
- Open problem sets with answers for evaluation: a separate list (below).

## Where things are

| what | where |
|---|---|
| tool | `tools/mathlib` (Rust), SQL in `tools/mathlib/sql/` |
| raw OAI-PMH responses (XML, gz) | `$MOVA_DATA/raw/math/arxiv-oai/<set>/pNNNNN.xml.gz` + `state.tsv` |
| catalog (DuckDB) | `$MOVA_DATA/db/math.duckdb`: tables `arxiv`, `books`, `files` |
| catalog TSV (DuckDB input) | `$MOVA_DATA/raw/math/arxiv.tsv` (866 MB), `books.tsv` |
| arXiv papers, PDF from the GCS mirror | `$MOVA_DATA/raw/math/arxiv-pdf/y<year>/<id>v<N>.pdf` + `list.tsv`, `manifest.jsonl`/`.tsv`, log |
| arXiv papers, LaTeX sources | `$MOVA_DATA/raw/math/arxiv-src/`: list ready, download stopped (HTTP 429, see route 2) |
| textbooks (PDF; OAPEN: PDF and text) | `$MOVA_DATA/raw/math/textbooks/<slug>/` + `manifest.jsonl`, `manifest-oapen.jsonl` |
| raw textbook catalog pages | `$MOVA_DATA/raw/math/textbooks/catalog/` |
| AWS prices, GCS listings | `$MOVA_DATA/raw/math/routes/` |
| runs and logs | `$MOVA_DATA/runs/mathlib-2026-09-26/` |

Papers and books do not go into git, only the code and this document.

## Licenses: what is allowed

| class | licenses | training a closed model | redistribution |
|---|---|---|---|
| **open** (free) | CC BY 3.0/4.0, CC0 1.0, old CC "public domain" | yes | yes, with attribution |
| **open, copyleft** | CC BY-SA 4.0, GNU FDL (textbooks) | for the Mova base (under CC BY-SA), yes; for **Mova Extended, no**: SA requires the same license (`licence/DATA.md`) | yes, under the same license |
| **nc** | CC BY-NC-SA, CC BY-NC-ND | no, reading and research only | no |
| **arxiv** | standard arXiv license (nonexclusive-distrib 1.0) and pre-2008 papers with no license field | no, reading and research only | no: only arXiv distributes |

Each paper's license code is the enum `raw::License` (arXiv) and `books::BookLicense` (textbooks). Negative
controls in tests: NC, ND, standard arXiv and unknown do not pass. The gate `en::license::commercial_gate`
will not let them through either. For Mova Extended it still has to be tightened with a separate list without SA.

**arXiv metadata is CC0 1.0** (arXiv API Terms of Use): the catalog may be stored and shared.

## arXiv math catalog (26.09.2026)

Collected 32 subsets `math:math:XX` and `cs:cs:SC`: 845 OAI-PMH pages, 1,075,897 records. After merging cross-listings, **789,040 papers** remain:

- `math_any` (at least one math.* or math-ph category): **787,109**;
  - math as primary category: 608,393;
  - math-ph primary: 34,820;
  - cross-listed into math from another primary: 178,716;
- cs.SC: 3,264, of which 1,931 outside math.

Coverage check: 29,900 records of the full `set=math` (23 pages before pausing) are all present in the union of subsets.

**Adjacent categories need not be added:** arXiv aliases already put them in math.
- math-ph ≡ math.MP: 92,608, all in math;
- stat.TH ≡ math.ST: 29,131;
- cs.IT ≡ math.IT: 56,264;
- cs.NA ≡ math.NA: 51,869.

Only cs.SC was worth collecting separately, and it is done. cs.DM and cs.LO without a cross-listing into math were not collected; they can be added in minutes (`--set cs:cs:DM`).

### Licenses (math_any)

| license | class | papers | % | sources, GB |
|---|---|---:|---:|---:|
| arXiv nonexclusive 1.0 | arxiv | 557 714 | 70.9 | 313.5 |
| CC BY 4.0 | open | 111 783 | 14.2 | 125.3 |
| no field (pre-2008) | arxiv | 74 324 | 9.4 | 4.0 |
| CC BY-NC-ND 4.0 | nc | 16 381 | 2.1 | 26.1 |
| CC BY-NC-SA 4.0 | nc | 9 991 | 1.3 | 13.7 |
| CC0 1.0 | open | 6 782 | 0.9 | 5.5 |
| CC BY-SA 4.0 | open | 5 802 | 0.7 | 6.4 |
| CC BY 3.0 | open | 1 941 | 0.2 | 0.4 |
| CC BY-NC-SA 3.0 | nc | 1 884 | 0.2 | 0.5 |
| old CC "public domain" | open | 507 | 0.1 | 0.1 |

- **Under CC BY, CC BY-SA, CC0 or public domain: 126,815 papers (16.1%)**, 137.7 GB of sources. Of these:
  - CC BY: 113,724;
  - CC BY-SA: 5,802;
  - CC0 and public domain: 7,289.
- **Under the standard arXiv license (reading and research only): 632,038 (80.3%)**, 317.5 GB.
- NC: 28,256 (3.6%).

Share of open papers: before 2008 almost zero, 2008–2019 1.2–3.5%, 2020 9%, from 2021 34–41% per year.

### Sizes

`size` from arXivRaw is the size of the submitted package: sources with figures. Not checked against files, because export is closed (429). Sum over latest versions:

- all math: **495.5 GB**, median 34 KB per paper, mean 615 KB;
- all versions: 830 GB;
- open: **137.7 GB** (median 48 KB).

By primary category the heaviest are math.NA (96 GB), math.OC (52 GB), math.DS (22 GB): figures. The rest are a few GB each.

### Years (math_any)

| years | papers per year | open |
|---|---:|---:|
| 1992–1999 | 360 → 3 350 | ~0 |
| 2000–2007 | 4 100 → 14 300 | ~0 |
| 2008–2014 | 15 600 → 32 200 | 1.2–1.7% |
| 2015–2019 | 34 900 → 43 000 | 1.7–3.5% |
| 2020 | 46 104 | 8.9% |
| 2021–2026 | 45 100 → 55 600 | 34–41% |

Full tables (by category, year, license) come from `sql/stats.sql`; snapshot in `$MOVA_DATA/runs/mathlib-2026-09-26/stats.md`.

## Routes

### 1. Metadata: arXiv OAI-PMH

- Endpoint: `https://oaipmh.arxiv.org/oai`. The old `export.arxiv.org/oai2` returns 301 to it.
- `arXivRaw` format: license, size of each version, dates, categories (first is primary), MSC, DOI.
- **Terms** (Terms of Use for arXiv APIs):
  - metadata under CC0;
  - no more than one request per 3 s, one connection across all our machines combined;
  - respect `Retry-After`.
  - We pause 4 s from the end of the previous response; on 503 wait as instructed; on 403/429 stop.
- **Speed.** The server returns the full `set=math` at ~24–34 s per page (1300 records), i.e. ~5–6 h for everything. Subsets `math:math:XX` take ~1 s per page. So we collect 32 subsets (AC…ST) plus `cs:cs:SC`. `parse` merges cross-listing duplicates by id, keeping the record with the later `datestamp`.
- **Token** `resumptionToken` is valid until the next midnight UTC. The server pages by `from=<date>&skip=<N>`. If the token expires (`badResumptionToken`), restart the subset: one subset takes minutes.
- **Updates.** To fetch new records, run `mathlib oai <dir> --set math:math:XX --from <date of last harvest>` into a new directory. `parse` takes the later `datestamp`.

### 2. Full texts from arXiv itself (`export.arxiv.org`)

- `https://export.arxiv.org/src/<id>v<N>`: sources (`.tar.gz`, `.gz` for a single .tex) or PDF if the author submitted only PDF. `e-print/…` redirects here too.
- Terms:
  - `info.arxiv.org/help/bulk_data`: programmatic downloading only via `export.arxiv.org` and politely; **do not download the whole corpus programmatically**; S3 is for that;
  - Terms of Use: paper content may be "retrieve, store, and use … for your own personal use, or for research purposes"; redistribution only with the rights holder's permission or under the paper's license;
  - Identify OAI: "Full-content harvesting not permitted (except by special arrangement)";
  - `export.arxiv.org/robots.txt`: `Disallow: /` for all robots.
- **On 26.09 at 16:53 the very first request got HTTP 429 "Rate exceeded."** Before that we had made ~880 OAI requests in 1.5 h (868 pages), and a HEAD on `/src/` at 15:09 returned 200. Per the fail-fast rule the route was closed; no retries.
- **Conclusion:** this route is good at most for targeted picks of tens to hundreds of papers, and only after a pause. For thousands, use S3.

### 3. Google Cloud mirror (Kaggle): `gs://arxiv-dataset`

- **Anonymously accessible:** `https://storage.googleapis.com/arxiv-dataset/<object>`, listing at `https://storage.googleapis.com/storage/v1/b/arxiv-dataset/o?prefix=…`. No key, no requester-pays: egress is paid by the bucket owner, **$0 for us**.
- **Contents:**
  - `arxiv/arxiv/pdf/YYMM/<id>v<N>.pdf`: one PDF per file, months 0704…2609, i.e. up to date;
  - `arxiv/<archive>/pdf/…`: old ids (math/…, alg-geom/…);
  - `ps/` and `html/`: for some papers;
  - `metadata-v5/`: outdated metadata snapshot (2020-08), not for us.
- **No LaTeX sources.** For the SLM the route is secondary: PDF → text loses formulas.
- Each PDF's license is the paper's license. Only the open class may be used for training.
- **Math PDF sizes.** Listing of months 1501, 1901, 2401 (`sql/routes.sql`), joined with the catalog:

  | month | objects (all categories) | math papers | math PDF, latest version | median | PDF / sources |
  |---|---:|---:|---:|---:|---:|
  | 1501 | 13 051 | 2 725 | 1.43 GB | 0.30 MB | 1.65 |
  | 1901 | 19 085 | 3 467 | 2.41 GB | 0.35 MB | 1.50 |
  | 2401 | 29 939 | 4 402 | 5.40 GB | 0.49 MB | 1.08 |

  Estimate for latest-version PDFs: all math ≈ 0.6–0.75 TB, open ≈ 150 GB (mostly 2021+, ratio ~1.1). All versions: roughly double.

### 4. AWS S3 requester-pays: `s3://arxiv`

- Region us-east-1. Tars of ~500 MB: `pdf/arXiv_pdf_YYMM_NNN.tar` and `src/arXiv_src_YYMM_NNN.tar`. Manifests: `pdf/arXiv_pdf_manifest.xml` and `src/arXiv_src_manifest.xml`.
- **Requires an AWS account** (with billing): requester-pays does not allow anonymous access even to the manifests. We have no account, so the manifests were not inspected.
- **Sizes** (arXiv):
  - PDF ≈ 2.7 TB, sources ≈ 2.9 TB (03.2023);
  - everything together ≈ 9.2 TB (04.2025), growing ~100 GB/month;
  - our estimate for 09.2026: ≈ 11–12 TB, of which sources ≈ 5–6 TB.
- **Prices** (AWS Price List API, us-east-1, published 2026-09-16/26):
  - internet egress: $0.09/GB for the first 10 TB per month, beyond the 100 GB/month free tier;
  - GET: $0.0004 per 1000.
- **Tars mix categories** (month by month), so there is no "math only" set. Three options:
  1. **All sources:** ≈ 5–6 TB × $0.09 ≈ **$450–550** and 5–6 TB of disk. Not suitable for us.
  2. **Math only via ranged GETs:** walk the tar headers with `Range` requests (512 B per member, ~2.7 million GETs ≈ **$1**) and fetch bytes of the needed papers only. Cost ≈ $0.09 × selected volume + ~$2:
     - open math papers (126,815, 137.7 GB of archives): **≈ $14**;
     - all math (495.5 GB): ≈ $47.
  3. **EC2 in us-east-1:** S3→EC2 within the same region is free. Filter there and pull only the selection at $0.09/GB, plus a few dollars for the instance.
- For LaTeX sources this is the only legal bulk route.

## Textbooks

**Catalog: 281 rows** (`books`, `$MOVA_DATA/raw/math/books.tsv`):
- AIM Open Textbook Initiative: 67 books (list and each book's page);
- Open Textbook Library: 189 from the mathematics section (JSON; `total_count` says 212, 189 visible);
- OpenStax: 25 in math (CMS API).

| source | total | allowed | NC | ND | "other" (copyright, terms without a license) |
|---|---:|---:|---:|---:|---:|
| AIM | 67 | 33 | 16 | 1 | 17 |
| OTL | 189 | 96 | 88 | 1 | 4 |
| OpenStax | 25 | 10 | 15 | 0 | 0 |
| **total** | 281 | **139** | 119 | 2 | 21 |

Allowed: CC BY, CC BY-SA, CC0, public domain, GFDL: 61 CC BY, 54 CC BY-SA, 23 GFDL, 1 PD. Topic and level are inferred from the title (`books::topic`); level `pre` (pre-university) is not downloaded.

**Findings:**
- **OpenStax moved its live editions (Calculus 1–3 and the 2e editions: Precalculus, College Algebra, Introductory Statistics…) to CC BY-NC-SA.** Only the first editions (status retired) and "Statistics" remain CC BY. OTL still lists the 2e editions as CC BY. We treat the license from the OpenStax API as authoritative.
- **LibreTexts:** catalog only, as instructed.
- **License conflict** (one source states closed terms, another an open license): not downloaded. This excluded Callahan "Calculus in Context" and Collingwood "Precalculus".
- "PDF" links in catalogs often lead to a page rather than a PDF. `books-pdf` takes the first book-like `.pdf` link on the page or site. Lab manuals, workbooks, answer keys and companions are filtered out.
- **Automation found no PDF for four strong books.** Two were added manually (`manual-list.tsv`, `manifest-manual.jsonl`, `sources verify` → ok=4):
  - Beezer, "A First Course in Linear Algebra" 3.50 (GFDL): the site moved from linear.ups.edu to linear.pugetsound.edu. Took the book and the **solutions manual**.
  - Hefferon, "Linear Algebra" (GFDL or CC BY-SA 2.5 at your choice, LICENSE file in the repository): **LaTeX sources from GitLab** and **answers to exercises** (jhanswer.pdf). `--compact-src` shrank the repository archive from 42.2 MB to **1.08 MB** of text: 293 files kept (259 .tex), 580 dropped (figures etc.). This is the first compression measurement: text is ~2.6% of the archive.

  Remaining: Austin, "Understanding Linear Algebra" (CC BY; only the workbook was found) and Active Calculus (CC BY-SA per AIM).

**Downloaded 38 PDFs, 0.806 GB** (`manifest.jsonl`; `sources verify` → ok=38; negative control with a corrupted md5 → FAIL):

| license | books | MB |
|---|---:|---:|
| CC BY | 16 | 450 |
| CC BY-SA | 13 | 86 |
| GFDL | 8 | 236 |
| public domain | 1 | 34 |

- **By topic:** calculus (Corral "Elementary Calculus", "Vector Calculus"), linear algebra (Kuttler, GNU Octave), differential equations (Lebl "Notes on Diffy Qs", Judson "ODE Project"), probability and statistics (Grinstead–Snell, Lane, OpenStax CC BY editions, etc.), discrete math (Levin, Keller–Trotter, Bogart, Davies, "Counting Rocks!"), algebra (Judson AATA), analysis (Lebl "Basic Analysis", Zakon, Howell "Complex Analysis"), proofs and logic (Ernst, Open Logic), number theory (Moser, Poritz), SageMath, precalculus (OpenStax CC BY, Yoshiwara, ORCCA).
- **Misses (7):** 404, 500, HTML instead of PDF, 403 Cloudflare (openscholarship.wustl.edu; host closed for the rest of the run).

**OAPEN: problem collections under CC BY 4.0**, license verified via MARC/ONIX in OAPEN itself:
- Siklos, "Advanced Problems in Mathematics: Preparing for University": STEP problems with solutions;
- Borovik, Gardiner, "The Essence of Mathematics Through Elementary Problems";
- Gardiner, "Teaching Mathematics at Secondary Level".

Downloaded PDF and OAPEN's ready-made text (`.txt`), 12 MB (`manifest-oapen.jsonl`, `sources verify` → ok=6). OAPEN HTML pages return 403, so we go through the REST API only.

## arXiv papers: extended download (tier by tier)

List: `sql/extend.sql`, only CC BY, CC BY-SA, CC0 and public domain, sources ≤ 30 MB:

| tier | what | papers | sources, GB |
|---:|---|---:|---:|
| 1 | olympiad and competition math, problem solving (olympiad, IMO, Putnam, competition, problem solving… in title or abstract) | 155 | 0.30 |
| 2 | math.HO | 855 | 1.23 |
| 3 | cs.SC; student-level methods (integration, equations, linear algebra in the title, plus a "student" marker), formerly the pilot | 836 | 0.97 |
| 4 | math.GM | 1 224 | 0.79 |
| 5 | NT, CO, algebra, CA, MG, CV, FA: survey or elementary, inequalities, elementary geometry, ≤ 40 pp. | 812 | 0.14 |
| 6 | remaining short ones (≤ 25 pp.) from NT, CO, MG, CA, RA, GR, AC | 9 029 | 1.29 |
| | **total** | **12 911** | 4.7 |

- **LaTeX route** (`export.arxiv.org/src`, `--compact-src`: only .tex/.bbl/.bib/.sty… are kept from the archive, figures dropped): closed, HTTP 429 from the first request.
- **So for now we download PDFs from the GCS mirror** ("PDF where nothing else exists"): free, 1 request per ~1.2 s, in tier order.
  - PDF cap: 7.4 GB; together with the rest of the library (metadata 1.3 GB, database 0.5 GB, textbooks 0.85 GB) this comes to ~10 GB.
  - `df` safeguard: < 150 GB means stop, checked every 10 files.
  - Pausing via `STOP` was tested: at 17:10 stopped at 596 files (0.905 GB), resumed from the same point.
  - List: `arxiv-pdf/list.tsv`, log: `$MOVA_DATA/runs/mathlib-2026-09-26/arxiv-pdf.log`.
- How to check, pause and resume:

  ```sh
  tail -3 $MOVA_DATA/runs/mathlib-2026-09-26/arxiv-pdf.log; ps aux | grep '[m]athlib-pdf'
  touch $MOVA_DATA/raw/math/arxiv-pdf/STOP     # pause before the next file; manifest is written
  rm $MOVA_DATA/raw/math/arxiv-pdf/STOP && $MOVA_DATA/runs/mathlib-2026-09-26/bin/mathlib-pdf fetch \
    $MOVA_DATA/raw/math/arxiv-pdf/list.tsv $MOVA_DATA/raw/math/arxiv-pdf $MOVA_DATA/raw/math/arxiv-pdf/manifest.jsonl \
    --delay-ms 1000 --max-bytes 7400000000 --file-max 40000000 --expect-pdf --min-free-gb 150   # does not re-download existing files
  target/release/sources verify $MOVA_DATA/raw/math/arxiv-pdf $MOVA_DATA/raw/math/arxiv-pdf/manifest.jsonl
  ```

- Once a LaTeX route opens (S3 or the 429 lifted), the same list `arxiv-src/list.tsv` with `--compact-src` will yield sources. The PDFs can then be removed.

## Open problem sets with answers, for evaluation

For the sub-agent that trains the math module. License and size checked via the Hugging Face API (`/api/datasets/…`, `datasets-server …/size`) on 26.09. None of these were downloaded.

| dataset | license | level | size | URL |
|---|---|---|---|---|
| Siklos, "Advanced Problems in Mathematics" (OBP) | CC BY 4.0 | university entrance (STEP), problems with full solutions | book, 2 MB PDF + 0.4 MB text (**downloaded**) | https://library.oapen.org/handle/20.500.12657/24356 |
| Borovik, Gardiner, "The Essence of Mathematics Through Elementary Problems" (OBP) | CC BY 4.0 | school olympiad → university, problems with solutions | book, 6 MB PDF + 0.7 MB text (**downloaded**) | https://library.oapen.org/handle/20.500.12657/24985 |
| Beezer, FCLA 3.50, solutions manual | GFDL | university, linear algebra | 0.7 MB PDF (**downloaded**) | https://linear.pugetsound.edu/download.html |
| Hefferon, "Linear Algebra", answers to exercises | CC BY-SA 2.5 / GFDL | university, linear algebra | 1.8 MB PDF (**downloaded**); exercises are in the LaTeX sources | https://hefferon.net/linearalgebra/ |
| OpenStax, CC BY editions (Precalculus, College Algebra, Introductory Statistics…), exercises with answers at the end | CC BY 4.0 | introductory university course | in the downloaded PDFs | https://openstax.org |
| GSM8K (`openai/gsm8k`) | MIT | school word problems, numeric answer | 17,584 rows (two configs), 5.9 MB | https://huggingface.co/datasets/openai/gsm8k |
| SVAMP (`ChilleD/SVAMP`) | MIT | simple word problems | 1,000, 0.2 MB | https://huggingface.co/datasets/ChilleD/SVAMP |
| AQuA-RAT (`deepmind/aqua_rat`) | Apache-2.0 | GMAT/GRE, algebra, multiple choice with rationale | 195,950 rows, 52 MB | https://huggingface.co/datasets/deepmind/aqua_rat |
| MathQA (`allenai/math_qa`) | Apache-2.0 | GRE level, multiple choice, solution program | count not checked: the HF viewer does not count it (dataset is a script) | https://huggingface.co/datasets/allenai/math_qa |
| MMLU, math subjects (`cais/mmlu`: abstract_algebra, college_mathematics, high_school_mathematics, elementary_mathematics) | MIT | school → university, multiple choice | hundreds per subject | https://huggingface.co/datasets/cais/mmlu |
| TheoremQA (`TIGER-Lab/TheoremQA`) | MIT | university, applying theorems (includes physics, CS, finance) | 800, 4.9 MB | https://huggingface.co/datasets/TIGER-Lab/TheoremQA |
| ProofNet (`hoskinson-center/proofnet`) | MIT | university textbook theorems (natural language + Lean 3) | 371, 0.2 MB | https://huggingface.co/datasets/hoskinson-center/proofnet |
| Lila (`allenai/lila`) | CC BY 4.0 | collection of 20+ datasets, from school to olympiad | 317,262, 78 MB; **check composition: some problems may come from MATH (DMCA)** | https://huggingface.co/datasets/allenai/lila |
| DeepMind Mathematics (`deepmind/math_dataset`) | no license in the HF card; original is Apache-2.0 (on GitHub, not checked; we do not touch GitHub) | generated school problems: arithmetic, algebra, calculus, probability | millions | https://huggingface.co/datasets/deepmind/math_dataset |

**Gray zone.** The dataset license is Apache-2.0, but the problems themselves come from competitions (AoPS, MAA AMC/AIME, IMO, etc.), and the compilers do not hold the rights. At most for internal evaluation, not training, with a provenance mark. The decision is the operator's.
- Omni-MATH (`KbsdJames/Omni-MATH`): 4,428, 3.4 MB.
- OlympiadBench (`Hothan/OlympiadBench`): 8,476, 104 MB.
- AIMO AIME (`AI-MO/aimo-validation-aime`): 90.
- NuminaMath-CoT (`AI-MO/NuminaMath-CoT`): 859,594, 1.2 GB.

**Forbidden:**
- `hendrycks/competition_math` (MATH): taken down via DMCA, `disabled: true` on HF;
- its copies and derivatives: `EleutherAI/hendrycks_math`, `lighteval/MATH`, MATH-500, PRM800K, MetaMathQA, anything containing MATH problems.

## Wave 1: in brief, once space frees up

1. **LaTeX sources of open math papers via S3** with ranged GETs:
   - 126,815 papers, 137.7 GB of archives, ≈ **$14**;
   - requires an AWS account (operator's decision);
   - only text goes to disk (`--compact-src`); volume not yet measured because export is closed. After that the GCS PDFs can be removed.
2. **Math books in OAPEN under CC BY**: Springer Open Access, OBP, etc. The OAPEN REST API serves PDF and ready-made text. Catalog by Thema classification P/PB and license from MARC.
3. **LaTeX sources of textbooks** where they are not on GitHub (like Hefferon on GitLab): 40 times more compact than PDF. Manually: Austin and Active Calculus.
4. **Add cs.DM and cs.LO** (metadata, minutes).

## The `tools/mathlib` tool

```sh
B=target/release/mathlib; M=$MOVA_DATA/raw/math; DB=$MOVA_DATA/db/math.duckdb
# 1. metadata: subset by subset (state in state.tsv; rerun resumes; STOP file pauses)
$B oai $M/arxiv-oai/math-AG --set math:math:AG        # …likewise for the 32 math.* categories and cs:cs:SC
# 2. catalog: arXivRaw → TSV → DuckDB
$B parse $M/arxiv.tsv $M/arxiv-oai/math-?? $M/arxiv-oai/cs-SC
$B books-fetch $M/textbooks/catalog                    # OpenStax, AIM (3 s), OTL (120 s, their Crawl-delay)
$B books $M/textbooks/catalog $M/books.tsv && $B books-pdf $M/textbooks/catalog $M/books.tsv && $B books $M/textbooks/catalog $M/books.tsv
$B sql $DB tools/mathlib/sql/load.sql arxiv=$M/arxiv.tsv books=$M/books.tsv "files=$M/*/manifest*.tsv"
$B sql $DB tools/mathlib/sql/stats.sql --mode markdown
$B gcs-list arxiv-dataset arxiv/arxiv/pdf/2401/ $M/routes/gcs-pdf-2401.tsv
$B sql $DB tools/mathlib/sql/routes.sql "gcs=$M/routes/gcs-pdf-*.tsv" --mode markdown
# 3. download: list → polite download → manifest → verification
$B sql $DB tools/mathlib/sql/books-pick.sql out=$M/books-list.tsv
$B fetch $M/books-list.tsv $M/textbooks $M/textbooks/manifest.jsonl --delay-ms 2000 --max-bytes 2000000000 --expect-pdf --kind textbook
$B sql $DB tools/mathlib/sql/extend.sql out=$M/arxiv-src/list.tsv gcs_out=$M/arxiv-pdf/list.tsv
$B fetch $M/arxiv-src/list.tsv $M/arxiv-src $M/arxiv-src/manifest.jsonl --delay-ms 3500 --compact-src   # LaTeX, when the route is open
$B fetch $M/arxiv-pdf/list.tsv $M/arxiv-pdf $M/arxiv-pdf/manifest.jsonl --delay-ms 1000 --max-bytes 7400000000 --expect-pdf
target/release/sources verify $M/textbooks $M/textbooks/manifest.jsonl
```

- **Licenses.** `fetch` downloads only allowed licenses: CC BY, CC BY-SA, CC0, public domain, GFDL (for textbooks). A list with any other license is an error before the first request.
- **Safeguards:**
  - free space in GiB, as in `df -h`, before start and every 10 files; threshold `--min-free-gb`, default 150;
  - total of files: `--max-bytes`; single file: `--file-max`;
  - 403/429: host closed for the rest of the run; 503: wait for `Retry-After`;
  - `STOP` next to the manifest: pause.
- **Log** of every row: `manifest.log` (appended): ok, miss, bad-signature, cap, stop-disk.
- **Manifest** `manifest.jsonl` is in the `tools/sources` format, one line per slug. Each file has `size`, `md5`, **`sha256`**, **`license`**, `license_url`, `key`, `content_type`, `fetched`. Compressed sources also have `orig_size`, `orig_sha256`, `text_files` and `dropped_files`. `sources verify` checks size, md5 and signature. Checkpoint every 100 files.
- **Reproducibility.** Raw pages and XML sit next to the data: the catalog can be rebuilt without network.
