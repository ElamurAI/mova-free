# algorithm-subset-sum-fptas

## descriptant

### subset_sum_fptas: mod.rs

Unsigned values,target and tiers1/2/5/10. Starting with{0}, form sorted feasible union L union(L+x), then retain zero and the next y only when100*n*y>(100*n+percent)*last. All arithmetic is checked integer arithmetic. Return greatest retained feasible sum.

## model

### Mathematical model

Unsigned values,target and tiers1/2/5/10. Starting with{0}, form sorted feasible union L union(L+x), then retain zero and the next y only when100*n*y>(100*n+percent)*last. All arithmetic is checked integer arithmetic. Return greatest retained feasible sum.

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

Unsigned values,target and tiers1/2/5/10. Starting with{0}, form sorted feasible union L union(L+x), then retain zero and the next y only when100*n*y>(100*n+percent)*last. All arithmetic is checked integer arithmetic. Return greatest retained feasible sum.

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

# Typed obernants: subset_sum_fptas mod.rs

## behavior_complement

For validated representable inputs the forward relation is constrained by: Every retained sum is a genuine subset sum. A deleted value y has retained representative z with z<=y<=(1+delta)z, delta=percent/(100n). After k steps each exact feasible sum has representative >=sum/(1+delta)^k. Since (1+epsilon/n)^(-n)>=1-epsilon, final sum is within the supported objective loss; zero optimum stays exact. Its complement includes output pairs violating these bounds, unsupported acceptance, out-of-domain values and untyped overflow. Categorical errors are outside numerical approximation, with validation precedence part of the relation.

## reverse_reconstruction

Returned sum0 admits no items, all zeros or every item above target. Sum100 can arise from[100],[40,60] or trimmed[100,101] at target101 and10%. Same reported sum does not imply equal optima or identical subsets. Metadata work further partitions but never generally identifies input.

## trim_interval_fiber

Each retained z represents deleted y in[z,z*(1+delta)]. For[100,101],target101,10%,n2 the 101 candidate lies in the100 cell, so output100. Strict > keeps the lower endpoint representative and excludes unbounded reuse.

## feasible_subset_fiber

Generated candidates add each current item once to the previous list. Every retained value belongs to an independently enumerated feasible subset set; the oracle checks membership, not only an upper bound.

## relative_loss_envelope

Returned V satisfies(100-percent)*OPT<=100V<=100OPT. This relation is about OPT, not target; empty optimum is exact zero. Finite seeded masks cover60 eight-item instances and all four profiles.

## candidate_budget_fiber

B counts inputs and twice each previous list length even when some sums exceed target. Same sums with different histories may have different B. Budget exactlyB succeeds; B-1 rejects categorically. Zero and over-target items skip generation.

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

Source SHA256: 2fd464a483122ac2c9550a54e3ab525b9186c577279fecea977afba3ffec742f. Partial source-derived relations; finite checks are not proofs.
