# db-src-util

## descriptant

### Time, card labels and simple filesystem glob

sha1_hex returns lowercase digest; ts decomposes seconds using Euclidean day/rem and Gregorian civil conversion. date_suffix takes final ten bytes and tests digit/dash shape. expand handles only ~ and ~/; glob walks components, star-matches directory names, suppresses unreadable directories, retains files and sorts. wild matches prefix, sequential interior fragments and suffix.

#### Retained limitations

No SQL evaluation was substituted with a mock process. Missing DuckDB is a dependency gap, not passing database evidence. Partial syntax has no macro expansion/name resolution/types; Rust helpers and fixtures do not prove all models.

## approximant

### Time, card labels and simple filesystem glob forward relation

sha1_hex returns lowercase digest; ts decomposes seconds using Euclidean day/rem and Gregorian civil conversion. date_suffix takes final ten bytes and tests digit/dash shape. expand handles only ~ and ~/; glob walks components, star-matches directory names, suppresses unreadable directories, retains files and sorts. wild matches prefix, sequential interior fragments and suffix.

#### Domain

Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

#### Approximation limits

Categorical parsing, exact identity, transactional effects and resource errors have no useful percentage approximation metric here. No executable surrogate is claimed. Future profiles, if justified, may use only 1%, 2%, 5%, 10% with a defined metric/reference/domain.

## composition

### Local scope composition

Parent relation: sha1_hex returns lowercase digest; ts decomposes seconds using Euclidean day/rem and Gregorian civil conversion. date_suffix takes final ten bytes and tests digit/dash shape. expand handles only ~ and ~/; glob walks components, star-matches directory names, suppresses unreadable directories, retains files and sorts. wild matches prefix, sequential interior fragments and suffix.

Child fragments isolate selected mechanisms and do not cover the entire file. Invocation order and shared state remain the parent's relation. Costs of a child are included in parent cost, not added a second time; independent branch scopes are tagged. A child reverse relation must preserve intermediate state compatibility, not imply unique whole-file reconstruction.

Children: [{"id":"db-src-util-wild","path":"wild.groupant"}]

## models

# Time, card labels and simple filesystem glob typed model catalog

Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Observation is one call/trace in the declared scope. Operators below are partial source-derived relations; complement is a proposed inner exclusion relation, reverse an outer compatible-antecedent relation retaining ambiguity. No universal proof follows from finite checks. Each oracle is the relevant named helper check below when it reaches the behavior; remaining SQL/effect cases are source review or proposed real-CLI checks.

## behavior_complement

Nonmatching glob names cannot enter the next component frontier, and directories cannot remain final outputs. Invalid date shape yields None on safe slicing domain; impossible calendar dates are not excluded.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## reverse_reconstruction

A timestamp's compatible seconds are unique within the civil conversion's valid domain, while a date suffix result admits arbitrary prefixes. Sorted glob output does not reconstruct directory iteration order or ignored read errors; SHA1 is many-to-one.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## epoch_euclidean_coordinates

secs=-1 decomposes into days=-1, rem=86399, avoiding negative time fields. ts uses proleptic Gregorian cycles; now_ts maps preepoch clock error to zero.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## wildcard_order

wild splits only star; leading fragment must be prefix, interior fragments found in order consuming their bytes, final fragment suffix. Question mark and brackets are literal, not shell syntax.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## card_label_alias

English and escaped legacy labels map to shared English keys; card_part normalizes only the two declared totals and preserves other labels.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## path_information_loss

parent_name/stem/file_name use lossy UTF-8; abs canonicalizes with an error on missing paths. expand leaves ~other unchanged; filesystem permissions and races remain external.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## tests

### Selected checks

Named target-scoped checks for parent source: ["util::tests::time_and_wildcards"]

Native commands and numeric status/log/outcomes live in external native-commands.json and receipts. Integration fixture checks use actual en::conllu parsing only. Legacy prag_layer_and_negative_controls, qa_table_loader_and_negative_controls and lossless_round_trip_and_negative_controls were inspected but not executed: real DuckDB unavailable. A fragment's listed parent tests may only cover part of its relation; SQL/query/transaction/IO-fault claims remain source-derived.

#### Negative controls

Six source-behavior mutations: quote doubling, TSV NULL rejection, QA positivity, prag bit truth, MWT counter, sentence boundary predicate. Each compiled, failed an assertion, restored exact original bytes and passed the selected positive. No assertion-only mutations and no surviving behavioral mutants.

## invariant-and-cost

### Time, card labels and simple filesystem glob invariants and costs

#### Invariant

glob output sorted lexicographically, files only; paths need not be canonical. date_suffix is not calendar validation and slicing requires a UTF-8 boundary at len-10. Timestamp computation on i64 seconds has bounded day arithmetic but display years may exceed four digits.

#### Cost

sha/time formatting O(input bytes)/O(1) arithmetic respectively; glob O(visited directory entries plus sort) with star matching substring scans. Memory proportional to candidates and returned paths; filesystem effects are reads.

Symbolic bounds are source-derived, not measured wall-clock speed or universal complexity proof. Allocation failures, integer exhaustion and concurrency are outside finite evidence.

## History

### 2026-10-31 · descriptant

#### Prospective epoch — 2026-10-31

Full source was reviewed before edits; external prospective.md records API states, failure order and predicted checks. Production prefix remains byte-identical; test additions are isolated cfg(test)/test items.

#### Observed epoch — 2026-10-31

2026-10-31 wave31: 16 selected target/name cases passed twice debug and twice release with identical sorted outcomes; six compiled source-behavior mutants failed assertions, exact bytes restored and positive checks passed. Evidence scope is only the named helpers/fixtures. External receipt directory: $MOVA_DATA/runs/octopus-wave31-store-2026-10-31. No DuckDB integration, production loader/corpus, training or GPU run.

### 2026-10-31 · tests

2026-10-31 wave31: 16 selected target/name cases passed twice debug and twice release with identical sorted outcomes; six compiled source-behavior mutants failed assertions, exact bytes restored and positive checks passed. Evidence scope is only the named helpers/fixtures. External receipt directory: $MOVA_DATA/runs/octopus-wave31-store-2026-10-31. No DuckDB integration, production loader/corpus, training or GPU run.
