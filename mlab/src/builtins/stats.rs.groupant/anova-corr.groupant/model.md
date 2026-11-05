# mlab-builtins-stats-anova-corr

## descriptant

### mlab-builtins-stats-anova-corr

ANOVA shifts by first observation, corrected second-pass means, SSB=sum ni(mi-gm)^2, SSW=sum residual², F=(SSB/(k-1))/(SSW/(N-k)). corr uses covariance normalization then t with n-2 df.

## approximant

### anova-corr: forward relation

ANOVA shifts by first observation, corrected second-pass means, SSB=sum ni(mi-gm)^2, SSW=sum residual², F=(SSB/(k-1))/(SSW/(N-k)). corr uses covariance normalization then t with n-2 df.

Domain: Finite, well-shaped bounded observations unless branch explicitly describes NaN/Inf. Direct public chi2/OLS helpers assume dimensions not checked at every index; wrappers add validation.

Executable surrogate gap: no measured cheaper implementation with a useful scoped metric is supplied. Categorical/error/effect relations are exact obligations, not percent losses. Future numeric caps only1/2/5/10%.

## invariant

### anova-corr: invariant and cost

k>=2 nonempty wrapper validation still permits N=k, so zero error df remains possible.

corr checks row equality before vector adaptation; empty-column and mismatched row-vector lengths remain unsupported direct cases.

Scope-specific cost: ANOVA O(N) copies and sums; corr O(n*p*q) covariance work and O(p*q) outputs, clones temporary observation columns.

## models

## behavior_complement

Centering reduces cancellation but cannot recover already rounded input differences. corr options Tail does not change two-sided corr_p.

## reverse_reconstruction

SSB/SSW admit many group permutations/within-group permutations and common shifts; correlation admits positive independent affine scales and offsets.

## anova_degrees

k>=2 nonempty wrapper validation still permits N=k, so zero error df remains possible.

## correlation_shape

corr checks row equality before vector adaptation; empty-column and mismatched row-vector lengths remain unsupported direct cases.

## branch_relation

ANOVA shifts by first observation, corrected second-pass means, SSB=sum ni(mi-gm)^2, SSW=sum residual², F=(SSB/(k-1))/(SSW/(N-k)). corr uses covariance normalization then t with n-2 df.

## resource_cost

ANOVA O(N) copies and sums; corr O(n*p*q) covariance work and O(p*q) outputs, clones temporary observation columns.

## tests

### Evidence boundary

See ../model.md#tests for the actual selected source test and mutant. These tests sample the file, not every fragment branch. Centering reduces cancellation but cannot recover already rounded input differences. corr options Tail does not change two-sided corr_p. Other branches here are source-derived and unexecuted unless the parent test explicitly calls them.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: isolated source-bound fragment over bytes [29854,37102); includes no claim beyond this relation. Source hash bdc3772b09d9113e108afa29e391fee97a5372caf9052bb1689996c41c94116f.
