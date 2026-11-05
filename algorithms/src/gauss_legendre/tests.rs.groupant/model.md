# algorithm-gauss-legendre-tests

## descriptant

### Description and epoch memory

gauss_legendre tests.rs. Original standard-library-only implementation with bounded work/storage.

Citation: Davis and Rabinowitz, Methods of Numerical Integration, 2nd ed., Gaussian quadrature.

### Combiner

Raw source selects recurrence and validation; forward equations describe real observation; complement excludes concrete outcomes; reverse relation retains ambiguous antecedents. Typed specialized relations describe polynomial/shape/sample structure. Partial syntax binds source bytes but has no semantic resolution. Sibling tests falsify selected claims. Composition transports absolute perturbations only under stated premises; no automatic error-tier inheritance.

## model

### Invariant and scope

Order1..5; finite -1e100<=a<=b<=1e100. Fixed rounded Legendre nodes and positive weights; value=(b-a)/2*sum w_i*f((a+b)/2+(b-a)/2*x_i). Order, nonfinite, domain, then callback validity precedence. Equal endpoints return0 without callback. FnMut assumed deterministic with finite |f|<=1e100; errors abort at first bad sample, panic propagates.

Exactly order callback evaluations on nondegenerate success, increasing tabulated nodes, O(order) arithmetic and constant stack. Caller callback cost/panics external, no allocations.

Validation order follows source. Ties at exact knots/endpoints take direct stored-value return after validation. Deterministic iteration, no heap allocation. Callback assumptions are explicit only for integration; interpolators have no callbacks. Real identities do not certify machine accuracy. Every successful result is finite. Arithmetic rejection does not imply invalid mathematical input. No unconditional relative-error percentage is claimed.

## approximant

### Forward approximant

Finite independent and adversarial tests for gauss_legendre. Production domain: Order1..5; finite -1e100<=a<=b<=1e100. Fixed rounded Legendre nodes and positive weights; value=(b-a)/2*sum w_i*f((a+b)/2+(b-a)/2*x_i). Order, nonfinite, domain, then callback validity precedence. Equal endpoints return0 without callback. FnMut assumed deterministic with finite |f|<=1e100; errors abort at first bad sample, panic propagates.

Exactly order callback evaluations on nondegenerate success, increasing tabulated nodes, O(order) arithmetic and constant stack. Caller callback cost/panics external, no allocations.

Citation: Davis and Rabinowitz, Methods of Numerical Integration, 2nd ed., Gaussian quadrature. Equations implemented independently, no copied solution code. Real exactness requires exact arithmetic; stored tables and arithmetic are rounded. No certified useful cheaper executable Rust surrogate is attached: error certification near cancellation/zero and unseen callbacks remains a gap. Tests are checks, not a surrogate.

## models

## behavior_complement

The suite excludes implementation variants violating named oracle assertions; passing does not exclude arbitrary unsampled floating faults. Production nonbehavior relation: No call on valid zero-width interval. Successful callback result NaN or magnitude>1e100 is excluded. No adaptive refinement or error estimate supplied.

## reverse_reconstruction

The all-pass observation admits both production and implementations differing only at unsampled queries. This nonunique observation fiber is broader than production preimages: Every odd integrand on [-1,1] has real quadrature sum0 by symmetry. x and zero function are distinct compatible antecedents; finite samples do not reconstruct an integrand.

## independent_oracle

Oracle equations independent of implementation recurrence: With exact algebraic nodes/weights and real arithmetic, degree<=2order-1 polynomials integrate exactly. Stored f64 constants and arithmetic break literal exactness; tests compare analytic antiderivatives with absolute tolerance1e-12.. Finite absolute tolerances include the zero-reference case; no percentage cap or unseen-domain theorem.

## boundary_partition

Named invalid/validation tests cover size or configuration precedence, nonfinite/domain rejection, exact endpoints or zero interval. Cases sharing the same error remain distinct input antecedents. Order1..5; finite -1e100<=a<=b<=1e100. Fixed rounded Legendre nodes and positive weights; value=(b-a)/2*sum w_i*f((a+b)/2+(b-a)/2*x_i). Order, nonfinite, domain, then callback validity precedence. Equal endpoints return0 without callback. FnMut assumed deterministic with finite |f|<=1e100; errors abort at first bad sample, panic propagates.

## compiled_fault_separation

A compiled arithmetic mutant of gauss_legendre must fail its named polynomial/affine/Bernstein oracle assertion. This distinguishes that fault from production; it does not establish completeness of the oracle.


## tests

Run cargo test -p mova-algorithms gauss_legendre::tests. Independent analytic polynomial or Bernstein oracle, shape constraints where applicable, validation boundaries and ambiguous fibers. Two debug and two release executions normalize named test outcomes before comparison. Compiled production arithmetic mutant fails named oracle; original bytes restored and positive test rerun. Evidence: (see History). Receipt records concrete result and hashes. No performance benchmark, semantic proof, or generic floating error cap.

### Tests and evidence

## History

### 2026-10-30 · descriptant

Prospective prediction preserved in production prospective.md before source changes: fixed stack and recurrence reuse reduce repeated work without a benchmark speed claim. 2026-10-30 finite independent oracles are the evidence, not universal proofs.

### 2026-10-30 · tests

Run cargo test -p mova-algorithms gauss_legendre::tests. Independent analytic polynomial or Bernstein oracle, shape constraints where applicable, validation boundaries and ambiguous fibers. Two debug and two release executions normalize named test outcomes before comparison. Compiled production arithmetic mutant fails named oracle; original bytes restored and positive test rerun. Evidence: $MOVA_DATA/runs/octopus-wave20-interpolation-2026-10-30/. Receipt records concrete result and hashes. No performance benchmark, semantic proof, or generic floating error cap.
