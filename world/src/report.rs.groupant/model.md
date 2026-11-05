# world-report

## model

# world-report

Measurement aggregation, deterministic sampling and manual verdict presentation.

## forward

eval_tale obtains state answer, anchor fallback and baseline then scores; Agg sums supplied r_slm/r_anchor/r_base and separately sums state-source r_slm. report folds rows into fixed attribute order plus ordered maps for explicitness/story/first-space intent. sample ranks eligible identities by salted wrapping FNV/finalizer. read_verdicts skips malformed rows, overwrites duplicate keys; manual_md averages only matched recognized-type rows.

## invariant

For Agg built from add, 0<=answered<=n; zero denominators use max(1). Only source determines answered membership; score fields are trusted and may be inconsistent with source. Manual scores are trusted rather than masked when Src::None. Unknown attributes contribute to all totals but lack a displayed attribute slice.

## cost

N rows and total text bytes L: aggregation O(N log G + L), formatting linear in emitted bytes, sampling scans N once per seven attributes and sorts each slice O(N log N) overall with repeated key allocation. Manual lookup O(N log V), verdict decoding O(L + V log V). eval_tale cost includes external answer/baseline/scoring dependencies; no measured speed claim.

## composition

main loads persisted worlds and Qa, calls eval_tale, rows_tsv, tale_md, sample/report, then read_verdicts/manual_md. Tests construct Row directly and exercise aggregation/format/sample/verdict joins; they do not execute World answering. Native Qa roundtrip tests elsewhere preserve input identity but do not prove full assessment wiring.

## epoch-memory

First source-bound modeling epoch. Synthetic native fixtures only; no held-out tales, live parsers or annotators. CSV lost a quoted empty final record; a zero-length known section prevented decomposition progress. The repaired scanner retains quoted-field presence and decomposition ignores empty candidates. Report observations preserve existing permissive score/label semantics rather than silently enforcing a new measurement policy. Native finite evidence does not establish unrestricted correctness.

## checks

Run cargo test -p world --lib report::wave9_models -- --test-threads=1. See the root sidecar model.md#evidence for repeated debug/release checks and compiled controls. Partial syntax excludes macro expansion and type/name resolution. No executable percentage approximant is claimed.

## behavior-complement

Subject: current bound source and this node's scope. Domain and observation: Universe: finite Row sequences, initial default Agg, outputs. Inner absent relation excludes answered>n, changes to answered sum from Src::None, or NaN for the empty default aggregate line. Arbitrary public Agg values and nonfinite score fields are outside these invariants.

Status: source-derived partial relation, supported only where named finite fixtures apply. Assume successful allocations, fixed dependencies and bounded native inputs; external filesystem/answer-generation effects remain unknown. The relation describes one terminating call, not all process traces.

## reverse-reconstruction

Subject: current bound source and this node's scope. Domain and observation: For a numeric/printed aggregate keep all ordered row lists with matching counts and sums, subject to state membership. Permutations, identities and answers are lost. Nine pairs from {0,.5,1} have five sum fibers of sizes 1,2,3,2,1. Rounded output merges additional nearby sums.

Status: source-derived partial relation, supported only where named finite fixtures apply. Assume successful allocations, fixed dependencies and bounded native inputs; external filesystem/answer-generation effects remain unknown. The relation describes one terminating call, not all process traces.

## answered-denominator

Subject: current bound source and this node's scope. Domain and observation: SLM mean denominator is max(n,1); state-only mean uses max(answered,1). Native 216 cases enumerate eight source masks and 27 score triples using independent half-integer sums; a denominator mutant is rejected.

Status: source-derived partial relation, supported only where named finite fixtures apply. Assume successful allocations, fixed dependencies and bounded native inputs; external filesystem/answer-generation effects remain unknown. The relation describes one terminating call, not all process traces.

## empty-score

Subject: current bound source and this node's scope. Domain and observation: Default Agg emits N=0 and all zero metrics, including 0% not in state. This is a display convention, not an estimate of an empty population. report emits all even when empty; manual emits no all row without matched verdicts.

Status: source-derived partial relation, supported only where named finite fixtures apply. Assume successful allocations, fixed dependencies and bounded native inputs; external filesystem/answer-generation effects remain unknown. The relation describes one terminating call, not all process traces.

## unknown-attribute

Subject: current bound source and this node's scope. Domain and observation: Unrecognized type is accumulated under ? but the output loop visits only ATTRS. Hence displayed type counts need not sum to all. Sampling/manual also omit unknown types. The native unknown fixture records current behavior; no policy repair is asserted.

Status: source-derived partial relation, supported only where named finite fixtures apply. Assume successful allocations, fixed dependencies and bounded native inputs; external filesystem/answer-generation effects remain unknown. The relation describes one terminating call, not all process traces.

## cell-fiber

Subject: current bound source and this node's scope. Domain and observation: cell escapes pipe then maps LF/TAB to space: distinct a-LF-b and a-TAB-b share output. It does not escape CR or backslashes. TSV applies it to content but not story/attribute/ex fields; unrestricted metadata can break column shape.

Status: source-derived partial relation, supported only where named finite fixtures apply. Assume successful allocations, fixed dependencies and bounded native inputs; external filesystem/answer-generation effects remain unknown. The relation describes one terminating call, not all process traces.

## sampling-identity

Subject: current bound source and this node's scope. Domain and observation: For fixed salt and exclusion keys (story,n), sample returns up to k per recognized attribute sorted by hash. Stable sort preserves input order on hash ties/duplicate identities, so permutation invariance requires distinct hashes. Fixture checks distinct native identities for two salts and k=0..4.

