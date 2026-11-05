# mova-seaspider-crawl

## model

# mova-seaspider-crawl

## Descriptant

The link downloader: queue loading, deterministic round-robin order, per-origin robots.txt with TTL and RFC status handling, reserved requests, re-checked redirects, records, links and follow mode. Source: `tools/seaspider/src/crawl.rs`.

## Forward approximant

run: normalise and deduplicate queue URLs, skip done ones, group FIFO per host, visit hosts round-robin in sorted order. visit: per hop resolve, robots_decision (fetch robots.txt via reserved requests and up to robots.max_redirects redirects: 2xx parse, 4xx except 429 allow all, 429/5xx/network unreachable for the retry period; record it), stop as disallowed or deferred (crawl-delay > cap), write the intent before the first page request, polite_get with host delay max(interval, crawl-delay), follow 3xx Location (fragment dropped) up to fetch.max_redirects. Store the final body, append the page record, write links for HTML, enqueue same-host non-nofollow links when follow is on and depth < max_depth and followed < max_pages.

## Invariant

No page request precedes an allowed robots decision for its origin; every request goes through Politeness; every redirect target gets its own robots decision and reservation; the visit order depends only on the queue and responses, not on timing.

## Cost

Wall time >= (requests on the busiest IP - 1) x 2 s; per page O(body) store work and O(html) link scan.

## Composition

Composes config, polite, fetch, robots, store and links; main drives fetch/robots commands.

## behavior_complement

Not done: concurrency, JavaScript, sitemaps, cross-process politeness, cross-host following, persistent robots cache, acting on noindex. Counterexample: a 302 to a disallowed path on another host stops before that request (redirects_are_rechecked; skipping the hop check turns it red). Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

From pages.jsonl the queue order, outcomes, chains, robots decisions and stored bodies are recoverable; the exact timing and the queue lines skipped as done or duplicate are not unique from the journal alone. No unique inverse is claimed.

## tests

### Checks

Nine end-to-end tests against a local server (robots, crawl-delay, unreachable robots, spacing, redirects, bounds, resume/determinism, links/follow/CLI, private addresses).

## History

### 2026-11-04 · document

#### Epoch 2026-11-04 (English)

2026-11-04: the robots note and explanation builders are shared (`note`, `explanation`, which adds `max_crawl_delay_secs` and `basis`), and `stored_explanation` decides from the latest robots record of an origin in the store within its TTL, without a request, with the crawler's status handling.

### 2026-11-04 · tests

Command: cargo test --manifest-path tools/seaspider/Cargo.toml --offline --locked (27 unit + 9 end-to-end tests against a local 127.0.0.1 HTTP server; two identical runs on 2026-11-04).

These are finite fixtures; the only live run was the 2026-11-04 smoke against the project's own site.

2026-11-04 (English): cargo test --manifest-path tools/seaspider/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin (31 unit + 9 end-to-end + 2 English tests on a local 127.0.0.1 server; two identical runs on 2026-11-04). Negative controls 2026-11-04: binding arguments to the first action instead of through the parse, allowing every robots decision, and ordering jobs by arrival only each turn the English tests red; restored afterwards.
