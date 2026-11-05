# mova-seaspider-links

## model

# mova-seaspider-links

## Descriptant

Outgoing links from HTML by a one-pass byte scanner with nofollow marks, URL normalisation and per-page deduplication. Source: `tools/seaspider/src/links.rs`.

## Forward approximant

extract_rel: skip comments and script/style/noscript/template bodies; collect href of a/area, link rel=canonical|alternate|next|prev, first base href; nofollow from rel nofollow|ugc|sponsored, <noindex>, meta robots nofollow (marks all links). normalize: http(s) only, join with base, drop fragment and utm_* parameters and media/archive extensions. outgoing: absolute deduplicated links in first-occurrence order, nofollow only if every occurrence is.

## Invariant

The scanner terminates on any byte input (each step advances); outgoing never yields a non-http(s) URL, a fragment or a utm_ parameter.

## Cost

O(n) bytes scanned per page apart from find_ci on skipped elements (O(n x tag) worst case).

## Composition

crawl::run writes outgoing() to links.jsonl and, in follow mode, enqueues same-host non-nofollow links.

## behavior_complement

Not extracted: links built by JavaScript, srcset, forms, CSS url(), meta refresh, sitemaps. Counterexample: a link inside <script> text is not a link (script_style_comment_skipped). Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

From a links line one recovers the set and order of distinct absolute targets; the HTML, anchor texts and duplicate occurrences are not recoverable. No unique inverse is claimed.

## tests

### Checks

Six unit tests from the Spider plus outgoing_dedups_in_order; end-to-end links/follow test.

## History

### 2026-11-04 · tests

Command: cargo test --manifest-path tools/seaspider/Cargo.toml --offline --locked (27 unit + 9 end-to-end tests against a local 127.0.0.1 HTTP server; two identical runs on 2026-11-04).

These are finite fixtures; the only live run was the 2026-11-04 smoke against the project's own site.
