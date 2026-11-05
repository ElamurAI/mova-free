# algorithm-sinkhorn-scaling

## descriptant

### sinkhorn_scaling algorithm

Original std-only implementation and fixtures; source citations appear in module documentation.

Dimensions1..64; all matrix and marginal values finite in[1e-100,1e100]. Shape then finite positive tolerance/budget<=10000 then values then marginal mass agreement within1e-12 relative. Alternate row scaling and column scaling. Report max absolute marginal discrepancy, iteration count and convergence flag.

Validation refinement: the initial rank-one fixture selected row targets proportional to its input row factors, so a compiled mutant omitting row scaling survived that oracle. The fixture now uses reversed row targets, independently of input factors; this mutant must fail convergence. Initial evidence is preserved in the external initial-validation directory. This repairs an oracle blind spot, not a production recurrence.

## model

### Invariants and state

Dimensions1..64; all matrix and marginal values finite in[1e-100,1e100]. Shape then finite positive tolerance/budget<=10000 then values then marginal mass agreement within1e-12 relative. Alternate row scaling and column scaling. Report max absolute marginal discrepancy, iteration count and convergence flag.

Converged excludes residual>tolerance, not large distance to a limit or failure of an optimization objective. Budget status excludes extra iterations. Nonpositive/nonfinite computed cells exclude successful output.

O(K*r*c) time, O(r*c) space, K<=10000. Standard allocation; no callbacks.

All allocations follow standard Rust process policy, not fallible allocation APIs. No callbacks or external effects. f64 roundoff and underflow remain explicit limitations.

## approximant

### Forward and cost approximant

Dimensions1..64; all matrix and marginal values finite in[1e-100,1e100]. Shape then finite positive tolerance/budget<=10000 then values then marginal mass agreement within1e-12 relative. Alternate row scaling and column scaling. Report max absolute marginal discrepancy, iteration count and convergence flag.

O(K*r*c) time, O(r*c) space, K<=10000. Standard allocation; no callbacks.

No executable cheaper surrogate or supported generic 1/2/5/10% profile is claimed. Residuals and floating estimates are not solution-error certificates.

## models

## behavior_complement

Converged excludes residual>tolerance, not large distance to a limit or failure of an optimization objective. Budget status excludes extra iterations. Nonpositive/nonfinite computed cells exclude successful output.

## reverse_reconstruction

Positive diagonal row/column rescalings share cross ratios and may converge to the same balanced matrix; original matrix and scaling factors are not recoverable uniquely. At budget0 the exact input matrix is retained.

## cross_ratio

Multiplying whole rows and columns preserves each 2x2 cross ratio in real arithmetic; finite checked fixture observes this separately from marginal checks.

## marginal_residual

Residual=max_i|sum_j xij-ri| and max_j|sum_i xij-cj|. It is absolute feasibility evidence only; near-equal input total tolerance can preclude stricter final feasibility.

## budget_partition

Check residual before each sweep including iteration0. Return converged if residual<=tolerance, else budget-exhausted at K. Zero budget does no scaling.

## positive_arithmetic

Positive entries and targets avoid mathematical division by zero; machine underflow/overflow is rejected after each complete sweep. Standard allocations, no callback/RNG or universal convergence rate.

## tests

#### Validation

Run cargo test -p mova-algorithms sinkhorn_scaling::tests. Evidence in (see History): repeated profiles, mutation logs and restored positive runs. Three named tests per module. See receipt.json for evidence hashes. No allocator fault injection, timing or full-school measurement.

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

Run cargo test -p mova-algorithms sinkhorn_scaling::tests. Evidence in $MOVA_DATA/runs/octopus-wave26-probability-2026-10-30: repeated profiles, mutation logs and restored positive runs. Three named tests per module. See receipt.json for evidence hashes. No allocator fault injection, timing or full-school measurement.
