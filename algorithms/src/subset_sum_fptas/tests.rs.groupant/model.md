# algorithm-subset-sum-fptas-tests

## descriptant

### subset_sum_fptas: tests.rs

Deterministic executable assertions for subset_sum_fptas; finite fixtures observe production outputs against independent masks or exact arithmetic. Passing is a finite evidence claim, not universal correctness.

## model

### Mathematical model

Deterministic executable assertions for subset_sum_fptas; finite fixtures observe production outputs against independent masks or exact arithmetic. Passing is a finite evidence claim, not universal correctness.

#### Invariant

Every retained sum is a genuine subset sum. A deleted value y has retained representative z with z<=y<=(1+delta)z, delta=percent/(100n). After k steps each exact feasible sum has representative >=sum/(1+delta)^k. Since (1+epsilon/n)^(-n)>=1-epsilon, final sum is within the supported objective loss; zero optimum stays exact.

#### Cost and resources

O(n L log L) time with sort_unstable, L=O(n log(max(2,target))/epsilon); O(L) live storage. Charged B=input length+sum 2*old-list-length bounds generated candidates and total sort cost O(B log B), not sort comparisons exactly. Fallible reservations, subtraction-based feasible addition, checked budget accumulation.

#### Observation limits

Integer certificates are scoped to successful calls and stated premises, with categorical validation, budget, allocation and overflow errors. usize targets up to64 bits are modeled. No floating approximation, timing benefit or universal error theorem inferred from tests. No retained selected subset witness.

#### Literature

CLRS, Introduction to Algorithms, approximation algorithms chapter, APPROX-SUBSET-SUM and trimming lemma.

#### Executable companion gap

The production kernel already realizes the scoped approximation. No separate cheaper Rust surrogate has been measured; no fake exact wrapper or speed claim is attached.

## approximant

### Mathematical model

Deterministic executable assertions for subset_sum_fptas; finite fixtures observe production outputs against independent masks or exact arithmetic. Passing is a finite evidence claim, not universal correctness.

#### Invariant

Every retained sum is a genuine subset sum. A deleted value y has retained representative z with z<=y<=(1+delta)z, delta=percent/(100n). After k steps each exact feasible sum has representative >=sum/(1+delta)^k. Since (1+epsilon/n)^(-n)>=1-epsilon, final sum is within the supported objective loss; zero optimum stays exact.

#### Cost and resources

O(n L log L) time with sort_unstable, L=O(n log(max(2,target))/epsilon); O(L) live storage. Charged B=input length+sum 2*old-list-length bounds generated candidates and total sort cost O(B log B), not sort comparisons exactly. Fallible reservations, subtraction-based feasible addition, checked budget accumulation.

#### Observation limits

Integer certificates are scoped to successful calls and stated premises, with categorical validation, budget, allocation and overflow errors. usize targets up to64 bits are modeled. No floating approximation, timing benefit or universal error theorem inferred from tests. No retained selected subset witness.

#### Literature

CLRS, Introduction to Algorithms, approximation algorithms chapter, APPROX-SUBSET-SUM and trimming lemma.

#### Executable companion gap

The production kernel already realizes the scoped approximation. No separate cheaper Rust surrogate has been measured; no fake exact wrapper or speed claim is attached.

## models

# Typed obernants: subset_sum_fptas tests.rs

## behavior_complement

Observation is a vector of named assertion outcomes. The pass fiber contains implementations agreeing on every exercised fixture; its complement contains at least one failing assertion, panic or error mismatch. A surviving mutant outside the tested domain can remain in the pass fiber. This relation explicitly leaves untested behavior unknown.

## reverse_reconstruction

All implementations producing the same finite assertion vector belong to one nonunique fiber. Original production and a mutant differing only on untested data may both pass; the compiled mutant (Force returned sum to zero; lower trim representative assertion fails.) is separated by trim_is_nonexact. A pass vector cannot reconstruct source, runtime or behavior on unseen inputs.

## independent_oracle

Fixtures target the mathematical observation directly: Unsigned values,target and tiers1/2/5/10. Starting with{0}, form sorted feasible union L union(L+x), then retain zero and the next y only when100*n*y>(100*n+percent)*last. All arithmetic is checked integer arithmetic. Return greatest retained feasible sum. Small mask enumeration is structurally independent of scaled/trimmed DP; exact integer formulas are independent of interpolation lookup and quantization branches. These finite witnesses falsify implementation claims without promoting sample agreement to proof.

## boundary_refusal_fiber

The validation/budget fixtures distinguish supported success from invalid tiers, dimensions, resource or overflow failures where relevant. Inputs sharing Error variants lose original details. Boundary assertions are executable witnesses of the broader production relation: candidate_budget_fiber|B counts inputs and twice each previous list length even when some sums exceed target. Same sums with different histories may have different B. Budget exactlyB succeeds; B-1 rejects categorically. Zero and over-target items skip generation.

## compiled_fault_separation

Force returned sum to zero; lower trim representative assertion fails. The intentional production edit must compile and fail trim_is_nonexact; restored byte hash and positive rerun close the experiment. It only demonstrates sensitivity to this fault, not mutation completeness.

## tests

### Executable evidence

Run cargo test -p mova-algorithms subset_sum_fptas::tests (also --release). Module has 3 tests. Twice debug and twice release full library502-test runs have equal normalized test names/results. Strict all-target Clippy passed.

Compiled mutant: Force returned sum to zero; lower trim representative assertion fails. Named assertion test: trim_is_nonexact. Production bytes restored and focused positive suite passed.

## prospective

Before implementation: knapsack uses exact rational profit scaling K=epsilon*vmax/n, minimum-weight DP, loss<=nK<=epsilon OPT. Subset sum uses rational trimmed sorted lists delta=epsilon/n and retains feasible lower representatives; (1+delta)^-n>=1-epsilon. Tiers exactly 1/2/5/10%, zeros exact. Checked work/state budgets and fallible allocation, no speed claims. Piecewise interpolation uses integer knots/query and exact rational values, conditional on caller's C² function, exact samples and |f''|<=M; absolute bound M*h²/8 only. Quantized dot uses integer truncation and exact absolute residual bound, normalized by sum|a_i b_i| with exact zero convention, never cancellation-sensitive scalar relative metric. All arithmetic checked. Tests compare exhaustive or independent arithmetic oracles, exercise invalid/overflow/budget boundaries and compiled production mutants.

## History

### 2026-10-30 · descriptant

#### Epoch2026-10-30

Prospective model was saved before implementation. Independent finite fixtures and exact integer arithmetic retain explicit domain boundaries. Initial cancellation test incorrectly expected10% rejection for a1.97% residual; corrected to1%. This was a fixture error, not a reason to weaken the certificate. Four full library runs passed502tests; focused module contains3tests. Compiled production mutation: Force returned sum to zero; lower trim representative assertion fails. No live workload, end-to-end gain or universal numerical guarantee measured.

### 2026-10-30 · tests

External logs: $MOVA_DATA/runs/octopus-wave19-approx-2026-10-30. receipt.json binds file hashes. Syntax projections are generated independently twice and compared; partial syntax only, not semantic AST. Allocation refusal is not deterministically injected.

### Undated · document

Source SHA256: 3e99e6d2161012cd21303938a92633a718fce29d0373accebaee56804398f1ea. Partial source-derived relations; finite checks are not proofs.
