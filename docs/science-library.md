# Science library — increment 1 (26.09.2026, wound down due to the subscription limit)

Brief: mathematics, physics, statistics and chemistry — "a large but not unlimited base"; take the
more fundamental material first; start small and grow incrementally. Increment 1 comes on top of the existing ~7 GB.
Mathematics is in `docs/math-library.md`; this file covers statistics, physics, chemistry and the new tool rules.

## Tool (`tools/mathlib`, branch `scilib`)

- `fetch`: the first request to each host fetches its robots.txt (RFC 9309: the `*` group or ours,
  longest match, `*`/`$`, Crawl-delay as a per-host pause; 4xx means no restrictions, 5xx means the host is closed).
- **GitHub safeguard via DNS** (`robots::github_host`): a GitHub host by name *or by address*
  (GitHub Pages under a custom domain: 185.199.108.0/22, 140.82.112.0/20, 2606:50c0::/32…) gets no
  HTTP request at all. The textbook catalog (`books`) also marks such books `github=true`.
- `books-fetch <dir> otl:<subject>` (physics, chemistry…); `books` reads all `otl-*-pN.json`,
  new column `science` (math/statistics/physics/chemistry/other); `BOOKS_ALL=1` also includes OpenStax books
  from the Science subject. The mathematics catalog is unchanged (checked: 23 columns identical).

## Finding: GitHub Pages under custom domains

In wave 0, five PDFs came from GitHub Pages hosts (the name did not give it away): minusthemath.com,
learnwithoutwalls.com, complexanalysis.org, danaernst.com, nordstrommath.com (15.4 MB in total,
`$MOVA_DATA/raw/math/textbooks/{otl-statistics-minus-the-math, otl-you-re-smarter-than-you-think-statistics,
aim-complex-analysis, aim-an-introduction-to-proof-via-inquiry-based-learning,
otl-introduction-to-game-theory-a-discovery-approach}`); two more sites were only scanned (mphitchman.com,
www.crumplab.com). In increment 1 there were two requests to learningstatisticswithr.com (also GitHub Pages), before
the DNS check existed. The licenses are open, but the rule "do not contact GitHub" was broken — these 5 PDFs should be
removed or kept at the operator's decision; the safeguard is now in the code.

## Verified licenses (increment 1)

| science | what | license | where |
|---|---|---|---|
| physics | OpenStax College Physics (1st ed., retired) | CC BY 4.0 (OpenStax API) | assets.openstax.org |
| physics | Crowell: Light and Matter, Simple Nature, Mechanics, Fields and Circuits, Modern Physics, General/Special Relativity, solutions; **LaTeX sources** `light_and_matter_source_code/lm.tar.gz` (230 MB → text only via `--compact-src`) | CC BY-SA 4.0 (Internet Archive metadata, uploaded by the author; the site says CC BY-SA 3.0) | archive.org (not lending) |
| physics | University Physics 1–3, College Physics 2e | **CC BY-NC-SA** — not downloading | OpenStax |
| chemistry | OpenStax Chemistry, Chemistry: Atoms First (1st eds., retired) | CC BY 4.0 | assets.openstax.org |
| chemistry | Chemistry 2e, Atoms First 2e, Organic Chemistry | **CC BY-NC-SA** — not downloading | OpenStax |
| statistics | OpenIntro Statistics | CC BY-SA 3.0, but the PDF only via Leanpub (email/registration), sources on GitHub → **skipped** | openintro.org |
| statistics | Learning Statistics with R | CC BY-SA 4.0, but the site is GitHub Pages → **skipped** | — |

Robots checked: storage.googleapis.com, physics.nist.gov, journal.r-project.org (this is CRAN at WU
Vienna, not GitHub), api.crossref.org, openintro.org — no robots (404); jstatsoft.org — only
/cache/; lightandmatter.com, archive.org, openstax.org — our paths are allowed; assets.openstax.org — 403
on robots.txt (per the RFC, no restrictions); open.umn.edu — Crawl-delay 120.

## Running in the background (not polled; how to check/stop)

- mathematics, tiers 1, 2, 3, 5 PDFs from GCS (≤3.5 GB): log `$MOVA_DATA/runs/mathlib-2026-09-26/arxiv-pdf.log`,
  stop — `touch $MOVA_DATA/raw/math/arxiv-pdf/STOP`;
- arXiv stat.* metadata (TH ME CO AP OT ML): log `$MOVA_DATA/runs/scilib-2026-09-26/oai-stat.log`,
  stop — `touch $MOVA_DATA/raw/science/stat/arxiv-oai/stat-<XX>/STOP` (pauses a subset) and
  `ps aux | grep '[o]ai-stat'` → kill only the chain shells;
- OTL physics catalog, then chemistry (120 s per page): log `$MOVA_DATA/runs/scilib-2026-09-26/otl-sci.log`.

Not done (limit): downloading the physics and chemistry textbooks, 30–50 JSS/R Journal papers (Crossref),
CODATA and the PubChem periodic table, `science.duckdb`. The lists and licenses above are ready for continuing.
