# db-src-duck

## descriptant

### DuckDB process and stage transport

open binds executable from DUCKDB or home/bin/duckdb without spawning. run constructs -bail,-batch, optional -readonly and mode, then db; writes SQL stdin before waiting. Success requires exit success and no literal Error: in stderr. Stage directories are pid/tag/atomic-counter named, existing collision removed; drop removes unless DB_KEEP is present.

#### Retained limitations

No SQL evaluation was substituted with a mock process. Missing DuckDB is a dependency gap, not passing database evidence. Partial syntax has no macro expansion/name resolution/types; Rust helpers and fixtures do not prove all models.

## approximant

### DuckDB process and stage transport forward relation

open binds executable from DUCKDB or home/bin/duckdb without spawning. run constructs -bail,-batch, optional -readonly and mode, then db; writes SQL stdin before waiting. Success requires exit success and no literal Error: in stderr. Stage directories are pid/tag/atomic-counter named, existing collision removed; drop removes unless DB_KEEP is present.

#### Domain

Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

#### Approximation limits

Categorical parsing, exact identity, transactional effects and resource errors have no useful percentage approximation metric here. No executable surrogate is claimed. Future profiles, if justified, may use only 1%, 2%, 5%, 10% with a defined metric/reference/domain.

## composition

### Local scope composition

Parent relation: open binds executable from DUCKDB or home/bin/duckdb without spawning. run constructs -bail,-batch, optional -readonly and mode, then db; writes SQL stdin before waiting. Success requires exit success and no literal Error: in stderr. Stage directories are pid/tag/atomic-counter named, existing collision removed; drop removes unless DB_KEEP is present.

Child fragments isolate selected mechanisms and do not cover the entire file. Invocation order and shared state remain the parent's relation. Costs of a child are included in parent cost, not added a second time; independent branch scopes are tagged. A child reverse relation must preserve intermediate state compatibility, not imply unique whole-file reconstruction.

Children: [{"id":"db-src-duck-stage-sql","path":"stage-sql.groupant"}]

## models

# DuckDB process and stage transport typed model catalog

Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Observation is one call/trace in the declared scope. Operators below are partial source-derived relations; complement is a proposed inner exclusion relation, reverse an outer compatible-antecedent relation retaining ambiguity. No universal proof follows from finite checks. Each oracle is the relevant named helper check below when it reaches the behavior; remaining SQL/effect cases are source review or proposed real-CLI checks.

## behavior_complement

Within successful process traces, nonzero exit or stderr containing Error: cannot produce Ok. Stage substitution cannot return a quoted directory. This excludes outputs, not database rollback: a failing process may already have written.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## reverse_reconstruction

Given returned stdout y, compatible executions include all successful child runs whose stdout lossily decodes to y and whose stderr lacks Error:. SQL, DB state, raw invalid UTF-8 and harmless stderr remain ambiguous; quote is uniquely invertible only on strings produced by quote.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## command_argument_transport

Readonly adds exactly one flag before mode and db; exec and read erase stdout and differ only in readonly. Oracle: direct argument inspection is source-derived, not executed SQL.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## stderr_failure_partition

The literal Error: test is case-sensitive and independent of exit status. Exit success with warnings without that substring is admitted. OS spawn/write/wait failures precede returned stdout.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## stage_resource_lifecycle

Directory uniqueness depends on process ID, tag and nonwrapped atomic counter. Existing collisions are recursively removed; DB_KEEP existence, including empty value, suppresses deletion. Fixtures use an isolated TMPDIR.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## sql_quote_representation

For input containing a apostrophes and b bytes, quote output length is b+a+2; doubling preserves backslashes literally. Finite oracle covers empty and two apostrophes.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## tests

### Selected checks

Named target-scoped checks for parent source: ["duck::wave31::quote_and_stage_boundary"]

Native commands and numeric status/log/outcomes live in external native-commands.json and receipts. Integration fixture checks use actual en::conllu parsing only. Legacy prag_layer_and_negative_controls, qa_table_loader_and_negative_controls and lossless_round_trip_and_negative_controls were inspected but not executed: real DuckDB unavailable. A fragment's listed parent tests may only cover part of its relation; SQL/query/transaction/IO-fault claims remain source-derived.

#### Negative controls

Six source-behavior mutations: quote doubling, TSV NULL rejection, QA positivity, prag bit truth, MWT counter, sentence boundary predicate. Each compiled, failed an assertion, restored exact original bytes and passed the selected positive. No assertion-only mutations and no surviving behavioral mutants.

## invariant-and-cost

### DuckDB process and stage transport invariants and costs

#### Invariant

SQL bytes are passed unchanged to stdin; returned text is lossy UTF-8 stdout, not a relational result guarantee. Stage substitution rejects apostrophes before replacing all instances of the selected brace key. quote doubles each apostrophe inside surrounding single quotes.

#### Cost

O(sql bytes + stdout bytes + stderr bytes) Rust transport memory/time plus unbounded external database work and process startup; stage cleanup proportional to contained filesystem entries. quote and substitution allocate linear output.

Symbolic bounds are source-derived, not measured wall-clock speed or universal complexity proof. Allocation failures, integer exhaustion and concurrency are outside finite evidence.

## History

### 2026-10-31 · descriptant

#### Prospective epoch — 2026-10-31

Full source was reviewed before edits; external prospective.md records API states, failure order and predicted checks. Production prefix remains byte-identical; test additions are isolated cfg(test)/test items.

#### Observed epoch — 2026-10-31

2026-10-31 wave31: 16 selected target/name cases passed twice debug and twice release with identical sorted outcomes; six compiled source-behavior mutants failed assertions, exact bytes restored and positive checks passed. Evidence scope is only the named helpers/fixtures. External receipt directory: $MOVA_DATA/runs/octopus-wave31-store-2026-10-31. No DuckDB integration, production loader/corpus, training or GPU run.

### 2026-10-31 · tests

2026-10-31 wave31: 16 selected target/name cases passed twice debug and twice release with identical sorted outcomes; six compiled source-behavior mutants failed assertions, exact bytes restored and positive checks passed. Evidence scope is only the named helpers/fixtures. External receipt directory: $MOVA_DATA/runs/octopus-wave31-store-2026-10-31. No DuckDB integration, production loader/corpus, training or GPU run.
