# mathlib-oai

## descriptant

### mathlib-oai

run creates directory, replays state, returns early for terminal token; otherwise initializes state, checks STOP/page limit, forms encoded URL, handles bounded consecutive 503 responses, parses OAI error/token, writes compressed page before durable state append.

## approximant

# Forward relation

run creates directory, replays state, returns early for terminal token; otherwise initializes state, checks STOP/page limit, forms encoded URL, handles bounded consecutive 503 responses, parses OAI error/token, writes compressed page before durable state append.

Model observations include returns, retained state and ordered partial effects; on external failures the successful-prefix relation replaces a total-function claim.

## Composition

run creates directory, replays state, returns early for terminal token; otherwise initializes state, checks STOP/page limit, forms encoded URL, handles bounded consecutive 503 responses, parses OAI error/token, writes compressed page before durable state append.

Scoped children isolate the declared contiguous source intervals. Their operations compose only where the source calls them; costs of overlapping parent/child descriptions are not added twice. Remaining effect paths retain explicit gaps.

## catalog

## behavior_complement

Terminal empty state token and STOP prevent requests. >20 consecutive 503 cannot continue sleeping; other HTTP status errors abort without checkpoint of that failed response. Source-derived network bounds.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## reverse_reconstruction

Last-state tuple admits many earlier lines, ignored short/header rows and unparseable counts mapped zero. Gzip page plus state constrains body/token at commit but cannot determine all request timing or discarded failed responses.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## checkpoint_order

Increment page/count, write gzip, finish+sync, append+sync state. State does not advance on page write failure; page may exist on append failure. Re-running can overwrite that uncheckpointed page.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## token_selection

resumption_token selects rfind textual tag, trims inner text, XML-unescapes with raw fallback. Missing, self-closing and explicit empty are distinct Option representations but all terminal after successful page.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## retry_partition

503 increases consecutive busy and waits clamp(retry_after or 30,5,600); successful text resets busy before OAI error parsing. NoRecordsMatch appends page+1 count zero; other OAI errors stop.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## record_count_observation

The checkpoint record count is body.matches('<record>').count(), a literal textual observation, not XML namespace-aware record counting. Tags with attributes or namespace prefixes are not counted; matching text in comments can contribute. It constrains the counter without promising semantic record cardinality.

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

Resume uses last valid >=3-cell nonheader line's page/token and cumulative parsed record counts. noRecordsMatch appends terminal state without gzip page. Successful page commit stores body before token checkpoint; crash can leave page ahead of state.

#### Cost

last_state O(state bytes); token/error substring scans O(body), gzip O(body) and full body memory. At most 20 waits for consecutive 503 before 21st abort; each clamped 5..600 seconds. Successful pagination itself unbounded.

#### Domain and gaps

No network/DNS/download, real catalog/corpus/model/teacher or database execution. Effect paths are source-derived; finite helper tests are not universal verification. No executable percentage surrogate or speed claim; future caps require metric/domain/reference and remain 1/2/5/10%.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective model retained before changes. Existing production bytes are preserved; additions are bounded inline tests. Nine source groupants expose ingestion and routing boundaries; typed children bind exact source bytes and reuse their parent syntax projection. Native tests run twice per profile; behavior mutants run as compiled source changes and exact bytes are restored before positive checks.

Important retained observation: network-looking helpers are not all pure (DNS inside github_host and books build); selection deliberately excludes those effects. Mixed-format manifest order is not deterministic, and error paths do not imply rollback.

No network/DNS/download, real catalog/corpus/model/teacher or database execution. Effect paths are source-derived; finite helper tests are not universal verification. No executable percentage surrogate or speed claim; future caps require metric/domain/reference and remain 1/2/5/10%.

### 2026-11-04 · descriptant

#### Epoch notes

- Comments repointed to consolidated docs, 2026-11-04.

### Undated · behavior_complement

Subject: mathlib-oai, source SHA-256 bad99249391507309f3ac75c270b7c802490e1b6ae7f6ffb54460d8790404846; finite transition observation.

### Undated · reverse_reconstruction

Subject: mathlib-oai, source SHA-256 bad99249391507309f3ac75c270b7c802490e1b6ae7f6ffb54460d8790404846; finite transition observation.

### Undated · checkpoint_order

Subject: mathlib-oai, source SHA-256 bad99249391507309f3ac75c270b7c802490e1b6ae7f6ffb54460d8790404846; finite transition observation.

### Undated · token_selection

Subject: mathlib-oai, source SHA-256 bad99249391507309f3ac75c270b7c802490e1b6ae7f6ffb54460d8790404846; finite transition observation.

### Undated · retry_partition

Subject: mathlib-oai, source SHA-256 bad99249391507309f3ac75c270b7c802490e1b6ae7f6ffb54460d8790404846; finite transition observation.
