# Correspondences

This page describes typed mathematical correspondences between algorithm
groupants and ordinary Mova sources: what a correspondence is, how the
`mova-correspondence` scanner checks the catalogue, how to repeat a review, the
relation algebra and evidence model, short worked cases (gcd, digits and
Horner, LCS, edit-distance continuation, penalty tuning), and how algorithm
ideas are transferred into Mova through explicit adapters. Groupants
themselves are described in [groupants](groupants.md); vidtvorants and typed
obernants in [mathematical modeling](mathematical-modeling.md).

## What a correspondence is

The ownership registry is a tree: every Rust file has one owner. Correspondences
form a separate graph across that tree. They live in `models/correspondences.json`
as typed, source- and model-bound finite formulas — explicit relation
instances, never equations inferred from prose or matching names.

- Each atom binds named input and output sorts and indexed pairs, and the hashes
  of the source and model it describes. Hashes detect drift; they do not prove
  full implementation correspondence.
- Each formula has a type: `forward`, `reverse_reconstruction` or
  `behavior_complement`. Converse swaps endpoints; complement is taken inside
  the declared rectangle.
- Composition must preserve intermediate sorts, observation, assumptions and
  effects. Distinct atom symbols keep their identities even when their finite
  tables coincide.
- A converse or complement derived from a forward record is a valid scoped
  model but not independent evidence: equality after a common converse repeats
  forward equality. The scanner reports such construction edges in
  `declared_derivation_provenance`. Scoped complements are meaningful only
  relative to their declared universe; a claim about complete source behavior
  also needs output closure and explicit failure outcomes.

Source coverage, typed-formula coverage, finite equality and proved abstract
laws are different measurements. Prose, registration and matching names never
fill a missing formalization.

## The scanner

```sh
cargo build --offline --locked --release -p mova-correspondence -p mova-modelcheck
target/release/mova-correspondence scan "$REPO" "$REGISTRY" "$REPO/models/correspondences.json" "$RUN/scan-1.json"
target/release/mova-correspondence scan "$REPO" "$REGISTRY" "$REPO/models/correspondences.json" "$RUN/scan-2.json"
cmp "$RUN/scan-1.json" "$RUN/scan-2.json"
```

`REPO` is the reviewed checkout, `REGISTRY` its public registry or a freshly
composed one (`mova-modelcheck compose-registry BASE OVERLAY NEW_JSON`; keep
overlay reports out of public exports), `RUN` a new external run directory.
Catalogue and bindings must describe the same checkout. The output must be new;
missing or stale bindings stop the scan instead of refreshing claims. Two scans
of the same inputs must be byte-identical.

The report lists missing roles, typed formula comparisons, finite
coincidences, counterexamples, incompatible contracts and admitted
compositions. Pure algorithm, pure non-algorithm and mixed ancestors have
separate meanings; keep the report's classification. The normalizer is
deliberately incomplete: it builds a deterministic order-preserving
minimum-height composition tree, commutes converse with complement, keeps a
bounded fallback when pushing converse would exceed the node budget, and must
stay idempotent. Composition summaries distinguish `total` and `surjective`
from `functional` (at most one output) and `injective` (univalent converse),
including empty endpoint sets. The finite relation AST does not parse Markdown,
arithmetic recurrences, Rust macros or program semantics. See the
[tool README](../tools/correspondence/README.md).

Native bridges in `tools/correspondence/tests/` (`bridges.rs`,
`lcs_bridges.rs`) call the actual public APIs and bind each catalogue row to
declared input labels and computed output labels.

## Relation algebra

For R:X→Y, S:Y→Z and a forbidden relation T:X→Z,

    (R;S) ∩ T = ∅  ⇔  (Rᵒ;T) ∩ S = ∅  ⇔  (T;Sᵒ) ∩ R = ∅

connects forward computation, converse and exclusions. The residual

    R;S ⊆ T  ⇔  S ⊆ ¬(Rᵒ;¬T)

constrains a following stage. The abstract laws are proved in Lean
([composition](../formal/RelationComposition.md),
[residual](../formal/RelationResidual.md)); that proof is separate evidence
from a finite interpretation and from source correspondence.

## Repeating a review

1. Read the working rules and prior epoch evidence. Check the source revision,
   worktree changes and catalogue hashes. Edit in an isolated worktree.
2. Scan the current registry and inspect missing roles, comparisons,
   counterexamples and admitted compositions.
3. Pick one bounded candidate from real source/model mechanisms: a shared
   recurrence, a projected observation, a conservation law or invariant, a
   monotone bound, a converse fiber, the conditioning of an approximate
   inverse. A matching word is only a lead.
4. Before implementing, state input and output types, domain, observation,
   assumptions, effects and the expected relation. Add a manually reviewed
   formula only for what can be represented faithfully.
