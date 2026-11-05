# mlab-builtins-stats-ols

## descriptant

### mlab-builtins-stats-ols

ols rejects n<=p, then Householder QR with rank threshold1e-12*max running norm; solves upper triangle and inverse for SE, residual metrics. fitlm drops rows with any NaN, prepends ones optionally, compares beta against mldivide relative norm with floor1e-300 and warns above1e-8.

## approximant

### ols: forward relation

ols rejects n<=p, then Householder QR with rank threshold1e-12*max running norm; solves upper triangle and inverse for SE, residual metrics. fitlm drops rows with any NaN, prepends ones optionally, compares beta against mldivide relative norm with floor1e-300 and warns above1e-8.

Domain: Finite, well-shaped bounded observations unless branch explicitly describes NaN/Inf. Direct public chi2/OLS helpers assume dimensions not checked at every index; wrappers add validation.

Executable surrogate gap: no measured cheaper implementation with a useful scoped metric is supplied. Categorical/error/effect relations are exact obligations, not percent losses. Future numeric caps only1/2/5/10%.

## invariant

### ols: invariant and cost

Rank judged without pivoting and running norm; first n<=p failure excludes QR iteration.

parse_formula admits additive names, 1/-1/0 toggles intercept; repeated names retained and can cause rank deficiency; formula validates response last.

Scope-specific cost: Householder O(n*p²) plus inverse O(p³), O(n*p+p²) memory; wrapper additionally solves independent mldivide and formats O(p) coefficient rows.

## models

## behavior_complement

Gate warning does not prevent returning coefficients. No unique inverse from beta to dataset; exact fit can produce zero SE and infinite t. Malformed column lengths can panic.

## reverse_reconstruction

beta compatible with every (X,y) whose finite QR solve yields beta, including y=Xbeta+residual orthogonal to design; original dropped NaN rows unidentifiable.

## rank_boundary

Rank judged without pivoting and running norm; first n<=p failure excludes QR iteration.

## formula_transport

parse_formula admits additive names, 1/-1/0 toggles intercept; repeated names retained and can cause rank deficiency; formula validates response last.

## branch_relation

ols rejects n<=p, then Householder QR with rank threshold1e-12*max running norm; solves upper triangle and inverse for SE, residual metrics. fitlm drops rows with any NaN, prepends ones optionally, compares beta against mldivide relative norm with floor1e-300 and warns above1e-8.

## resource_cost

Householder O(n*p²) plus inverse O(p³), O(n*p+p²) memory; wrapper additionally solves independent mldivide and formats O(p) coefficient rows.

## tests

### Evidence boundary

See ../model.md#tests for the actual selected source test and mutant. These tests sample the file, not every fragment branch. Gate warning does not prevent returning coefficients. No unique inverse from beta to dataset; exact fit can produce zero SE and infinite t. Malformed column lengths can panic. Other branches here are source-derived and unexecuted unless the parent test explicitly calls them.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: isolated source-bound fragment over bytes [37102,47343); includes no claim beyond this relation. Source hash bdc3772b09d9113e108afa29e391fee97a5372caf9052bb1689996c41c94116f.
