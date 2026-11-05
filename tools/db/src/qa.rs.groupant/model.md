# db-src-qa

## descriptant

### Question anchors and replacement staging

read_rows validates exact eight-column header, parses para then k as i64, requires k>=1 and at least one trimmed nonempty comma anchor. load rejects duplicate doc/para/k, queries deduplicated wanted docs/sents, checks run identities, docs, per-anchor sentence cardinality/doc/para and replacement row availability. Stages qids with anchors as JSON sids, executes SQL, checks row and bidirectional replacement totals.

#### Retained limitations

No SQL evaluation was substituted with a mock process. Missing DuckDB is a dependency gap, not passing database evidence. Partial syntax has no macro expansion/name resolution/types; Rust helpers and fixtures do not prove all models.

## approximant

### Question anchors and replacement staging forward relation

read_rows validates exact eight-column header, parses para then k as i64, requires k>=1 and at least one trimmed nonempty comma anchor. load rejects duplicate doc/para/k, queries deduplicated wanted docs/sents, checks run identities, docs, per-anchor sentence cardinality/doc/para and replacement row availability. Stages qids with anchors as JSON sids, executes SQL, checks row and bidirectional replacement totals.

#### Domain

Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

#### Approximation limits

Categorical parsing, exact identity, transactional effects and resource errors have no useful percentage approximation metric here. No executable surrogate is claimed. Future profiles, if justified, may use only 1%, 2%, 5%, 10% with a defined metric/reference/domain.

## composition

### Local scope composition

Parent relation: read_rows validates exact eight-column header, parses para then k as i64, requires k>=1 and at least one trimmed nonempty comma anchor. load rejects duplicate doc/para/k, queries deduplicated wanted docs/sents, checks run identities, docs, per-anchor sentence cardinality/doc/para and replacement row availability. Stages qids with anchors as JSON sids, executes SQL, checks row and bidirectional replacement totals.

Child fragments isolate selected mechanisms and do not cover the entire file. Invocation order and shared state remain the parent's relation. Costs of a child are included in parent cost, not added a second time; independent branch scopes are tagged. A child reverse relation must preserve intermediate state compatibility, not imply unique whole-file reconstruction.

Children: [{"id":"db-src-qa-question-parser","path":"question-parser.groupant"}]

## models

# Question anchors and replacement staging typed model catalog

Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Observation is one call/trace in the declared scope. Operators below are partial source-derived relations; complement is a proposed inner exclusion relation, reverse an outer compatible-antecedent relation retaining ambiguity. No universal proof follows from finite checks. Each oracle is the relevant named helper check below when it reaches the behavior; remaining SQL/effect cases are source review or proposed real-CLI checks.

## behavior_complement

k<=0, no nonempty anchors, foreign header or duplicate question key cannot reach successful exec. Cross-document/paragraph or ambiguous anchor cannot be silently linked. Existing replacement pointers exclude replacing that row again.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## reverse_reconstruction

Given stored anchors [sid1,sid1,sid2], compatible TSV anchor strings differ in trimming and empty comma fields but preserve duplicate/order after filtering. Stored new/old pointer pairs constrain prior row IDs and NULL replaced_by; they do not recover question provenance or original whitespace.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## anchor_coordinate_join

Each resolved anchor must have exactly one candidate with doc=docs[doc_key] and para=Some(input para). Missing para is incompatible even with numeric zero; duplicate input anchors are accepted and repeat the same sid.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## replacement_history

With replaces run, every question key must exist in that run and remain unreplaced. Old rows remain, new rows point backward and SQL marks old forward. Rust postcheck compares total linked/marked to requested replacement count, not each pair's identity.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## parser_failure_precedence

Width precedes para parse, then k parse/positivity, then nonempty anchor check. Header-only succeeds in read_rows and fails in load. Tests isolate boundaries without a process transport substitute.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## commit_accounting

run=max_run+1 and qid=max_qid+1+i assume no overflow/concurrent writer. Post reconciliation occurs after commit; an error here does not imply no DB changes.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## tests

### Selected checks

Named target-scoped checks for parent source: ["qa::wave31::rows_anchor_and_number_domain"]

Native commands and numeric status/log/outcomes live in external native-commands.json and receipts. Integration fixture checks use actual en::conllu parsing only. Legacy prag_layer_and_negative_controls, qa_table_loader_and_negative_controls and lossless_round_trip_and_negative_controls were inspected but not executed: real DuckDB unavailable. A fragment's listed parent tests may only cover part of its relation; SQL/query/transaction/IO-fault claims remain source-derived.

#### Negative controls

Six source-behavior mutations: quote doubling, TSV NULL rejection, QA positivity, prag bit truth, MWT counter, sentence boundary predicate. Each compiled, failed an assertion, restored exact original bytes and passed the selected positive. No assertion-only mutations and no surviving behavioral mutants.

## invariant-and-cost

### Question anchors and replacement staging invariants and costs

#### Invariant

Repeated anchors remain repeated in stored JSON; requested state-query IDs are deduplicated independently. Each replacement targets one old matching doc/para/k whose replaced_by is NULL. Source parser allows negative para, arbitrary qtype/ex_or_im and empty answer; DB CHECKs own remaining domain.

#### Cost

O(file bytes + anchor count + state rows + staged bytes) expected work; BTreeSet wanted IDs adds O(u log u), replacement map expected O(old rows + new rows). Full row/state vectors are retained; DB execution unmeasured.

Symbolic bounds are source-derived, not measured wall-clock speed or universal complexity proof. Allocation failures, integer exhaustion and concurrency are outside finite evidence.

## History

### 2026-10-31 · descriptant

#### Prospective epoch — 2026-10-31

Full source was reviewed before edits; external prospective.md records API states, failure order and predicted checks. Production prefix remains byte-identical; test additions are isolated cfg(test)/test items.

#### Observed epoch — 2026-10-31

2026-10-31 wave31: 16 selected target/name cases passed twice debug and twice release with identical sorted outcomes; six compiled source-behavior mutants failed assertions, exact bytes restored and positive checks passed. Evidence scope is only the named helpers/fixtures. External receipt directory: $MOVA_DATA/runs/octopus-wave31-store-2026-10-31. No DuckDB integration, production loader/corpus, training or GPU run.

### 2026-10-31 · tests

2026-10-31 wave31: 16 selected target/name cases passed twice debug and twice release with identical sorted outcomes; six compiled source-behavior mutants failed assertions, exact bytes restored and positive checks passed. Evidence scope is only the named helpers/fixtures. External receipt directory: $MOVA_DATA/runs/octopus-wave31-store-2026-10-31. No DuckDB integration, production loader/corpus, training or GPU run.
