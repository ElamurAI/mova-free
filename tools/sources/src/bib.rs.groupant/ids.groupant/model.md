# sources-bibliography-ids

## descriptant

### sources-bibliography-ids

Validate default topic; read/parse wanted lines; read/parse bibliography; map last entry per nonempty ID; emit ordered rows and missing; reject missing before write.

## approximant

### Forward model

Validate default topic; read/parse wanted lines; read/parse bibliography; map last entry per nonempty ID; emit ordered rows and missing; reject missing before write.

Domain and assumptions: First requested duplicate wins after validation; last bibliography duplicate wins. read_ids domain priority 1..3; helper manifest_for_ids permits all u8.

This source-derived relation abstracts allocation, formatting and external failures; no executable percentage surrogate is asserted. Categorical decisions/effects have no supported error metric yet. Future budget caps are 1%, 2%, 5%, 10% only.

## catalog

# Typed model catalog

Subject: exact manifest source SHA-256 and optional byte scope. Domain/assumptions: forward and invariant companion. Observation is one completed helper transition or the stated IO prefix; external interference is excluded for finite fixtures. All models are partial source-derived specifications; the complement is an inner set of excluded behaviors, the reverse a compatible-antecedent relation. Evidence: model.md#tests; unsampled branches are not proven.

## behavior_complement

Any missing ID excludes manifest write; malformed duplicate line still rejects before deduplication.

Operator and approximation: the stated constraint on the source transition, with all unmentioned state held fixed; not a total solver or uniqueness claim. Falsifiable oracle: compare actual helper outputs/effect order on a bounded input satisfying the domain. Current executed subset and unsupported observations are listed in model.md#tests.

## reverse_reconstruction

Successful rows constrain surviving wanted order but not duplicate lines or shadowed bibliography entries; missing list only constrains unmatched IDs.

Operator and approximation: the stated constraint on the source transition, with all unmentioned state held fixed; not a total solver or uniqueness claim. Falsifiable oracle: compare actual helper outputs/effect order on a bounded input satisfying the domain. Current executed subset and unsupported observations are listed in model.md#tests.

## first_request

Seen insertion follows validation, retaining earliest valid ID topic/priority.

Operator and approximation: the stated constraint on the source transition, with all unmentioned state held fixed; not a total solver or uniqueness claim. Falsifiable oracle: compare actual helper outputs/effect order on a bounded input satisfying the domain. Current executed subset and unsupported observations are listed in model.md#tests.

## last_entry

HashMap collect overwrites earlier entry under equal anthology_id, independently of requested order.

Operator and approximation: the stated constraint on the source transition, with all unmentioned state held fixed; not a total solver or uniqueness claim. Falsifiable oracle: compare actual helper outputs/effect order on a bounded input satisfying the domain. Current executed subset and unsupported observations are listed in model.md#tests.

## failure_precedence

Default-topic failure precedes ids read/parse, bib read/parse, missing check, write.

Operator and approximation: the stated constraint on the source transition, with all unmentioned state held fixed; not a total solver or uniqueness claim. Falsifiable oracle: compare actual helper outputs/effect order on a bounded input satisfying the domain. Current executed subset and unsupported observations are listed in model.md#tests.

## priority_domain

Missing/empty priority defaults 1; numeric 0/4, nonnumeric and extra columns reject.

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

Compiled source controls change bibliography license threshold, sources size guard and value status guard; targeted assertions must reject, then exact backup restoration and positive rerun. Finite controls are not comprehensive fault coverage. See ../model.md#evidence at source root for receipts summary.

## invariant-and-cost

### Invariants and costs

First requested duplicate wins after validation; last bibliography duplicate wins. read_ids domain priority 1..3; helper manifest_for_ids permits all u8.

O(input/output bytes) plus expected hash operations; O(E+W) maps/vectors. Filesystem failures can stop before output.

Bounds are symbolic, conditional on stated domain and successful allocation. No unmeasured speedup or universal finite-test guarantee.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective full-source read preceded edits. Model separates source-derived effects from bounded actual-helper evidence. Bibliography host identity old code admitted foreign authorities; a failing fixture motivated the narrow HTTP(S) exact-authority repair. Other production bytes were retained; added test modules are cfg(test). Topic run now uses the same canonical ID guard for JSONL admission; an old-code failing fixture confirms foreign matched entries remain only in abstract listings. Namespace compilation is representation evidence, not independent runtime coverage. Value workflow and bench transport remain unexecuted. No live data, network, model, DB or benchmark invocation was used.
