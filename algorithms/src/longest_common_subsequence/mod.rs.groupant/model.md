# algorithm-longest-common-subsequence

## descriptant

### longest_common_subsequence mod.rs

Original standard-library implementation of a classical dynamic programming algorithm, not a novelty claim. Source is authoritative; sidecar models and syntax are partial. Models compose recurrence, bounds and public result; tests observe this composition rather than establish a universal proof. No approximate execution profile is supplied because this API promises exact results; finite evidence is not an analytical error guarantee.

## model

# longest_common_subsequence mod.rs model

## Forward

Returns maximum common ordered-subsequence length using F(i,j)=F(i-1,j-1)+1 for equal final symbols, otherwise max(F(i-1,j),F(i,j-1)); empty prefixes have length zero.

## Invariant

Before each cell, above is F(i-1,j), row[j-1] is F(i,j-1), diagonal is F(i-1,j-1). Each value lies in 0..min(i,j); the result cannot overflow usize.

## Cost

O(nm) equality calls, O(min(n,m)) words; equality's own cost is caller-dependent. Empty case O(1). No timing measured.

## nonbehavior

Type: behavior_complement. For lawful Eq and successful termination exclude results above min lengths, results below any common subsequence, results without an attaining subsequence, mutated inputs or an emitted witness. Eq panics and allocation aborts are not ordinary returned lengths.

## preimages

Type: reverse_reconstruction. A result k admits exactly pairs having a common subsequence of length k and none of length k+1. Many unrelated pairs map to k; symbol values, lengths and witness cannot be uniquely recovered.

## swap

Type: metamorphic_relation. L(a,b)=L(b,a) under symmetric lawful Eq. Transposing inputs also selects the same smaller dimension, not a full matrix.

## reverse

Type: metamorphic_relation. Reversing both inputs reverses every common subsequence bijectively and preserves length; reversing just one need not preserve it (ab versus ba).

## append

Type: metamorphic_relation. Appending the same symbol to both gives L(a+[x],b+[x])=L(a,b)+1. Appending only to one changes length by zero or one. Identity L(a,a)=len(a).

## diagonal

Type: state_relation. Updating a row is lossy: previous above must be saved before overwrite. Given new row alone, old diagonal is generally ambiguous. Equal-symbol recurrence consumes the old diagonal, not the newly updated left cell.

## order-fiber

Type: reverse_reconstruction. Length zero admits all pairs with disjoint equality-class support, including empty inputs. For k>0, multiplicity intersection alone is insufficient: ab/ab yields two, ab/ba one, despite equal histograms.

## resources

Type: failure_partition. Empty shorter slice returns zero without allocation. Otherwise checked length+1 may panic, Vec byte-capacity/allocation failure may panic or abort, Eq may panic, or ordinary recurrence returns. These are not typed returned errors; impossible extreme allocations are not tested.

## evidence

### Evidence

LCS independent oracle enumerates subsequences of each first argument and scans the second, checking all 63 squared binary-word pairs through length five (3,969 pairs). Scheduling independently enumerates subsets and pairwise compatibility for all 5,461 arrays through length six over four item choices. Metamorphic tests include explicit input-fiber ambiguity, boundary and invalid-input cases. Allocator exhaustion and unreachable allocated-slice dimension/weight overflow cases were not executed.

Strict Clippy all-targets with -D warnings passed. rustfmt --check passed for all four owned Rust files. Installed mova-modelcheck syntax ran twice per source and produced byte-identical projections. Syntax is partial: macros and semantic typing are not expanded.

## prospective

### Prospective

Search of algorithms/src found neither named implementation. Original implementations of classical dynamic programming, no algorithmic novelty claimed. LCS saves the overwritten diagonal and uses one shorter row: O(nm) equality calls, O(min(n,m)) memory. Scheduling sorts by (end,start,index), binary-searches compatible prefixes and reconstructs strict improvements: O(n log n) time, O(n) memory. Adjacent half-open intervals are compatible; invalid start>=end rejected first; ties skip the current sorted item. Inputs unchanged. Validate independent exponential oracles, boundary and metamorphic relations, repeated debug/release outcomes and compiled production mutants. Allocation exhaustion and pathological Eq are outside checked behavior; no timing claims.

