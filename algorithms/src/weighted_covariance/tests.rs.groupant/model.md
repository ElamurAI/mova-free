# algorithm-weighted-covariance-tests

## descriptant

### weighted_covariance checks

Original std-only implementation and fixtures; source citations appear in module documentation.

Independent finite oracle and boundary tests for weighted_covariance. Production: Dimension1..64; finite |coordinates|<=1e100, weight[0,1e100]. Push validates dimension then point then weight before zero-weight no-op. Merge validates equal dimensions before empty neutrality. W=A+B, delta=meanB-meanA, mean=meanA+delta*B/W, M2=M2A+M2B+outer(delta)*A*(B/W). Population covariance M2/W; empty None.

## model

### Invariants and state

Independent finite oracle and boundary tests for weighted_covariance. Production: Dimension1..64; finite |coordinates|<=1e100, weight[0,1e100]. Push validates dimension then point then weight before zero-weight no-op. Merge validates equal dimensions before empty neutrality. W=A+B, delta=meanB-meanA, mean=meanA+delta*B/W, M2=M2A+M2B+outer(delta)*A*(B/W). Population covariance M2/W; empty None.

All-pass excludes represented oracle mismatches and rejected boundary violations in these finite fixtures only. Production excluded-output relation: Reported arithmetic failures exclude partial updates; invalid zero-weight observations still error. Negative input weights and mismatched dimensions exclude success. Covariance is population rather than unbiased sample estimator.

O(d*d) push/merge/storage; getters mean/weight O(1), covariance O(d*d) allocation.

All allocations follow standard Rust process policy, not fallible allocation APIs. No callbacks or external effects. f64 roundoff and underflow remain explicit limitations.

## approximant

### Forward and cost approximant

Independent finite oracle and boundary tests for weighted_covariance. Production: Dimension1..64; finite |coordinates|<=1e100, weight[0,1e100]. Push validates dimension then point then weight before zero-weight no-op. Merge validates equal dimensions before empty neutrality. W=A+B, delta=meanB-meanA, mean=meanA+delta*B/W, M2=M2A+M2B+outer(delta)*A*(B/W). Population covariance M2/W; empty None.

O(d*d) push/merge/storage; getters mean/weight O(1), covariance O(d*d) allocation.

No executable cheaper surrogate or supported generic 1/2/5/10% profile is claimed. Residuals and floating estimates are not solution-error certificates.

## models

## behavior_complement

All-pass excludes represented oracle mismatches and rejected boundary violations in these finite fixtures only. Production excluded-output relation: Reported arithmetic failures exclude partial updates; invalid zero-weight observations still error. Negative input weights and mismatched dimensions exclude success. Covariance is population rather than unbiased sample estimator.

## reverse_reconstruction

All-pass observations admit implementations differing on unsampled inputs. This finite test fiber does not uniquely identify source or prove universality. Production fiber: Many weighted point multisets share W, mean, M2; permutations and splitting one observation into same-point weights preserve real moments. Original sample count and points are not reconstructible. Grouping can change floating results.

## independent_oracle

Tiny path enumeration, analytical rank-one matrices or independent weighted moments reconstruct expectations without calling the production recurrence; scope is finite.

## boundary_partition

Shape, zero/empty, invalid finite values, ties or budget boundaries are explicitly asserted. Numeric tolerance is fixture-specific.

## compiled_mutation

A compiled changed production recurrence must fail the named oracle assertion. Restoration plus positive rerun checks that the gate can distinguish this wrong behavior.

## tests

#### Validation

Run cargo test -p mova-algorithms weighted_covariance::tests. Evidence in (see History): repeated profiles, mutation logs and restored positive runs. Three named tests per module. See receipt.json for evidence hashes. No allocator fault injection, timing or full-school measurement.

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

Run cargo test -p mova-algorithms weighted_covariance::tests. Evidence in $MOVA_DATA/runs/octopus-wave26-probability-2026-10-30: repeated profiles, mutation logs and restored positive runs. Three named tests per module. See receipt.json for evidence hashes. No allocator fault injection, timing or full-school measurement.
