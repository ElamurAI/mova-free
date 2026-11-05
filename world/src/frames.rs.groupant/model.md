# frame-adapter

## descriptant

# frame-adapter

## Forward

Adapter re-exports prag frames, acquires world annotator, supplies parse_cased words to read_with, and implements CLI diagnostics/lexical reports. Reading mutates discourse; course traversals use one context per lesson. Parser, pragmatic reader and lexicons remain dependency boundaries.

## Invariant

Query methods borrow immutable worlds; the frame reader mutates caller-owned discourse. Reverse models preserve compatible hidden inputs, never assume unique recovery. Ordering and failure assumptions in catalogs are part of the contract.

## Cost

For F facts,T tokens,N frames,V distinct verbs/classes: sum parser/reader dependency costs dominates. Aggregation O((T+N) log V), sorting O(V log V), storage O(V+current discourse/read). vnfreq output O(rows+bytes). No parser complexity or speedup claim.

## Composition

Fragments cover parser adapter, coverage accounting, and diagnostic recording/NOMLEX. CLI selects one branch; read checks annotator then read_with; parse_words acquires cached annotator per sentence. prag read_with skips empty trees and mutates Ctx. main.rs passes args to main_args; fbench/prules call read, but knowledge.rs also calls re-exported read_with with another parser. Recorder state is global, outside Ctx.

## AST gap

Source-bound syn-2 partial syntax is present. Macro expansion, name resolution, type inference, cfg evaluation, external-module loading and semantic AST remain absent. Child envelopes may overlap and need not cover the file.

## Epoch memory

## catalog

# Typed models: frame-adapter

Observations are single terminating calls unless a recording trace is named. Preconditions: finite well-formed inputs, stable dependencies, no allocation failure or integer overflow.

## nonbehavior

Type: behavior_complement. Operator/domain/observation: U=(entrypoint,input,initial discourse/dependencies,return,effects) over terminating adapter calls; R delegates parsing/reading/reporting. Inner U minus R contains read success after initial annotator Err, parse_words surfacing Err as Result, and filtered be/Bare/nominal frames increasing verb counts.

Approximation: source-derived conditional relation; unknown behavior outside its declared domain remains unknown. Assumptions: shared preconditions above and descriptant dependency boundaries. Evidence: inspected source and callers; model.md#tests distinguishes executed checks. Falsifiable oracle: construct admissible inputs, execute the named operation and compare this predicate; contrary output refutes it.

## antecedents

Type: reverse_reconstruction. Operator/domain/observation: Observed Read/final Ctx admits all (text,initial Ctx,question,parser/dependency states) satisfying read_with after successful acquisition. Parsing/context aggregation are lossy; no unique inverse parser.

Approximation: source-derived conditional relation; unknown behavior outside its declared domain remains unknown. Assumptions: shared preconditions above and descriptant dependency boundaries. Evidence: inspected source and callers; model.md#tests distinguishes executed checks. Falsifiable oracle: construct admissible inputs, execute the named operation and compare this predicate; contrary output refutes it.

## parse-error

Type: failure_partition. Operator/domain/observation: parse_words returns parse_cased words on annotator Ok, empty Vec on Err. read first uses annotator()?; failure is observable Err rather than successful empty Read.

Approximation: source-derived conditional relation; unknown behavior outside its declared domain remains unknown. Assumptions: shared preconditions above and descriptant dependency boundaries. Evidence: inspected source and callers; model.md#tests distinguishes executed checks. Falsifiable oracle: construct admissible inputs, execute the named operation and compare this predicate; contrary output refutes it.

## reader-delegation

Type: dependency_relation. Operator/domain/observation: read forwards text,mutable ctx,question,parse_words exactly to prag read_with. Re-exported read_with is also called directly by knowledge.rs with a different parser; it bypasses this adapter.

Approximation: source-derived conditional relation; unknown behavior outside its declared domain remains unknown. Assumptions: shared preconditions above and descriptant dependency boundaries. Evidence: inspected source and callers; model.md#tests distinguishes executed checks. Falsifiable oracle: construct admissible inputs, execute the named operation and compare this predicate; contrary output refutes it.

