# mlab-builtins-stats-descriptive

## descriptant

### mlab-builtins-stats-descriptive

quantile cleans NaNs, sorts then interpolates pos=np+.5 with endpoints clamped; mode delegates table agg. moments drops NaN, covariance does not. cov scalar0/1 second arg treated normalization when first numel>1. histogram final edge included in last bin.

## approximant

### descriptive: forward relation

quantile cleans NaNs, sorts then interpolates pos=np+.5 with endpoints clamped; mode delegates table agg. moments drops NaN, covariance does not. cov scalar0/1 second arg treated normalization when first numel>1. histogram final edge included in last bin.

Domain: Finite, well-shaped bounded observations unless branch explicitly describes NaN/Inf. Direct public chi2/OLS helpers assume dimensions not checked at every index; wrappers add validation.

Executable surrogate gap: no measured cheaper implementation with a useful scoped metric is supplied. Categorical/error/effect relations are exact obligations, not percent losses. Future numeric caps only1/2/5/10%.

## invariant

### descriptive: invariant and cost

s=[1,3], p=.5 gives2; p=0 gives1; empty givesNaN.

N or N-1 denominator selected by cov weight; constant diagonal corrcoef forced1, off-diagonal mayNaN.

Scope-specific cost: Quantile/mode O(n log n), moments O(n), covariance O(n*p²), histogram O(n log bins) using partition_point, storage O(n+p²).

## models

## behavior_complement

NaN-clean moments cannot reconstruct missing count. corrcoef empty produces NaN; this does not guarantee cov empty safety (cs[0]). Invalid quantile probabilities fail before sorting.

## reverse_reconstruction

Quantiles admit sorted arrays sharing interpolated positions and arbitrary source permutations/NaNs. Covariance admits translated observation sets and rotational symmetries.

## quantile_coordinates

s=[1,3], p=.5 gives2; p=0 gives1; empty givesNaN.

## covariance_policy

N or N-1 denominator selected by cov weight; constant diagonal corrcoef forced1, off-diagonal mayNaN.

## branch_relation

quantile cleans NaNs, sorts then interpolates pos=np+.5 with endpoints clamped; mode delegates table agg. moments drops NaN, covariance does not. cov scalar0/1 second arg treated normalization when first numel>1. histogram final edge included in last bin.

## resource_cost

Quantile/mode O(n log n), moments O(n), covariance O(n*p²), histogram O(n log bins) using partition_point, storage O(n+p²).

## tests

### Evidence boundary

See ../model.md#tests for the actual selected source test and mutant. These tests sample the file, not every fragment branch. NaN-clean moments cannot reconstruct missing count. corrcoef empty produces NaN; this does not guarantee cov empty safety (cs[0]). Invalid quantile probabilities fail before sorting. Other branches here are source-derived and unexecuted unless the parent test explicitly calls them.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: isolated source-bound fragment over bytes [7891,15577); includes no claim beyond this relation. Source hash bdc3772b09d9113e108afa29e391fee97a5372caf9052bb1689996c41c94116f.
