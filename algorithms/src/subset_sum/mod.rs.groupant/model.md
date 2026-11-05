# algorithm-subset-sum

## descriptant

### subset_sum mod.rs groupant

Raw Rust is authoritative outside this sidecar. Original implementation and independent tests; mathematical citations live in the production source and forward model. Source syntax projection is partial: no macro expansion, semantic resolution or complete dependency graph.

Composition: the forward recurrence/sweep, invariants, behavior complement and distinct reverse preimages constrain the same call. The test groupant observes production through independent specification and metamorphic projections; passing leaves alone do not prove parent wiring. No useful approximate semantics are selected; no executable surrogate or substitution claim.

## approximant

### Forward, invariant and cost

Let S(V,t)={I subset of [0,n): sum(V[i],i in I)=t}. For t<=1,000,000 and successful reservations, return None iff S is empty, otherwise the deterministic first-discovered increasing-index witness. The empty witness wins at t=0. State parent[s] is unset or the first item's index establishing positive sum s. For each nonzero value v<=t, scan s descending; an unset s receives i iff s=v or s-v was reachable. Never overwrite an established parent. Thus predecessors have smaller indices and sum decreases strictly, preventing reuse and guaranteeing termination. Reverse the decreasing chain to produce increasing indices. Initialization costs O(t+1); transitions O(n(t+1)); reconstruction at most min(n,t) steps. O(t+1) parent slots and O(min(n,t)) reserved witness slots; actual witness length can be smaller. Assumes Rust Vec reservation contract. Allocation refusal is explicit but not injected. No executable approximate surrogate: exact feasibility and index identity have no declared useful error metric.

## models

# Typed relations for subset_sum mod.rs

Subject/revision: adjacent source SHA-256 in manifest.json. Scope: one pure function call (tests: finite harness run). Status: source-derived partial relational specification, with finite Rust evidence. Mathematical relations assume successful allocation unless modeling errors. No semantic proof or automatically executable inverse is claimed. Each section defines its operator and domain; checks and remaining gaps appear in model.md#tests.

## behavior_complement

On the universe of V,t,result triples, successful storage and t<=MAX_TARGET exclude Some(I) with repeated, unordered or invalid indices, nonzero sum mismatch, selected zero items or a non-first-discovery witness; exclude None whenever S(V,t) is nonempty. t>MAX_TARGET excludes all Ok outcomes. This is a source-derived inner exclusion model, not proof by unobserved tests.

## reverse_reconstruction

For fixed t and observed Some(I), compatible antecedents are all V whose selected positions sum to t, indices are strictly increasing and first-discovery DP yields exactly I. Summation alone is a necessary outer approximation of this fiber; adding the stated tie rule makes it exact conditional on storage. None has precisely the infeasible V fiber. Many V map to the same I; t=0 admits every V with empty I.

## permutation

Permuting V bijects subsets and preserves existence; returned indices and tie witness may change. With allocation success and fixed accepted t, feasibility is invariant. This intentionally projects away witness identity.

## scale_equivariance

For positive q with q*t<=MAX_TARGET and all q*V[i] representable, scaling values and target leaves each reachable-state correspondence and first discovery unchanged, hence the index witness is identical. Allocation errors are outside this relation.

## zero_extension

Appending any number of zero values preserves output witness and feasibility under successful allocation. Inserting zeros shifts indices at or after insertion while keeping selected nonzero values. At target zero no item is selected.

## prefix_stability

Once a prefix reaches positive t, any suffix leaves its exact witness unchanged: the loop stops at the first reaching prefix. This holds even if a later singleton would shorten the witness, exposing the distinction from minimum cardinality.

## single_use

For singleton [v] and positive accepted t, a witness exists exactly when v=t; 2*v=t is insufficient for v>0. Descending iteration is the dependency-order premise. Duplicate values at distinct indices may each be selected.

## resource_partition

t>MAX_TARGET returns TargetTooLarge before any allocation including empty V. t=0 returns the empty witness without reservations. Other calls attempt t+1 Option<usize> slots and, only on reachability, min(n,t) usize slots; refusal returns AllocationFailed. Cap is independent of mathematical feasibility and platform allocation success.

## tests

Five tests in algorithms/src/subset_sum/tests.rs exercise independent finite oracles, boundaries and metamorphic relations. External evidence directory: (see History). See model.md#validation for measured outcomes. Allocation refusal is not injected; successful reservations are a premise. No teacher, API, GPU or whole-system accuracy claim.

### Checks and evidence

## validation

### Validation

All 266 algorithm-library tests passed twice each in debug and release after the final edit; this includes five new tests per algorithm, 15 total. Sorted test-name/outcome records were identical across all four runs. Strict algorithm-library Clippy with all targets and -D warnings passed. Focused rustfmt check passed. The six new source syntax projections were generated twice and compared byte-for-byte. Shared library exports used for execution were restored before commit. These are finite CPU correctness observations, not speed measurements or a full-system proof.

Independent oracle counts: subset_sum checks 114,681 vector/target cases against subset masks and validates every returned witness; binomial_coefficient compares 45,451 coefficients against checked additive Pascal rows through n=300; interval_union checks 54,241 interval lists against unit-cell coverage. Boundaries separately exercise target cap, u64::MAX coefficients, cancellation at 131 choose 65, invalid k, first reversed interval and the full signed endpoint span.

Three production-source mutants compiled, then failed real assertions: reachable subsets rejected, binomial accumulator initialized to 2, touching intervals left separate. The interval mutant initially survived the focused nested/overlapping fixture because overlaps bridged the touching boundary; a direct [0,1),[1,2) case was added, and the same mutant now fails. The initial survival log is retained externally. The exhaustive interval oracle already covered touching-only cases; the correction makes the focused control diagnostic.

## History

### 2026-10-30 · descriptant

Epoch 2026-10-30, prospective prediction: cancellation prevents representable binomial intermediate overflow; descending subset updates forbid index reuse; signed widening and touching merge preserve interval coverage. Expected complexity is stated in model.md#approximant before final validation. Finite checks can refute these expectations; they do not establish a universal theorem. Preserve these premises and review model.md#validation when updating the source.

### 2026-10-30 · tests

Five tests in algorithms/src/subset_sum/tests.rs exercise independent finite oracles, boundaries and metamorphic relations. External evidence directory: octopus-wave7-discrete-2026-10-30. See model.md#validation for measured outcomes. Allocation refusal is not injected; successful reservations are a premise. No teacher, API, GPU or whole-system accuracy claim.

### 2026-10-30 · validation

Heading `### Validation epoch 2026-10-30` stays in validation as `### Validation`.

Source SHA-256: `f7a1a2cce9891699aa89b3b2bbc9f739d5d08c7148ae01f57fa8d65931219d81`. Evidence directory basename: `octopus-wave7-discrete-2026-10-30` (external run storage).

Normalized debug record SHA-256: `3fbabf34d336c176185133e60994376de2245a94ce5c0642bd7eefd44fa8fd1c`. Mutation failure log SHA-256 for subset_sum: `f5a626c84e9d865cff09d5316292f344c3b0acbb12cfedd49a23dba4a1a06058`. Syntax SHA-256: `5ac90ef9d2e52e19fe99db0b7563a39edb5b087df41ff0840378db76b26455eb`. Allocation refusal and machine exhaustion were not injected. Models remain partial, with no automatic inverse or approximate production substitution.
