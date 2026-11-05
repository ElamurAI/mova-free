# algorithm-quantized-dot-tests

## descriptant

### quantized_dot: tests.rs

Deterministic executable assertions for quantized_dot; finite fixtures observe production outputs against independent masks or exact arithmetic. Passing is a finite evidence claim, not universal correctness.

## model

### Mathematical model

Deterministic executable assertions for quantized_dot; finite fixtures observe production outputs against independent masks or exact arithmetic. Passing is a finite evidence claim, not universal correctness.

#### Invariant

Exact identity ab-qa*qb=qa*rb+qb*ra+ra*rb gives |exact-approx|<=E by triangle inequality. Quantization preserves sign or becomes zero. Normalized error is E/L1 when L1>0; L1=0 forces E=0. Cancellation of dot terms does not invalidate an L1 certificate.

#### Cost and resources

O(n) time, O(1) storage, no allocation. Checked i128 accumulation and u128 certificate sums. Quotient/remainder certification avoids overflow from multiplying L1 by100. Work budget bounds pairs; unsupported and uncertified domains fail with no hidden fallback.

#### Observation limits

Integer certificates are scoped to successful calls and stated premises, with categorical validation, budget, allocation and overflow errors. usize targets up to64 bits are modeled. No floating approximation, timing benefit or universal error theorem inferred from tests. No retained selected subset witness.

#### Literature

Higham, Accuracy and Stability of Numerical Algorithms, perturbation/error analysis of dot products; here the residual identity is exact integer arithmetic.

#### Executable companion gap

The production kernel already realizes the scoped approximation. No separate cheaper Rust surrogate has been measured; no fake exact wrapper or speed claim is attached.

## approximant

### Mathematical model

Deterministic executable assertions for quantized_dot; finite fixtures observe production outputs against independent masks or exact arithmetic. Passing is a finite evidence claim, not universal correctness.

#### Invariant

Exact identity ab-qa*qb=qa*rb+qb*ra+ra*rb gives |exact-approx|<=E by triangle inequality. Quantization preserves sign or becomes zero. Normalized error is E/L1 when L1>0; L1=0 forces E=0. Cancellation of dot terms does not invalidate an L1 certificate.

#### Cost and resources

O(n) time, O(1) storage, no allocation. Checked i128 accumulation and u128 certificate sums. Quotient/remainder certification avoids overflow from multiplying L1 by100. Work budget bounds pairs; unsupported and uncertified domains fail with no hidden fallback.

#### Observation limits

Integer certificates are scoped to successful calls and stated premises, with categorical validation, budget, allocation and overflow errors. usize targets up to64 bits are modeled. No floating approximation, timing benefit or universal error theorem inferred from tests. No retained selected subset witness.

#### Literature

Higham, Accuracy and Stability of Numerical Algorithms, perturbation/error analysis of dot products; here the residual identity is exact integer arithmetic.

#### Executable companion gap

The production kernel already realizes the scoped approximation. No separate cheaper Rust surrogate has been measured; no fake exact wrapper or speed claim is attached.

## models

# Typed obernants: quantized_dot tests.rs

## behavior_complement

Observation is a vector of named assertion outcomes. The pass fiber contains implementations agreeing on every exercised fixture; its complement contains at least one failing assertion, panic or error mismatch. A surviving mutant outside the tested domain can remain in the pass fiber. This relation explicitly leaves untested behavior unknown.

## reverse_reconstruction

All implementations producing the same finite assertion vector belong to one nonunique fiber. Original production and a mutant differing only on untested data may both pass; the compiled mutant (Disable certificate rejection; unsupported5% result violates named boundary assertion.) is separated by tier_boundary_and_validation. A pass vector cannot reconstruct source, runtime or behavior on unseen inputs.

## independent_oracle

Fixtures target the mathematical observation directly: Equal-length i64 arrays, positive u64 step and supported percent. Let qa=trunc(a/step)*step and ra=a-qa. Accumulate approximate sum qa*qb and E=sum(|qa|*|rb|+|qb|*|ra|+|ra*rb|). L1=sum|a*b|. Return only if E<=floor(percent*L1/100). Small mask enumeration is structurally independent of scaled/trimmed DP; exact integer formulas are independent of interpolation lookup and quantization branches. These finite witnesses falsify implementation claims without promoting sample agreement to proof.

## boundary_refusal_fiber

The validation/budget fixtures distinguish supported success from invalid tiers, dimensions, resource or overflow failures where relevant. Inputs sharing Error variants lose original details. Boundary assertions are executable witnesses of the broader production relation: checked_accumulation_fiber|Three MIN*MIN products overflow signed i128 and return Overflow. One MIN*1 is represented exactly. Budget and length checks precede arithmetic. Arithmetic failure cannot be relabeled as a small numerical error.

## compiled_fault_separation

Disable certificate rejection; unsupported5% result violates named boundary assertion. The intentional production edit must compile and fail tier_boundary_and_validation; restored byte hash and positive rerun close the experiment. It only demonstrates sensitivity to this fault, not mutation completeness.

## tests

### Executable evidence

Run cargo test -p mova-algorithms quantized_dot::tests (also --release). Module has 3 tests. Twice debug and twice release full library502-test runs have equal normalized test names/results. Strict all-target Clippy passed.

Compiled mutant: Disable certificate rejection; unsupported5% result violates named boundary assertion. Named assertion test: tier_boundary_and_validation. Production bytes restored and focused positive suite passed.

## prospective

Before implementation: knapsack uses exact rational profit scaling K=epsilon*vmax/n, minimum-weight DP, loss<=nK<=epsilon OPT. Subset sum uses rational trimmed sorted lists delta=epsilon/n and retains feasible lower representatives; (1+delta)^-n>=1-epsilon. Tiers exactly 1/2/5/10%, zeros exact. Checked work/state budgets and fallible allocation, no speed claims. Piecewise interpolation uses integer knots/query and exact rational values, conditional on caller's C² function, exact samples and |f''|<=M; absolute bound M*h²/8 only. Quantized dot uses integer truncation and exact absolute residual bound, normalized by sum|a_i b_i| with exact zero convention, never cancellation-sensitive scalar relative metric. All arithmetic checked. Tests compare exhaustive or independent arithmetic oracles, exercise invalid/overflow/budget boundaries and compiled production mutants.

## History

### 2026-10-30 · descriptant

#### Epoch2026-10-30

Prospective model was saved before implementation. Independent finite fixtures and exact integer arithmetic retain explicit domain boundaries. Initial cancellation test incorrectly expected10% rejection for a1.97% residual; corrected to1%. This was a fixture error, not a reason to weaken the certificate. Four full library runs passed502tests; focused module contains3tests. Compiled production mutation: Disable certificate rejection; unsupported5% result violates named boundary assertion. No live workload, end-to-end gain or universal numerical guarantee measured.

### 2026-10-30 · tests

External logs: $MOVA_DATA/runs/octopus-wave19-approx-2026-10-30. receipt.json binds file hashes. Syntax projections are generated independently twice and compared; partial syntax only, not semantic AST. Allocation refusal is not deterministically injected.

### Undated · document

Source SHA256: 1c8d36000e8b7236183612e5eb877fabb6f5c0063bdf07f6e16ab9b3864a7e81. Partial source-derived relations; finite checks are not proofs.
