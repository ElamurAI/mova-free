# sources-bibliography

## descriptant

### sources-bibliography

parse scans entries and normalizes values; manifest_for_ids maps IDs to last bibliography entries, walks wanted order, returns JSONL and missing IDs. run_ids writes only when no IDs missing. run groups title/proceedings/abstract matches by topic and minimum priority, sharing first matching prefix and per-entry slug. Topic JSONL admission uses the same canonical ID helper after rendering the abstract listing.

## approximant

### Forward model

parse scans entries and normalizes values; manifest_for_ids maps IDs to last bibliography entries, walks wanted order, returns JSONL and missing IDs. run_ids writes only when no IDs missing. run groups title/proceedings/abstract matches by topic and minimum priority, sharing first matching prefix and per-entry slug. Topic JSONL admission uses the same canonical ID helper after rendering the abstract listing.

Domain and assumptions: Bounded UTF-8, representable depth/indices, available allocation. Missing fields read empty. read_ids limits priority to 1..3; row accepts any u8. Host identity requires HTTP(S) and exact case-insensitive aclanthology.org authority, then last query/fragment-free path segment; no port/userinfo support. Topic run requires nonempty anthology_id for JSONL admission, preserving all matched abstract listings. Writes are not transactional. Exact-host admission and id extraction do not normalize row URLs: row appends '.pdf' to the original URL text, so query/fragment-bearing official URLs need not produce usable canonical PDF paths. The actual plan/check join fixture uses a query-free canonical path.

This source-derived relation abstracts allocation, formatting and external failures; no executable percentage surrogate is asserted. Categorical decisions/effects have no supported error metric yet. Future budget caps are 1%, 2%, 5%, 10% only.

## composition

### Local composition

parse scans entries and normalizes values; manifest_for_ids maps IDs to last bibliography entries, walks wanted order, returns JSONL and missing IDs. run_ids writes only when no IDs missing. run groups title/proceedings/abstract matches by topic and minimum priority, sharing first matching prefix and per-entry slug. Topic JSONL admission uses the same canonical ID helper after rendering the abstract listing.

Children are scoped observations of the same source and share syntax.json. Their state/output constraints must agree at actual call boundaries; they are not independent interchangeable stages. The library public namespace joins bibliography row JSON into sources Resource parsing, covered by the bounded plan/check join fixture. Value is an independent tagged orchestration path, not a bibliography caller.

## catalog

# Typed model catalog

Subject: exact manifest source SHA-256 and optional byte scope. Domain/assumptions: forward and invariant companion. Observation is one completed helper transition or the stated IO prefix; external interference is excluded for finite fixtures. All models are partial source-derived specifications; the complement is an inner set of excluded behaviors, the reverse a compatible-antecedent relation. Evidence: model.md#tests; unsampled branches are not proven.

## behavior_complement

Missing IDs exclude destination write in run_ids; foreign authority excludes nonempty anthology_id. No URLs are fetched. Inner exclusions derived from source, not universal malformed-input safety.

Operator and approximation: the stated constraint on the source transition, with all unmentioned state held fixed; not a total solver or uniqueness claim. Falsifiable oracle: compare actual helper outputs/effect order on a bounded input satisfying the domain. Current executed subset and unsupported observations are listed in model.md#tests.

## reverse_reconstruction

Rows admit all BibTeX and requested lists yielding same last-ID map and projection. Braces, whitespace, shadowed entries and duplicate requests are unrecoverable; output is many-to-one.

Operator and approximation: the stated constraint on the source transition, with all unmentioned state held fixed; not a total solver or uniqueness claim. Falsifiable oracle: compare actual helper outputs/effect order on a bounded input satisfying the domain. Current executed subset and unsupported observations are listed in model.md#tests.

## priority_lattice

Per entry/topic priority is minimum of title/proceedings hit 1 and abstract hit 2. Each pattern conjunction holds within one field; cross-field split does not match.

Operator and approximation: the stated constraint on the source transition, with all unmentioned state held fixed; not a total solver or uniqueness claim. Falsifiable oracle: compare actual helper outputs/effect order on a bounded input satisfying the domain. Current executed subset and unsupported observations are listed in model.md#tests.

## license_threshold

parse_u32(year).unwrap_or(0)>=2016 selects open; otherwise open-nc. Malformed year belongs to zero-year fiber, not external license verification.

Operator and approximation: the stated constraint on the source transition, with all unmentioned state held fixed; not a total solver or uniqueness claim. Falsifiable oracle: compare actual helper outputs/effect order on a bounded input satisfying the domain. Current executed subset and unsupported observations are listed in model.md#tests.

## slug_collision

Base prefix-year-lastSurname-upToFiveNonSTOPWords; a collision appends ASCII anthology ID once. Third identical entry may repeat fallback slug; no global uniqueness theorem.

Operator and approximation: the stated constraint on the source transition, with all unmentioned state held fixed; not a total solver or uniqueness claim. Falsifiable oracle: compare actual helper outputs/effect order on a bounded input satisfying the domain. Current executed subset and unsupported observations are listed in model.md#tests.

## host_boundary

Authority equality excludes lookalikes/path matches/userinfo/ports. HTTP(S) and host case ignored, path case retained. Empty path returns empty; percent escapes are not decoded. Exact-host admission and id extraction do not normalize row URLs: row appends '.pdf' to the original URL text, so query/fragment-bearing official URLs need not produce usable canonical PDF paths. The actual plan/check join fixture uses a query-free canonical path. Topic run excludes foreign hosts from JSONL while retaining their matched abstract listing.

