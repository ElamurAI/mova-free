# algorithm-monotone-cubic-tests

## descriptant

### Description and epoch memory

monotone_cubic tests.rs. Original standard-library-only implementation with bounded work/storage.

Citation: Fritsch and Butland (1984), SIAM J. Sci. Stat. Comput. 5, 300–304.

### Combiner

Raw source selects recurrence and validation; forward equations describe real observation; complement excludes concrete outcomes; reverse relation retains ambiguous antecedents. Typed specialized relations describe polynomial/shape/sample structure. Partial syntax binds source bytes but has no semantic resolution. Sibling tests falsify selected claims. Composition transports absolute perturbations only under stated premises; no automatic error-tier inheritance.

## model

### Invariant and scope

Sorted 2..64 nodes in [-1,1], gaps >=1e-8, finite |y|<=1e100, hull query. Interior derivative is weighted harmonic mean of adjacent secants when nonzero and same sign, zero otherwise. Endpoint uses one-sided derivative then sign/3*secant clipping. Cubic Hermite basis evaluates the selected segment. Size, nonfinite, domain, spacing then arithmetic precedence. Two nodes give linear interpolation; exact node returns stored y.

O(n) validation plus partition lookup O(log n); only neighboring secants/slopes evaluated; O(1) stack, no allocation/callback.

Validation order follows source. Ties at exact knots/endpoints take direct stored-value return after validation. Deterministic iteration, no heap allocation. Callback assumptions are explicit only for integration; interpolators have no callbacks. Real identities do not certify machine accuracy. Every successful result is finite. Arithmetic rejection does not imply invalid mathematical input. No unconditional relative-error percentage is claimed.

## approximant

### Forward approximant

Finite independent and adversarial tests for monotone_cubic. Production domain: Sorted 2..64 nodes in [-1,1], gaps >=1e-8, finite |y|<=1e100, hull query. Interior derivative is weighted harmonic mean of adjacent secants when nonzero and same sign, zero otherwise. Endpoint uses one-sided derivative then sign/3*secant clipping. Cubic Hermite basis evaluates the selected segment. Size, nonfinite, domain, spacing then arithmetic precedence. Two nodes give linear interpolation; exact node returns stored y.

O(n) validation plus partition lookup O(log n); only neighboring secants/slopes evaluated; O(1) stack, no allocation/callback.

Citation: Fritsch and Butland (1984), SIAM J. Sci. Stat. Comput. 5, 300–304. Equations implemented independently, no copied solution code. Real exactness requires exact arithmetic; stored tables and arithmetic are rounded. No certified useful cheaper executable Rust surrogate is attached: error certification near cancellation/zero and unseen callbacks remains a gap. Tests are checks, not a surrogate.

## models

## behavior_complement

The suite excludes implementation variants violating named oracle assertions; passing does not exclude arbitrary unsampled floating faults. Production nonbehavior relation: No extrapolation; with monotone real data a segment value outside endpoint ordinate interval is excluded in the real model. No machine theorem or relative accuracy promise.

## reverse_reconstruction

The all-pass observation admits both production and implementations differing only at unsampled queries. This nonunique observation fiber is broader than production preimages: For two nodes, midpoint of [0,2] and [-1,3] is1 with identical x=[0,1]; one query cannot recover endpoint values or slopes.

## independent_oracle

Oracle equations independent of implementation recurrence: If adjacent secants have opposite signs or either zero, interior slope is exactly zero. At local maximum [(0,0),(.3,1),(1,0)] the central slope vanishes and segment shape remains bounded in the real model.. Finite absolute tolerances include the zero-reference case; no percentage cap or unseen-domain theorem.

## boundary_partition

Named invalid/validation tests cover size or configuration precedence, nonfinite/domain rejection, exact endpoints or zero interval. Cases sharing the same error remain distinct input antecedents. Sorted 2..64 nodes in [-1,1], gaps >=1e-8, finite |y|<=1e100, hull query. Interior derivative is weighted harmonic mean of adjacent secants when nonzero and same sign, zero otherwise. Endpoint uses one-sided derivative then sign/3*secant clipping. Cubic Hermite basis evaluates the selected segment. Size, nonfinite, domain, spacing then arithmetic precedence. Two nodes give linear interpolation; exact node returns stored y.

## compiled_fault_separation

A compiled arithmetic mutant of monotone_cubic must fail its named polynomial/affine/Bernstein oracle assertion. This distinguishes that fault from production; it does not establish completeness of the oracle.


## tests

Run cargo test -p mova-algorithms monotone_cubic::tests. Independent analytic polynomial or Bernstein oracle, shape constraints where applicable, validation boundaries and ambiguous fibers. Two debug and two release executions normalize named test outcomes before comparison. Compiled production arithmetic mutant fails named oracle; original bytes restored and positive test rerun. Evidence: (see History). Receipt records concrete result and hashes. No performance benchmark, semantic proof, or generic floating error cap.

### Tests and evidence

## History

### 2026-10-30 · descriptant

Prospective prediction preserved in production prospective.md before source changes: fixed stack and recurrence reuse reduce repeated work without a benchmark speed claim. 2026-10-30 finite independent oracles are the evidence, not universal proofs.

### 2026-10-30 · tests

Run cargo test -p mova-algorithms monotone_cubic::tests. Independent analytic polynomial or Bernstein oracle, shape constraints where applicable, validation boundaries and ambiguous fibers. Two debug and two release executions normalize named test outcomes before comparison. Compiled production arithmetic mutant fails named oracle; original bytes restored and positive test rerun. Evidence: $MOVA_DATA/runs/octopus-wave20-interpolation-2026-10-30/. Receipt records concrete result and hashes. No performance benchmark, semantic proof, or generic floating error cap.
