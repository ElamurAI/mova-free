# algorithm-forward-backward

## descriptant

### forward_backward algorithm

Original std-only implementation and fixtures; source citations appear in module documentation.

n1..64,T0..4096; finite initial, transition and time-major emission entries in[0,1]. Validate shape, then values, then each computed initial/transition row sum s with abs(s-1)<=1e-10. This checks the input weights without renormalizing them; accepted weights need not be exactly stochastic. Alpha log-sum-exp over incoming paths and beta log-sum-exp over outgoing paths use those original weights. Only posterior rows exp(alpha+beta-logZ) are normalized by their computed sum. Empty returns logZ0/no rows after validation; zero evidence returns None.

## model

### Invariants and state

n1..64,T0..4096; finite initial, transition and time-major emission entries in[0,1]. Validate shape, then values, then each computed initial/transition row sum s with abs(s-1)<=1e-10. This checks the input weights without renormalizing them; accepted weights need not be exactly stochastic. Alpha log-sum-exp over incoming paths and beta log-sum-exp over outgoing paths use those original weights. Only posterior rows exp(alpha+beta-logZ) are normalized by their computed sum. Empty returns logZ0/no rows after validation; zero evidence returns None.

Negative or nonnormalized posterior rows are excluded by the real model; represented rows are normalized by their computed sum. Impossible evidence excludes a posterior. Validation errors take precedence over empty-sequence success.

O((T+1)*n*n) total work including validation. For T>0, O(T*n+n) auxiliary space; log-sum-exp allocates a temporary bounded n-vector per recurrence. For T=0, validation still scans the transition matrix in O(n*n) time, then returns with O(1) auxiliary space.

All allocations follow standard Rust process policy, not fallible allocation APIs. No callbacks or external effects. f64 roundoff and underflow remain explicit limitations.

## approximant

### Forward and cost approximant

n1..64,T0..4096; finite initial, transition and time-major emission entries in[0,1]. Validate shape, then values, then each computed initial/transition row sum s with abs(s-1)<=1e-10. This checks the input weights without renormalizing them; accepted weights need not be exactly stochastic. Alpha log-sum-exp over incoming paths and beta log-sum-exp over outgoing paths use those original weights. Only posterior rows exp(alpha+beta-logZ) are normalized by their computed sum. Empty returns logZ0/no rows after validation; zero evidence returns None.

O((T+1)*n*n) total work including validation. For T>0, O(T*n+n) auxiliary space; log-sum-exp allocates a temporary bounded n-vector per recurrence. For T=0, validation still scans the transition matrix in O(n*n) time, then returns with O(1) auxiliary space.

No executable cheaper surrogate or supported generic 1/2/5/10% profile is claimed. Residuals and floating estimates are not solution-error certificates.

## models

## behavior_complement

Negative or nonnormalized posterior rows are excluded by the real model; represented rows are normalized by their computed sum. Impossible evidence excludes a posterior. Validation errors take precedence over empty-sequence success.

## reverse_reconstruction

Posterior and likelihood admit many hidden models, e.g. a one-state model with different emission factorizations sharing the same product. Observed marginals do not recover joint state paths or transitions uniquely.

## sum_product

Alpha sums all prefix path masses; beta sums suffix masses. Tiny exhaustive enumeration independently reconstructs Z and state numerators.

## log_stability

Subtracting the maximum prevents exp overflow; all-negative-infinity reductions remain negative infinity. Log arithmetic avoids whole-path product underflow but cannot preserve arbitrarily small posterior mass.

## posterior_normalization

Per-row normalization compensates accumulated log rounding; it does not prove a universal posterior error bound.

## empty_and_resources

Validate dimensions/probabilities/computed row sums before T0 return; impossible sequences return None. Accepted row sums differ from one by at most the absolute 1e-10 check; validation does not renormalize inputs. Total work O((T+1)n²); O(1) auxiliary storage at T0 and O(Tn+n) for T>0. No callbacks, allocation recovery or RNG.
## admitted_weight_discrepancy

Domain: n=1, T=1, transition=[[1]], emissions=[[1]], and a finite f64 initial weight p in[0,1] whose computed abs(p-1)<=1e-10. Hold the source revision and all other inputs fixed. In the one-state recurrence the observed log likelihood is p.ln(), while the ideal input-renormalized model replaces p by1 and gives log likelihood0. Both posterior rows are [1]; equal posterior observations do not establish equal likelihood models.

The source-derived witness p=1-5e-11 is accepted and has a negative log likelihood, distinguishing the accepted-weight relation from the exactly normalized ideal. Assumptions: ordinary Rust f64 library operations and successful standard allocation. This relation identifies an input-model discrepancy; it supplies no general floating error cap, long-sequence likelihood bound or posterior guarantee. Existing fixtures and historical receipts remain unchanged; no new tolerance-boundary runtime test is claimed here.

## tests

#### Validation

Run cargo test -p mova-algorithms forward_backward::tests. Evidence in (see History): repeated profiles, mutation logs and restored positive runs. Three named tests per module. See receipt.json for evidence hashes. No allocator fault injection, timing or full-school measurement.

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

Run cargo test -p mova-algorithms forward_backward::tests. Evidence in $MOVA_DATA/runs/octopus-wave26-probability-2026-10-30: repeated profiles, mutation logs and restored positive runs. Three named tests per module. See receipt.json for evidence hashes. No allocator fault injection, timing or full-school measurement.

### 2026-10-31 · descriptant

#### Source review 2026-10-31

Current models distinguish the computed row-sum tolerance from input renormalization and include transition validation cost for empty observations. Specialized relations are source-derived; no new runtime verification, timing or numerical certificate is claimed. Historical prospective notes and test receipts remain unchanged.
