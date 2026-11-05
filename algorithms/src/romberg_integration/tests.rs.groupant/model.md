# algorithm-romberg-integration-tests

## descriptant

### Description and epoch memory

romberg_integration tests.rs. Original standard-library-only implementation with bounded work/storage.

Citation: Romberg (1955), Det Kongelige Norske Videnskabers Selskab Forhandlinger28, 30–36.

### Combiner

Raw source selects recurrence and validation; forward equations describe real observation; complement excludes concrete outcomes; reverse relation retains ambiguous antecedents. Typed specialized relations describe polynomial/shape/sample structure. Partial syntax binds source bytes but has no semantic resolution. Sibling tests falsify selected claims. Composition transports absolute perturbations only under stated premises; no automatic error-tier inheritance.

## model

### Invariant and scope

Finite |a|,|b|<=1e100, a<=b; finite tolerance>0; levels1..20, evaluation budget>=2. Configuration precedes bounds checks. T_k=T_(k-1)/2+h*sum new odd-grid samples. R_(k,j)=R_(k,j-1)+(R_(k,j-1)-R_(k-1,j-1))/(4^j-1). Callback deterministic finite |f|<=1e100, panics propagate. Successive diagonal difference is only heuristic absolute error. Configuration/domain validated before degenerate return.

At complete level L>=1, 2^(L-1)+1 calls; zero interval0 calls. O(2^L+L²) arithmetic, fixed20 f64 stack, no allocation. Refinement budget checked before new level, partial coordinate-stagnated level may consume samples and report previous completed level value.

Validation order follows source. Ties at exact knots/endpoints take direct stored-value return after validation. Deterministic iteration, no heap allocation. Callback assumptions are explicit only for integration; interpolators have no callbacks. Real identities do not certify machine accuracy. Every successful result is finite. Arithmetic rejection does not imply invalid mathematical input. No unconditional relative-error percentage is claimed.

## approximant

### Forward approximant

Finite independent and adversarial tests for romberg_integration. Production domain: Finite |a|,|b|<=1e100, a<=b; finite tolerance>0; levels1..20, evaluation budget>=2. Configuration precedes bounds checks. T_k=T_(k-1)/2+h*sum new odd-grid samples. R_(k,j)=R_(k,j-1)+(R_(k,j-1)-R_(k-1,j-1))/(4^j-1). Callback deterministic finite |f|<=1e100, panics propagate. Successive diagonal difference is only heuristic absolute error. Configuration/domain validated before degenerate return.

At complete level L>=1, 2^(L-1)+1 calls; zero interval0 calls. O(2^L+L²) arithmetic, fixed20 f64 stack, no allocation. Refinement budget checked before new level, partial coordinate-stagnated level may consume samples and report previous completed level value.

Citation: Romberg (1955), Det Kongelige Norske Videnskabers Selskab Forhandlinger28, 30–36. Equations implemented independently, no copied solution code. Real exactness requires exact arithmetic; stored tables and arithmetic are rounded. No certified useful cheaper executable Rust surrogate is attached: error certification near cancellation/zero and unseen callbacks remains a gap. Tests are checks, not a surrogate.

## models

## behavior_complement

The suite excludes implementation variants violating named oracle assertions; passing does not exclude arbitrary unsampled floating faults. Production nonbehavior relation: No callback count above budget and no refinement above20 levels. Converged excludes only a large latest diagonal difference, not large true integration error. Sample aliasing can fool it.

## reverse_reconstruction

The all-pass observation admits both production and implementations differing only at unsampled queries. This nonunique observation fiber is broader than production preimages: All functions agreeing on the finite sampled dyadic grid give the same report under deterministic callbacks. Adding c*product(x-x_j)^2 preserves every sampled value but changes the true integral; no inverse reconstruction of full function or certified error follows.

## independent_oracle

Oracle equations independent of implementation recurrence: T_k reuses previous trapezoid contribution and samples only2^(k-1) new odd points. Complete-level count is2^k+1, tested independently through report counters.. Finite absolute tolerances include the zero-reference case; no percentage cap or unseen-domain theorem.

## boundary_partition

Named invalid/validation tests cover size or configuration precedence, nonfinite/domain rejection, exact endpoints or zero interval. Cases sharing the same error remain distinct input antecedents. Finite |a|,|b|<=1e100, a<=b; finite tolerance>0; levels1..20, evaluation budget>=2. Configuration precedes bounds checks. T_k=T_(k-1)/2+h*sum new odd-grid samples. R_(k,j)=R_(k,j-1)+(R_(k,j-1)-R_(k-1,j-1))/(4^j-1). Callback deterministic finite |f|<=1e100, panics propagate. Successive diagonal difference is only heuristic absolute error. Configuration/domain validated before degenerate return.

## compiled_fault_separation

A compiled arithmetic mutant of romberg_integration must fail its named polynomial/affine/Bernstein oracle assertion. This distinguishes that fault from production; it does not establish completeness of the oracle.


## tests

Run cargo test -p mova-algorithms romberg_integration::tests. Independent analytic polynomial or Bernstein oracle, shape constraints where applicable, validation boundaries and ambiguous fibers. Two debug and two release executions normalize named test outcomes before comparison. Compiled production arithmetic mutant fails named oracle; original bytes restored and positive test rerun. Evidence: (see History). Receipt records concrete result and hashes. No performance benchmark, semantic proof, or generic floating error cap.

### Tests and evidence

## History

### 2026-10-30 · descriptant

Prospective prediction preserved in production prospective.md before source changes: fixed stack and recurrence reuse reduce repeated work without a benchmark speed claim. 2026-10-30 finite independent oracles are the evidence, not universal proofs.

### 2026-10-30 · tests

Run cargo test -p mova-algorithms romberg_integration::tests. Independent analytic polynomial or Bernstein oracle, shape constraints where applicable, validation boundaries and ambiguous fibers. Two debug and two release executions normalize named test outcomes before comparison. Compiled production arithmetic mutant fails named oracle; original bytes restored and positive test rerun. Evidence: $MOVA_DATA/runs/octopus-wave20-interpolation-2026-10-30/. Receipt records concrete result and hashes. No performance benchmark, semantic proof, or generic floating error cap.
