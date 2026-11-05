# frame-adapter-parser

## descriptant

# frame-adapter-parser

## Forward

Direct parser emptiness differs from public read Result failure; successful read forwards mutable discourse.

## Invariant

Query methods borrow immutable worlds; the frame reader mutates caller-owned discourse. Reverse models preserve compatible hidden inputs, never assume unique recovery. Ordering and failure assumptions in catalogs are part of the contract.

## Cost

For F facts,T tokens,N frames,V distinct verbs/classes: sum parser/reader dependency costs dominates. Aggregation O((T+N) log V), sorting O(V log V), storage O(V+current discourse/read). vnfreq output O(rows+bytes). No parser complexity or speedup claim.

## Composition

Selected source functions only; parent composes shared identity and dependencies. These are not independent pipeline stages or a complete partition.

## AST gap

Source-bound syn-2 partial syntax is present. Macro expansion, name resolution, type inference, cfg evaluation, external-module loading and semantic AST remain absent. Child envelopes may overlap and need not cover the file.

## Epoch memory

## catalog

# Typed models: frame-adapter-parser

Observations are single terminating calls unless a recording trace is named. Preconditions: finite well-formed inputs, stable dependencies, no allocation failure or integer overflow.

## nonbehavior

Type: behavior_complement. Operator/domain/observation: U=(text,ctx,q,annotator outcome,return,final ctx), R checks acquisition then read_with. Inner U minus R includes successful read after initial annotator Err.

Approximation: source-derived conditional relation; unknown behavior outside its declared domain remains unknown. Assumptions: shared preconditions above and descriptant dependency boundaries. Evidence: inspected source and callers; model.md#tests distinguishes executed checks. Falsifiable oracle: construct admissible inputs, execute the named operation and compare this predicate; contrary output refutes it.

## antecedents

Type: reverse_reconstruction. Operator/domain/observation: Read/final Ctx admits all text/initial Ctx/parser states producing it; skipped empty trees prevent unique original text reconstruction.

Approximation: source-derived conditional relation; unknown behavior outside its declared domain remains unknown. Assumptions: shared preconditions above and descriptant dependency boundaries. Evidence: inspected source and callers; model.md#tests distinguishes executed checks. Falsifiable oracle: construct admissible inputs, execute the named operation and compare this predicate; contrary output refutes it.

## direct-empty

Type: failure_fiber. Operator/domain/observation: Empty parse_words vector admits annotator Err or successful empty parse; the observation cannot distinguish them.

Approximation: source-derived conditional relation; unknown behavior outside its declared domain remains unknown. Assumptions: shared preconditions above and descriptant dependency boundaries. Evidence: inspected source and callers; model.md#tests distinguishes executed checks. Falsifiable oracle: construct admissible inputs, execute the named operation and compare this predicate; contrary output refutes it.

## delegate-state

Type: state_transition. Operator/domain/observation: Fixed dependencies make successful read equal read_with(text,ctx,q,&parse_words); read neither clones nor resets ctx. Reader correctness is external.

Approximation: source-derived conditional relation; unknown behavior outside its declared domain remains unknown. Assumptions: shared preconditions above and descriptant dependency boundaries. Evidence: inspected source and callers; model.md#tests distinguishes executed checks. Falsifiable oracle: construct admissible inputs, execute the named operation and compare this predicate; contrary output refutes it.


## tests

### Checks

File-root ../model.md#evidence records structural checks and a bounded source-extraction Rust probe of attention and coverage arithmetic only. Other catalog oracles are prospective. world/src/tests.rs contains existing history and state-query tests. No complete course, teacher, GPU or neural-parser run is claimed. Parser artifacts, pragmatic reader, lexical tables, external NOMLEX and course loading remain dependency gaps.

## History

### 2026-10-29 · document

Revision 2026-10-29; SHA-256 86c42a1bd562ae24c760a7cbfc3b9b2bab20845da9d7945f3d80f091b8ed44b0.

### 2026-10-29 · Epoch memory

2026-10-29: source-derived first epoch. Historical snapshots do not freeze current labels; direct read_with callers bypass the adapter. Recording errors can retain global state. Preserve these observations when strengthening models. Runtime source is unchanged.

### 2026-10-29 · document

Subject: ../../frames.rs; SHA-256 86c42a1bd562ae24c760a7cbfc3b9b2bab20845da9d7945f3d80f091b8ed44b0. Revision 2026-10-29.