## lesson-context

Type: state_reset. Operator/domain/observation: lexicon_coverage/vnfreq/nominal_check create default Ctx per lesson and reuse across that lesson's facts; no discourse carry across lessons.

Approximation: source-derived conditional relation; unknown behavior outside its declared domain remains unknown. Assumptions: shared preconditions above and descriptant dependency boundaries. Evidence: inspected source and callers; model.md#tests distinguishes executed checks. Falsifiable oracle: construct admissible inputs, execute the named operation and compare this predicate; contrary output refutes it.

## verb-inclusion

Type: filter_partition. Operator/domain/observation: Count verb iff nonempty, not be, not Bare and not By::Nominal. Nominal frames increment event counts by how.class before exclusion. Roles-by uses same inclusion as verb counts.

Approximation: source-derived conditional relation; unknown behavior outside its declared domain remains unknown. Assumptions: shared preconditions above and descriptant dependency boundaries. Evidence: inspected source and callers; model.md#tests distinguishes executed checks. Falsifiable oracle: construct admissible inputs, execute the named operation and compare this predicate; contrary output refutes it.

## weighted-coverage

Type: ratio_fiber. Operator/domain/observation: For map n, t=sum counts,y=key count,tv=sum known(v)*n[v],yv=known key count. cover_line prints 100*tv/t and 100*yv/y to one decimal, zero for zero denominator; rounded outputs admit many maps.

Approximation: source-derived conditional relation; unknown behavior outside its declared domain remains unknown. Assumptions: shared preconditions above and descriptant dependency boundaries. Evidence: inspected source and callers; model.md#tests distinguishes executed checks. Falsifiable oracle: construct admissible inputs, execute the named operation and compare this predicate; contrary output refutes it.

## phrasal-known

Type: fallback_relation. Operator/domain/observation: known(v) iff classes_of(v) nonempty OR classes_of(first literal-space component) nonempty. It does not trim or split underscores/other whitespace.

Approximation: source-derived conditional relation; unknown behavior outside its declared domain remains unknown. Assumptions: shared preconditions above and descriptant dependency boundaries. Evidence: inspected source and callers; model.md#tests distinguishes executed checks. Falsifiable oracle: construct admissible inputs, execute the named operation and compare this predicate; contrary output refutes it.

## ranked-report

Type: order_constraint. Operator/domain/observation: Missing verbs and nominal event classes sort descending count then ascending name; only top 30 missing/top 40 event classes printed. BTreeMaps order other aggregates for fixed read results.

Approximation: source-derived conditional relation; unknown behavior outside its declared domain remains unknown. Assumptions: shared preconditions above and descriptant dependency boundaries. Evidence: inspected source and callers; model.md#tests distinguishes executed checks. Falsifiable oracle: construct admissible inputs, execute the named operation and compare this predicate; contrary output refutes it.

## recording-trace

Type: failure_effect. Operator/domain/observation: vnfreq starts global recording before course loading, takes/stops only after all reads succeed. Course/read errors retain active recorder; concurrent reads can contaminate counts. Successful model assumes exclusive recorder access; fs::write is not atomic.

Approximation: source-derived conditional relation; unknown behavior outside its declared domain remains unknown. Assumptions: shared preconditions above and descriptant dependency boundaries. Evidence: inspected source and callers; model.md#tests distinguishes executed checks. Falsifiable oracle: construct admissible inputs, execute the named operation and compare this predicate; contrary output refutes it.

## nominal-agreement

Type: conditional_ratio. Operator/domain/observation: nominal_check keeps nomlex rows with >=7 tab fields and verb-nom in comma-split field 4; first noun row wins. same_t counted only where both lexicons exist AND verbs agree. Only NOUN tokens; external-file errors precede courses.

Approximation: source-derived conditional relation; unknown behavior outside its declared domain remains unknown. Assumptions: shared preconditions above and descriptant dependency boundaries. Evidence: inspected source and callers; model.md#tests distinguishes executed checks. Falsifiable oracle: construct admissible inputs, execute the named operation and compare this predicate; contrary output refutes it.

