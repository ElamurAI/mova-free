# algorithm-viterbi

## descriptant

### viterbi algorithm

Original std-only implementation and fixtures; source citations appear in module documentation.

n1..64,T0..4096; finite initial, transition and time-major emission entries in[0,1]. Shape validation precedes value validation, then each computed initial/transition row sum s must satisfy abs(s-1)<=1e-10. These checks do not renormalize the accepted input weights. log score(t,j)=log emission(t,j)+max_i(score(t-1,i)+log transition(i,j)) uses the original weights; store predecessor. Empty sequence returns empty path/log0 after validation; impossible returns None.

## model

### Invariants and state

n1..64,T0..4096; finite initial, transition and time-major emission entries in[0,1]. Shape validation precedes value validation, then each computed initial/transition row sum s must satisfy abs(s-1)<=1e-10. These checks do not renormalize the accepted input weights. log score(t,j)=log emission(t,j)+max_i(score(t-1,i)+log transition(i,j)) uses the original weights; store predecessor. Empty sequence returns empty path/log0 after validation; impossible returns None.

Any path with strictly lower represented max-plus score than the selected terminal score is excluded. None excludes positive-probability paths in the validated model. Structural or numeric errors exclude a success even for empty emissions.

O((T+1)*n*n) total work including validation. For T>0, O(T*n+n) auxiliary storage with standard allocation. For T=0, validation still scans the transition matrix in O(n*n) time, then returns with O(1) auxiliary space.

All allocations follow standard Rust process policy, not fallible allocation APIs. No callbacks or external effects. f64 roundoff and underflow remain explicit limitations.

## approximant

### Forward and cost approximant

n1..64,T0..4096; finite initial, transition and time-major emission entries in[0,1]. Shape validation precedes value validation, then each computed initial/transition row sum s must satisfy abs(s-1)<=1e-10. These checks do not renormalize the accepted input weights. log score(t,j)=log emission(t,j)+max_i(score(t-1,i)+log transition(i,j)) uses the original weights; store predecessor. Empty sequence returns empty path/log0 after validation; impossible returns None.

O((T+1)*n*n) total work including validation. For T>0, O(T*n+n) auxiliary storage with standard allocation. For T=0, validation still scans the transition matrix in O(n*n) time, then returns with O(1) auxiliary space.

No executable cheaper surrogate or supported generic 1/2/5/10% profile is claimed. Residuals and floating estimates are not solution-error certificates.

## models

## behavior_complement

Any path with strictly lower represented max-plus score than the selected terminal score is excluded. None excludes positive-probability paths in the validated model. Structural or numeric errors exclude a success even for empty emissions.

## reverse_reconstruction

A selected state path and log score admit infinitely many HMMs differing on unused edges, subject to the path remaining optimal. The path cannot recover initial probabilities or the full transition matrix.

## tie_policy

Strict greater comparisons retain the lowest predecessor index; terminal strict greater retains the lowest terminal index. This local rule is not global lexicographic path order.

## log_semiring

Zero probabilities map to negative infinity. Finite nonzero f64 logs accumulate without direct-product underflow; tiny path differences can round into ties.

## backpointer_witness

Each state at time t-1 is stored predecessor for the selected state at t. Recomputing direct tiny path mass independently checks witness and objective.

## validation_resources

Dimensions and time budget precede value and stochastic-row validation. Empty valid input has unit likelihood. No callbacks or RNG; allocator failures use standard Rust policy.
## empty_validation_frontier

Domain: empty emissions T=0 with n in1..64, an n-vector initial distribution and an n-by-n transition matrix. Hold the source revision fixed. The return path remains validate(shape) -> validate(all values) -> validate(computed stochastic-row sums) -> empty states/log_probability0. A valid matrix therefore incurs O(n²) validation work even though the recurrence performs zero transitions; the empty branch uses O(1) auxiliary storage. This corrects total work to O((T+1)n²), not O(Tn²) over the admitted T=0 domain.

Concrete source witness: initial=[1], transition=[[1]], emissions=[] reaches the empty success; replacing that transition by [[NaN]] returns Probability before the same empty branch. Assumptions: ordinary Rust execution and valid borrowed slice shapes where stated; no allocation, timing or speedup claim for this empty branch. This model relates validation work and first failure to a fixed output horizon, not a bound on nonempty decoding accuracy. Existing validation tests remain the recorded finite evidence; the work count and new witness are source reasoning, with no new runtime verification claim.

