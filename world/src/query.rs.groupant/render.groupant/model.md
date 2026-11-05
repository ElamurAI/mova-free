# world-query-render

## descriptant

# world-query-render

## Forward

Current snapshot rendering and lexical resolution produce labels, kind strings, ancestor strings and ID vectors.

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

# Typed models: world-query-render

Observations are single terminating calls unless a recording trace is named. Preconditions: finite well-formed inputs, stable dependencies, no allocation failure or integer overflow.

## nonbehavior

Type: behavior_complement. Operator/domain/observation: U=(W,render/resolve call,return); R follows current snapshots/lexical rules. Inner U minus R contains invented labels for missing IDs and matches for unsupported prefixes; terminating calls only.

Approximation: source-derived conditional relation; unknown behavior outside its declared domain remains unknown. Assumptions: shared preconditions above and descriptant dependency boundaries. Evidence: inspected source and callers; model.md#tests distinguishes executed checks. Falsifiable oracle: construct admissible inputs, execute the named operation and compare this predicate; contrary output refutes it.

## antecedents

Type: reverse_reconstruction. Operator/domain/observation: For observed label/ID vector y, retain all W/inputs yielding y; hidden fields, case alternatives and overwritten states stay free. No unique text-to-world reconstruction.

Approximation: source-derived conditional relation; unknown behavior outside its declared domain remains unknown. Assumptions: shared preconditions above and descriptant dependency boundaries. Evidence: inspected source and callers; model.md#tests distinguishes executed checks. Falsifiable oracle: construct admissible inputs, execute the named operation and compare this predicate; contrary output refutes it.

## text-encoding

Type: information_loss. Operator/domain/observation: Text, comma-joined Words and Tag can yield equal text_of results; all other values yield None. Embedded commas prevent unique Words recovery.

Approximation: source-derived conditional relation; unknown behavior outside its declared domain remains unknown. Assumptions: shared preconditions above and descriptant dependency boundaries. Evidence: inspected source and callers; model.md#tests distinguishes executed checks. Falsifiable oracle: construct admissible inputs, execute the named operation and compare this predicate; contrary output refutes it.

## plural-boundary

Type: lexical_constraint. Operator/domain/observation: hit(a,w) iff a=w OR a+s=w OR w+s=a. No irregular stemming, trimming or substring matching is present.

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
