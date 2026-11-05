# db-src-tsv

## descriptant

### Raw TSV representation and partial writes

create truncates/creates file and writes joined header. row checks arity then writes columns left-to-right with tabs; Null emits backslash-N, strings reject tab/LF/CR/sentinel, integers decimal and booleans lowercase. Successful newline increments rows. finish flushes and returns count. read splits every file line without removing header, mapping exact sentinel to None; s/int check presence then parse.

#### Retained limitations

No SQL evaluation was substituted with a mock process. Missing DuckDB is a dependency gap, not passing database evidence. Partial syntax has no macro expansion/name resolution/types; Rust helpers and fixtures do not prove all models.

## approximant

### Raw TSV representation and partial writes forward relation

create truncates/creates file and writes joined header. row checks arity then writes columns left-to-right with tabs; Null emits backslash-N, strings reject tab/LF/CR/sentinel, integers decimal and booleans lowercase. Successful newline increments rows. finish flushes and returns count. read splits every file line without removing header, mapping exact sentinel to None; s/int check presence then parse.

#### Domain

Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

#### Approximation limits

Categorical parsing, exact identity, transactional effects and resource errors have no useful percentage approximation metric here. No executable surrogate is claimed. Future profiles, if justified, may use only 1%, 2%, 5%, 10% with a defined metric/reference/domain.

## composition

### Local scope composition

Parent relation: create truncates/creates file and writes joined header. row checks arity then writes columns left-to-right with tabs; Null emits backslash-N, strings reject tab/LF/CR/sentinel, integers decimal and booleans lowercase. Successful newline increments rows. finish flushes and returns count. read splits every file line without removing header, mapping exact sentinel to None; s/int check presence then parse.

Child fragments isolate selected mechanisms and do not cover the entire file. Invocation order and shared state remain the parent's relation. Costs of a child are included in parent cost, not added a second time; independent branch scopes are tagged. A child reverse relation must preserve intermediate state compatibility, not imply unique whole-file reconstruction.

Children: [{"id":"db-src-tsv-row-write","path":"row-write.groupant"}]

## models

# Raw TSV representation and partial writes typed model catalog

Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Observation is one call/trace in the declared scope. Operators below are partial source-derived relations; complement is a proposed inner exclusion relation, reverse an outer compatible-antecedent relation retaining ambiguity. No universal proof follows from finite checks. Each oracle is the relevant named helper check below when it reaches the behavior; remaining SQL/effect cases are source review or proposed real-CLI checks.

## behavior_complement

Wrong arity cannot write any row prefix; invalid later cell cannot produce a successful row count increment. A literal backslash-N string cannot be represented as a successful non-NULL cell.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## reverse_reconstruction

Given valid headerless encoded rows, None uniquely indicates Null and other cells are string observations. Value variants remain ambiguous: integer 1 and string 1 encode identically, true and boolean true likewise. Source lexical numeric spelling is not reconstructed by int.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## null_empty_separation

read(backslash-N)=None and read(empty cell)=Some(empty). No quote or escape processing exists. JSON control escapes are ordinary bytes if already serialized before row.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## partial_write_frontier

Width check precedes all writes; separator precedes validation of each later string. After writing prefix then rejecting sentinel, observed file may end in prefix-tab and rows remains zero. Direct synthetic file test checks this exact effect.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## numeric_range

I uses full signed-i64 decimal; int accepts parseable signed-i64 strings and rejects overflow/missing/null. Finite boundaries include MIN, MAX and MAX+1 string.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## header_as_data

Tsv::create always writes a header line even for zero columns; read does not skip it. Writer/read are not direct inverse APIs unless caller separates headers.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## tests

### Selected checks

Named target-scoped checks for parent source: ["tsv::wave31::actual_tsv_null_empty_and_numeric_boundaries","tsv::wave31::rejected_rows_preserve_counter_but_may_write_prefix"]

Native commands and numeric status/log/outcomes live in external native-commands.json and receipts. Integration fixture checks use actual en::conllu parsing only. Legacy prag_layer_and_negative_controls, qa_table_loader_and_negative_controls and lossless_round_trip_and_negative_controls were inspected but not executed: real DuckDB unavailable. A fragment's listed parent tests may only cover part of its relation; SQL/query/transaction/IO-fault claims remain source-derived.

#### Negative controls

Six source-behavior mutations: quote doubling, TSV NULL rejection, QA positivity, prag bit truth, MWT counter, sentence boundary predicate. Each compiled, failed an assertion, restored exact original bytes and passed the selected positive. No assertion-only mutations and no surviving behavioral mutants.

## invariant-and-cost

### Raw TSV representation and partial writes invariants and costs

#### Invariant

rows counts only fully successful row calls, not physically complete lines after an earlier partial failure. Strings and NULL have disjoint encodings on the admitted domain; empty string remains Some(empty). Failure after earlier cells may leave a prefix in buffer/file.

#### Cost

O(total encoded bytes) streaming write time with buffered memory plus value formatting; read materializes O(file bytes + cells). Error preview takes at most 80 chars but row validation scans entire offending string.

Symbolic bounds are source-derived, not measured wall-clock speed or universal complexity proof. Allocation failures, integer exhaustion and concurrency are outside finite evidence.

## History

### 2026-10-31 · descriptant

#### Prospective epoch — 2026-10-31

Full source was reviewed before edits; external prospective.md records API states, failure order and predicted checks. Production prefix remains byte-identical; test additions are isolated cfg(test)/test items.

#### Observed epoch — 2026-10-31

2026-10-31 wave31: 16 selected target/name cases passed twice debug and twice release with identical sorted outcomes; six compiled source-behavior mutants failed assertions, exact bytes restored and positive checks passed. Evidence scope is only the named helpers/fixtures. External receipt directory: $MOVA_DATA/runs/octopus-wave31-store-2026-10-31. No DuckDB integration, production loader/corpus, training or GPU run.

### 2026-10-31 · tests

2026-10-31 wave31: 16 selected target/name cases passed twice debug and twice release with identical sorted outcomes; six compiled source-behavior mutants failed assertions, exact bytes restored and positive checks passed. Evidence scope is only the named helpers/fixtures. External receipt directory: $MOVA_DATA/runs/octopus-wave31-store-2026-10-31. No DuckDB integration, production loader/corpus, training or GPU run.