5. Check an independent witness against the actual Rust APIs. Include a
   plausible false candidate: lost state, a different scalar field, changed
   error behavior, non-injectivity, undefined arithmetic. Keep successful and
   rejected outcomes. Never certify a program by copying its desired result
   into both sides of a test.
6. Only after a useful mathematical result, consider a production change.
   Preserve reference behavior and measure the affected path before claiming
   speed or accuracy. Error composition needs compatible metrics, image/domain
   containment and a downstream sensitivity bound.
7. Update source-bound groupants, syntax artifacts and parent composition
   models; run focused tests and the modelcheck gates
   ([groupants](groupants.md#registry-and-checks)).

Rescanning an unchanged catalogue rediscovers the same relations. Progress is
a justified new formalization, a better adapter or a useful rejected lead —
not more identical reports. The loop needs no teacher, training, corpus or
held-out assessment.

## Worked cases

### Gcd and modular arithmetic

The algorithm gcd equals public arbitrary-integer `math::nt::gcd` after
lossless unsigned conversion on the declared domain. Both return zero at `(0,0)`. The converse keeps all compatible input pairs: output two has several
antecedents, so no unique inverse follows. For modular multiplication with
m≠0, an output y has gcd(a,m) preimages when the gcd divides y and none
otherwise; units give unique reconstruction. Algorithm CRT and `math::nt::crt`
agree on nonempty valid inputs whose lcm fits u64; their empty, zero-modulus
and overflow contracts differ.

Elimination: small unit-diagonal integer systems satisfy both rational
equations and modular reductions, but direct solver equivalence is rejected.
`998244353*x=1` is uniquely solvable over Q and inconsistent over
F_998244353.

### Digits and Horner

`math::nt::from_digits` folds most-significant-first integer coefficients;
the algorithm Horner API takes ascending coefficients and returns a value and a
derivative behind a `Result`. Reversing coefficients connects their values on
an exact domain. Write H_b(d)=Σ d_i b^(k-1-i) and D_b(n) for `digits(n,b)`
(u32 b≥2, canonical word of |n|, zero as `[0]`).

| relation | domain and observation |
|---|---|
| H_b(D_b(n)) = \|n\| | integer n, b≥2; the sign is lost |
| D_b⁻¹(c) = {H_b(c), −H_b(c)} | canonical nonempty words; the zero fiber is a singleton |
| N_b = D_b∘H_b, N_b² = N_b | magnitude normalization, not signed-value preservation |
| H_b⁻¹(n) = {(p, n − b·H_b(p))} | fixed length k≥1, p ∈ Z^(k−1); infinitely many antecedents for k≥2 |

Adjacent carries `(u,v)→(u+t, v−b·t)` preserve H. A successful Horner value
equals H when every conversion and value operation is exact; bounding base,
coefficients and intermediates by 2^53 is a sufficient premise. Real
counterexamples reject wider transfers: an intermediate rounding at b=2^53, a
base that aliases on conversion at b=2^53+1, and a derivative overflow that
makes the algorithm reject an input whose value is exactly zero. Without
reversal, `[1,2]` at base 10 gives 21 instead of 12.

Unrestricted Horner versus `Poly::eval_f64` also fails: `[0,0,10^308]` at one
overflows the derivative, so the algorithm rejects while the ordinary value is
finite.

### Longest common subsequence

`en::text::lcs` (`en/src/text.rs`) computes LCS length for tokenizer evaluation
and text metrics. It keeps two rolling rows and returns zero before allocating
when either side is empty; the comparison schedule is unchanged. The catalogue
holds three forward rows with one contract — `lcs.ordinary`, `lcs.algorithm`,
`lcs.bitparallel` — observing length only on listed byte fixtures (shorter side
at most 64, successful returns). The scanner reports finite coincidences
between the ordinary row and each algorithm row; same-direction LCS relations
do not compose with one another.

The histogram bound `Σ_t min(count_a(t), count_b(t)) ≤ min(len a, len b)` is an
upper bound only: `[1,2]` versus `[2,1]` has bound 2 and LCS 1. It is an
interval/filter idea, not a substitute value and not a percentage-error
certificate. Callers pass tokens or characters; raw UTF-8 byte LCS cannot
replace character LCS.

    cargo test --offline --locked -p mova-correspondence --test lcs_bridges

### Edit-distance continuation

For a fixed pattern p, let F_p(t) be unit-cost Levenshtein distance. A cache
that keeps only Q_p(t)=(F_p(t), |t|) cannot update the distance when a byte
arrives:

| pattern | history | score | length | next byte | new score |
|---|---|---|---|---|---|
| ab | a | 1 | 1 | b | 0 |
| ab | b | 1 | 1 | b | 1 |

The same arguments would need two different results. The converse of Q_p
followed by append and distance is a many-valued relation (T_ab(1,1,b)={0,1});
picking one representative history does not preserve future observations. A
parent model must keep the ambiguity or retain a stronger state (a row or bit
vector). Empty and singleton patterns, and an `Eq` type with one class, do admit
scalar updates. The metric still gives the partial exclusion
`max(0,d−1) ≤ F_p(tc) ≤ d+1`.

The Myers implementation checks pattern length ≤64 before text length ≤65536.
The metric is symmetric but the full `Result` is not: a 65-byte pattern against
empty text errors, while the exchanged call succeeds. Orienting the shorter
input as pattern would admit more calls but change error observations, so it
is not an equivalent replacement. Reproduce the check:

```sh
rustc --edition=2024 -O --crate-name continuation_check \
  algorithms/src/myers_bitvector/mod.rs.groupant/continuation-check.rs.txt \
  -o "$RUN/continuation-check"
"$RUN/continuation-check"
```

Models: [Myers](../algorithms/src/myers_bitvector/mod.rs.groupant/model.md#continuation),
[generic](../algorithms/src/levenshtein/mod.rs.groupant/model.md#continuation),
[parent composition](../models/algorithm-library.groupant/model.md#edit-continuation).

### Penalty model for segmentation

General Viterbi and the segmentation shortcut agree in objective for finite
nonnegative switch costs; paths may differ on ties, so this is not a
substitution. For fixed emissions, `V(λ)=max_p(E(p) − λ·switches(p))` is a
maximum of affine functions: convex, nonincreasing and piecewise linear for
λ≥0. At a unique optimum its derivative is minus the switch count; at ties the
subgradients span the competing slopes. This gives cheap consistency checks
and penalty tuning. A penalty that forces a constant path must be derived from
emission-score ranges, not chosen as an arbitrary large constant.

## Algorithm transfers

An algorithm model helps an application only through an explicit adapter that
preserves the application's observation. For reference M, algorithm A, input
adapter α and output projection β:

    Obs(M(x)) = β(x, A(α(x)))   on a declared domain.

Surrounding validation and effects stay part of the application contract.
Transfers add edges to the correspondence graph; they do not change algorithm
ownership or invent production calls. The catalogue records reviewed
transfers; it is not an automatic semantic matcher, and nothing is substituted
automatically.

| transfer | adapter and retained observation | resource policy |
|---|---|---|
| BFS → annotation gate | reverse each validated token→head arc; from root reachability pick the first unreachable token; keep Count/Form/Head/Registry/Roots precedence | original walks for ≤64 tokens; larger calls spend ≤4n head steps before whole-input BFS |
| root reachability → conversion gate | keep parsing, range, root-label and first-failed-start policies; coloring realizes the graph property without BFS | original walks for n≤64; otherwise n+1 colors and ≤2n head transitions |
| heap order → coreference mention matching | exact spans first; residual edges by descending overlap, prediction ordinal, gold ordinal; accept only free endpoints; keep greedy identities | sort only when both sides have ≥128 mentions and the checked product is ≤262144; otherwise scan |
| Hungarian storage schedule → assignment score | reuse slack/visited buffers, reset every augmentation; keep floating-point arithmetic and selection order | two scratch allocations instead of two per row |
| BFS adjacency → spatial inference | project directional facts and lawful inverse edges into ordered rows; keep vertex, far, capped steps and all-qualified flags | scan for E≤64; larger queries reject absent outgoing starts before building rows |

The root-reachability gate in closed form: after preflight, set
`successor[0]=0`, `successor[i]=head[i]`; a tree is accepted iff every token
shares token 0's cycle ID, and the first error is the least token outside that
basin. A parse success alone does not imply a tree.

Models: [syntax validation](../models/syntax-validation.groupant/),
[coreference observation](../models/coreference-observation.groupant/),
[spatial inference](../models/spatial-inference.groupant/).

### Rejected combinations

Failed combinations show what an adapter must retain:

- The first cycle member is not the first failed head-chain start: heads
  `[3,0,4,3]` fail at token 1, while the cycle is 3↔4.
- Global maximum assignment cannot replace greedy mention matching: overlaps
  `[[5,4],[4,0]]` give greedy total 5 and maximum 8, with different identities.
- Integer Hungarian cannot replace fractional CEAF assignment: truncating
  `[[.75,.5],[.5,0]]` changes the maximum from 1 to 0.
- A shortest path does not determine spatial Far: a longer path can carry Far
  or two qualified steps; the aggregate must keep that witness.
- A production replacement is adopted only after measuring the real path;
  candidates that regress on boundary or large shapes are rejected and the
  previous policy stays.

## Limits

- Finite CPU observations do not establish universal Rust equivalence,
  numerical error certificates, complete source semantics, GPU behavior or
  performance.
- Correspondences do not measure school results, training or linguistic
  accuracy.
- Partial syntax, registration and catalogue audits do not establish
  mathematical truth or universal floating-point accuracy.
- Exact derived constructions are dependent evidence, not independent checks.
