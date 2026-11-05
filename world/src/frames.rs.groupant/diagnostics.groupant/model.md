# frame-adapter-diagnostics

## descriptant

# frame-adapter-diagnostics

## Forward

Diagnostic dispatch and recording expose file/reader errors and nontransactional side effects.

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

# Typed models: frame-adapter-diagnostics

Observations are single terminating calls unless a recording trace is named. Preconditions: finite well-formed inputs, stable dependencies, no allocation failure or integer overflow.

## nonbehavior

Type: behavior_complement. Operator/domain/observation: U=(args,env,files,courses,recorder,return,effects); R executes dispatch/diagnostics. Inner U minus R includes successful nominal_check after missing external file and unsupported text selecting a named diagnostic.

Approximation: source-derived conditional relation; unknown behavior outside its declared domain remains unknown. Assumptions: shared preconditions above and descriptant dependency boundaries. Evidence: inspected source and callers; model.md#tests distinguishes executed checks. Falsifiable oracle: construct admissible inputs, execute the named operation and compare this predicate; contrary output refutes it.

## antecedents

Type: reverse_reconstruction. Operator/domain/observation: vnfreq summary admits all recorder maps/read traces with matching verb,row,clause totals and course/header strings; summary does not determine table rows.

Approximation: source-derived conditional relation; unknown behavior outside its declared domain remains unknown. Assumptions: shared preconditions above and descriptant dependency boundaries. Evidence: inspected source and callers; model.md#tests distinguishes executed checks. Falsifiable oracle: construct admissible inputs, execute the named operation and compare this predicate; contrary output refutes it.

## record-error

Type: residual_state. Operator/domain/observation: After record_start, course/read Err precedes record_take and leaves RECORD true. A write Err after record_take leaves recording off and counts drained. No rollback or atomic file replacement.

Approximation: source-derived conditional relation; unknown behavior outside its declared domain remains unknown. Assumptions: shared preconditions above and descriptant dependency boundaries. Evidence: inspected source and callers; model.md#tests distinguishes executed checks. Falsifiable oracle: construct admissible inputs, execute the named operation and compare this predicate; contrary output refutes it.

## nomlex-first

Type: priority_fiber. Operator/domain/observation: or_insert retains first qualifying noun row, so later duplicates cannot alter it. same_t excludes verb-disagreement pairs even if transitivity flags match.

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
