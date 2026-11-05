# algorithm-longest-increasing-subsequence-rust-approximant

## descriptant

### Certified LIS witness companion

Prediction: accepted O(n) comparisons and O(L) output storage can beat the
production O(n log n) algorithm and its O(n) predecessor storage on nearly
increasing data. Vec grows with actual output, not preallocated n on traps.
Drop the greedy allocation before fallback. Rejected inputs pay O(n) overhead
before exact reconstruction. No universal speedup or corpus-wide bound claimed.
Allocation/panic behavior and Ord comparison side effects are not equivalent.

Invariants: indices are in bounds and strictly increasing; referenced values
strictly increase. Output length never exceeds V. No mutation, cloning, hashing,
or approximate comparisons. Accepted paths need not match production indices
or tie choices. Exact fallback preserves production tie policy. Input values
must supply a consistent total Ord; allocation and comparisons must succeed.

Composition: length-only consumers inherit the relative objective bound.
Consumers of identities, tie choices, downstream branches or values need their
own argument; no parent-wide approximation bound follows automatically.

## models

# Typed models

Subject: rust_approximant.rs at manifest SHA; finite slices, lawful stable Ord,
normal termination, successful allocation. Observations include witness I,
upper bound U=n and fallback flag f. Let G(x) denote the deterministic greedy
scan and E(x) the production reconstruction. Budget p is one of 1/2/5/10.

## behavior_complement

Universe: (x,p,I,U,f). An inner bound on absent outputs consists of invalid
indices, non-strict referenced order, U != n, objective loss above p/100, or
empty input returning nonempty I. An exact scoped relation is the complement
of [(certificate(G(x),n,p) and I=G(x) and not f) or
(not certificate(G(x),n,p) and I=E(x) and f)], with U=n.
This relation describes behavior, not only a list of negative tests. It relies
on the exact reference specification and excludes resource/trait failure.

## reverse_reconstruction

Given (I,U,f,p), compatible x have length U and either greedy selection I
with passing certificate when f=false, or rejected greedy certificate and
production reconstruction I when f=true. For the greedy branch, each chosen
index is the first later index whose value exceeds the preceding chosen value;
index zero is chosen for every nonempty input; skipped values do not exceed
the latest selected value. Final trailing values also do not exceed its tail.
These constraints plus certificate characterize that branch's preimage.
Many order-isomorphic and translated inputs share an output, so recovery is
relational, not unique. The fallback preimage inherits production tie policy.

## invariant

Every push uses a later input index with strictly greater value. Thus the
witness is feasible and L <= OPT <= n. Certificate and exact fallback establish
the cap separately; replacing strict comparison or removing the gate breaks it.

## cost

n scan steps, at most n-1 comparisons, amortized O(L) output pushes; O(n) time
with constant-cost Ord. Fallback drops output then pays production O(n log n)
time and O(n) space. Geometric capacity growth follows L, never eagerly n.

## tests

### Evidence

Five Rust tests passed twice in debug. Independent subset-mask oracle checks
every ternary sequence of lengths 0..7 against all four budgets and the
production optimum. Results are repeated and structurally validated. Empty,
singleton, duplicates, descending order and signed extremes are covered.
Certificate arithmetic is compared with cross multiplication on small bounds,
including one-past-bound invalid lower values and usize::MAX overflow edges.

For every budget p, n=ceil(200/p), input [2,0,1,3,...,n-1] gives a certified
nonoptimal witness n-2 versus optimum n-1. Thus approximate execution is real.
The trap [100,0,1,2,3,4] must use exact fallback. Compiling a mutant which
replaces the acceptance gate with true makes that trap test fail on the
objective-loss assertion (exit 101). No modified production source was used.

Finite checks support implementation; the interval argument supplies the
scoped analytical bound. Central runner measurements remain separate; no
speed or whole-system guarantee is claimed by this evidence.

The same five tests also passed with optimized surrogate compilation. Strict
standalone compilation and rustfmt checks cover the new companion; full shared
runner and global model-check integration are delegated to the parent branch.

## History

### 2026-10-30 · descriptant

Prospective model, 2026-10-30, recorded before implementation: greedily retain
index zero and each subsequent value strictly above the last retained value.
The resulting witness has length L; exact optimum V satisfies L <= V <= n.
Accept only n-L <= floor(p*n/100), for p in 1, 2, 5, 10. Therefore
(V-L)/V <= (n-L)/n <= p/100 for nonempty inputs. Empty inputs return the
empty witness with zero loss. Failed certificates invoke production indices.
The upper bound returned is n on both paths. This conservative interval may
reject witnesses that already meet the budget; rejection does not imply error.

### 2026-10-30 · tests

Heading `### Evidence, 2026-10-30` stays in tests as `### Evidence`.

### 2026-11-04 · document

Migration: consolidated into model.md, 2026-11-04, operator decision; fresh hash-bound references to the former files were rebound to the sections below.
