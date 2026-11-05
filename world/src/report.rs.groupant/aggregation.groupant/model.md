# world-report-aggregation

Automated scoring adapter and aggregate report slices.

## forward

eval_tale obtains state answer, anchor fallback and baseline then scores; Agg sums supplied r_slm/r_anchor/r_base and separately sums state-source r_slm. report folds rows into fixed attribute order plus ordered maps for explicitness/story/first-space intent. sample ranks eligible identities by salted wrapping FNV/finalizer. read_verdicts skips malformed rows, overwrites duplicate keys; manual_md averages only matched recognized-type rows.

## invariant

For Agg built from add, 0<=answered<=n; zero denominators use max(1). Only source determines answered membership; score fields are trusted and may be inconsistent with source. Manual scores are trusted rather than masked when Src::None. Unknown attributes contribute to all totals but lack a displayed attribute slice.

## cost

N rows and total text bytes L: aggregation O(N log G + L), formatting linear in emitted bytes, sampling scans N once per seven attributes and sorts each slice O(N log N) overall with repeated key allocation. Manual lookup O(N log V), verdict decoding O(L + V log V). eval_tale cost includes external answer/baseline/scoring dependencies; no measured speed claim.

## composition

eval_tale masks nonstate automated r_slm to zero, but public Row can contain other scores; Agg does not repeat that mask.

## epoch-memory

First source-bound modeling epoch. Synthetic native fixtures only; no held-out tales, live parsers or annotators. CSV lost a quoted empty final record; a zero-length known section prevented decomposition progress. The repaired scanner retains quoted-field presence and decomposition ignores empty candidates. Report observations preserve existing permissive score/label semantics rather than silently enforcing a new measurement policy. Native finite evidence does not establish unrestricted correctness.

## checks

Run cargo test -p world --lib report::wave9_models -- --test-threads=1. See the root sidecar ../model.md#evidence for repeated debug/release checks and compiled controls. Partial syntax excludes macro expansion and type/name resolution. No executable percentage approximant is claimed.

## behavior-complement

Subject: current bound source and this node's scope. Domain and observation: For default/add histories state count cannot exceed total count. No absence claim covers manually constructed inconsistent Agg or nonfinite scores.

Status: source-derived partial relation, supported only where named finite fixtures apply. Assume successful allocations, fixed dependencies and bounded native inputs; external filesystem/answer-generation effects remain unknown. The relation describes one terminating call, not all process traces.

## reverse-reconstruction

Subject: current bound source and this node's scope. Domain and observation: Sums/counts forget row identity and ordering; fixed all-state half-unit pair sum yields fibers [1,2,3,2,1] in the finite oracle.

Status: source-derived partial relation, supported only where named finite fixtures apply. Assume successful allocations, fixed dependencies and bounded native inputs; external filesystem/answer-generation effects remain unknown. The relation describes one terminating call, not all process traces.

## denominators

Subject: current bound source and this node's scope. Domain and observation: All-row and answered-only ratios have distinct denominators; empty ratios use one. Native 216-case integer oracle tests arithmetic before and after formatting.

Status: source-derived partial relation, supported only where named finite fixtures apply. Assume successful allocations, fixed dependencies and bounded native inputs; external filesystem/answer-generation effects remain unknown. The relation describes one terminating call, not all process traces.

## slice-loss

Subject: current bound source and this node's scope. Domain and observation: report omits the ? bucket from the type table while retaining it in all and other maps. Intent key is first substring before ASCII space, possibly empty; it is not a parsed intent enum.

Status: source-derived partial relation, supported only where named finite fixtures apply. Assume successful allocations, fixed dependencies and bounded native inputs; external filesystem/answer-generation effects remain unknown. The relation describes one terminating call, not all process traces.

## History

### 2026-10-30 · epoch-memory

2026-10-30: first source-bound modeling epoch. Synthetic native fixtures only; no held-out tales, live parsers or annotators. CSV lost a quoted empty final record; a zero-length known section prevented decomposition progress. The repaired scanner retains quoted-field presence and decomposition ignores empty candidates. Report observations preserve existing permissive score/label semantics rather than silently enforcing a new measurement policy. Native finite evidence does not establish unrestricted correctness.
