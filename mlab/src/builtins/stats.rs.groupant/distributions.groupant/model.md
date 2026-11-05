# mlab-builtins-stats-distributions

## descriptant

### mlab-builtins-stats-distributions

Registry installs scalar-broadcast distributions. bcast checks arity/types then identical nonscalar shape, defaults scalar parameters. phi=erfc(-z/sqrt2)/2 with infinities explicit. phi_inv uses statrs seed plus2 Newton steps. dinv linearly returns least k with CDF(k)>=p, p=1 returns kmax.

## approximant

### distributions: forward relation

Registry installs scalar-broadcast distributions. bcast checks arity/types then identical nonscalar shape, defaults scalar parameters. phi=erfc(-z/sqrt2)/2 with infinities explicit. phi_inv uses statrs seed plus2 Newton steps. dinv linearly returns least k with CDF(k)>=p, p=1 returns kmax.

Domain: Finite, well-shaped bounded observations unless branch explicitly describes NaN/Inf. Direct public chi2/OLS helpers assume dimensions not checked at every index; wrappers add validation.

Executable surrogate gap: no measured cheaper implementation with a useful scoped metric is supplied. Categorical/error/effect relations are exact obligations, not percent losses. Future numeric caps only1/2/5/10%.

## invariant

### distributions: invariant and cost

Scalar parameters broadcast to first nonscalar shape, not general singleton-axis broadcasting.

libm erfc avoids 1-erf cancellation; numerical implementation not a universal accuracy certificate.

Scope-specific cost: Broadcast O(ncells*nparams + distribution evaluation), discrete inverse O(returned k) CDF calls without bounded cap; phi_inv two Newton steps.

## models

## behavior_complement

Invalid p excludes finite inverse except NaN mapping; continuous unsupported parameters become NaN, not interpreter errors. Discrete p=1 can yield infinity.

## reverse_reconstruction

CDF observation admits parameters and x satisfying F_theta(x)=p; discrete quantile admits interval F(k-1)<p<=F(k), endpoint conventions special.

## broadcast_shape

Scalar parameters broadcast to first nonscalar shape, not general singleton-axis broadcasting.

## tail_precision

libm erfc avoids 1-erf cancellation; numerical implementation not a universal accuracy certificate.

## branch_relation

Registry installs scalar-broadcast distributions. bcast checks arity/types then identical nonscalar shape, defaults scalar parameters. phi=erfc(-z/sqrt2)/2 with infinities explicit. phi_inv uses statrs seed plus2 Newton steps. dinv linearly returns least k with CDF(k)>=p, p=1 returns kmax.

## resource_cost

Broadcast O(ncells*nparams + distribution evaluation), discrete inverse O(returned k) CDF calls without bounded cap; phi_inv two Newton steps.

## tests

### Evidence boundary

See ../model.md#tests for the actual selected source test and mutant. These tests sample the file, not every fragment branch. Invalid p excludes finite inverse except NaN mapping; continuous unsupported parameters become NaN, not interpreter errors. Discrete p=1 can yield infinity. Other branches here are source-derived and unexecuted unless the parent test explicitly calls them.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: isolated source-bound fragment over bytes [907,7891); includes no claim beyond this relation. Source hash bdc3772b09d9113e108afa29e391fee97a5372caf9052bb1689996c41c94116f.
