# algorithm-levenshtein

## descriptant

### Algorithm file groupant

This is the original standalone levenshtein implementation. The authoritative Rust
source lives outside this sidecar. Forward recurrence, invariant and cost are
in model.md#approximant; five typed models are in model.md#models. The manifest binds the
current source hash and partial syntax projection. Syntax extraction does not
expand macros, resolve names or prove semantic correctness.

Composition: conjunction of the source identity, recurrence and invariant gives
a conditional implementation model; typed models answer distinct questions
about that relation. Their statements do not add to a global inverse identity.
Cost and tests qualify the model, rather than proving a larger caller's wiring.

Review refinement: generic slices can contain usize::MAX zero-sized elements.
An explicit guard now rejects nonempty maximal dimensions before row allocation;
a should-panic witness passed twice. The byte/char and full-table tests passed
again, the equality mutant remained red, and syntax/source identity was rebuilt.

## approximant

### Forward, invariant and cost

D(0,j)=j; D(i,0)=i; D(i,j)=min(D(i-1,j)+1,D(i,j-1)+1,
D(i-1,j-1)+[a_i!=b_j]). Put the shorter slice on columns, compute one row,
and swap row buffers. Return the final column of the final row.
Invariant: previous is the exact preceding row; current[0..=j] is the exact
current prefix. Old current suffix cells are overwritten before any read.

Time O(n*m) for nonempty slices and O(1) for an empty slice. Two rows require
O(min(n,m)) auxiliary storage (plus constant metadata).
No timing speedup is claimed; costly Eq operations multiply comparison costs.

## models

# Levenshtein models

Subject/revision: mod.rs SHA-256 in manifest.json. Observation is a single
terminating call on finite Eq slices; Eq must be a stable equivalence relation.
Allocation success is assumed. With both slices nonempty, maximum length must
be less than usize::MAX; larger zero-sized-element slices panic before allocation.
Empty-side calls return the other length without allocation.
Models are partial source-derived specifications, not machine-checked proofs.

## behavior_complement

Universe U consists of input pairs (a,b) and proposed natural distances d.
Define D by the unit-cost full-table edit recurrence. C={(a,b,d)|d!=D(a,b)}
is the exact specification complement and hypothesized absent-behavior inner
bound for Rust under the assumptions. In particular d<abs(|a|-|b|),
d>max(|a|,|b|), and d=0 with a!=b are excluded.
Oracle: exhaustive_table and a compiled inverted-equality mutant.

## reverse_reconstruction

Fix target b and result d. Compatible inputs are the edit sphere
{a | D(a,b)=d}; different edit scripts and different inputs can yield d.
For a fixed finite alphabet and length bound, filtering every sequence by the
full-table oracle gives a complete bounded preimage. Outside that bound
completeness is not claimed. Explicit preimage enumeration remains future work.

## symmetry_fiber

Type: argument_exchange. The operator swaps a and b, preserving D because
insertion/deletion are both unit cost and substitution equality is symmetric.
This is an exact specification invariance under the Eq assumption.
exhaustive_table checks both orientations of every pair.

## zero_fiber

Type: identity_preimage. Fix b and d=0; the sole compatible sequence value is
a=b, although memory addresses and representations may differ.
Zero-cost equality and strictly positive edits imply this relation.
Equal inputs and every other binary pair <=5 are checked by exhaustive_table.

## unit_projection

Type: representation_counterfactual. Hold strings fixed, change elements from
UTF-8 bytes to Unicode scalar values. The resulting distance need not be equal:
café🙂 to cafe🙂 has byte distance 2 and scalar distance 1.
This is a defined alternate observation, not an inverse encoding theorem.
caller_chooses_unicode_units executes this witness. Grapheme distances and
Unicode normalization are explicitly outside the model.

## tests

### Checks and limits

The neighboring source contains bounded independent reference-oracle tests.
A copied source outside the repository was mutated, compiled, and run against
the unchanged tests; nonzero exit was required and observed. No production
source mutation remains. Source was formatted before syntax projection and
has not changed since passing checks. Full-workspace integration is the
parent groupant's responsibility. No exhaustive machine-domain proof is claimed.

Three tests pass: 63 binary sequences in each position = 3,969 full-table pairs,
symmetry/bounds, generic integers and Unicode scalar/byte distinctions.
A maximal zero-sized-element slice is rejected before allocation.
Mutant replaces inequality with equality in substitution cost and fails both
semantic tests; the dimension guard test still passes.

## continuation

# Scalar fibers under right concatenation

## score_length_kernel_obstruction

