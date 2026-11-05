# db-src-main

## descriptant

### Database CLI dispatch and option adapters

main collects args, checks minimum arity, creates a Duck handle and dispatches commands by name/arity. load_cmd parses source/dialect and value-bearing options, then loads files sequentially. prag_cmd/qa_cmd collect allowed flags in BTreeMap with last duplicate winning, parse optional JSON params and call loaders. init checks existence before parent creation; verify prints notes/failures then exits 1 for zero files or any bad result.

#### Retained limitations

No SQL evaluation was substituted with a mock process. Missing DuckDB is a dependency gap, not passing database evidence. Partial syntax has no macro expansion/name resolution/types; Rust helpers and fixtures do not prove all models.

## approximant

### Database CLI dispatch and option adapters forward relation

main collects args, checks minimum arity, creates a Duck handle and dispatches commands by name/arity. load_cmd parses source/dialect and value-bearing options, then loads files sequentially. prag_cmd/qa_cmd collect allowed flags in BTreeMap with last duplicate winning, parse optional JSON params and call loaders. init checks existence before parent creation; verify prints notes/failures then exits 1 for zero files or any bad result.

#### Domain

Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

#### Approximation limits

Categorical parsing, exact identity, transactional effects and resource errors have no useful percentage approximation metric here. No executable surrogate is claimed. Future profiles, if justified, may use only 1%, 2%, 5%, 10% with a defined metric/reference/domain.

## composition

### Local scope composition

Parent relation: main collects args, checks minimum arity, creates a Duck handle and dispatches commands by name/arity. load_cmd parses source/dialect and value-bearing options, then loads files sequentially. prag_cmd/qa_cmd collect allowed flags in BTreeMap with last duplicate winning, parse optional JSON params and call loaders. init checks existence before parent creation; verify prints notes/failures then exits 1 for zero files or any bad result.

Child fragments isolate selected mechanisms and do not cover the entire file. Invocation order and shared state remain the parent's relation. Costs of a child are included in parent cost, not added a second time; independent branch scopes are tagged. A child reverse relation must preserve intermediate state compatibility, not imply unique whole-file reconstruction.

Children: [{"id":"db-src-main-load-options","path":"load-options.groupant"}]

## models

# Database CLI dispatch and option adapters typed model catalog

Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Observation is one call/trace in the declared scope. Operators below are partial source-derived relations; complement is a proposed inner exclusion relation, reverse an outer compatible-antecedent relation retaining ambiguity. No universal proof follows from finite checks. Each oracle is the relevant named helper check below when it reaches the behavior; remaining SQL/effect cases are source review or proposed real-CLI checks.

## behavior_complement

Missing commands, invalid load source/docs/match, unknown options and missing values cannot reach a successful loader dispatch. init cannot overwrite an already existing path. BrokenPipe from export stdout is explicitly admitted as success.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## reverse_reconstruction

A parsed prag/qa option map corresponds to all flag sequences with the same final values, including overwritten duplicates. Defaults make omitted flags compatible with explicit default values; printed run count cannot uniquely recover CLI flags or DB state.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## dispatch_arity

tales requires at least six args, export at least three, sql exactly three; unrelated extra args on some other commands are ignored by dispatch. This is a source-derived command-language partition.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## parameter_precedence

prag requires run then source before params file parsing; qa defaults source opus and tool qagen source. Last duplicate flag wins in ordered map, but file errors precede invoking load.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## initialization_effect_order

Existence rejection occurs before create_dir_all and exec. If schema exec succeeds but schema_version fails, init returns an error with a created database potentially present.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## batch_partial_progress

corpus validates each six-column row and globs before loading that row's files; previously loaded files persist when a later row fails. No cross-manifest transaction is implied.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## tests

### Selected checks

Named target-scoped checks for parent source: ["wave31::cli_rejections_precede_database_effects"]

Native commands and numeric status/log/outcomes live in external native-commands.json and receipts. Integration fixture checks use actual en::conllu parsing only. Legacy prag_layer_and_negative_controls, qa_table_loader_and_negative_controls and lossless_round_trip_and_negative_controls were inspected but not executed: real DuckDB unavailable. A fragment's listed parent tests may only cover part of its relation; SQL/query/transaction/IO-fault claims remain source-derived.

#### Negative controls

Six source-behavior mutations: quote doubling, TSV NULL rejection, QA positivity, prag bit truth, MWT counter, sentence boundary predicate. Each compiled, failed an assertion, restored exact original bytes and passed the selected positive. No assertion-only mutations and no surviving behavioral mutants.

## invariant-and-cost

### Database CLI dispatch and option adapters invariants and costs

#### Invariant

Unknown flag or missing option value returns before loader calls. Constructing Duck alone has no DB effect. Existing init target is left untouched. Corpus order is manifest order and matching/load failures stop later entries, without rolling back earlier files.

#### Cost

O(argument and optional params bytes + manifest bytes) parsing; loader, glob, SQL and report costs compose sequentially. Whole corpus and tales costs are unbounded here and were not run.

Symbolic bounds are source-derived, not measured wall-clock speed or universal complexity proof. Allocation failures, integer exhaustion and concurrency are outside finite evidence.

## History

### 2026-10-31 · descriptant

#### Prospective epoch — 2026-10-31

Full source was reviewed before edits; external prospective.md records API states, failure order and predicted checks. Production prefix remains byte-identical; test additions are isolated cfg(test)/test items.

#### Observed epoch — 2026-10-31

2026-10-31 wave31: 16 selected target/name cases passed twice debug and twice release with identical sorted outcomes; six compiled source-behavior mutants failed assertions, exact bytes restored and positive checks passed. Evidence scope is only the named helpers/fixtures. External receipt directory: $MOVA_DATA/runs/octopus-wave31-store-2026-10-31. No DuckDB integration, production loader/corpus, training or GPU run.

### 2026-10-31 · tests

2026-10-31 wave31: 16 selected target/name cases passed twice debug and twice release with identical sorted outcomes; six compiled source-behavior mutants failed assertions, exact bytes restored and positive checks passed. Evidence scope is only the named helpers/fixtures. External receipt directory: $MOVA_DATA/runs/octopus-wave31-store-2026-10-31. No DuckDB integration, production loader/corpus, training or GPU run.
