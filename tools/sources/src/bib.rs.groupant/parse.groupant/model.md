# sources-bibliography-parse

## descriptant

### sources-bibliography-parse

Byte state (i,depth,fields,out) recognizes quoted/braced/bare values, consumes # tails and clean, then inserts lowercased field names.

## approximant

### Forward model

Byte state (i,depth,fields,out) recognizes quoted/braced/bare values, consumes # tails and clean, then inserts lowercased field names.

Domain and assumptions: ASCII delimiters scanned as bytes; normalization uses chars/NFC. Duplicate fields overwrite. Tolerant truncation is not grammar validation.

This source-derived relation abstracts allocation, formatting and external failures; no executable percentage surrogate is asserted. Categorical decisions/effects have no supported error metric yet. Future budget caps are 1%, 2%, 5%, 10% only.

## catalog

# Typed model catalog

Subject: exact manifest source SHA-256 and optional byte scope. Domain/assumptions: forward and invariant companion. Observation is one completed helper transition or the stated IO prefix; external interference is excluded for finite fixtures. All models are partial source-derived specifications; the complement is an inner set of excluded behaviors, the reverse a compatible-antecedent relation. Evidence: model.md#tests; unsampled branches are not proven.

## behavior_complement

clean removes all braces and performs no IO. Recognized # tail cannot become next field.

Operator and approximation: the stated constraint on the source transition, with all unmentioned state held fixed; not a total solver or uniqueness claim. Falsifiable oracle: compare actual helper outputs/effect order on a bounded input satisfying the domain. Current executed subset and unsupported observations are listed in model.md#tests.

## reverse_reconstruction

Equivalent whitespace, field-name case, braces and earlier duplicate fields share output; exact source not recoverable.

Operator and approximation: the stated constraint on the source transition, with all unmentioned state held fixed; not a total solver or uniqueness claim. Falsifiable oracle: compare actual helper outputs/effect order on a bounded input satisfying the domain. Current executed subset and unsupported observations are listed in model.md#tests.

## delimiter_state

Quoted closes at depth zero; braced closes at depth zero after opening increment. Escape byte skip applies only to quoted values.

Operator and approximation: the stated constraint on the source transition, with all unmentioned state held fixed; not a total solver or uniqueness claim. Falsifiable oracle: compare actual helper outputs/effect order on a bounded input satisfying the domain. Current executed subset and unsupported observations are listed in model.md#tests.

## concatenation_tail

Horizontal whitespace then # appends quoted/alphanumeric token; bare month macro text retained rather than expanded.

Operator and approximation: the stated constraint on the source transition, with all unmentioned state held fixed; not a total solver or uniqueness claim. Falsifiable oracle: compare actual helper outputs/effect order on a bounded input satisfying the domain. Current executed subset and unsupported observations are listed in model.md#tests.

## normalization_fiber

NFC accent conversion, brace stripping, whitespace split/join identifies clean('{A}  B') with clean('A B').

Operator and approximation: the stated constraint on the source transition, with all unmentioned state held fixed; not a total solver or uniqueness claim. Falsifiable oracle: compare actual helper outputs/effect order on a bounded input satisfying the domain. Current executed subset and unsupported observations are listed in model.md#tests.

## truncated_value

Unclosed braced/quoted value extends to available end. Existing tests cover listed fragments only, not all malformed Unicode boundaries.

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

ASCII delimiters scanned as bytes; normalization uses chars/NFC. Duplicate fields overwrite. Tolerant truncation is not grammar validation.

O(B) stored text; repeated suffix collections may be quadratic; BTreeMap adds log field count.

Bounds are symbolic, conditional on stated domain and successful allocation. No unmeasured speedup or universal finite-test guarantee.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective full-source read preceded edits. Model separates source-derived effects from bounded actual-helper evidence. Bibliography host identity old code admitted foreign authorities; a failing fixture motivated the narrow HTTP(S) exact-authority repair. Other production bytes were retained; added test modules are cfg(test). Topic run now uses the same canonical ID guard for JSONL admission; an old-code failing fixture confirms foreign matched entries remain only in abstract listings. Namespace compilation is representation evidence, not independent runtime coverage. Value workflow and bench transport remain unexecuted. No live data, network, model, DB or benchmark invocation was used.
