# algorithm-myers-bitvector

## descriptant

### Memory

Original bounded byte myers_bitvector. Reference: Myers (1999), JACM 46, A Fast Bit-Vector Algorithm for Approximate String Matching. Prospective equations preceded implementation. Exact semantics and observed checks are distinct from syntax binding. Round-trip cannot alone validate two mutually wrong codecs; retain hand-derived streams and malformed checks. No timing claim or percentage approximant.

## model

### Mathematical combiner

For source S, partial syntax P(S), forward relation F and typed relations O, the sidecar records (hash(S),P(S),F,O,I). Source identity binds descriptions to bytes but does not prove F. Tests intersect compatible implementations with finite observations. Composition with callers requires domain caps and exact model identity; no global Mova accuracy claim follows. Distance zero admits all equal byte-pairs, e.g. (a,a) and (b,b); d alone cannot reconstruct either input.

## approximant

### Forward, invariant, and cost

distance is D[m,n]; D[i,j]=min(D[i-1,j]+1,D[i,j-1]+1,D[i-1,j-1]+[a!=b]). pv/mv encode adjacent row differences, top bit updates score. Empty pattern returns n.

O(256+m+n) time and constant 256-word mask storage; no allocation.

Pattern cap 64 precedes text cap 65536; even empty pattern observes text cap. Deliberate wrapping_add propagates bit carry modulo 2^64; score is nonnegative <=max(m,n), shifts are <=63.

Invariant: A returned d different from the edit-grid minimum is excluded; in particular equal bytes cannot yield positive distance.

Original implementation informed by Myers (1999), JACM 46, A Fast Bit-Vector Algorithm for Approximate String Matching; no copied implementation. Exact categorical semantics; no useful certified percentage surrogate is attached. Finite fixtures do not constitute universal proof.

## models

# Typed relations

## behavior_complement
A returned d different from the edit-grid minimum is excluded; in particular equal bytes cannot yield positive distance.

## reverse_reconstruction
Distance zero admits all equal byte-pairs, e.g. (a,a) and (b,b); d alone cannot reconstruct either input.

## transition_relation
distance is D[m,n]; D[i,j]=min(D[i-1,j]+1,D[i,j-1]+1,D[i-1,j-1]+[a!=b]). pv/mv encode adjacent row differences, top bit updates score. Empty pattern returns n.

## finite_arithmetic
Pattern cap 64 precedes text cap 65536; even empty pattern observes text cap. Deliberate wrapping_add propagates bit carry modulo 2^64; score is nonnegative <=max(m,n), shifts are <=63.

## resource_observation
O(256+m+n) time and constant 256-word mask storage; no heap allocation. No callbacks (count zero), no IO, no randomized state. Input errors are values. There is no allocator-failure branch in this body; stack exhaustion or process interruption is outside this call-level value model.

## deterministic_ties
The mask table is a fixed byte-indexed array; both input loops traverse in order. The API returns a scalar distance and does not select an edit traceback or expose optimal-path ties. Codec dictionary policies, patience witnesses and BPE merge ties belong to other modules; the old batch-wide wording did not describe this source.

## tests

Run cargo test -p mova-algorithms myers_bitvector::tests in debug/release twice. Preserve normalized transcripts, compiled negative control, and exact restoration under (see History). No benchmark evidence yet.

### Checks

## prospective

### Prospective model
Original bounded byte myers_bitvector implementation. Before implementation: exact transform, deterministic traversal, no callbacks; input cap 65536 (diff 4096), dictionary 4096, output cap min(caller,65536). Allocation may panic/abort; resource checks prevent size-driven overflow but do not certify allocator success. Myers uses bit-vector edit row recurrence with wrapping addition; LZW extends previous phrase by next first byte, freezing at dictionary capacity; LZ77 greedy longest previous match permits overlap; BPE repeatedly merges most frequent pair, lexicographically smallest tie, applying learned merges in order; patience diff recursively anchors unique shared bytes by increasing subsequence and uses delete/insert fallback. Expected costs respectively O(n), O(n * phrase length), O(n*window*match), O(merges*n*log n), O(n^3) conservative. Correctness is exact distance or lossless reconstruction; patience edit count is not guaranteed minimal. No percentage approximant justified. Independent DP/exhaustive and round-trip/malformed checks planned; no timing claim.

