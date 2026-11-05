# mathlib-oai-state

## descriptant

### mathlib-oai-state

If state absent return None; otherwise scan lines, skip short/header rows, parse page strictly, add records with malformed count->0, replace last tuple with current page,total,token.

## approximant

### Forward relation

If state absent return None; otherwise scan lines, skip short/header rows, parse page strictly, add records with malformed count->0, replace last tuple with current page,total,token.

Model observations include returns, retained state and ordered partial effects; on external failures the successful-prefix relation replaces a total-function claim.

## catalog

## behavior_complement

Malformed page in a qualifying line cannot produce Ok(last); parse error aborts even if later rows would be valid. Missing file cannot produce a resume tuple.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## reverse_reconstruction

For tuple (p,t,k), compatible logs have final qualifying page p/token k and counts summing t; earlier pages need not be monotone. Short lines and header lines can be inserted freely.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## count_default

Malformed records count contributes zero while page parse is strict. This asymmetry is intentional modeled behavior; state_resume_and_stop_without_requests tests it.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## terminal_token

Third field empty is preserved as empty String and run treats it as completed; a later nonempty token would supersede earlier completion when replayed.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## line_order

Reordering qualifying lines changes returned page/token and may preserve total. This is a fold over log order, not map keyed by page number.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## header_skip_precedence

Rows with fewer than three tab fields or first field exactly page are ignored before parsing numeric page/count. Whitespace around page is not trimmed. Adding an ignored header leaves the returned tuple invariant, whereas a qualifying malformed page causes error regardless of earlier valid state.

Subject and source identity: manifest.json. Domain: scoped forward/invariant; relation is source-derived and partial. Evidence: model.md#tests distinguishes actual checks from unexecuted effect paths; finite fixtures do not prove all inputs.

## tests

### Bounded evidence

Selected actual source checks:

- mathlib::oai::tests::tokens_and_errors
- mathlib::oai::wave33::state_resume_and_stop_without_requests

These checks are file-level evidence; fragment branches not reached by these names remain source-derived.

External wave33 mathlib receipt retains native-commands.json, numeric statuses, raw debug/release logs, sorted exact outcomes, mutation logs and original/restored hashes. Seventeen named checks pass twice per profile; six production behavior mutants reject in both profiles, with restored positives. Source syntax is regenerated twice after formatting and compared byte-for-byte. Structural validation tests bindings, not Rust semantics.

No network/DNS/download, real catalog/corpus/model/teacher or database execution. Effect paths are source-derived; finite helper tests are not universal verification. No executable percentage surrogate or speed claim; future caps require metric/domain/reference and remain 1/2/5/10%.

## invariant-and-cost

### Invariant and cost

#### Invariant

Returned page is last qualifying line, not maximum page. Count sums all qualifying records; token is third field without escaping.

#### Cost

O(file bytes) reads, O(longest line) working storage and cumulative u64 arithmetic.

#### Domain and gaps

No network/DNS/download, real catalog/corpus/model/teacher or database execution. Effect paths are source-derived; finite helper tests are not universal verification. No executable percentage surrogate or speed claim; future caps require metric/domain/reference and remain 1/2/5/10%.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective model retained before changes. Existing production bytes are preserved; additions are bounded inline tests. Nine source groupants expose ingestion and routing boundaries; typed children bind exact source bytes and reuse their parent syntax projection. Native tests run twice per profile; behavior mutants run as compiled source changes and exact bytes are restored before positive checks.

Important retained observation: network-looking helpers are not all pure (DNS inside github_host and books build); selection deliberately excludes those effects. Mixed-format manifest order is not deterministic, and error paths do not imply rollback.

No network/DNS/download, real catalog/corpus/model/teacher or database execution. Effect paths are source-derived; finite helper tests are not universal verification. No executable percentage surrogate or speed claim; future caps require metric/domain/reference and remain 1/2/5/10%.

### Undated · behavior_complement

Subject: mathlib-oai-state, source SHA-256 bad99249391507309f3ac75c270b7c802490e1b6ae7f6ffb54460d8790404846; finite transition observation.

### Undated · reverse_reconstruction

Subject: mathlib-oai-state, source SHA-256 bad99249391507309f3ac75c270b7c802490e1b6ae7f6ffb54460d8790404846; finite transition observation.

### Undated · count_default

Subject: mathlib-oai-state, source SHA-256 bad99249391507309f3ac75c270b7c802490e1b6ae7f6ffb54460d8790404846; finite transition observation.

### Undated · terminal_token

Subject: mathlib-oai-state, source SHA-256 bad99249391507309f3ac75c270b7c802490e1b6ae7f6ffb54460d8790404846; finite transition observation.

### Undated · line_order

Subject: mathlib-oai-state, source SHA-256 bad99249391507309f3ac75c270b7c802490e1b6ae7f6ffb54460d8790404846; finite transition observation.