## cli-dispatch

Type: control_partition. Operator/domain/observation: main_args dispatches args[1] to six named diagnostic branches; otherwise joins args[1..], question=text.contains('?'), fresh Ctx. Require len>=1: empty args panic. FRAMES_TREE/HOW test successful environment lookup including empty values.

Approximation: source-derived conditional relation; unknown behavior outside its declared domain remains unknown. Assumptions: shared preconditions above and descriptant dependency boundaries. Evidence: inspected source and callers; model.md#tests distinguishes executed checks. Falsifiable oracle: construct admissible inputs, execute the named operation and compare this predicate; contrary output refutes it.


## tests

### Checks

File-root model.md#evidence records structural checks and a bounded source-extraction Rust probe of attention and coverage arithmetic only. Other catalog oracles are prospective. world/src/tests.rs contains existing history and state-query tests. No complete course, teacher, GPU or neural-parser run is claimed. Parser artifacts, pragmatic reader, lexical tables, external NOMLEX and course loading remain dependency gaps.

## evidence

### Frame-adapter evidence

Baseline: 4279da541. Source unchanged. Root frame-adapter has 12 typed models; parser, coverage and diagnostics fragments have four each. All eight nodes across query/frames bind current source and partial syntax; total 48 typed entries.

Read world/src/frames.rs, prag::frames::read_with entry (empty-tree skip and caller-owned Ctx), prag::vn recording functions, world main/fbench/prules and knowledge callers. The knowledge path can use re-exported read_with with a separate parser. No universal routing through parse_words is claimed. read prechecks annotator and propagates failure; direct parse_words swallows acquisition failure into an empty vector.

The actual unchanged known and cover_line bodies were extracted by ../query.rs.groupant/probe.rs.txt into a std-only Rust harness with a deliberately small Lex stub. Sixty-four frequency assignments, three exact preimages of one formatted coverage report and three phrasal head boundaries pass. A false token-weight formula is rejected. Two runs have identical summary bytes. This covers accounting and lookup fallback mechanics, not real VerbNet/WordNet content, parsed frames or course behavior. Full probe procedure and combined structural checks are recorded in ../query.rs.groupant/model.md#evidence.

Potential runtime defect retained explicitly: vnfreq starts process-global recording before fallible course/read operations; early Err does not call record_take and leaves recording active. After successful reads record_take drains counts before a fallible direct fs::write, with no atomic replacement or rollback. This was established by source inspection, not live fault injection. No source edits or diagnostic/course executions were made.

NOMLEX remains external test-only data. The nominal agreement transitivity count is conditional on matching verbs, not all shared nouns. No such data was read or embedded. Parser initialization, neural artifacts, pragmatic reading, real lexicons and external-course/file availability remain unresolved dependencies; no teacher, GPU, full course, full Cargo or semantic AST run is claimed.

Syntax regeneration and unchanged-registry structural validation repeat byte-identically. registry-node.json is an integration descriptor for the parent; no registration or complete inference proof is asserted here.

## History

### 2026-10-29 · document

Revision 2026-10-29; SHA-256 86c42a1bd562ae24c760a7cbfc3b9b2bab20845da9d7945f3d80f091b8ed44b0.

### 2026-10-29 · Epoch memory

2026-10-29: source-derived first epoch. Historical snapshots do not freeze current labels; direct read_with callers bypass the adapter. Recording errors can retain global state. Preserve these observations when strengthening models. Runtime source is unchanged.

### 2026-10-29 · document

Subject: ../frames.rs; SHA-256 86c42a1bd562ae24c760a7cbfc3b9b2bab20845da9d7945f3d80f091b8ed44b0. Revision 2026-10-29.

### 2026-10-29 · evidence

Heading `### Frame-adapter evidence — 2026-10-29` stays in evidence as `### Frame-adapter evidence`.

Source SHA-256: 86c42a1bd562ae24c760a7cbfc3b9b2bab20845da9d7945f3d80f091b8ed44b0.

Syntax artifact SHA-256: a3afef2e2b3b6e8cdd70bc6da25df30bc345318e44242049e3c422e84ade3070.