## correspondence

# LCS correspondence models

Let `L(a,b)` be the returned length, `n=|a|`, `m=|b|`, `r=min(n,m)`;
`a <=seq b` means increasing indices in `b` match every element of `a`.
The common domain is finite slices with stable, lawful `Eq`, representable row
width, successful allocation and completed calls. Equality classes use the
caller's `Eq`, not spelling or hashes. Panicking or changing comparisons,
process failure and timing are outside these scalar relations. Exact symbolic
targets are source-derived; finite tests do not prove all-input correspondence.

## Extremal score and empty trace fibers

Type: `reverse_reconstruction`. Hold `n,m` fixed. For scalar observation,

    L^-1(0) = {(a,b): support(a) and support(b) are disjoint}
    L^-1(r) = {(a,b): shorter <=seq longer}.

Here support consists of occurring equality classes. With equal lengths the
upper condition is pointwise equality; with `r=0`, both fibers are the entire
fixed-length domain. A common class supplies a length-one subsequence, proving
the lower characterization. A subsequence of length `r` consumes every shorter
position, proving the upper one. Neither fiber determines literal tokens,
longer input, or matching indices.

For the richer source observation `(result,row-construction-entered,
equality-invocations)`, `(0,false,0)` has precisely the empty-side pairs as its
preimage. Every completed nonempty call constructs one `r+1`-word row and makes
exactly `n*m` equality invocations (a mathematical count, not a Rust product).
Allocator internals are not observed. Thus `[]/[a]` and `[a]/[b]` for unequal
`a,b` share scalar zero but have different traces. Inferring emptiness from zero
alone is false. `[a]/[a,b]` and `[a]/[b,a]` both score one, so saturation does
not reconstruct positions either.

Oracles: [boundaries_and_generic_eq,
symmetry_identity_reversal_and_extension, and
preimage_ambiguity_and_false_multiset_model](../tests.rs) cover empty, identity,
bounds, unequal lengths and disjoint nonempty support. The independent
`exhaustive_subsequence_oracle` covers 3,969 binary-word pairs through length
five. Trace counts and general fiber characterizations are source-reviewed;
these existing tests do not instrument allocation or enumerate all fibers.

## Equality transport through alphabet renaming

Type: `equality_transport`. Hold positions and lengths fixed. A one-element
map `h` into another lawful stable equality domain satisfying

    x == y iff h(x) == h(y)

on the occurring elements preserves `L(h(a),h(b))=L(a,b)`. Every DP equality
branch agrees, so induction preserves the entire mathematical DP table. The
shorter-side orientation and row width also agree. Callback effects, equality
costs and input element sizes need not agree. A global alphabet bijection is
unnecessary; preserving and reflecting equality on the finite support suffices.

If `h` only preserves equality, matching index sequences survive and
`L(h(a),h(b)) >= L(a,b)`, but equality can fail: merging distinct `a,b` changes
`[a]/[b]` from zero to one. A map that splits an equality class lacks even this
premise. Renaming produces additional compatible antecedents of each score;
it does not reconstruct the original alphabet.

Finite witness: [preimage_ambiguity_and_false_multiset_model](../tests.rs)
checks `ab/ab` and `cd/cd` at score two. The exhaustive oracle checks a fixed
binary alphabet. General renaming and full-table transport are source-derived
derivations, not separately executed properties in these existing tests.

## Histogram exclusion and order loss

Type: `behavior_complement`, inner bound. Let `count_a(c)` be multiplicity of
an occurring equality class. Then

    H(a,b) = sum_c min(count_a(c), count_b(c))
    0 <= L(a,b) <= H(a,b) <= r.

