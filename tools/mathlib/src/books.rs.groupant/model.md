# mathlib-books

## descriptant

### mathlib-books

fetch caches raw OpenStax/AIM/OTL pages; build constructs Rows in OpenStax, sorted OTL subject/page, AIM order, then derives license/topic/PDF and cross-catalog joins. find_pdfs caches pages, skips prohibited candidates, ranks links and merges previous output rows.

## approximant

# Forward relation

fetch caches raw OpenStax/AIM/OTL pages; build constructs Rows in OpenStax, sorted OTL subject/page, AIM order, then derives license/topic/PDF and cross-catalog joins. find_pdfs caches pages, skips prohibited candidates, ranks links and merges previous output rows.

Model observations include returns, retained state and ordered partial effects; on external failures the successful-prefix relation replaces a total-function claim.

## Composition

fetch caches raw OpenStax/AIM/OTL pages; build constructs Rows in OpenStax, sorted OTL subject/page, AIM order, then derives license/topic/PDF and cross-catalog joins. find_pdfs caches pages, skips prohibited candidates, ranks links and merges previous output rows.

Scoped children isolate the declared contiguous source intervals. Their operations compose only where the source calls them; costs of overlapping parent/child descriptions are not added twice. Remaining effect paths retain explicit gaps.

## catalog

## behavior_complement

Inner exclusions: explicit closed license conflict forces Other, so the same row cannot remain allowed; NC/ND/unknown variants never satisfy BookLicense::allowed. Cached page save does not make a request. This does not imply build is network-free: GitHub DNS checks remain.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## reverse_reconstruction

For a TSV row y, compatible raw catalog rows are all schema inputs whose normalized title/surname, precedence-selected license and PDF projections produce y. Entity decoding, whitespace collapse and dropped fields create many antecedents; cross-catalog provenance cannot be uniquely reconstructed from the row alone.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## license_precedence

parse splits on or/and/semicolon/dual and selects minimum enum order; one classifies public-domain before CC0 before GFDL before CC tuple. build applies AIM precision, current OpenStax match, then explicit-closed veto in that order. Check book_licenses; schema-level precedence remains source-derived.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## catalog_join

Join key K=(ASCII-alphanumeric lower title, lower last word of first author). OTL/AIM maps retain first row per K. Distinct editions/authors can collide; matching K supplies PDF or license only under the source-specific predicates, not semantic identity.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## pdf_rank

Site rows require >=5 fields, rank<90 and not-book regex false. Minimum rank wins; equal ranks retain earlier. Missing or malformed rank=999 is excluded. looks_pdf strips query/fragment and recognizes .pdf or viewcontent.cgi. Check topics_and_hosts resolution; rank selection source-derived.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## science_priority

Science first tests title for chemistry, physics then statistics; only absent title match consults course, then Science-without-math falls back other. Topic and level remain separate observations: a physics title can yield science=physics and topic=not-math. Actual topics_and_hosts checks both.

Subject and source identity: manifest.json. Domain: scoped forward/invariant; relation is source-derived and partial. Evidence: model.md#tests distinguishes actual checks from unexecuted effect paths; finite fixtures do not prove all inputs.

## tests

### Bounded evidence

Selected actual source checks:

- mathlib::books::tests::book_licenses
- mathlib::books::tests::topics_and_hosts
- mathlib::books::tests::aim_list_parses

These checks are file-level evidence; fragment branches not reached by these names remain source-derived.

External wave33 mathlib receipt retains native-commands.json, numeric statuses, raw debug/release logs, sorted exact outcomes, mutation logs and original/restored hashes. Seventeen named checks pass twice per profile; six production behavior mutants reject in both profiles, with restored positives. Source syntax is regenerated twice after formatting and compared byte-for-byte. Structural validation tests bindings, not Rust semantics.

No network/DNS/download, real catalog/corpus/model/teacher or database execution. Effect paths are source-derived; finite helper tests are not universal verification. No executable percentage surrogate or speed claim; future caps require metric/domain/reference and remain 1/2/5/10%.

## invariant-and-cost

### Invariant and cost

#### Invariant

OTL repeated integer ids retain the first sorted-page occurrence (missing ids share -1). Title plus surname keys are normalization classes, not identities. License overrides precede PDF substitution, so PDF substitution is not a second license reconciliation pass. Output order follows row construction; HashMap count ties need not print stably.

#### Cost

For B input bytes, R rows and L extracted links: parsing/normalization O(B), sort O(P log P), hash joins expected O(R), AIM/link dedup can be quadratic. File and DNS/network latencies unbounded; cache bodies and rows use O(B+R) storage.

#### Domain and gaps

No network/DNS/download, real catalog/corpus/model/teacher or database execution. Effect paths are source-derived; finite helper tests are not universal verification. No executable percentage surrogate or speed claim; future caps require metric/domain/reference and remain 1/2/5/10%.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective model retained before changes. Existing production bytes are preserved; additions are bounded inline tests. Nine source groupants expose ingestion and routing boundaries; typed children bind exact source bytes and reuse their parent syntax projection. Native tests run twice per profile; behavior mutants run as compiled source changes and exact bytes are restored before positive checks.

Important retained observation: network-looking helpers are not all pure (DNS inside github_host and books build); selection deliberately excludes those effects. Mixed-format manifest order is not deterministic, and error paths do not imply rollback.

No network/DNS/download, real catalog/corpus/model/teacher or database execution. Effect paths are source-derived; finite helper tests are not universal verification. No executable percentage surrogate or speed claim; future caps require metric/domain/reference and remain 1/2/5/10%.

### Undated · behavior_complement

Subject: mathlib-books, source SHA-256 c45e71334ccd3618e9fad4d6117ac8048db42001ffea61d02fa6d4121d1e5060; finite transition observation.

### Undated · reverse_reconstruction

Subject: mathlib-books, source SHA-256 c45e71334ccd3618e9fad4d6117ac8048db42001ffea61d02fa6d4121d1e5060; finite transition observation.

### Undated · license_precedence

Subject: mathlib-books, source SHA-256 c45e71334ccd3618e9fad4d6117ac8048db42001ffea61d02fa6d4121d1e5060; finite transition observation.

### Undated · catalog_join

Subject: mathlib-books, source SHA-256 c45e71334ccd3618e9fad4d6117ac8048db42001ffea61d02fa6d4121d1e5060; finite transition observation.

### Undated · pdf_rank

Subject: mathlib-books, source SHA-256 c45e71334ccd3618e9fad4d6117ac8048db42001ffea61d02fa6d4121d1e5060; finite transition observation.
