# algorithm-knapsack-fptas-tests

## descriptant

### knapsack_fptas: tests.rs

Deterministic executable assertions for knapsack_fptas; finite fixtures observe production outputs against independent masks or exact arithmetic. Passing is a finite evidence claim, not universal correctness.

## model

### Mathematical model

Deterministic executable assertions for knapsack_fptas; finite fixtures observe production outputs against independent masks or exact arithmetic. Passing is a finite evidence claim, not universal correctness.

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

## approximant

### Mathematical model

Deterministic executable assertions for knapsack_fptas; finite fixtures observe production outputs against independent masks or exact arithmetic. Passing is a finite evidence claim, not universal correctness.

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

# Typed obernants: knapsack_fptas tests.rs

## behavior_complement

Observation is a vector of named assertion outcomes. The pass fiber contains implementations agreeing on every exercised fixture; its complement contains at least one failing assertion, panic or error mismatch. A surviving mutant outside the tested domain can remain in the pass fiber. This relation explicitly leaves untested behavior unknown.

## reverse_reconstruction

All implementations producing the same finite assertion vector belong to one nonunique fiber. Original production and a mutant differing only on untested data may both pass; the compiled mutant (Force returned feasible objective to zero; named single-use assertion fails.) is separated by zero_weight_and_single_use. A pass vector cannot reconstruct source, runtime or behavior on unseen inputs.

## independent_oracle

Fixtures target the mathematical observation directly: Nonnegative integer item profits and usize weights; capacity, one supported percent and caller work budget. Scale q_i=floor(100*n*v_i/(percent*vmax)) exactly. DP stores minimum feasible weight per scaled profit, ties greater original value. Return maximum retained original objective V and D=percent*vmax, so OPT-V<=D/100<=percent*OPT/100. Small mask enumeration is structurally independent of scaled/trimmed DP; exact integer formulas are independent of interpolation lookup and quantization branches. These finite witnesses falsify implementation claims without promoting sample agreement to proof.

## boundary_refusal_fiber

The validation/budget fixtures distinguish supported success from invalid tiers, dimensions, resource or overflow failures where relevant. Inputs sharing Error variants lose original details. Boundary assertions are executable witnesses of the broader production relation: resource_boundary|Unsupported tiers reject before scanning. Input length>budget rejects before extrema. Positive case checks n*width+len before fallible allocation. Equality is accepted and one-less budget rejected; allocator and arithmetic errors are categorical, not percent error.

## compiled_fault_separation

Force returned feasible objective to zero; named single-use assertion fails. The intentional production edit must compile and fail zero_weight_and_single_use; restored byte hash and positive rerun close the experiment. It only demonstrates sensitivity to this fault, not mutation completeness.

## tests

### Executable evidence

Run cargo test -p mova-algorithms knapsack_fptas::tests (also --release). Module has 4 tests. Twice debug and twice release full library502-test runs have equal normalized test names/results. Strict all-target Clippy passed.

Compiled mutant: Force returned feasible objective to zero; named single-use assertion fails. Named assertion test: zero_weight_and_single_use. Production bytes restored and focused positive suite passed.

## prospective

Before implementation: knapsack uses exact rational profit scaling K=epsilon*vmax/n, minimum-weight DP, loss<=nK<=epsilon OPT. Subset sum uses rational trimmed sorted lists delta=epsilon/n and retains feasible lower representatives; (1+delta)^-n>=1-epsilon. Tiers exactly 1/2/5/10%, zeros exact. Checked work/state budgets and fallible allocation, no speed claims. Piecewise interpolation uses integer knots/query and exact rational values, conditional on caller's C² function, exact samples and |f''|<=M; absolute bound M*h²/8 only. Quantized dot uses integer truncation and exact absolute residual bound, normalized by sum|a_i b_i| with exact zero convention, never cancellation-sensitive scalar relative metric. All arithmetic checked. Tests compare exhaustive or independent arithmetic oracles, exercise invalid/overflow/budget boundaries and compiled production mutants.

## History

### 2026-10-30 · descriptant

#### Epoch2026-10-30

Prospective model was saved before implementation. Independent finite fixtures and exact integer arithmetic retain explicit domain boundaries. Initial cancellation test incorrectly expected10% rejection for a1.97% residual; corrected to1%. This was a fixture error, not a reason to weaken the certificate. Four full library runs passed502tests; focused module contains4tests. Compiled production mutation: Force returned feasible objective to zero; named single-use assertion fails. No live workload, end-to-end gain or universal numerical guarantee measured.

### 2026-10-30 · tests

External logs: $MOVA_DATA/runs/octopus-wave19-approx-2026-10-30. receipt.json binds file hashes. Syntax projections are generated independently twice and compared; partial syntax only, not semantic AST. Allocation refusal is not deterministically injected.

### Undated · document

Source SHA256: 9bbc0147cc986419764726104fb8a8584691c6dbf46916cb88fdb6a70881a7eb. Partial source-derived relations; finite checks are not proofs.
