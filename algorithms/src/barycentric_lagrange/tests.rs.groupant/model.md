# algorithm-barycentric-lagrange-tests

## descriptant

### Description and epoch memory

barycentric_lagrange tests.rs. Original standard-library-only implementation with bounded work/storage.

Citation: Berrut and Trefethen (2004), SIAM Review 46(3), 501–517.

### Combiner

Raw source selects recurrence and validation; forward equations describe real observation; complement excludes concrete outcomes; reverse relation retains ambiguous antecedents. Typed specialized relations describe polynomial/shape/sample structure. Partial syntax binds source bytes but has no semantic resolution. Sibling tests falsify selected claims. Composition transports absolute perturbations only under stated premises; no automatic error-tier inheritance.

## model

### Invariant and scope

Sorted 1..64 nodes in [-1,1], gaps >=1e-8, finite |y|<=1e100 and query in hull. Reciprocal weights w_i=1/product_{j!=i}(x_i-x_j); output sum(w_i*y_i/(x-x_i))/sum(w_i/(x-x_i)). Exact node returns its stored y after all validation. Scale terms by maximum absolute term before summing. Size precedes nonfinite, domain, spacing, then arithmetic checks. Underflow in node products can produce Arithmetic even for valid bounded inputs.

O(n²) multiplies, O(n) sums; stack 64 f64, no allocation or callbacks.

Validation order follows source. Ties at exact knots/endpoints take direct stored-value return after validation. Deterministic iteration, no heap allocation. Callback assumptions are explicit only for integration; interpolators have no callbacks. Real identities do not certify machine accuracy. Every successful result is finite. Arithmetic rejection does not imply invalid mathematical input. No unconditional relative-error percentage is claimed.

## approximant

### Forward approximant

Finite independent and adversarial tests for barycentric_lagrange. Production domain: Sorted 1..64 nodes in [-1,1], gaps >=1e-8, finite |y|<=1e100 and query in hull. Reciprocal weights w_i=1/product_{j!=i}(x_i-x_j); output sum(w_i*y_i/(x-x_i))/sum(w_i/(x-x_i)). Exact node returns its stored y after all validation. Scale terms by maximum absolute term before summing. Size precedes nonfinite, domain, spacing, then arithmetic checks. Underflow in node products can produce Arithmetic even for valid bounded inputs.

O(n²) multiplies, O(n) sums; stack 64 f64, no allocation or callbacks.

Citation: Berrut and Trefethen (2004), SIAM Review 46(3), 501–517. Equations implemented independently, no copied solution code. Real exactness requires exact arithmetic; stored tables and arithmetic are rounded. No certified useful cheaper executable Rust surrogate is attached: error certification near cancellation/zero and unseen callbacks remains a gap. Tests are checks, not a surrogate.

## models

## behavior_complement

The suite excludes implementation variants violating named oracle assertions; passing does not exclude arbitrary unsampled floating faults. Production nonbehavior relation: No successful extrapolation. For valid exact-node query an output other than its stored ordinate is excluded. Does not certify conditioning or floating accuracy.

## reverse_reconstruction

The all-pass observation admits both production and implementations differing only at unsampled queries. This nonunique observation fiber is broader than production preimages: At x=0, nodes [(-1,2),(1,4)] and [(-1,1),(1,5)] both map to3; a single evaluation cannot reconstruct node ordinates.

## independent_oracle

Oracle equations independent of implementation recurrence: In exact real arithmetic, every polynomial of degree <n is reproduced at every hull query. Machine tests compare x^d for d=0..5 on seven equally spaced nodes; tolerance is absolute 1e-12, not a universal bound.. Finite absolute tolerances include the zero-reference case; no percentage cap or unseen-domain theorem.

## boundary_partition

Named invalid/validation tests cover size or configuration precedence, nonfinite/domain rejection, exact endpoints or zero interval. Cases sharing the same error remain distinct input antecedents. Sorted 1..64 nodes in [-1,1], gaps >=1e-8, finite |y|<=1e100 and query in hull. Reciprocal weights w_i=1/product_{j!=i}(x_i-x_j); output sum(w_i*y_i/(x-x_i))/sum(w_i/(x-x_i)). Exact node returns its stored y after all validation. Scale terms by maximum absolute term before summing. Size precedes nonfinite, domain, spacing, then arithmetic checks. Underflow in node products can produce Arithmetic even for valid bounded inputs.

## compiled_fault_separation

A previously compiled arithmetic mutant was rejected by an oracle assertion. That observation concerns that particular mutation; it does not imply that every arithmetic fault is caught. This file's explicit independent oracle is the sampled monomial expression, not a Bernstein oracle.


## tests

Run cargo test -p mova-algorithms barycentric_lagrange::tests. This file checks sampled monomials, exact nodes, validation boundaries and one ambiguous fiber. Earlier interpolation-wide evidence includes other oracle types and a rejected arithmetic mutant: (see History). The point-fault control in model.md#models passes this file's tests and differs on a valid off-grid query. No performance benchmark, semantic proof, or generic floating error cap follows.

### Tests and evidence

## History

### 2026-10-30 · descriptant

Prospective prediction preserved in production prospective.md before source changes: fixed stack and recurrence reuse reduce repeated work without a benchmark speed claim. 2026-10-30 finite independent oracles are the evidence, not universal proofs.

### 2026-10-30 · tests

Run cargo test -p mova-algorithms barycentric_lagrange::tests. This file checks sampled monomials, exact nodes, validation boundaries and one ambiguous fiber. Earlier interpolation-wide evidence includes other oracle types and a rejected arithmetic mutant: $MOVA_DATA/runs/octopus-wave20-interpolation-2026-10-30/. The 2026-11-01 point-fault control in model.md#models passes this file's tests and differs on a valid off-grid query. No performance benchmark, semantic proof, or generic floating error cap follows.

### 2026-11-01 · reverse_reconstruction

On 2026-11-01, an actual-source harness included this unmodified test module twice: once over production and once over a wrapper returning 42 at the single query q=0.123456789 and delegating elsewhere. Both test modules passed. For nodes [(-1,0),(1,2)] the independent affine value is q+1, and the production API agrees within 1e-12 while the wrapper returns 42. Thus the passing transcript has at least these two implementation antecedents. This constructed point fault demonstrates nonuniqueness, not the prevalence of such faults or a complete converse.
