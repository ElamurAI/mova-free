# coref-score-matching

## forward
Exact-span pairs first; remaining same-key edges selected by descending overlap and ascending original prediction/gold order. Bounded sorted-edge path and scan fallback have the same selector.

## invariant
Inputs are immutable. Matching never reuses a gold mention and never crosses key buckets. Scratch lifetime changes preserve row-entry values; each successful augmenting path increases matched rows by one. Metric ratios retain zero-denominator behavior.

## behavior-complement
On admitted normally terminating inputs, excluded observations include reused gold identities, cross-bucket pairs, a residual selection preceding a strictly higher-priority available edge, and changed assignment objective under scratch reuse. This is a necessary-condition inner model of absent outputs, not exhaustive behavior enumeration.

## reverse-reconstruction
An output admits many compatible antecedents: unmatched mentions and nonselected edge weights can vary while leaving decisions unchanged; scalar metrics forget concrete mention labels. The preimage consists of inputs inducing the same ordered exact/greedy choices and numeric counts. Uniform label renaming gives compatible partition metrics, but recovering original inputs uniquely is impossible.

## tie-selection
Exact-span matches use first available gold input; residual priorities are (negative overlap, prediction ordinal, gold ordinal). Unique edge keys remove dependence on heap stability. Hungarian strict slack improvements and ascending scans remain unchanged.

## resource-partition
Residual min side below 128, checked product above 262144 or reserve error selects original scan before mutation. Successful static ordering has at most 262144 three-usize edges and two masks. This does not bound original group maps or runtime of fallback.

## state-reset
Persistent minv and used vectors are refilled each row. Equality to freshly allocated vectors at the loop boundary gives equal subsequent operations. Values from previous rows must not leak into current search.

## observation-projection
The full matching vector and exact f64 bits are differential observations; checking only overlap totals would miss identity changes. CEAF-e fractions are not quantized. Counts and output labels are separate observables.

## cost
Legacy residual scan O(PG min(P,G)); admitted sorted path O(PG log(PG)) time/O(PG) storage. Exact-span first pass unchanged. Assignment O(r²c) time/O(r+c) scratch; two scratch allocations replace 2r. Dense overlap O(KR+mentions) space/time terms remain. Pilot threshold16 regressions led to admission128; warmed final timing remains pending.

## composition
Metrics depend on matched identities, not merely overlap objective. Exact matching then residual priority establishes ids before score_doc partitions. Hungarian component alignment retains original arithmetic order. This fragment is part of the parent scorer; fragment correctness alone does not prove end-to-end metric interpretation.

## checks
Nine focused Rust tests twice debug/release; 2694 external differential cases twice debug/release. Independent assignment enumeration and residual choice oracle, explicit tie and resource gates. Compiled mutants and restoration recorded in parent evidence.

## epoch-memory

Initial prospective threshold16 and resource model preceded edits; exploratory regressions at16/32 prompted prospective amendment128, the lowest measured winning size, not an optimal crossover. See ../model.md#epoch1-evidence and ../model.md#prospective for retained history. Initial zero-slack mutant survived existing nonnegative assignment oracle; the negative-cost row-reset witness detects it. Preserve signed-domain fixtures when refactoring assignment scratch.

## limitations
Allocator failure is handled for added buffers but not legacy allocations. Invalid span arithmetic remains baseline behavior. Partial syntax projection has no semantic resolution, macro expansion or general correctness proof. No corpus, teacher, GPU, model or database test. No numeric approximant attached: outputs are exact categorical/float identity comparisons rather than an approximate substitute.

## History

### 2026-10-30 · epoch-memory

2026-10-30 initial prospective threshold16 and resource model preceded edits; exploratory regressions at16/32 prompted prospective amendment128, the lowest measured winning size, not an optimal crossover. See ../model.md#epoch1-evidence and ../model.md#prospective for retained history. Initial zero-slack mutant survived existing nonnegative assignment oracle; the negative-cost row-reset witness detects it. Preserve signed-domain fixtures when refactoring assignment scratch.