The universe is completed-call triples `(a,b,q)` with `q: usize`; actual scalar
behavior is `q=L(a,b)`. The model excludes `q>H(a,b)`. Each common subsequence
uses at most each class's smaller multiplicity. This is a sufficient exclusion,
not the full complement: `ab/ba` has `H=2`, `L=1`, and the impossible result two
survives the bound. There need not be a survivor for every input (`H=0` is exact).

For `k>=1` distinct tokens, a list against its reversal has `H=k`, `L=1`,
and gap `k-1`, strictly positive for `k>=2`; against itself the gap is zero.
Disjoint support gives both zero. The histogram discards order and is not a
percentage-error surrogate. Reconstructing only pairs with `L=H` loses valid
antecedents.

[preimage_ambiguity_and_false_multiset_model](../tests.rs) refutes the unordered
value model. The integration bridge
[histogram_bound_is_a_partial_excluded_relation_not_the_value](../../../../tools/correspondence/tests/lcs_bridges.rs)
checks the same counterexample and partial exclusion on seven inputs and four
candidate outputs. `actual_en_algorithm_bitparallel_and_oracle_agree_on_small_byte_domain`
checks `L<=H<=r` on 3,969 binary pairs. These finite checks are not complete
absence enumeration.

## Representation and trait admission for API correspondence

Type: `observation_domain`. Observe only the returned scalar on identical
input elements. For byte slices with `r<=64`,

    longest_common_subsequence::length(a,b)
      = en::text::lcs(a,b)
      = bit_parallel_lcs::length(a,b).unwrap().

For lawful `Eq` tokens the first equality also applies without the byte cap,
subject to successful execution. This algorithm chooses longer rows and
shorter columns; the English API keeps left-argument rows. Wave37 English LCS
reuses two rows; this source saves an old diagonal in one row. Scalar equality
does not imply equal schedules, allocations, comparison receiver order or
callback effects. Bit-parallel LCS returns an error when both byte inputs
exceed 64; this generic API has no such semantic cap.

The generic bound is `T: Eq`, whereas the English API accepts `T: PartialEq`.
An asymmetric type implementing only `PartialEq` cannot instantiate this API;
a dishonest `impl Eq` does not satisfy the mathematical premise. The bridge
`comparison_order_is_a_major_with_left_receiver` uses an asymmetric logging
probe only with English LCS. It tests that API's schedule, not an admissible
counterexample to this lawful-`Eq` model. Stable lawful `Eq` may log calls too,
so even within the domain only scalar correspondence is claimed.

Tokens, Unicode scalar values and UTF-8 bytes are distinct observation units.
For `éa/aé`, character LCS is one and byte LCS is two. Neither API implicitly
tokenizes, decodes, normalizes or groups graphemes. A one-element encoding that
preserves and reflects equality fits alphabet transport; variable-length UTF-8
does not. Encoding arbitrary tokens into bytes also requires at most 256
occurring equality classes.

In [lcs_bridges.rs](../../../../tools/correspondence/tests/lcs_bridges.rs),
`actual_en_algorithm_bitparallel_and_oracle_agree_on_small_byte_domain` compares
all three actual APIs and a subsequence oracle on 3,969 binary pairs, then
English/generic/oracle on 14,641 ternary pairs through length four.
`unicode_characters_are_not_bytes_and_bitparallel_boundary` checks the unit
counterexample and 64/65 boundary. `catalogue_forward_rows_bind_actual_outputs`
checks seven catalogue observations and rejects swapped outputs/domain labels.
`common_contract_makes_ordinary_and_algorithm_rows_finite_equal` reports finite
coincidence and rejects incompatible assumptions. These integration references
were reviewed in the wave37 bridge source; integrated execution and final
cross-source hashes belong to the root receipt.

## Validation and remaining limits

Existing source and forward-model bytes are preserved. The wave37 integration
bridge was reviewed but not run in this isolated baseline worktree; the root
integration receipt owns those executions and ancestor bindings. General
alphabet/state transport, trace counts and full fibers remain source-derived.
No corpus, training, live model, GPU, speedup, numerical approximation budget
or complete-system correctness claim follows.

