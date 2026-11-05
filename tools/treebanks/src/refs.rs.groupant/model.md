# treebanks-refs

## descriptant

### Descriptant

Unicode token folding retains original UTF-8 byte boundaries. Index stores first id/DOI and full/article-skipped title variants. Matcher gathers URL and ACL DOI ids, other DOIs, bibtex, then library titles followed by ACL titles; contained ranks longest spans first and skips overlaps. best prefers exact year, author count, ACL-bearing record then index. lift maps ACL match into library by anthology id; duplicates keep first method. near records exactly-one-failed checks and excludes final hits.

## approximant

# Forward approximant

Unicode token folding retains original UTF-8 byte boundaries. Index stores first id/DOI and full/article-skipped title variants. Matcher gathers URL and ACL DOI ids, other DOIs, bibtex, then library titles followed by ACL titles; contained ranks longest spans first and skips overlaps. best prefers exact year, author count, ACL-bearing record then index. lift maps ACL match into library by anthology id; duplicates keep first method. near records exactly-one-failed checks and excludes final hits.

## Composition

Child fragments model one narrow helper each; their correctness alone does not establish orchestration.

The containing source wires this scoped relation to its callers. Treeagree proposal→gate→commit and treebanks parse→match→collect→render are actual joins; parsecmp is an independent diagnostic path. The external group model distinguishes these tags.

## catalog

# Typed models for treebanks-refs

## behavior_complement

Missing author/year/boundary cannot become free-title hit unless allowed fuzzy year/boundary alternatives apply. Short titles (<3 words or <20 chars) never enter trigram search. Explicit identifiers can still match without those title constraints.

Operator: inner exclusion on admitted domain. Evidence: source branch derivation plus model.md#tests; unexercised branches remain hypotheses. Oracle: evaluate the scoped actual helper on admissible fixtures and compare this observation. No universal proof.

## reverse_reconstruction

Hit preimages include explicit id strings and normalized title/author/year variants; normalization is many-to-one, and best tie-break loses candidate identities except selected alts. Not a unique paper correctness proof.

Operator: compatible antecedent relation, retaining ambiguity. Evidence: source branch derivation plus model.md#tests; unexercised branches remain hypotheses. Oracle: evaluate the scoped actual helper on admissible fixtures and compare this observation. No universal proof.

## span_priority

Library contained consumes spans before ACL regardless of global longest cross-index title; length priority is local per index pass.

Operator: specialized observation relation under the stated fixed inputs. Evidence: source branch derivation plus model.md#tests; unexercised branches remain hypotheses. Oracle: evaluate the scoped actual helper on admissible fixtures and compare this observation. No universal proof.

## year_tolerance

Free text needs >=6 alphabetic title words for ±1 year; bibtex needs >=4; exact year always wins candidate scoring.

Operator: specialized observation relation under the stated fixed inputs. Evidence: source branch derivation plus model.md#tests; unexercised branches remain hypotheses. Oracle: evaluate the scoped actual helper on admissible fixtures and compare this observation. No universal proof.

## byte_coordinates

toks offsets are original bytes; acl_ids finds offsets on lowercased string then indexes original, so length-changing Unicode before URL is an unsupported safety/fidelity domain.

Operator: specialized observation relation under the stated fixed inputs. Evidence: source branch derivation plus model.md#tests; unexercised branches remain hypotheses. Oracle: evaluate the scoped actual helper on admissible fixtures and compare this observation. No universal proof.

## method_identity

Duplicate (src,idx) hits keep first How; a later title match cannot downgrade an earlier URL hit, but still consumes its span.

Operator: specialized observation relation under the stated fixed inputs. Evidence: source branch derivation plus model.md#tests; unexercised branches remain hypotheses. Oracle: evaluate the scoped actual helper on admissible fixtures and compare this observation. No universal proof.

## tests

### Bounded evidence

- `refs::tests::normalization_and_ids`
- `refs::tests::title_matching_positive`
- `refs::tests::title_matching_negative_controls`
- `refs::tests::ids_and_bibtex`
- `refs::year_tests::fuzzy_year_article_and_detex`
- `refs::line_tests::title_line_then_authors_line`

These names passed twice in debug and twice in release; the complete treebank/comparison slice has 25 target-qualified names and identical sorted outcomes in all four runs. Ten compiled behavior mutations (one per owned source) failed named runtime assertions; exact original/restored SHA-256 equality and positive reruns were checked. Zero assertion-only mutants, zero surviving exploratory mutants, zero production fixes. Fresh raw logs, numeric exits, commands and mutation hashes are external in the wave33 trees receipt set.

#### Scope and gaps

Only named finite assertions cover selected inputs; other branches and invalid-domain behavior remain source-derived. No corpus, network, downloads, annotation model, live teacher, full release build or site execution. The treebanks join is entirely in memory and parsecmp uses a tiny uniquely named temporary directory. Treeagree Registry::load uses the embedded UD registry. Tests are finite CPU observations, not universal semantic proof, title-match quality estimates or corpus accuracy.

## invariant-and-cost

### Invariant and cost

Index O(title bytes plus entries), expected hash lookups. For T item tokens and C title candidates of length L, contained O(T+C*(L+T+authors)) plus span sorting and overlap scans O(S²); raw token/string costs additional. Library scans bounded directory depth and reads YAML; no downloads.

Missing author/year/boundary cannot become free-title hit unless allowed fuzzy year/boundary alternatives apply. Short titles (<3 words or <20 chars) never enter trigram search. Explicit identifiers can still match without those title constraints.

The complement and specialized catalog constrain preserved coordinates and effect boundaries. Empty/malformed domains are only guaranteed where explicitly admitted. No whole-program cost bound follows from a helper bound.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective source reading identified conditional input alignment, information loss and failure order before tests. Existing production code is retained; added tests use actual helpers. Finite results update this source-bound model, not all parser/corpus semantics.

No executable percentage surrogate has a justified observation metric for these categorical/effectful APIs. Future profiles require metric, domain and real-reference evidence at caps 1/2/5/10%; no higher cap. Syntax is partial lexical Rust projection without macro expansion or semantic resolution.

### 2026-10-31 · tests

Heading `### Bounded evidence — 2026-10-31` stays in tests as `### Bounded evidence`.

Source `tools/treebanks/src/refs.rs` at SHA-256 `478c68b66f1dd86a691861bb26d634ffce9dd47aef7572e437a7fa1cc6e95db5`. Selected actual-source tests in `treebanks`:

### Undated · document

Subject: source SHA-256 `478c68b66f1dd86a691861bb26d634ffce9dd47aef7572e437a7fa1cc6e95db5`; manifest supplies exact source/byte scope. Domain: admitted states in forward model, nonoverflow arithmetic and successful allocations. Observation: one terminating invocation, except explicitly described file trace. Source-derived partial relations; named finite tests constrain selected cases only. Each entry holds arguments and environment fixed except its stated intervention.
