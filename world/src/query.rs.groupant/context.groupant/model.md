# world-query-context

## descriptant

# world-query-context

## Forward

Mention counts partition characters; independent event and cursor scans project discourse.

## Invariant

Query methods borrow immutable worlds; the frame reader mutates caller-owned discourse. Reverse models preserve compatible hidden inputs, never assume unique recovery. Ordering and failure assumptions in catalogs are part of the contract.

## Cost

For D replayed deltas, M mentions, E events, C cursors, K characters and L output bytes: state/history O(D+L), attention O(M+K log K), context adds O(E+C) and O(E) references. diff adds sorting and quadratic vector membership for set deltas. Resolve replays relevant entities and compares words/alternatives. Label recursion/ancestry cost depends on structure; cycles excluded.

## Composition

Selected source functions only; parent composes shared identity and dependencies. These are not independent pipeline stages or a complete partition.

## AST gap

Source-bound syn-2 partial syntax is present. Macro expansion, name resolution, type inference, cfg evaluation, external-module loading and semantic AST remain absent. Child envelopes may overlap and need not cover the file.

## Epoch memory

## catalog

# Typed models: world-query-context

Observations are single terminating calls unless a recording trace is named. Preconditions: finite well-formed inputs, stable dependencies, no allocation failure or integer overflow.

## nonbehavior

Type: behavior_complement. Operator/domain/observation: U=(W,s,attention/context return); R counts and selects. Inner U minus R contains zero counts, descending-ID tie order, and secondary membership at exactly half maximum.

Approximation: source-derived conditional relation; unknown behavior outside its declared domain remains unknown. Assumptions: shared preconditions above and descriptant dependency boundaries. Evidence: inspected source and callers; model.md#tests distinguishes executed checks. Falsifiable oracle: construct admissible inputs, execute the named operation and compare this predicate; contrary output refutes it.

## antecedents

Type: reverse_reconstruction. Operator/domain/observation: Ranked counts admit all mention multisets with those cutoff counts and arbitrary later mentions. Context constrains last qualifying event/cursor but retains unobserved earlier entries.

Approximation: source-derived conditional relation; unknown behavior outside its declared domain remains unknown. Assumptions: shared preconditions above and descriptant dependency boundaries. Evidence: inspected source and callers; model.md#tests distinguishes executed checks. Falsifiable oracle: construct admissible inputs, execute the named operation and compare this predicate; contrary output refutes it.

## half-threshold

Type: threshold_fiber. Operator/domain/observation: For positive maximum T, main c>=ceil(T/2), secondary 1<=c<ceil(T/2). Empty attention gives empty partitions. Arithmetic assumes no overflow.

Approximation: source-derived conditional relation; unknown behavior outside its declared domain remains unknown. Assumptions: shared preconditions above and descriptant dependency boundaries. Evidence: inspected source and callers; model.md#tests distinguishes executed checks. Falsifiable oracle: construct admissible inputs, execute the named operation and compare this predicate; contrary output refutes it.

## last-speech

Type: selection_boundary. Operator/domain/observation: Latest admitted non-narrator speech lacking Agent gives speaker=None despite older speaker; To independently selects addressee from the same event. No per-role backwards search.

Approximation: source-derived conditional relation; unknown behavior outside its declared domain remains unknown. Assumptions: shared preconditions above and descriptant dependency boundaries. Evidence: inspected source and callers; model.md#tests distinguishes executed checks. Falsifiable oracle: construct admissible inputs, execute the named operation and compare this predicate; contrary output refutes it.


## tests

### Checks

File-root ../model.md#evidence records structural checks and a bounded source-extraction Rust probe of attention and coverage arithmetic only. Other catalog oracles are prospective. world/src/tests.rs contains existing history and state-query tests. No complete course, teacher, GPU or neural-parser run is claimed. Parser artifacts, pragmatic reader, lexical tables, external NOMLEX and course loading remain dependency gaps.

## History

### 2026-10-29 · document

Revision 2026-10-29; SHA-256 27dd12e09019d22b057ab892136aec5be542da170e529573638c631b44ddd37f.

### 2026-10-29 · Epoch memory

2026-10-29: source-derived first epoch. Historical snapshots do not freeze current labels; direct read_with callers bypass the adapter. Recording errors can retain global state. Preserve these observations when strengthening models. Runtime source is unchanged.

### 2026-10-29 · document

Subject: ../../query.rs; SHA-256 27dd12e09019d22b057ab892136aec5be542da170e529573638c631b44ddd37f. Revision 2026-10-29.
