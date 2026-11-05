# algorithm-bernstein-de-casteljau-tests

## descriptant

### Description and epoch memory

bernstein_de_casteljau tests.rs. Original standard-library-only implementation with bounded work/storage.

Citation: Farin, Curves and Surfaces for CAGD, 5th ed., chapter4.

### Combiner

Raw source selects recurrence and validation; forward equations describe real observation; complement excludes concrete outcomes; reverse relation retains ambiguous antecedents. Typed specialized relations describe polynomial/shape/sample structure. Partial syntax binds source bytes but has no semantic resolution. Sibling tests falsify selected claims. Composition transports absolute perturbations only under stated premises; no automatic error-tier inheritance.

## model

### Invariant and scope

1..128 finite controls |c|<=1e100; t in[0,1]. Repeated triangle c_i^(r)=(1-t)c_i^(r-1)+t*c_(i+1)^(r-1) yields sum binomial(n,i)*(1-t)^(n-i)*t^i*c_i. Size, nonfinite, domain, arithmetic precedence. t=0 and1 shortcuts return exact stored controls.

n(n-1)/2 blends, O(n²) time; stack128 f64, no allocation/callback.

Validation order follows source. Ties at exact knots/endpoints take direct stored-value return after validation. Deterministic iteration, no heap allocation. Callback assumptions are explicit only for integration; interpolators have no callbacks. Real identities do not certify machine accuracy. Every successful result is finite. Arithmetic rejection does not imply invalid mathematical input. No unconditional relative-error percentage is claimed.

## approximant

### Forward approximant

Finite independent and adversarial tests for bernstein_de_casteljau. Production domain: 1..128 finite controls |c|<=1e100; t in[0,1]. Repeated triangle c_i^(r)=(1-t)c_i^(r-1)+t*c_(i+1)^(r-1) yields sum binomial(n,i)*(1-t)^(n-i)*t^i*c_i. Size, nonfinite, domain, arithmetic precedence. t=0 and1 shortcuts return exact stored controls.

n(n-1)/2 blends, O(n²) time; stack128 f64, no allocation/callback.

Citation: Farin, Curves and Surfaces for CAGD, 5th ed., chapter4. Equations implemented independently, no copied solution code. Real exactness requires exact arithmetic; stored tables and arithmetic are rounded. No certified useful cheaper executable Rust surrogate is attached: error certification near cancellation/zero and unseen callbacks remains a gap. Tests are checks, not a surrogate.

## models

## behavior_complement

The suite excludes implementation variants violating named oracle assertions; passing does not exclude arbitrary unsampled floating faults. Production nonbehavior relation: For valid t=0 output unequal to first control is excluded. In the real model values outside the control convex hull are excluded. No universal f64 percentage claim.

## reverse_reconstruction

The all-pass observation admits both production and implementations differing only at unsampled queries. This nonunique observation fiber is broader than production preimages: At t=.5, controls [0,2] and [-1,3] both yield1. At t=0 all tails with the same first control are compatible antecedents.

## independent_oracle

Oracle equations independent of implementation recurrence: Every intermediate real control is a convex combination; induction preserves min(c)<=row<=max(c). Fixed finite tests inspect129-or-fewer point sweeps; floating rounding is separate.. Finite absolute tolerances include the zero-reference case; no percentage cap or unseen-domain theorem.

## boundary_partition

Named invalid/validation tests cover size or configuration precedence, nonfinite/domain rejection, exact endpoints or zero interval. Cases sharing the same error remain distinct input antecedents. 1..128 finite controls |c|<=1e100; t in[0,1]. Repeated triangle c_i^(r)=(1-t)c_i^(r-1)+t*c_(i+1)^(r-1) yields sum binomial(n,i)*(1-t)^(n-i)*t^i*c_i. Size, nonfinite, domain, arithmetic precedence. t=0 and1 shortcuts return exact stored controls.

## compiled_fault_separation

A compiled arithmetic mutant of bernstein_de_casteljau must fail its named polynomial/affine/Bernstein oracle assertion. This distinguishes that fault from production; it does not establish completeness of the oracle.


## tests

Run cargo test -p mova-algorithms bernstein_de_casteljau::tests. Independent analytic polynomial or Bernstein oracle, shape constraints where applicable, validation boundaries and ambiguous fibers. Two debug and two release executions normalize named test outcomes before comparison. Compiled production arithmetic mutant fails named oracle; original bytes restored and positive test rerun. Evidence: (see History). Receipt records concrete result and hashes. No performance benchmark, semantic proof, or generic floating error cap.

### Tests and evidence

## History

### 2026-10-30 · descriptant

Prospective prediction preserved in production prospective.md before source changes: fixed stack and recurrence reuse reduce repeated work without a benchmark speed claim. 2026-10-30 finite independent oracles are the evidence, not universal proofs.

### 2026-10-30 · tests

Run cargo test -p mova-algorithms bernstein_de_casteljau::tests. Independent analytic polynomial or Bernstein oracle, shape constraints where applicable, validation boundaries and ambiguous fibers. Two debug and two release executions normalize named test outcomes before comparison. Compiled production arithmetic mutant fails named oracle; original bytes restored and positive test rerun. Evidence: $MOVA_DATA/runs/octopus-wave20-interpolation-2026-10-30/. Receipt records concrete result and hashes. No performance benchmark, semantic proof, or generic floating error cap.

### 2026-11-01 · document

#### 2026-11-01 test-observation fiber

For the 361 call occurrences in this source, an implementation equal to `evaluate` except that it adds one at `([42], 0.5)` has the same returned results on every call. The actual API returns 42 at that valid witness and the mutant returns 43. Thus the all-pass converse fiber contains at least two distinct implementations; it is not a reconstruction of the evaluator. This is a finite source-call audit, assuming deterministic execution and exact guard equality, not a general floating-point error guarantee. The witness also falsifies the proposed exclusion of all incorrect singleton results. The independent direct Bernstein sum tests 12 control vectors at 21 parameter values, not all controls of those degrees. The hull sweep and endpoint/fiber and error assertions have separate finite domains. Arithmetic-error reachability and correctness on unsampled inputs remain open.
