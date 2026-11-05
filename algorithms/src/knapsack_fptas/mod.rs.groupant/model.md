# algorithm-knapsack-fptas

## descriptant

### knapsack_fptas: mod.rs

Nonnegative integer item profits and usize weights; capacity, one supported percent and caller work budget. Scale q_i=floor(100*n*v_i/(percent*vmax)) exactly. DP stores minimum feasible weight per scaled profit, ties greater original value. Return maximum retained original objective V and D=percent*vmax, so OPT-V<=D/100<=percent*OPT/100.

## model

### Mathematical model

Nonnegative integer item profits and usize weights; capacity, one supported percent and caller work budget. Scale q_i=floor(100*n*v_i/(percent*vmax)) exactly. DP stores minimum feasible weight per scaled profit, ties greater original value. Return maximum retained original objective V and D=percent*vmax, so OPT-V<=D/100<=percent*OPT/100.

#### Invariant

At each item layer the row represents feasible subsets, each item used at most once by descending scaled-profit traversal. All minima are capacity-feasible; q=0 transitions read the old cell once. Equal-weight ties retain larger original profit. Scaled optimal subset loses less than nK where K=percent*vmax/(100n); final scaled optimum is feasible and bounds original OPT-V. No reconstruction witness is retained.

#### Cost and resources

O(n^3/epsilon) time and O(n^2/epsilon) cells; work=n*width+input length, an upper bound on DP cell visits plus input count, not exact CPU instructions. Multiple validation scans are constant factors. Checked usize/u128 dimensions; fallible reservation; no capacity-sized array.

#### Observation limits

Integer certificates are scoped to successful calls and stated premises, with categorical validation, budget, allocation and overflow errors. usize targets up to64 bits are modeled. No floating approximation, timing benefit or universal error theorem inferred from tests. No retained selected subset witness.

#### Literature

Ibarra and Kim, Fast Approximation Algorithms for the Knapsack and Sum of Subset Problems, JACM22(4),1975.

#### Executable companion gap

The production kernel already realizes the scoped approximation. No separate cheaper Rust surrogate has been measured; no fake exact wrapper or speed claim is attached.

#### Wave19 measured cost calibration

Two bounded release executions calibrate this cost model; see model.md#benchmark-wave19. Integer certificates and floating checks retain their original scope. A smaller asymptotic recurrence does not guarantee faster execution on a small checked input.

## approximant

### Mathematical model

Nonnegative integer item profits and usize weights; capacity, one supported percent and caller work budget. Scale q_i=floor(100*n*v_i/(percent*vmax)) exactly. DP stores minimum feasible weight per scaled profit, ties greater original value. Return maximum retained original objective V and D=percent*vmax, so OPT-V<=D/100<=percent*OPT/100.

#### Invariant

At each item layer the row represents feasible subsets, each item used at most once by descending scaled-profit traversal. All minima are capacity-feasible; q=0 transitions read the old cell once. Equal-weight ties retain larger original profit. Scaled optimal subset loses less than nK where K=percent*vmax/(100n); final scaled optimum is feasible and bounds original OPT-V. No reconstruction witness is retained.

#### Cost and resources

O(n^3/epsilon) time and O(n^2/epsilon) cells; work=n*width+input length, an upper bound on DP cell visits plus input count, not exact CPU instructions. Multiple validation scans are constant factors. Checked usize/u128 dimensions; fallible reservation; no capacity-sized array.

#### Observation limits

Integer certificates are scoped to successful calls and stated premises, with categorical validation, budget, allocation and overflow errors. usize targets up to64 bits are modeled. No floating approximation, timing benefit or universal error theorem inferred from tests. No retained selected subset witness.

#### Literature

Ibarra and Kim, Fast Approximation Algorithms for the Knapsack and Sum of Subset Problems, JACM22(4),1975.

#### Executable companion gap

The production kernel already realizes the scoped approximation. No separate cheaper Rust surrogate has been measured; no fake exact wrapper or speed claim is attached.

## models

# Typed obernants: knapsack_fptas mod.rs

## behavior_complement

For validated representable inputs the forward relation is constrained by: At each item layer the row represents feasible subsets, each item used at most once by descending scaled-profit traversal. All minima are capacity-feasible; q=0 transitions read the old cell once. Equal-weight ties retain larger original profit. Scaled optimal subset loses less than nK where K=percent*vmax/(100n); final scaled optimum is feasible and bounds original OPT-V. No reconstruction witness is retained. Its complement includes output pairs violating these bounds, unsupported acceptance, out-of-domain values and untyped overflow. Categorical errors are outside numerical approximation, with validation precedence part of the relation.

