# algorithm-kmp

## descriptant

### Algorithm file groupant

This is the original standalone kmp implementation. The authoritative Rust
source lives outside this sidecar. Forward recurrence, invariant and cost are
in model.md#approximant; five typed models are in model.md#models. The manifest binds the
current source hash and partial syntax projection. Syntax extraction does not
expand macros, resolve names or prove semantic correctness.

Composition: conjunction of the source identity, recurrence and invariant gives
a conditional implementation model; typed models answer distinct questions
about that relation. Their statements do not add to a global inverse identity.
Cost and tests qualify the model, rather than proving a larger caller's wiring.

## approximant

### Forward, invariant and cost

Forward: construct longest proper prefix/suffix lengths for every pattern
prefix, scan the text, emit every full match and resume from its longest border.
Invariant: matched is the length of a pattern prefix equal to a suffix of the
scanned text; after emitting, fallback preserves candidate overlapping matches.
The prefix table obeys 0<=prefix[i]<=i and identifies the longest proper border.

Time O(n+m+z), where z is output length (z<=n+1); auxiliary space O(m), plus
O(z) output. Each fallback decreases matched, charged against prior increments.
No wall-clock benchmark or speedup is claimed.

## models

# KMP models

Subject/revision: mod.rs SHA-256 in manifest.json. Observation is a single
terminating call, not intermediate execution traces. Allocation success is assumed.
All five models are source-derived partial specifications; finite evidence below
does not prove universal correspondence.

## behavior_complement

Domain U: byte text t, byte pattern p, any proposed finite list L of offsets.
Let W(t,p) be all i in ascending order with t[i..i+|p|]=p and
0<=i<=|t|-|p|, or every boundary 0..=|t| when p is empty.
The operator C={(t,p,L) in U | L != W(t,p)} is an exact complement of the
specification graph, a hypothesized inner bound on absent Rust behaviors under
normal termination. Missing matches, duplicate offsets, wrong order and omitted
overlaps are explicitly excluded. Oracle: exhaustive_windows, plus offset mutant.

## reverse_reconstruction

Fix p and an observed list L. The compatible preimage is
{t | W(t,p)=L}; no particular original text is recovered.
For a finite alphabet and length bound, enumerate every text and retain equality.
This is complete within that bounded domain, and generally many-to-one.
Oracle: the window specification defining exhaustive_windows; explicit preimage
enumeration is a future check.

## boundary_fiber

Type: boundary_preimage. Fix empty pattern and observed length k of output.
The preimage contains every text of length k-1 if k>=1, and none if k=0.
This erases all byte information. Exact specification relation under normal
termination. Empty and nonempty binary texts in exhaustive_windows are evidence.

## overlap_constraint

Type: overlap_compatibility. Fix pattern p and two proposed starts i<j with
j-i<|p|. Simultaneous matches require p[j-i..]=p[..|p|-(j-i)].
This is necessary and sufficient for compatibility of these two placements,
not for the complete output list because other matches may occur.
Oracle: aaa in aaaaa yields starts 0,1,2; conflicting borders remain excluded
by the exhaustive window oracle.

## byte_relabeling

Type: alphabet_equivariance. For any injective byte relabeling f on the used
alphabet, W(f(t),f(p))=W(t,p). Noninjective maps may create matches.
Exact relation of the equality-based specification. Binary exhaustive tests
support equality structure; universal relabeling is source reasoning, not a
separately executed exhaustive check.

## tests

### Checks and limits

The neighboring source contains bounded independent reference-oracle tests.
A copied source outside the repository was mutated, compiled, and run against
the unchanged tests; nonzero exit was required and observed. No production
source mutation remains. Source was formatted before syntax projection and
has not changed since passing checks. Full-workspace integration is the
parent groupant's responsibility. No exhaustive machine-domain proof is claimed.

Two tests pass: 127 binary texts times 31 binary patterns = 3,937 oracle pairs;
overlap, fallback, empty input and non-ASCII byte witnesses. Mutant adds one
to emitted offsets and fails both tests.

## History

### 2026-10-29 · descriptant

Epoch 2026-10-29: prospective expectations were saved outside the repository
before writing source. Independent finite oracles passed twice. A compiled
source mutation failed the same suite, demonstrating a red gate. These bounded
observations support the recurrence; they do not prove all inputs. Resource
exhaustion, expensive element comparisons and complete preimage execution are
explicit gaps. Preserve this evidence when extending the implementation.

Test extraction epoch, 2026-10-29: inline tests moved to sibling tests.rs with
their own source-bound groupant and registry entry; this module delegates through
a cfg(test) declaration. Production prefix remains byte-identical. Two new tests
extend bounded coverage; no production fix or timing gain is claimed.

### 2026-10-29 · tests

2026-10-29: compile independently with
`rustc --edition 2021 --test --crate-name mova_algorithm_kmp algorithms/src/kmp/mod.rs -o TEST_BINARY`.
Run TEST_BINARY twice with --test-threads=1. Both runs passed; test case outcomes
are deterministic. Elapsed-time footer is observational and may differ.

2026-10-29 expansion: old tests moved unchanged to ../tests.rs; two new tests
add broader independent-oracle and property coverage. See ../tests.rs.groupant/
for exact scope, costs and compiled negative-control evidence. Production prefix
was compared byte-for-byte with the pre-extraction snapshot and remains equal.
