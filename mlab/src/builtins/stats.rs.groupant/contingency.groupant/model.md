# mlab-builtins-stats-contingency

## descriptant

### mlab-builtins-stats-contingency

chi2_ind takes rectangular nonempty counts, computes marginals and rejects <=0 totals; Eij=ri*cj/N, statistic sum(O-E)^2/E, df=(r-1)(c-1)+fault. crosstab categorizes using table groups, counting missing category as group.

## approximant

### contingency: forward relation

chi2_ind takes rectangular nonempty counts, computes marginals and rejects <=0 totals; Eij=ri*cj/N, statistic sum(O-E)^2/E, df=(r-1)(c-1)+fault. crosstab categorizes using table groups, counting missing category as group.

Domain: Finite, well-shaped bounded observations unless branch explicitly describes NaN/Inf. Direct public chi2/OLS helpers assume dimensions not checked at every index; wrappers add validation.

Executable surrogate gap: no measured cheaper implementation with a useful scoped metric is supplied. Categorical/error/effect relations are exact obligations, not percent losses. Future numeric caps only1/2/5/10%.

## invariant

### contingency: invariant and cost

Expected row/column sums reproduce observed marginals under finite arithmetic up to rounding.

NaN categories group, unlike missing join keys; wrapper chi2test requires>=2x2 before options.

Scope-specific cost: O(r*c) time and expected matrix storage, plus category sorting O(n log n); invalid marginals fail before matrix fill.

## models

## behavior_complement

Positive marginals do not ensure nonnegative cells; direct core does not validate that. crosstab degenerate category dimensions returns chi2=0,p=1.

## reverse_reconstruction

Expected table admits many observed tables with same marginals; chi2 scalar further collapses dimensional arrangement. Category indices admit all value relabelings preserving sort order.

## marginal_transport

Expected row/column sums reproduce observed marginals under finite arithmetic up to rounding.

## category_missing

NaN categories group, unlike missing join keys; wrapper chi2test requires>=2x2 before options.

## branch_relation

chi2_ind takes rectangular nonempty counts, computes marginals and rejects <=0 totals; Eij=ri*cj/N, statistic sum(O-E)^2/E, df=(r-1)(c-1)+fault. crosstab categorizes using table groups, counting missing category as group.

## resource_cost

O(r*c) time and expected matrix storage, plus category sorting O(n log n); invalid marginals fail before matrix fill.

## tests

### Evidence boundary

See ../model.md#tests for the actual selected source test and mutant. These tests sample the file, not every fragment branch. Positive marginals do not ensure nonnegative cells; direct core does not validate that. crosstab degenerate category dimensions returns chi2=0,p=1. Other branches here are source-derived and unexecuted unless the parent test explicitly calls them.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: isolated source-bound fragment over bytes [25404,29854); includes no claim beyond this relation. Source hash bdc3772b09d9113e108afa29e391fee97a5372caf9052bb1689996c41c94116f.