## continuation

# Oriented admission and continuation observations

## ordered_error_fibers

Domain: finite valid byte slices p,t, with lengths m,n; usize arithmetic and
the current caps. Observation includes the error tag, not only a successful d.
Assumptions: ordinary process execution; no unsafe invalid slices.
The exact preflight fibers are
`PatternTooLong ⇔ m>64` and
`TextTooLong ⇔ m≤64 ∧ n>65536`.
On the remaining rectangle, success is specified as `Ok(D(p,t))`, where D is
unit-cost global Levenshtein distance. These error predicates follow directly
from the ordered guards; successful bit-recurrence correctness is a separate
source-derived claim with finite oracle evidence.
Behavior complement in this tagged universe excludes success outside the
rectangle, excludes either error outside its fiber and excludes `Ok(q)`
with q≠D inside it. Observing an error does not imply a large edit distance:
two identical 65-byte inputs give PatternTooLong although D=0.
The error fibers constrain lengths but do not identify bytes or an edit script.
At (m,n)=(65537,65537), PatternTooLong wins; TextTooLong is excluded.
Evidence: the 128 ordered cap-grid calls in `continuation-evidence.json`.

## orientation_admission_closure

Type: argument-exchange domain closure. Domain and assumptions as above.
An orientation admits a value iff its pattern length≤64 and text length≤65536.
Consequently at least one of (p,t),(t,p) succeeds exactly when
`min(m,n)≤64 ∧ max(m,n)≤65536`; putting the shorter side in pattern position
admits that union. If both orientations succeed, both values specify D, by
symmetry of unit-cost edits. Full Result symmetry is false:
(65 zero bytes, empty) gives PatternTooLong; its exchange gives Ok(65).
For (empty,65537 zero bytes), the two error tags are TextTooLong and
PatternTooLong, so exchanging cannot rescue every length-rejected call.
This is an analysis of a possible adapter, not a new adapter implementation.
It deliberately changes which error is seen and cannot be called a full-API
equivalence. The cap grid checks the union predicate independently of contents.
No percentage approximation or measured speed advantage follows.

## right_context_kernel_obstruction

Type: failure of projected-state sufficiency. Fix p and let
`F_p(t)=D(p,t)`, `Q_p(t)=(F_p(t),|t|)`, and `A_c(t)=t concatenated with c`.
Domain: byte strings with |p|≤64 and |t|+1≤65536, so the continuation remains
admitted. A deterministic update g_p satisfying
`F_p(A_c(t))=g_p(Q_p(t),c)` for every such t,c would require
`Q_p(u)=Q_p(v) ⇒ F_p(A_c(u))=F_p(A_c(v))`.
Counterexample: p=ab, u=a, v=b, c=b. Both projections are (1,1), but the next
distances are 0 and 1. Thus there exist fixed patterns, including ab, for which current score,
consumed length and next byte cannot determine the next score. The converse fiber loses
information needed by append. This refutes a scalar-only streaming cache; it
does not refute retaining pv/mv or a full dynamic-programming row.
Finite evidence: an independent matrix oracle and both actual Rust APIs checked
1,278 equal-score/length/context comparisons, with 176 separating pairs in the
declared binary domain through length three. Of the 1,278 ordered comparisons,
450 compare a history with itself and 828 are non-reflexive (414 unordered).
The 176 separating ordered pairs are 88 unordered pairs. These are test
observations, not equivalence classes or universal coverage.

This is not an obstruction for every pattern. For an empty pattern F(t)=|t|,
so append increments the score. For a one-byte pattern p and l>0,
F_p(t)=l-[p[0] occurs in t]. Membership is determined by l and d; on an
empty history it is false. Thus the next score is
l+1-[previous membership or c=p[0]], an exact scalar/length update.
The probe checks 90 such empty/single-byte-pattern contexts. This positive
boundary is a source-derived metric argument with finite implementation checks.

## append_preimage_transport

