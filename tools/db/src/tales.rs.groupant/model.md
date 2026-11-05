# db-src-tales

## descriptant

### Synthetic book segmentation and corpus orchestration

body selects lines between first ordered Gutenberg markers and removes CR endings. split removes illustration lines, uses TOC minimum-indent entries with normalized ignores and ordered blank-surrounded matching headings, or title/start/end single section, then normalized skip. paragraphs collapse whitespace and label head/lg/p. sentences emits half-open token spans after terminal/closers unless next starts lowercase/internal punctuation. run trains annotator, writes all CoNLL files/index then loads sequentially.

#### Retained limitations

No SQL evaluation was substituted with a mock process. Missing DuckDB is a dependency gap, not passing database evidence. Partial syntax has no macro expansion/name resolution/types; Rust helpers and fixtures do not prove all models.

## approximant

### Synthetic book segmentation and corpus orchestration forward relation

body selects lines between first ordered Gutenberg markers and removes CR endings. split removes illustration lines, uses TOC minimum-indent entries with normalized ignores and ordered blank-surrounded matching headings, or title/start/end single section, then normalized skip. paragraphs collapse whitespace and label head/lg/p. sentences emits half-open token spans after terminal/closers unless next starts lowercase/internal punctuation. run trains annotator, writes all CoNLL files/index then loads sequentially.

#### Domain

Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

#### Approximation limits

Categorical parsing, exact identity, transactional effects and resource errors have no useful percentage approximation metric here. No executable surrogate is claimed. Future profiles, if justified, may use only 1%, 2%, 5%, 10% with a defined metric/reference/domain.

## composition

### Local scope composition

Parent relation: body selects lines between first ordered Gutenberg markers and removes CR endings. split removes illustration lines, uses TOC minimum-indent entries with normalized ignores and ordered blank-surrounded matching headings, or title/start/end single section, then normalized skip. paragraphs collapse whitespace and label head/lg/p. sentences emits half-open token spans after terminal/closers unless next starts lowercase/internal punctuation. run trains annotator, writes all CoNLL files/index then loads sequentially.

Child fragments isolate selected mechanisms and do not cover the entire file. Invocation order and shared state remain the parent's relation. Costs of a child are included in parent cost, not added a second time; independent branch scopes are tagged. A child reverse relation must preserve intermediate state compatibility, not imply unique whole-file reconstruction.

Children: [{"id":"db-src-tales-sentence-spans","path":"sentence-spans.groupant"}]

## models

# Synthetic book segmentation and corpus orchestration typed model catalog

Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Observation is one call/trace in the declared scope. Operators below are partial source-derived relations; complement is a proposed inner exclusion relation, reverse an outer compatible-antecedent relation retaining ambiguity. No universal proof follows from finite checks. Each oracle is the relevant named helper check below when it reaches the behavior; remaining SQL/effect cases are source review or proposed real-CLI checks.

## behavior_complement

Missing/out-of-order Gutenberg markers cannot produce body. Missing TOC entries/start/end or empty retained tale cannot return split success. A terminal followed by lowercase cannot alone force a boundary under the current rule.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## reverse_reconstruction

Paragraph output admits all original line/whitespace layouts that collapse identically and satisfy block classification. Sentence span output constrains terminal/closer and following-character predicates, not token lexical identities. Raw book, removed illustrations and skipped sections are not recoverable.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## heading_selection

Only minimum indent TOC items survive before ignore filtering; headings must appear in TOC order surrounded by blanks. Case/curly single quote/whitespace normalization creates equivalence classes, not fuzzy matching.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## paragraph_block_priority

A one-line all-uppercase alphabetic paragraph is head; otherwise multiple wholly indented lines are lg; all remaining are p. Classification uses pre-collapse line count/indentation, so equal output text can have different block labels.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## token_boundary_partition

At terminal index i, j advances through terminal/closer run. If next is lowercase or ,;:-–— then no split, otherwise emit [start,j). Final residual suffix is emitted once.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## orchestration_effect_boundary

run creates output directory before training, writes per-book files and index before DB loads. Later failure leaves earlier files and possibly loaded books. It is not an all-books transaction and was not invoked.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## tests

### Selected checks

Named target-scoped checks for parent source: ["tales::tests::sentence_bounds","tales::tests::split_by_contents"]

Native commands and numeric status/log/outcomes live in external native-commands.json and receipts. Integration fixture checks use actual en::conllu parsing only. Legacy prag_layer_and_negative_controls, qa_table_loader_and_negative_controls and lossless_round_trip_and_negative_controls were inspected but not executed: real DuckDB unavailable. A fragment's listed parent tests may only cover part of its relation; SQL/query/transaction/IO-fault claims remain source-derived.

#### Negative controls

Six source-behavior mutations: quote doubling, TSV NULL rejection, QA positivity, prag bit truth, MWT counter, sentence boundary predicate. Each compiled, failed an assertion, restored exact original bytes and passed the selected positive. No assertion-only mutations and no surviving behavioral mutants.

## invariant-and-cost

### Synthetic book segmentation and corpus orchestration invariants and costs

#### Invariant

Sentence spans partition token indices in order for nonempty tokens; empty list yields no spans. Paragraph text loses original whitespace. conllu disambiguates equal slugs with -n, emits document metadata on first actual sentence and paragraph markers per tokenized paragraph; real annotator/corpus execution is outside evidence.

#### Cost

body/paragraphs/sentences linear in materialized bytes/tokens; heading search may revisit normalization and scan up to O(headings*lines*line length). CoNLL assembly allocates full output; annotation training/inference and DB loading are additional opaque costs.

Symbolic bounds are source-derived, not measured wall-clock speed or universal complexity proof. Allocation failures, integer exhaustion and concurrency are outside finite evidence.

## History

### 2026-10-31 · descriptant

#### Prospective epoch — 2026-10-31

Full source was reviewed before edits; external prospective.md records API states, failure order and predicted checks. Production prefix remains byte-identical; test additions are isolated cfg(test)/test items.

#### Observed epoch — 2026-10-31

2026-10-31 wave31: 16 selected target/name cases passed twice debug and twice release with identical sorted outcomes; six compiled source-behavior mutants failed assertions, exact bytes restored and positive checks passed. Evidence scope is only the named helpers/fixtures. External receipt directory: $MOVA_DATA/runs/octopus-wave31-store-2026-10-31. No DuckDB integration, production loader/corpus, training or GPU run.

### 2026-10-31 · tests

2026-10-31 wave31: 16 selected target/name cases passed twice debug and twice release with identical sorted outcomes; six compiled source-behavior mutants failed assertions, exact bytes restored and positive checks passed. Evidence scope is only the named helpers/fixtures. External receipt directory: $MOVA_DATA/runs/octopus-wave31-store-2026-10-31. No DuckDB integration, production loader/corpus, training or GPU run.