## reverse_reconstruction

Output value zero admits empty input and all-infeasible/zero-profit inputs. Positive objective admits permutations, redundant infeasible items and distinct optimal subsets. A reported value does not identify item indices; certificate and work narrow but do not uniquely invert these fibers.

## single_use_fiber

Descending traversal separates 0/1 subsets from unbounded multisets: one weight-1,value-11 item at capacity2 can contribute only11. Zero-weight items are included once; two distinct copies remain two choices. The zero_weight_and_single_use fixture checks this boundary.

## scaling_cell_fiber

For fixed n,vmax,percent, q corresponds to the exact interval q*percent*vmax <=100*n*v<(q+1)*percent*vmax. Multiple profits coalesce. The nonexact_scaling fixture has low profits with q=0 and preserves the loss bound rather than asserting exactness.

## feasibility_envelope

Admissible outputs lie in [max(0,OPT-D/100),OPT]. Any value above OPT, negative value or loss beyond D/100 belongs to the complement. Exhaustive subset enumeration on50 seven-item instances checks both inequalities for every tier.

## resource_boundary

Unsupported tiers reject before scanning. Input length>budget rejects before extrema. Positive case checks n*width+len before fallible allocation. Equality is accepted and one-less budget rejected; allocator and arithmetic errors are categorical, not percent error.

## tests

### Executable evidence

Run cargo test -p mova-algorithms knapsack_fptas::tests (also --release). Module has 4 tests. Twice debug and twice release full library502-test runs have equal normalized test names/results. Strict all-target Clippy passed.

Compiled mutant: Force returned feasible objective to zero; named single-use assertion fails. Named assertion test: zero_weight_and_single_use. Production bytes restored and focused positive suite passed.

## prospective

Before implementation: knapsack uses exact rational profit scaling K=epsilon*vmax/n, minimum-weight DP, loss<=nK<=epsilon OPT. Subset sum uses rational trimmed sorted lists delta=epsilon/n and retains feasible lower representatives; (1+delta)^-n>=1-epsilon. Tiers exactly 1/2/5/10%, zeros exact. Checked work/state budgets and fallible allocation, no speed claims. Piecewise interpolation uses integer knots/query and exact rational values, conditional on caller's C² function, exact samples and |f''|<=M; absolute bound M*h²/8 only. Quantized dot uses integer truncation and exact absolute residual bound, normalized by sum|a_i b_i| with exact zero convention, never cancellation-sensitive scalar relative metric. All arithmetic checked. Tests compare exhaustive or independent arithmetic oracles, exercise invalid/overflow/budget boundaries and compiled production mutants.

## benchmark-wave19

This source was measured in two release executions, with three warmups and seven alternating samples. See [complete results and prospective model](../../../../models/algorithm-library.groupant/model.md#wave19-benchmark). Horner is slower than the unchecked direct-monomial baseline; the FPTAS fixed instance is faster and returns the same objective. These facts apply only to their named fixtures. Source SHA-256 (see History). Evidence receipt SHA-256 (see History).

### Wave19 cost calibration

## History

### 2026-10-30 · descriptant

#### Epoch2026-10-30

Prospective model was saved before implementation. Independent finite fixtures and exact integer arithmetic retain explicit domain boundaries. Initial cancellation test incorrectly expected10% rejection for a1.97% residual; corrected to1%. This was a fixture error, not a reason to weaken the certificate. Four full library runs passed502tests; focused module contains4tests. Compiled production mutation: Force returned feasible objective to zero; named single-use assertion fails. No live workload, end-to-end gain or universal numerical guarantee measured.

### 2026-10-30 · tests

External logs: $MOVA_DATA/runs/octopus-wave19-approx-2026-10-30. receipt.json binds file hashes. Syntax projections are generated independently twice and compared; partial syntax only, not semantic AST. Allocation refusal is not deterministically injected.

### Undated · document

Source SHA256: 03795683bef91f2d5e9ac96b4069b8cdf6bb134044c7121bef8289f3b7200392. Partial source-derived relations; finite checks are not proofs.

### Undated · benchmark-wave19

This source was measured in two release executions, with three warmups and seven alternating samples. See [complete results and prospective model](../../../../models/algorithm-library.groupant/model.md#wave19-benchmark). Horner is slower than the unchecked direct-monomial baseline; the FPTAS fixed instance is faster and returns the same objective. These facts apply only to their named fixtures. Source SHA-256 03795683bef91f2d5e9ac96b4069b8cdf6bb134044c7121bef8289f3b7200392. Evidence receipt SHA-256 1e26c2f14bcfe85fff4a56094ff8b61f13c4b12142e036a7644396a9ed805715.
