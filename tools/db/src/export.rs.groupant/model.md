# db-src-export

## descriptant

### Filtered CoNLL export and verification

filters accepts twelve literal keys, splitting at first equals and preserving order/duplicates. export materializes files/sets/rows TSV via SQL, groups raw ten-column rows by aid, then Doc::text. Exactness is selected only for one file filter and one file record; otherwise leading blanks zero, final newline true and each block gets >=1 trailing blank. verify compares disk bytes and load-time SHA1, independent DB counters, optional card counters.

#### Retained limitations

No SQL evaluation was substituted with a mock process. Missing DuckDB is a dependency gap, not passing database evidence. Partial syntax has no macro expansion/name resolution/types; Rust helpers and fixtures do not prove all models.

## approximant

### Filtered CoNLL export and verification forward relation

filters accepts twelve literal keys, splitting at first equals and preserving order/duplicates. export materializes files/sets/rows TSV via SQL, groups raw ten-column rows by aid, then Doc::text. Exactness is selected only for one file filter and one file record; otherwise leading blanks zero, final newline true and each block gets >=1 trailing blank. verify compares disk bytes and load-time SHA1, independent DB counters, optional card counters.

#### Domain

Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

#### Approximation limits

Categorical parsing, exact identity, transactional effects and resource errors have no useful percentage approximation metric here. No executable surrogate is claimed. Future profiles, if justified, may use only 1%, 2%, 5%, 10% with a defined metric/reference/domain.

## composition

### Local scope composition

Parent relation: filters accepts twelve literal keys, splitting at first equals and preserving order/duplicates. export materializes files/sets/rows TSV via SQL, groups raw ten-column rows by aid, then Doc::text. Exactness is selected only for one file filter and one file record; otherwise leading blanks zero, final newline true and each block gets >=1 trailing blank. verify compares disk bytes and load-time SHA1, independent DB counters, optional card counters.

Child fragments isolate selected mechanisms and do not cover the entire file. Invocation order and shared state remain the parent's relation. Costs of a child are included in parent cost, not added a second time; independent branch scopes are tagged. A child reverse relation must preserve intermediate state compatibility, not imply unique whole-file reconstruction.

Children: [{"id":"db-src-export-ud-counts","path":"ud-counts.groupant"}]

## models

# Filtered CoNLL export and verification typed model catalog

Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Observation is one call/trace in the declared scope. Operators below are partial source-derived relations; complement is a proposed inner exclusion relation, reverse an outer compatible-antecedent relation retaining ambiguity. No universal proof follows from finite checks. Each oracle is the relevant named helper check below when it reaches the behavior; remaining SQL/effect cases are source review or proposed real-CLI checks.

## behavior_complement

Unknown filter keys or missing equals cannot yield parsed filters. Empty set materialization cannot yield a successful empty export. A missing disk file with matching export SHA1 cannot itself create a failure note in bad.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## reverse_reconstruction

For exact export text, compatible store states contain ordered raw rows/comments and blank/newline metadata producing that text; IDs, unused runs and other annotation sets remain unobserved. Nonexact output loses original leading/trailing blank information, so its preimage is wider.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## filter_representation

file=a=b yields key file and value a=b; empty values and duplicate keys are syntactically admitted. SQL variable last-write semantics are not established by parser checks.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## ud_surface_count

For one MWT span 1-2, words 1,2,3 and empty node 3.1, counters are [1,2,3,1,1]. The inside_to cursor resets per sentence; malformed IDs fall back to zero rather than structural validation.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## verification_partition

bytes_ok requires equal bytes plus SHA1 when disk exists, or SHA1 alone when absent. counts_ok compares stored file/database count vectors independently; matching bytes cannot imply counter agreement.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## card_numeric_projection

Inside a recognized Size section, five cells are reduced to ASCII digits and parsed; punctuation/sign information is discarded. Repeated part names overwrite previous JSON entries; unrelated sections disable collection.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## tests

### Selected checks

Named target-scoped checks for parent source: ["export::tests::ud_counts_mwt_and_empty"]

Native commands and numeric status/log/outcomes live in external native-commands.json and receipts. Integration fixture checks use actual en::conllu parsing only. Legacy prag_layer_and_negative_controls, qa_table_loader_and_negative_controls and lossless_round_trip_and_negative_controls were inspected but not executed: real DuckDB unavailable. A fragment's listed parent tests may only cover part of its relation; SQL/query/transaction/IO-fault claims remain source-derived.

#### Negative controls

Six source-behavior mutations: quote doubling, TSV NULL rejection, QA positivity, prag bit truth, MWT counter, sentence boundary predicate. Each compiled, failed an assertion, restored exact original bytes and passed the selected positive. No assertion-only mutations and no surviving behavioral mutants.

## invariant-and-cost

### Filtered CoNLL export and verification invariants and costs

#### Invariant

The Rust assembly preserves JSON comment order and returned row order; SQL must supply deterministic order. Empty matched sets is an error. ud_counts counts a multiword row as one surface token, suppresses covered word tokens, counts decimal IDs as empty nodes.

#### Cost

O(materialized TSV bytes + output bytes + verification file bytes) memory and Rust work, with additional card-cache lookup and database query cost. Verification invokes one export per selected file. Hash maps have expected linear processing, not worst-case proof.

Symbolic bounds are source-derived, not measured wall-clock speed or universal complexity proof. Allocation failures, integer exhaustion and concurrency are outside finite evidence.

## History

### 2026-10-31 · descriptant

#### Prospective epoch — 2026-10-31

Full source was reviewed before edits; external prospective.md records API states, failure order and predicted checks. Production prefix remains byte-identical; test additions are isolated cfg(test)/test items.

#### Observed epoch — 2026-10-31

2026-10-31 wave31: 16 selected target/name cases passed twice debug and twice release with identical sorted outcomes; six compiled source-behavior mutants failed assertions, exact bytes restored and positive checks passed. Evidence scope is only the named helpers/fixtures. External receipt directory: $MOVA_DATA/runs/octopus-wave31-store-2026-10-31. No DuckDB integration, production loader/corpus, training or GPU run.

### 2026-10-31 · tests

2026-10-31 wave31: 16 selected target/name cases passed twice debug and twice release with identical sorted outcomes; six compiled source-behavior mutants failed assertions, exact bytes restored and positive checks passed. Evidence scope is only the named helpers/fixtures. External receipt directory: $MOVA_DATA/runs/octopus-wave31-store-2026-10-31. No DuckDB integration, production loader/corpus, training or GPU run.
