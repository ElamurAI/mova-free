# algorithm-stable-marriage-tests

## descriptant

### Description and epoch memory

Two equal-sized populations with complete strict n-entry permutation rows; total 2*n*n entries at most 2000000. Empty populations allowed.

Independent finite oracles inspect both primal/dual cover cardinalities and every stable permutation at small sizes. Original source and fixtures; citation describes the classical method. Models and partial syntax do not prove semantics; macros remain opaque and semantic AST is absent.

## approximant

### Forward and cost

Execute original bounded fixtures against independent oracles. Validate equal sizes, checked quadratic budget, then proposer and receiver rows. Build receiver inverse ranks. FIFO free proposers propose to the next untried receiver; a receiver retains its best proposal so far and requeues the rejected proposer. Return receiver index per proposer when the queue empties. Test observations are named pass/fail assertions, not a semantic proof.

Enumerates sum over n<=3 of (n!)^(2n)=46674 profiles, each with n! assignment candidates and O(n cubed) blocking checks using linear rank lookup. The separate n=1000 accepted boundary uses O(n squared) entries. Fixed bounded exhaustive domain; no scalable testing claim.

Attribution: D. Gale and L. S. Shapley, College Admissions and the Stability of Marriage, American Mathematical Monthly 69(1) (1962), 9–15. Implementation and fixtures are original. No optimization or speedup is claimed.

## invariant

### Invariants

Each receiver holds at most one proposer; each held proposer is absent from the free queue. A proposer never repeats a proposal. Receiver held ranks strictly improve on replacement. Completeness prevents preference exhaustion by a free proposer. Final mates form a permutation without blocking pairs and are proposer-optimal among stable permutations.

Test enumeration uses independent subset/assignment predicates rather than repeating production traversal. Bounds do not prove unbounded behavior.

## catalog

# Typed models

Each relation is source-derived, exact as a specification; finite executable evidence is partial. Assumes deterministic std-only execution with successful allocation. No executable inverse solver or approximation error cap is claimed.

## behavior-complement

Test observation universe is fixture cases crossed with pass/fail and candidate answers. A pass is excluded if an independently enumerated cover minimum or stable-permutation constraint disagrees, if an expected boundary error differs, or if a repeated witness changes. Universe: two preference matrices crossed with candidate receiver vectors and errors. Absent successful behaviors include a non-permutation, a blocking pair (p,r) where both prefer each other to assigned mates, or a stable permutation giving a proposer a worse partner than another stable permutation. Invalid row lengths/permutations cannot succeed. Unequal sizes beat the budget error, which beats malformed rows; within each row, length beats duplicate/out-of-range entry. The finite exhaustive oracle supports these exact source-derived constraints without a general Rust proof. Untested larger cases are outside the observed test transition universe.

## reverse-reconstruction

A passing named test has many compatible production programs: arbitrary changes outside the exercised inputs, unused functions or histories can preserve all assertions. A failing optimality assertion retains every candidate answer violating the corresponding enumerated inequality, not a unique bug. For an observed permutation m, retain every pair of complete preference matrices for which m is stable and proposer-optimal: for each stable alternative s and proposer p, rank_p(m[p]) <= rank_p(s[p]). This finite relational characterization is ambiguous: with two proposers each ranking a distinct receiver first, either strict ordering in each receiver row still yields m. Observing a matching loses unproposed suffixes and proposal/rejection history. Error preimages comprise all profiles sharing the first failed validation predicate and observed indices.

## proposal-budget

The sum of next-proposal cursors increases exactly once per loop and is bounded by n squared. Receiver ranks only improve; a rejected proposer can never obtain that receiver in a proposer-optimal stable solution. Complete strict preferences are required.

## optimality-fiber

Fix preferences and enumerate all n! assignments; discard blocking assignments. For each proposer, the returned partner has minimum ordinal rank among the remaining assignments. Tests enumerate every profile up to n=3; this is finite evidence only.

## validation-partition

Equal size is inspected before checked total entries, then proposer rows before receiver rows. Row length is inspected before entry validity, duplicates share the invalid-preference variant. Later malformed rows remain unconstrained in an early-error preimage.

## tests

### Bounded checks

All 46674 strict preference profiles for n=0 through 3: every assignment checked for blocking pairs and every stable alternative compared proposer by proposer. Replacement witness; row/duplicate/range/error precedence; n=1000 boundary and n=1001 rejection. Three named tests.

Debug and release run twice with normalized named outcomes. Compiled production mutants flip the cover left complement and receiver replacement comparison respectively; named oracle assertions reject them. Restored positives and strict algorithms clippy are required. External receipts bind final source/evidence hashes. No timing or full-system claim.

See [executed receipts](model.md#evidence) for final source hashes, named control failures and repeated test log hashes. All checks stated above completed.

## evidence

### Executed evidence

Compiled receiver rank comparison reversal was rejected by `receiver_replacement_witness`: [0,1] instead of [1,0].

Raw logs remain in the external wave14 graphs run bundle. Earlier tooling attempts exposed two Clippy style findings, absent rg, and stale syntax outputs; final receipts above bind corrected sources and successful checks. No source correctness repair was required by positive oracle findings. No general proof, speedup, full-system or allocation-failure claim.

## History

### 2026-10-30 · descriptant

2026-10-30 initial epoch: chose existing matching plus explicit alternating closure for cover, and FIFO deferred acceptance plus inverse receiver ranks for marriage. Prospective cost: Enumerates sum over n<=3 of (n!)^(2n)=46674 profiles, each with n! assignment candidates and O(n cubed) blocking checks using linear rank lookup. The separate n=1000 accepted boundary uses O(n squared) entries. Fixed bounded exhaustive domain; no scalable testing claim. No approximation surrogate is useful for the exact witness contract.

### 2026-10-30 · evidence

Heading `### Executed evidence, 2026-10-30` stays in evidence as `### Executed evidence`.

Source SHA-256: `7367e4c8c4f0cfc5f4182fe3b3dabbe8c9d9865beba9943ee3999f672791dc2c`. Three tests passed twice in debug and twice in release with identical normalized named results across all four runs. Both restored debug suites passed. Strict algorithms all-targets Clippy and narrow rustfmt checks passed. Four syntax projections repeated byte-identically; structural scaffold passed twice with identical reports (one intentionally missing external scaffold sidecar).

- `stable_marriage-debug-1.log`: `1cc0bb3ad3f61e469e7d0cad8e599f68e00fa38e58b23443bcbc968bb74000f2`

- `stable_marriage-debug-2.log`: `c58528570e494e55479656404370958481b56ad9a169f8b0c3bbc1d024e635d8`

- `stable_marriage-release-1.log`: `f21c6a741c558489f5f82244bb5aa1175e88a2e4215a0475b83f29d7ef8e58ee`

- `stable_marriage-release-2.log`: `e276961b9bcd1acba127bf4fd564a3eab98d805207f4cb1a79f4c03f11d8d13b`

- `stable_marriage-restored.log`: `35e1ef64eae39d69d913ad0b46522792295273bdd0abd535163d8cd62a88a477`

- `marriage-mutant.log`: `c2d96973907a7c62f74fd385378c46db21fc0df733de17cfcc5cf38885aa6ee6`

### Undated · document

Source snapshot: `7367e4c8c4f0cfc5f4182fe3b3dabbe8c9d9865beba9943ee3999f672791dc2c`. Domain: Two equal-sized populations with complete strict n-entry permutation rows; total 2*n*n entries at most 2000000. Empty populations allowed.
