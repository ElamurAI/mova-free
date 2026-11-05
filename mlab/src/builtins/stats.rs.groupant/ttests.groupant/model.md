# mlab-builtins-stats-ttests

## descriptant

### mlab-builtins-stats-ttests

test_opts checks Alpha before Tail; t_one removes NaNs, computes unbiased SD and SE, t=(mean-m0)/SE. t_two uses pooled or Welch variance. t_p guards NaN/nonpositive df then uses survival tail. Wrappers extract arguments first; ttest rejects incompatible paired lengths before options. Both wrappers parse options before checking the minimum cleaned sample count; verdict uses strict p<alpha.

## approximant

### ttests: forward relation

test_opts checks Alpha before Tail; t_one removes NaNs, computes unbiased SD and SE, t=(mean-m0)/SE. t_two uses pooled or Welch variance. t_p guards NaN/nonpositive df then uses survival tail. Wrappers extract arguments first; ttest rejects incompatible paired lengths before options. Both wrappers parse options before checking the minimum cleaned sample count; verdict uses strict p<alpha.

Domain: Finite, well-shaped bounded observations unless branch explicitly describes NaN/Inf. Direct public chi2/OLS helpers assume dimensions not checked at every index; wrappers add validation.

Executable surrogate gap: no measured cheaper implementation with a useful scoped metric is supplied. Categorical/error/effect relations are exact obligations, not percent losses. Future numeric caps only1/2/5/10%.

## invariant

### ttests: invariant and cost

Left(t)=Right(-t), Both=2SF(abs(t)) capped1; fault substitutes Normal only.

Both CI uses tcrit(1-alpha/2); one-sided CI has one infinite endpoint. Reports do not validate independence/normality.

Scope-specific cost: T cores O(n+m) time and cleaned copies, option scan O(number of pairs), constant-size result and optional report.

## models

## behavior_complement

NaN p does not reject H0; printed nonreject text is not proof p>=alpha. Both-tail symmetric p cannot reveal t sign. Invalid option precedes insufficient-sample error.

## reverse_reconstruction

Given t,df,p, compatible samples include all location/scale transforms preserving standardized effect; t alone loses raw observations. Paired differences admit common offsets to x,y.

## tail_duality

Left(t)=Right(-t), Both=2SF(abs(t)) capped1; fault substitutes Normal only.

## confidence_interval

Both CI uses tcrit(1-alpha/2); one-sided CI has one infinite endpoint. Reports do not validate independence/normality.

## branch_relation

test_opts checks Alpha before Tail; t_one removes NaNs, computes unbiased SD and SE, t=(mean-m0)/SE. t_two uses pooled or Welch variance. t_p guards NaN/nonpositive df then uses survival tail. Wrappers extract arguments first; ttest rejects incompatible paired lengths before options. Both wrappers parse options before checking the minimum cleaned sample count; verdict uses strict p<alpha.

## resource_cost

T cores O(n+m) time and cleaned copies, option scan O(number of pairs), constant-size result and optional report.

## tests

### Evidence boundary

See ../model.md#tests for the actual selected source test and mutant. These tests sample the file, not every fragment branch. NaN p does not reject H0; printed nonreject text is not proof p>=alpha. Both-tail symmetric p cannot reveal t sign. Invalid option precedes insufficient-sample error. Other branches here are source-derived and unexecuted unless the parent test explicitly calls them.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: isolated source-bound fragment over bytes [15577,25404); includes no claim beyond this relation. Source hash bdc3772b09d9113e108afa29e391fee97a5372caf9052bb1689996c41c94116f.