Observation Q_p(t)=(D(p,t),|t|), with pattern p fixed and D unit-cost
Levenshtein. A proposed scalar continuation must factor D(p,t concatenated
with c) through (Q_p(t),c). Its necessity condition is that equal Q projections
stay indistinguishable after append followed by the distance observation.
There exist patterns with no such factorization: for two inequivalent values
A,B, choose p=[A,B], histories [A] and [B], next element B. Both Q values are
(1,1), but next distances are 0 and 1. The byte instance is p=ab, histories a,b,
next byte b. A pattern witnessing this collision and at least two inequivalent
element values are needed; this is not an obstruction for every Eq type or
every pattern.

For an Eq type with one equivalence class, D(p,t)=abs(|p|-|t|), and lengths
alone determine the continuation. The actual generic API checked 16 unit-type
cases (lengths0..3) including appended histories. Empty and singleton patterns
also admit score/length updates; the byte probe checks 90 contexts. These
positive cases reject a universal reading of the obstruction.

A full DP row contains more information; this file does not claim that every
entry is necessary or that the current two-row implementation exposes a
persistent streaming API. Source-derived obstruction and metric arguments
retain finite evidence, not a proof of universal implementation equivalence.
The compiled false factorization is rejected by unequal next values for the
same proposed update arguments.

## append_distance_residual

Type: reverse-fiber/forward transport. For fixed p,d,l,c define
`V(p,d,l,c)={D(p,t c) | D(p,t)=d, |t|=l}`.
It can be multivalued; the preceding witness gives {0,1} on the bounded alphabet
{a,b} and l=1. This relation composes compatible histories with the next action,
rather than undoing an append uniquely. The metric bound
`max(0,d-1)≤D(p,t c)≤d+1` excludes outside values but cannot identify a history
or force every value inside the interval. An exact inner/outer equality for
arbitrary alphabets and lengths is not claimed.

The matrix probe enumerates binary histories through length three: 1,278
ordered equal-score/length/context comparisons, including 450 reflexive and
828 non-reflexive (414 unordered). The 176 separating ordered pairs are 88
unordered pairs. No percentage approximation, throughput change or complete
unbounded preimage solver follows.

## History

### 2026-10-29 · descriptant

Epoch 2026-10-29: prospective expectations were saved outside the repository
before writing source. Independent finite oracles passed twice. A compiled
source mutation failed the same suite, demonstrating a red gate. These bounded
observations support the recurrence; they do not prove all inputs. Resource
exhaustion, expensive element comparisons and complete preimage execution are
explicit gaps. Preserve this evidence when extending the implementation.

Clippy refinement, 2026-10-29: the independent full-table oracle initializes
its first row through mutable enumeration. This retains D(0,j)=j and removes
a needless indexed loop. All three standalone tests passed twice after this
test-only change; source identity and partial syntax were regenerated.

Test extraction epoch, 2026-10-29: inline tests moved to sibling tests.rs with
their own source-bound groupant and registry entry; this module delegates through
a cfg(test) declaration. Production prefix remains byte-identical. Two new tests
extend bounded coverage; no production fix or timing gain is claimed.

### 2026-10-29 · tests

2026-10-29: compile independently with
`rustc --edition 2021 --test --crate-name mova_algorithm_levenshtein algorithms/src/levenshtein/mod.rs -o TEST_BINARY`.
Run TEST_BINARY twice with --test-threads=1. Both runs passed; test case outcomes
are deterministic. Elapsed-time footer is observational and may differ.

2026-10-29 expansion: old tests moved unchanged to ../tests.rs; two new tests
add broader independent-oracle and property coverage. See ../tests.rs.groupant/
for exact scope, costs and compiled negative-control evidence. Production prefix
was compared byte-for-byte with the pre-extraction snapshot and remains equal.

### 2026-11-01 · descriptant

2026-11-01 continuation review: see model.md#continuation and the source-bound continuation evidence. For pattern ab, score plus consumed length has nonfunctional right-context transport; the ab/a,b/b collision separates equal projections. Historical observations remain. Myers operational admission and scalar metric symmetry are separate; no production change or proof is claimed.

### 2026-11-01 · document

Source: `../mod.rs`, hash in owner manifest. Date: 2026-11-01.
Domain: finite caller-chosen Eq sequences with lawful, stable equivalence;
ordinary completed calls and successful allocation; both nonempty lengths
below usize::MAX. Elements and edit costs must stay the same between calls.
Every original and appended call must satisfy these conditions; with a
nonempty pattern, the appended history length must remain below usize::MAX.
No text normalization or change from bytes to scalar values is implicit.