Type: converse/forward cross-composition. Fix p,d,l,c in the admitted domain.
The next-score relation is
`T_p(d,l,c)={F_p(A_c(t)) | F_p(t)=d ∧ |t|=l}`.
This is the converse of Q followed by append and F, retaining fixed p,c.
It is not generally a function: `T_ab(1,1,b)` contains both 0 and 1.
For the bounded alphabet {a,b}, length l=1, exhaustive enumeration gives exactly
{0,1}. Outside that finite alphabet, completeness is not claimed by enumeration.
Reverse observations preserve a set of compatible histories, not a unique
reconstruction. Replacing that set with one representative may invalidate a
downstream answer even when the stored score and length are exactly correct.
Evidence: the fixed collision and independent enumeration in the same harness.
No executable complete unbounded reconstructor is attached.

## append_drift_partial_exclusion

Type: partial excluded relation, distinct from full behavior complement.
On admitted p,t,c as above, `|F_p(A_c(t))-F_p(t)|≤1`.
One insertion gives the upper bound; deletion of the appended byte gives the
reverse bound by the edit metric triangle inequality. Therefore a proposed
next score outside `[max(0,d-1),d+1]` is impossible, but a value inside that
interval need not be realizable for a particular history. This interval is not
a percentage error guarantee and does not determine the continuation.
At the text cap, append may yield TextTooLong, which is outside this numeric
observation; do not extend the interval over errors.
Evidence: each equal-projection continuation in the finite probe checked drift.
The argument is mathematical/source-derived, not a machine-checked source proof.

## Historical scope clarification

Earlier prospective, dated descriptant and deterministic-tie notes were written for a five-codec
batch and mention LZW, LZ77, BPE and patience diff. They remain dated history,
not behavior of this Myers function. This module has no dictionary or edit
traceback selection. Its result is global distance, not minimum substring
matching distance. No historical observation or known failure is erased.

## validation

### Observed validation

The five-module batch adds 14 native tests. All pass twice debug and twice release; sorted named test records match across all four runs. Initial integrated library run: 632 passed. Strict library Clippy passed. Each production file has a compiled mutant rejected by a named assertion, followed by byte-for-byte restoration and a passing module rerun: Myers score+1/exhaustive_dp; LZW flipped decoded byte/known_and_malformed; LZ77 match length-1/greedy_overlap_and_invalid; BPE tie comparison >=/ties_and_model_identity; patience anchor Insert instead of Equal/unique_anchor_and_nonminimal_witness.

## History

### 2026-10-30 · descriptant

Heading `### Epoch memory — 2026-10-30` stays in descriptant as `### Memory`.

### 2026-10-30 · tests

Run cargo test -p mova-algorithms myers_bitvector::tests in debug/release twice. Preserve normalized transcripts, compiled negative control, and exact restoration under $MOVA_DATA/runs/octopus-wave25-sequences-2026-10-30. No benchmark evidence yet.

### 2026-10-30 · prospective

Heading `### Prospective model — 2026-10-30` stays in prospective as `### Prospective model`.

### 2026-10-30 · validation

Heading `### Observed validation — 2026-10-30` stays in validation as `### Observed validation`.

Each of ten source/test files has paired identical partial syntax projections, refreshed after all mutants were restored. Receipts: $MOVA_DATA/runs/octopus-wave25-sequences-2026-10-30/{focused-debug-1.log,focused-debug-2.log,focused-release-1.log,focused-release-2.log,normalized-*,mutations.log,mutant-*,restored-*,syntax.log,clippy.log}. No timing or speedup evidence; allocator failures untested. These finite observations and lexical source bindings do not establish a semantic proof or global Mova quality improvement.

### 2026-11-01 · descriptant

2026-11-01 continuation review: see model.md#continuation and the source-bound continuation evidence. For pattern ab, score plus consumed length has nonfunctional right-context transport; the ab/a,b/b collision separates equal projections. Historical observations remain. Myers operational admission and scalar metric symmetry are separate; no production change or proof is claimed.

### 2026-11-01 · document

Source: `../mod.rs`, SHA-256 in the owner manifest. Date: 2026-11-01.
Observation: one completed call returning `Result<usize, Error>`; input elements
are bytes. This is a source-derived specification and finite actual-source
evidence, not a formal correctness proof. No production replacement is proposed.