Status: source-derived partial relation, supported only where named finite fixtures apply. Assume successful allocations, fixed dependencies and bounded native inputs; external filesystem/answer-generation effects remain unknown. The relation describes one terminating call, not all process traces.

## verdict-overwrite

Subject: current bound source and this node's scope. Domain and observation: Lines with fewer than five columns or parse failures skip; duplicate (story,n) uses last parsed row, optional comment at column six and later columns discarded. Score parser accepts NaN, negative and >1 numbers; no [0,1] validity follows.

Status: source-derived partial relation, supported only where named finite fixtures apply. Assume successful allocations, fixed dependencies and bounded native inputs; external filesystem/answer-generation effects remain unknown. The relation describes one terminating call, not all process traces.

## manual-selection

Subject: current bound source and this node's scope. Domain and observation: Manual denominator is count of sampled recognized rows with a matching verdict, including duplicate sampled rows. It is not sample length, unique-key count or state-source count. Missing verdicts are omitted and none is a separate count.

Status: source-derived partial relation, supported only where named finite fixtures apply. Assume successful allocations, fixed dependencies and bounded native inputs; external filesystem/answer-generation effects remain unknown. The relation describes one terminating call, not all process traces.

## manual-score-fiber

Subject: current bound source and this node's scope. Domain and observation: Given displayed percent totals, retain all matched verdict maps and sample row multiplicities with those rounded sums. Src::None does not force manual score zero; fixture with manual score one proves source and adjudication are independent. Assessment truth and score calibration remain external.

Status: source-derived partial relation, supported only where named finite fixtures apply. Assume successful allocations, fixed dependencies and bounded native inputs; external filesystem/answer-generation effects remain unknown. The relation describes one terminating call, not all process traces.

## evidence

### Bounded native evidence

CSV canonical encoder exercises 196 calls; reconstruction checks three whitespace fibers and four/five-word thresholds. Aggregation checks 216 source/score cases plus nine pair fibers. Sampling checks two salts and five budgets with exclusion/permutation fixtures; collisions remain outside that invariance claim.

Old CSV code fails csv_finite_encoder_oracle on a quoted empty field at EOF. Old empty-known candidate binary is stopped by an owned one-second timeout (exit 124); the fixed direct test terminates. Compiled mutants change orphan threshold five to six and answered denominator to all rows, respectively; each produces a named assertion failure, not a compile error.

Structural syntax projections and registry checks bind artifacts, not semantic truth. File import/read transactions and eval_tale answering remain source-inspected only. Public structs, unrestricted metadata, section counts above u16, accepted nonfinite/manual scores and unknown types remain explicit domain limits.

Structural validation: syntax regeneration compares byte-identically twice per source. External combined registry groupants passes with no diagnostics; changed finds both owned sources registered. The isolated temporary root has no repository sidecar; integration ancestors remain separately owned.

## History

### 2026-10-30 · epoch-memory

2026-10-30: first source-bound modeling epoch. Synthetic native fixtures only; no held-out tales, live parsers or annotators. CSV lost a quoted empty final record; a zero-length known section prevented decomposition progress. The repaired scanner retains quoted-field presence and decomposition ignores empty candidates. Report observations preserve existing permissive score/label semantics rather than silently enforcing a new measurement policy. Native finite evidence does not establish unrestricted correctness.

### 2026-10-30 · evidence

2026-10-30. Twelve new tests (six per file) pass twice in debug and twice in release with one test thread; normalized test identities/results match across all four runs. The restored production bytes pass twelve tests again after controls. No held-out data, live parser, annotator, teacher or GPU was invoked. Existing unrelated compiler warnings remain.

Raw logs: $MOVA_DATA/runs/octopus-wave9-input-2026-10-30/.

- old-csv.log: SHA-256 2f4192a8d7e601430b29c46e4599ec12de09a59650a5d0d4f334b173aa94d358

- old-progress.log: SHA-256 11d953211240bedb2e9e1a08d22a8648e9150db53ed1af2dda0f7a632f19e98d

- debug-1.log: SHA-256 dd36598815808526e70d8f81e2e975848c8a27530c6db317d8190aa41358f41a

- debug-2.log: SHA-256 9ea954b576e5750e2261d40269bdd4114ca2629b29ae1f3c6ac2586abb18fd5e

- release-1.log: SHA-256 775721320b289426e2c1988d5f250b548a19a275a0bcbfe60b5f22b253baa98c

- release-2.log: SHA-256 50412e040cfa318f3351da7c87503902f61acb2d499e1c3c803673ccdcc74c24

- mutant-ftqa.log: SHA-256 2e78f80ab321ac151c458fd5ea9d5f58ba5ecbfb8afd275ea71955e838e21e22

- mutant-report.log: SHA-256 d052828184aff07844b48d407e8f40ff0bcc216b5427dc5af79a0d7b17a4df03

- restored.log: SHA-256 06ec03b5b2ed109aea50cd5d592b59c9c7d3e98d6a3ad82859a68a4eaeab056b

Final source SHA-256: 73053fdec39edc3c16980b73db6418c477bcb60b9fa93a708aff431950e69825.

- groupants-full.json SHA-256: 004960d36ae5898dbb9da1f797cccd1195faeab73f9e0f676e4141045e36573c

- changed.json SHA-256: 27c2b43111467637ca924746f390c47d76c4142bd22a59b34d237e54de03e19f