## History

### 2026-10-30 · descriptant

Epoch 2026-10-30: prospective cost and invariants retained in implementation model.md#prospective. The reverse models retain ambiguous input fibers and separate behavior exclusions from reconstruction. No timing optimization claimed.

### 2026-10-30 · evidence

Heading `### Evidence epoch 2026-10-30` stays in evidence as `### Evidence`.

Final source validation: two debug and two release executions of the complete mova-algorithms package passed 176 unit tests and one doctest each. Nine new tests belong to these two algorithms (four LCS, five scheduling). Sorted named test outcomes were identical across all four runs; elapsed time and build chatter were excluded. Normalized outcome SHA-256: c38c9be833de92508d6cd6304a57c36c196c4c4ef57561fa9c7f361dd7816255.

Compiled production negative control: replace diagonal + 1 by diagonal; real tests fail. Mutant log SHA-256 06baa16dba164cddf913d8455300a11c29c12bbb561def59459eb3576c3c5346. Original source restored before final repeated runs.

Artifacts live under $MOVA_DATA/runs/octopus-wave4-dynamic-2026-10-30. A temporary local lib declaration was used for package integration tests and restored. Registry-entry copies are supplied for main-instance integration; shared registry hierarchy checks remain the integrator's responsibility. These observations establish bounded correctness evidence and sensitivity to the named fault, not a proof or benchmark gain.

### 2026-10-30 · prospective

Heading `### Prospective epoch 2026-10-30` stays in prospective as `### Prospective`.

### 2026-10-31 · descriptant

#### Correspondence refinement, 2026-10-31

Four [source-reviewed relations](model.md#correspondence) add fixed-length extremal
and empty-trace fibers, equality-preserving alphabet transport, histogram inner
exclusion, and the shared observation domain of the actual three LCS APIs.
The histogram loses order; the bound does not define the value or full behavior
complement. Token/character/byte units and `Eq` versus `PartialEq` admission are
explicit. The one-row algorithm may reorder comparison receivers to keep the
shorter dimension; matching scalars do not imply trace equivalence with English
LCS. Existing forward-model bytes, source, syntax and historic evidence remain
unchanged. Finite module checks and reviewed integration test references do not
establish a whole-domain API substitution or any approximation error budget.

### 2026-10-31 · document

Source review, 2026-10-31. Subject: [length](../mod.rs), SHA-256
`fec6e6aeea147e5954f49bcc927484c2ba68465f1495a01ad1bbb507e5f72f51`.
The original [forward model](model.md#model) remains unchanged. These additions refine
observations and compatible antecedents; they supply no executable surrogate.

### 2026-10-31 · Validation and remaining limits

Worker validation, 2026-10-31: compile each unchanged algorithm module with
`rustc --edition=2024 --test`, once normally and once with `-O`; run each binary
twice with `--test-threads=1`. All four existing tests per module pass in every
run (eight distinct checks); normalized named outcomes match across repetitions
and profiles. This invokes the actual module and its adjacent test source, not
a copied implementation. No new executable property is inferred from prose.

The existing `mova-modelcheck groupants` binary validates the full unchanged
registry plus these sidecars twice; reports match byte-for-byte, SHA-256
`2ab01879fc6282dab0f5ff3c871b9a8fffbb63d44918a1819bf7616bca1a3c2f`.
A temporary nonexistent H2 selector in the bit-parallel manifest is rejected
with `valid:false`; its report SHA-256 is
`81a00d6ebb1b4a1cec0a073067f7abcae434e583af53df89d290938b3bdf0939`.
The exact positive manifest is restored. This control tests reference checking,
not mathematical adequacy. Raw logs and reports are under
`$MOVA_DATA/runs/octopus-wave37-lcs-groupants-2026-10-31`.

### 2026-11-04 · document

Migration: consolidated into model.md, 2026-11-04, operator decision; fresh hash-bound references to the former files were rebound to the sections below.