Operator and approximation: the stated constraint on the source transition, with all unmentioned state held fixed; not a total solver or uniqueness claim. Falsifiable oracle: compare actual helper outputs/effect order on a bounded input satisfying the domain. Current executed subset and unsupported observations are listed in model.md#tests.

## tests

### Checks and limits

Exact target-qualified native names for this source family:

- bib::tests::latex_accents_become_unicode
- bib::tests::concatenated_month_keeps_following_fields
- bib::tests::malformed_snippets_do_not_panic
- bib::ids_tests::ids_to_manifest
- bib::ids_tests::slug_words_keep_accented_capitals
- bib::ids_tests::unknown_id_and_bad_topic_are_red
- bib::wave33_tests::ids_missing_preserves_output_and_license_boundary
- bib::wave33_host_tests::anthology_id_requires_exact_http_host
- bib::wave33_topic_tests::foreign_matching_entry_stays_in_abstracts_not_manifest

Run each selected exact name twice debug and twice release, one test thread, offline/locked dependencies. External wave33 sources receipts contain numeric statuses/raw logs and sorted outcome comparisons. Scoped children inherit only relevant evidence: parse/IDs directly exercised; admission/check/merge actual helpers exercised; subprocess actual helper exercised. bench_transport has source-derived evidence only, never an executed benchmark. Library namespace is compile/representation evidence.

Compiled source controls change bibliography license threshold, sources size guard and value status guard; targeted assertions must reject, then exact backup restoration and positive rerun. Finite controls are not comprehensive fault coverage. See model.md#evidence at source root for receipts summary.

## evidence

### Wave33 bounded evidence

Four source groupants, seven scoped child groupants and 66 typed models. Four independently paired syntax projections compare byte-for-byte; children share projections with explicit byte ranges. No new Rust source files.

Seventeen exact target-qualified tests pass twice debug/twice release (68 exits zero), with identical sorted named outcomes. sources library ten; sources binary six; value binary one. Namespace is compile/representation evidence plus actual parse/row projection. Synthetic bibliography-to-Resource-to-plan/check and topic admission fixtures exercise actual helpers. Value executes bounded sh only; bench/main never run.

Three compiled behavior mutants reject at runtime: bibliography license threshold and sources size guard via direct assertions; value disabled exit-status guard via unwrap_err panic on unexpected Ok. Each original/restored SHA-256 pair and byte comparison matches, followed by positive rerun. No assertion-only or surviving mutants. This is finite sensitivity, not universal fault coverage.

Production repair has two connected sites. Entry::anthology_id rejects foreign authorities and accepts canonical case-insensitive HTTP(S), stripping query/fragment before last path segment. Topic run now uses that canonical helper for JSONL admission after rendering abstracts. Old actual-helper witnesses fail (exit 101) for lookalike identity and two-row foreign topic admission; repaired fixtures pass, retaining foreign matched abstract text but only official JSONL. Relative URLs, userinfo and ports unsupported. Other production bytes unchanged; appended modules cfg(test).

Final exact tests, compiled controls and syntax receipts were refreshed after the topic repair. Earlier receipts remain in previous-receipts externally. Base-registry structural validation passes; integration owns registration and ancestor bindings. Initial disconnected augmented-registry attempt remains an honest diagnostic, not a passing check.

Raw logs/statuses, backups/hashes, outcomes, prospective documents, native metadata and production-delta.json reside in external wave33 sources receipts. No external network/data/model/DB, benchmark workflow, publication, universal proof, speed or percentage-surrogate claim.

## invariant-and-cost

### Invariants and costs

Bounded UTF-8, representable depth/indices, available allocation. Missing fields read empty. read_ids limits priority to 1..3; row accepts any u8. Host identity requires HTTP(S) and exact case-insensitive aclanthology.org authority, then last query/fragment-free path segment; no port/userinfo support. Topic run requires nonempty anthology_id for JSONL admission, preserving all matched abstract listings. Writes are not transactional. Exact-host admission and id extraction do not normalize row URLs: row appends '.pdf' to the original URL text, so query/fragment-bearing official URLs need not produce usable canonical PDF paths. The actual plan/check join fixture uses a query-free canonical path.

B bytes,E entries,R rules,P patterns,L field length: O(B) storage plus outputs, matching O(E R P L), ordered maps add log E; repeated suffix scanning/accents may be superlinear. No timing claim.

Bounds are symbolic, conditional on stated domain and successful allocation. No unmeasured speedup or universal finite-test guarantee.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective full-source read preceded edits. Model separates source-derived effects from bounded actual-helper evidence. Bibliography host identity old code admitted foreign authorities; a failing fixture motivated the narrow HTTP(S) exact-authority repair. Other production bytes were retained; added test modules are cfg(test). Topic run now uses the same canonical ID guard for JSONL admission; an old-code failing fixture confirms foreign matched entries remain only in abstract listings. Namespace compilation is representation evidence, not independent runtime coverage. Value workflow and bench transport remain unexecuted. No live data, network, model, DB or benchmark invocation was used.

### 2026-10-31 · evidence

Heading `### Wave33 bounded evidence — 2026-10-31` stays in evidence as `### Wave33 bounded evidence`.
