# db-src-prag

## descriptant

### Pragmatic label parser and staged load

read_labels accepts exact first eight header columns plus optional trace, skips empty lines and requires row width. Missing trace becomes empty. load rejects empty/duplicate sent_id, stages wanted IDs, queries maxima/runs/sentences, rejects duplicate run then missing then ambiguous IDs. It writes run/annset/pragmatics rows, validates bits, maps empty means/trace to NULL, executes SQL and checks post row count.

#### Retained limitations

No SQL evaluation was substituted with a mock process. Missing DuckDB is a dependency gap, not passing database evidence. Partial syntax has no macro expansion/name resolution/types; Rust helpers and fixtures do not prove all models.

## approximant

### Pragmatic label parser and staged load forward relation

read_labels accepts exact first eight header columns plus optional trace, skips empty lines and requires row width. Missing trace becomes empty. load rejects empty/duplicate sent_id, stages wanted IDs, queries maxima/runs/sentences, rejects duplicate run then missing then ambiguous IDs. It writes run/annset/pragmatics rows, validates bits, maps empty means/trace to NULL, executes SQL and checks post row count.

#### Domain

Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

#### Approximation limits

Categorical parsing, exact identity, transactional effects and resource errors have no useful percentage approximation metric here. No executable surrogate is claimed. Future profiles, if justified, may use only 1%, 2%, 5%, 10% with a defined metric/reference/domain.

## composition

### Local scope composition

Parent relation: read_labels accepts exact first eight header columns plus optional trace, skips empty lines and requires row width. Missing trace becomes empty. load rejects empty/duplicate sent_id, stages wanted IDs, queries maxima/runs/sentences, rejects duplicate run then missing then ambiguous IDs. It writes run/annset/pragmatics rows, validates bits, maps empty means/trace to NULL, executes SQL and checks post row count.

Child fragments isolate selected mechanisms and do not cover the entire file. Invocation order and shared state remain the parent's relation. Costs of a child are included in parent cost, not added a second time; independent branch scopes are tagged. A child reverse relation must preserve intermediate state compatibility, not imply unique whole-file reconstruction.

Children: [{"id":"db-src-prag-label-parser","path":"label-parser.groupant"}]

## models

# Pragmatic label parser and staged load typed model catalog

Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Observation is one call/trace in the declared scope. Operators below are partial source-derived relations; complement is a proposed inner exclusion relation, reverse an outer compatible-antecedent relation retaining ambiguity. No universal proof follows from finite checks. Each oracle is the relevant named helper check below when it reaches the behavior; remaining SQL/effect cases are source review or proposed real-CLI checks.

## behavior_complement

Foreign header, wrong width, no rows or duplicate IDs cannot reach write exec. Invalid bit cannot yield a successful load; it may have already written temporary rows. A post-count error does not exclude a committed DB state.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## reverse_reconstruction

Given stored pragmatic row, compatible TSVs may omit trace or supply empty trace, and use either 1/true or 0/false. Empty means and trace collapse to NULL; lexical bool spelling is unrecoverable. Unused source/database fields remain unconstrained.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## optional_trace_projection

Eight columns append one empty trace field; nine require trace as final header. Header-only parser output is empty but load subsequently rejects. Empty lines are skipped, whitespace-only lines are not.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## boolean_domain

bit maps exactly {0,false}->false and {1,true}->true, excluding yes, case changes, negatives and empty. The direct helper check exercises these finite cases.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## identity_cardinality

Missing IDs are reported before ambiguous IDs after duplicate run rejection. Exactly one (sid,doc) is required; identical duplicate candidates still count as ambiguity.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## migration_suffix

migrate_sql returns a borrowed suffix beginning at the first literal -- migrate:v substring. It neither parses SQL nor checks migration idempotence; actual DDL semantics remain unexecuted.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## tests

### Selected checks

Named target-scoped checks for parent source: ["prag::wave31::labels_bits_and_migration_boundaries"]

Native commands and numeric status/log/outcomes live in external native-commands.json and receipts. Integration fixture checks use actual en::conllu parsing only. Legacy prag_layer_and_negative_controls, qa_table_loader_and_negative_controls and lossless_round_trip_and_negative_controls were inspected but not executed: real DuckDB unavailable. A fragment's listed parent tests may only cover part of its relation; SQL/query/transaction/IO-fault claims remain source-derived.

#### Negative controls

Six source-behavior mutations: quote doubling, TSV NULL rejection, QA positivity, prag bit truth, MWT counter, sentence boundary predicate. Each compiled, failed an assertion, restored exact original bytes and passed the selected positive. No assertion-only mutations and no surviving behavioral mutants.

## invariant-and-cost

### Pragmatic label parser and staged load invariants and costs

#### Invariant

Each admitted label maps to exactly one prior sentence/doc and one new aid on the safe nonoverflow domain. Source does not validate closed form/act sets. IDs are max+1 increments; bits accept only 0,false,1,true with exact spelling.

#### Cost

O(label bytes + queried rows + output TSV bytes) expected work/memory, plus SQL. Each input ID is scanned for duplicates and missing/ambiguous membership; diagnostic output limits displayed IDs to five but counts all.

Symbolic bounds are source-derived, not measured wall-clock speed or universal complexity proof. Allocation failures, integer exhaustion and concurrency are outside finite evidence.

## History

### 2026-10-31 · descriptant

#### Prospective epoch — 2026-10-31

Full source was reviewed before edits; external prospective.md records API states, failure order and predicted checks. Production prefix remains byte-identical; test additions are isolated cfg(test)/test items.

#### Observed epoch — 2026-10-31

2026-10-31 wave31: 16 selected target/name cases passed twice debug and twice release with identical sorted outcomes; six compiled source-behavior mutants failed assertions, exact bytes restored and positive checks passed. Evidence scope is only the named helpers/fixtures. External receipt directory: $MOVA_DATA/runs/octopus-wave31-store-2026-10-31. No DuckDB integration, production loader/corpus, training or GPU run.

### 2026-10-31 · tests

2026-10-31 wave31: 16 selected target/name cases passed twice debug and twice release with identical sorted outcomes; six compiled source-behavior mutants failed assertions, exact bytes restored and positive checks passed. Evidence scope is only the named helpers/fixtures. External receipt directory: $MOVA_DATA/runs/octopus-wave31-store-2026-10-31. No DuckDB integration, production loader/corpus, training or GPU run.