## tests

#### Validation

Run cargo test -p mova-algorithms viterbi::tests. Evidence in (see History): repeated profiles, mutation logs and restored positive runs. Three named tests per module. See receipt.json for evidence hashes. No allocator fault injection, timing or full-school measurement.

## prospective

### Probability batch prospective math

Written before implementation. All code original, std-only. Bounded allocations use standard Vec; allocator failure follows Rust process policy, no callbacks or RNG.

Alias: Walker (1977), An efficient method for generating discrete random variables with general distributions; Vose (1991), A linear algorithm for generating random numbers with a given distribution. n 1..4096, finite nonnegative weights, positive maximum. Scale by maximum then total to avoid overflow. Small/large work lists conserve total scaled mass. Column uniform and coin uniform [0,1) produce the specified categorical distribution in real arithmetic; caller explicitly supplies coordinates. O(n) build/storage, O(1) draw. No probabilistic draw generation or numeric percentage guarantee.

HMM: Rabiner (1989), A tutorial on hidden Markov models and selected applications in speech recognition; Viterbi (1967), Error bounds for convolutional codes and an asymptotically optimum decoding algorithm. n1..64, T0..4096; finite probabilities in [0,1], initial/transition rows sum to one within 1e-10. Emissions time-major likelihoods in [0,1], need not sum across states. Validate structure then numeric values before empty sequence. Empty log likelihood0/path empty. Viterbi log max-plus recurrence, predecessor ties lowest index and terminal ties lowest index (not global lexicographic). No positive path =>None. Forward/backward log-sum-exp, posterior proportional exp(alpha+beta-logZ); per-row normalization corrects rounding. O(Tn²) time/O(Tn) space. Tiny exhaustive state-sequence enumeration is independent oracle. Log scales avoid direct-product underflow; no general floating error bound.

Sinkhorn: Sinkhorn and Knopp (1967), Concerning nonnegative matrices and doubly stochastic matrices. Positive matrix entries and marginals restricted [1e-100,1e100], dims1..64, equal marginal totals within tolerance1e-12 relative. Alternate row and column multiplication to target marginals; residual max absolute marginal discrepancy. Budget0..10000, finite tolerance>0. Stop on residual<=tolerance or budget; overflow/underflow to zero explicit arithmetic error. O(iterations*r*c) time/O(rc) space. Residual is feasibility evidence, not solution distance/optimality guarantee. Scale-invariant input fibers and budget status explicit.

Weighted covariance: West (1979), Updating mean and variance estimates: an improved method; Chan, Golub, LeVeque (1983), Algorithms for computing the sample variance. dimension1..64; finite coordinates |x|<=1e100, weights[0,1e100]. Population covariance M2/W, not unbiased sample covariance. Zero weight validates coordinates then does nothing. Merge dimensions must match, empty neutral. Merge W=A+B; delta=muB-muA; mu=muA+delta*B/W; M2=M2A+M2B+delta outer delta*A*(B/W). Atomic error updates, finite intermediates required. O(d²) push/merge/storage. Independent batch weighted centered moments oracle, partition merge equivalence within tolerance. No order-bitwise identity or universal covariance error certificate.

## History

### 2026-10-30 · descriptant

Epoch 2026-10-30: independent tiny-domain oracle, repeated debug/release and compiled production mutation. No timing or end-to-end Mova claims.

### 2026-10-30 · tests

#### Validation

Run cargo test -p mova-algorithms viterbi::tests. Evidence in $MOVA_DATA/runs/octopus-wave26-probability-2026-10-30: repeated profiles, mutation logs and restored positive runs. Three named tests per module. See receipt.json for evidence hashes. No allocator fault injection, timing or full-school measurement.

### 2026-10-31 · descriptant

#### Source review 2026-10-31

Current models distinguish the computed row-sum tolerance from input renormalization and include transition validation cost for empty observations. Specialized relations are source-derived; no new runtime verification, timing or numerical certificate is claimed. Historical prospective notes and test receipts remain unchanged.
