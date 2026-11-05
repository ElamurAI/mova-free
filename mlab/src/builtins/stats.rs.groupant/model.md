# mlab-builtins-stats

## descriptant

### mlab-builtins-stats

Numeric matrices, pure test result records, interpreter display/fault flags and result tables.

distributions: Registry installs scalar-broadcast distributions. bcast checks arity/types then identical nonscalar shape, defaults scalar parameters. phi=erfc(-z/sqrt2)/2 with infinities explicit. phi_inv uses statrs seed plus2 Newton steps. dinv linearly returns least k with CDF(k)>=p, p=1 returns kmax.

descriptive: quantile cleans NaNs, sorts then interpolates pos=np+.5 with endpoints clamped; mode delegates table agg. moments drops NaN, covariance does not. cov scalar0/1 second arg treated normalization when first numel>1. histogram final edge included in last bin.

ttests: test_opts checks Alpha before Tail; t_one removes NaNs, computes unbiased SD and SE, t=(mean-m0)/SE. t_two uses pooled or Welch variance. t_p guards NaN/nonpositive df then uses survival tail. Wrappers extract arguments first; ttest rejects incompatible paired lengths before options. Both wrappers parse options before checking the minimum cleaned sample count; verdict uses strict p<alpha.

contingency: chi2_ind takes rectangular nonempty counts, computes marginals and rejects <=0 totals; Eij=ri*cj/N, statistic sum(O-E)^2/E, df=(r-1)(c-1)+fault. crosstab categorizes using table groups, counting missing category as group.

anova-corr: ANOVA shifts by first observation, corrected second-pass means, SSB=sum ni(mi-gm)^2, SSW=sum residual², F=(SSB/(k-1))/(SSW/(N-k)). corr uses covariance normalization then t with n-2 df.

ols: ols rejects n<=p, then Householder QR with rank threshold1e-12*max running norm; solves upper triangle and inverse for SE, residual metrics. fitlm drops rows with any NaN, prepends ones optionally, compares beta against mldivide relative norm with floor1e-300 and warns above1e-8.

## approximant

### Forward approximant

distributions: Registry installs scalar-broadcast distributions. bcast checks arity/types then identical nonscalar shape, defaults scalar parameters. phi=erfc(-z/sqrt2)/2 with infinities explicit. phi_inv uses statrs seed plus2 Newton steps. dinv linearly returns least k with CDF(k)>=p, p=1 returns kmax.

descriptive: quantile cleans NaNs, sorts then interpolates pos=np+.5 with endpoints clamped; mode delegates table agg. moments drops NaN, covariance does not. cov scalar0/1 second arg treated normalization when first numel>1. histogram final edge included in last bin.

ttests: test_opts checks Alpha before Tail; t_one removes NaNs, computes unbiased SD and SE, t=(mean-m0)/SE. t_two uses pooled or Welch variance. t_p guards NaN/nonpositive df then uses survival tail. Wrappers extract arguments first; ttest rejects incompatible paired lengths before options. Both wrappers parse options before checking the minimum cleaned sample count; verdict uses strict p<alpha.

contingency: chi2_ind takes rectangular nonempty counts, computes marginals and rejects <=0 totals; Eij=ri*cj/N, statistic sum(O-E)^2/E, df=(r-1)(c-1)+fault. crosstab categorizes using table groups, counting missing category as group.

anova-corr: ANOVA shifts by first observation, corrected second-pass means, SSB=sum ni(mi-gm)^2, SSW=sum residual², F=(SSB/(k-1))/(SSW/(N-k)). corr uses covariance normalization then t with n-2 df.

ols: ols rejects n<=p, then Householder QR with rank threshold1e-12*max running norm; solves upper triangle and inverse for SE, residual metrics. fitlm drops rows with any NaN, prepends ones optionally, compares beta against mldivide relative norm with floor1e-300 and warns above1e-8.

#### Domain

Finite, well-shaped bounded observations unless branch explicitly describes NaN/Inf. Direct public chi2/OLS helpers assume dimensions not checked at every index; wrappers add validation.

#### Executable gap

No cheaper Rust surrogate has been measured for this source. Exact categorical/effect behavior is not a percentage metric; numerical surrogates need a separately specified domain, zero policy and supported1/2/5/10% cap. No speed or error-bound claim.

## invariant

### Invariants and cost

distributions: Invalid p excludes finite inverse except NaN mapping; continuous unsupported parameters become NaN, not interpreter errors. Discrete p=1 can yield infinity.

descriptive: NaN-clean moments cannot reconstruct missing count. corrcoef empty produces NaN; this does not guarantee cov empty safety (cs[0]). Invalid quantile probabilities fail before sorting.

ttests: NaN p does not reject H0; printed nonreject text is not proof p>=alpha. Both-tail symmetric p cannot reveal t sign. Invalid option precedes insufficient-sample error.

contingency: Positive marginals do not ensure nonnegative cells; direct core does not validate that. crosstab degenerate category dimensions returns chi2=0,p=1.

anova-corr: Centering reduces cancellation but cannot recover already rounded input differences. corr options Tail does not change two-sided corr_p.

ols: Gate warning does not prevent returning coefficients. No unique inverse from beta to dataset; exact fit can produce zero SE and infinite t. Malformed column lengths can panic.

Descriptive sorts O(n log n); pairwise covariance O(np²); t/ANOVA O(n); chi2 O(rc); OLS Householder O(np²+p³), memory O(np+p²). Discrete inverse linear in returned quantile, without fixed small cap.

Allocations can fail; public malformed shapes and extreme integer arithmetic are outside the bounded checked domain. Floating calculations retain their source-specific NaN and tie policies; no invented global finite guarantee.

## composition

### Scoped composition

distributions: Registry installs scalar-broadcast distributions. bcast checks arity/types then identical nonscalar shape, defaults scalar parameters. phi=erfc(-z/sqrt2)/2 with infinities explicit. phi_inv uses statrs seed plus2 Newton steps. dinv linearly returns least k with CDF(k)>=p, p=1 returns kmax.

descriptive: quantile cleans NaNs, sorts then interpolates pos=np+.5 with endpoints clamped; mode delegates table agg. moments drops NaN, covariance does not. cov scalar0/1 second arg treated normalization when first numel>1. histogram final edge included in last bin.

ttests: test_opts checks Alpha before Tail; t_one removes NaNs, computes unbiased SD and SE, t=(mean-m0)/SE. t_two uses pooled or Welch variance. t_p guards NaN/nonpositive df then uses survival tail. Wrappers extract arguments first; ttest rejects incompatible paired lengths before options. Both wrappers parse options before checking the minimum cleaned sample count; verdict uses strict p<alpha.

contingency: chi2_ind takes rectangular nonempty counts, computes marginals and rejects <=0 totals; Eij=ri*cj/N, statistic sum(O-E)^2/E, df=(r-1)(c-1)+fault. crosstab categorizes using table groups, counting missing category as group.

anova-corr: ANOVA shifts by first observation, corrected second-pass means, SSB=sum ni(mi-gm)^2, SSW=sum residual², F=(SSB/(k-1))/(SSW/(N-k)). corr uses covariance normalization then t with n-2 df.

ols: ols rejects n<=p, then Householder QR with rank threshold1e-12*max running norm; solves upper triangle and inverse for SE, residual metrics. fitlm drops rows with any NaN, prepends ones optionally, compares beta against mldivide relative norm with floor1e-300 and warns above1e-8.

Children bind disjoint operation spans in the original source and share one syntax projection. Registration/reporting/glue between named spans remains represented by the parent projection.

distributions: CDF observation admits parameters and x satisfying F_theta(x)=p; discrete quantile admits interval F(k-1)<p<=F(k), endpoint conventions special.

descriptive: Quantiles admit sorted arrays sharing interpolated positions and arbitrary source permutations/NaNs. Covariance admits translated observation sets and rotational symmetries.

ttests: Given t,df,p, compatible samples include all location/scale transforms preserving standardized effect; t alone loses raw observations. Paired differences admit common offsets to x,y.

contingency: Expected table admits many observed tables with same marginals; chi2 scalar further collapses dimensional arrangement. Category indices admit all value relabelings preserving sort order.

anova-corr: SSB/SSW admit many group permutations/within-group permutations and common shifts; correlation admits positive independent affine scales and offsets.

ols: beta compatible with every (X,y) whose finite QR solve yields beta, including y=Xbeta+residual orthogonal to design; original dropped NaN rows unidentifiable.

The mathematical combiner keeps source equations, branch exclusions, compatible antecedents and bounded tests distinct. Passing children does not prove caller preconditions; dimension, missingness and effect adapters must preserve their own contracts.

## models

## behavior_complement

distributions: Invalid p excludes finite inverse except NaN mapping; continuous unsupported parameters become NaN, not interpreter errors. Discrete p=1 can yield infinity.

descriptive: NaN-clean moments cannot reconstruct missing count. corrcoef empty produces NaN; this does not guarantee cov empty safety (cs[0]). Invalid quantile probabilities fail before sorting.

ttests: NaN p does not reject H0; printed nonreject text is not proof p>=alpha. Both-tail symmetric p cannot reveal t sign. Invalid option precedes insufficient-sample error.

contingency: Positive marginals do not ensure nonnegative cells; direct core does not validate that. crosstab degenerate category dimensions returns chi2=0,p=1.

anova-corr: Centering reduces cancellation but cannot recover already rounded input differences. corr options Tail does not change two-sided corr_p.

ols: Gate warning does not prevent returning coefficients. No unique inverse from beta to dataset; exact fit can produce zero SE and infinite t. Malformed column lengths can panic.

## reverse_reconstruction

distributions: CDF observation admits parameters and x satisfying F_theta(x)=p; discrete quantile admits interval F(k-1)<p<=F(k), endpoint conventions special.

descriptive: Quantiles admit sorted arrays sharing interpolated positions and arbitrary source permutations/NaNs. Covariance admits translated observation sets and rotational symmetries.

ttests: Given t,df,p, compatible samples include all location/scale transforms preserving standardized effect; t alone loses raw observations. Paired differences admit common offsets to x,y.

contingency: Expected table admits many observed tables with same marginals; chi2 scalar further collapses dimensional arrangement. Category indices admit all value relabelings preserving sort order.

anova-corr: SSB/SSW admit many group permutations/within-group permutations and common shifts; correlation admits positive independent affine scales and offsets.

ols: beta compatible with every (X,y) whose finite QR solve yields beta, including y=Xbeta+residual orthogonal to design; original dropped NaN rows unidentifiable.

## representation_transport

broadcast_shape: Scalar parameters broadcast to first nonscalar shape, not general singleton-axis broadcasting.
tail_precision: libm erfc avoids 1-erf cancellation; numerical implementation not a universal accuracy certificate.

quantile_coordinates: s=[1,3], p=.5 gives2; p=0 gives1; empty givesNaN.
covariance_policy: N or N-1 denominator selected by cov weight; constant diagonal corrcoef forced1, off-diagonal mayNaN.

tail_duality: Left(t)=Right(-t), Both=2SF(abs(t)) capped1; fault substitutes Normal only.
confidence_interval: Both CI uses tcrit(1-alpha/2); one-sided CI has one infinite endpoint. Reports do not validate independence/normality.

marginal_transport: Expected row/column sums reproduce observed marginals under finite arithmetic up to rounding.
category_missing: NaN categories group, unlike missing join keys; wrapper chi2test requires>=2x2 before options.

anova_degrees: k>=2 nonempty wrapper validation still permits N=k, so zero error df remains possible.
correlation_shape: corr checks row equality before vector adaptation; empty-column and mismatched row-vector lengths remain unsupported direct cases.

rank_boundary: Rank judged without pivoting and running norm; first n<=p failure excludes QR iteration.
formula_transport: parse_formula admits additive names, 1/-1/0 toggles intercept; repeated names retained and can cause rank deficiency; formula validates response last.

## failure_precedence

distributions: Invalid p excludes finite inverse except NaN mapping; continuous unsupported parameters become NaN, not interpreter errors. Discrete p=1 can yield infinity.

descriptive: NaN-clean moments cannot reconstruct missing count. corrcoef empty produces NaN; this does not guarantee cov empty safety (cs[0]). Invalid quantile probabilities fail before sorting.

ttests: NaN p does not reject H0; printed nonreject text is not proof p>=alpha. Both-tail symmetric p cannot reveal t sign. Invalid option precedes insufficient-sample error.

contingency: Positive marginals do not ensure nonnegative cells; direct core does not validate that. crosstab degenerate category dimensions returns chi2=0,p=1.

anova-corr: Centering reduces cancellation but cannot recover already rounded input differences. corr options Tail does not change two-sided corr_p.

ols: Gate warning does not prevent returning coefficients. No unique inverse from beta to dataset; exact fit can produce zero SE and infinite t. Malformed column lengths can panic.

## resource_cost

Descriptive sorts O(n log n); pairwise covariance O(np²); t/ANOVA O(n); chi2 O(rc); OLS Householder O(np²+p³), memory O(np+p²). Discrete inverse linear in returned quantile, without fixed small cap.

## composition_relation

distributions: Registry installs scalar-broadcast distributions. bcast checks arity/types then identical nonscalar shape, defaults scalar parameters. phi=erfc(-z/sqrt2)/2 with infinities explicit. phi_inv uses statrs seed plus2 Newton steps. dinv linearly returns least k with CDF(k)>=p, p=1 returns kmax.

descriptive: quantile cleans NaNs, sorts then interpolates pos=np+.5 with endpoints clamped; mode delegates table agg. moments drops NaN, covariance does not. cov scalar0/1 second arg treated normalization when first numel>1. histogram final edge included in last bin.

ttests: test_opts checks Alpha before Tail; t_one removes NaNs, computes unbiased SD and SE, t=(mean-m0)/SE. t_two uses pooled or Welch variance. t_p guards NaN/nonpositive df then uses survival tail. Wrappers extract arguments first; ttest rejects incompatible paired lengths before options. Both wrappers parse options before checking the minimum cleaned sample count; verdict uses strict p<alpha.

contingency: chi2_ind takes rectangular nonempty counts, computes marginals and rejects <=0 totals; Eij=ri*cj/N, statistic sum(O-E)^2/E, df=(r-1)(c-1)+fault. crosstab categorizes using table groups, counting missing category as group.

anova-corr: ANOVA shifts by first observation, corrected second-pass means, SSB=sum ni(mi-gm)^2, SSW=sum residual², F=(SSB/(k-1))/(SSW/(N-k)). corr uses covariance normalization then t with n-2 df.

ols: ols rejects n<=p, then Householder QR with rank threshold1e-12*max running norm; solves upper triangle and inverse for SE, residual metrics. fitlm drops rows with any NaN, prepends ones optionally, compares beta against mldivide relative norm with floor1e-300 and warns above1e-8.

## tests

### Bounded evidence

Selected actual-source test: builtins::stats::wave30::cores_and_failure_order. It uses only synthetic values. Retained native-commands.json specifies exact cargo selection; source-test-map.json maps logs, profiles and controls externally. Paired debug/release observations and compiled mutant status are recorded in receipt.json only after execution.

No Boost/NIST data suite, default Monte Carlo calibration, filesystem dataset loading, GPU or benchmark was run. The test samples a subset of source branches; fragment prose remains source-derived for all uncalled APIs. No Clippy cleanliness claim: broad dependency lints outside owned sources not addressed.

## prospective

### Prospective statistics

Read all seven sources before editing. CPU synthetic checks only; no external Boost/NIST data, default Monte Carlo, GPU, live files, teachers or corpus. No speed claim or production algorithm change planned.

- stats.rs: distribution broadcast validates arity then matrix shapes; quantile cleans NaNs and interpolates at (k-.5)/n; t cores clean NaNs, wrappers validate sample sizes/options; chi2 core assumes nonempty rectangular rows, rejects nonpositive marginals; ANOVA centers on first sample and corrects means; Householder OLS rejects n<=p before rank checks then computes coefficient tables and compares against mldivide. Tests predict median 2, empty NaN, t=0 p=1 on symmetric sample, chi2=0 p=1 for independent counts, exact line coefficients within rounding, singular rejection. Risks: infinity arithmetic, malformed direct core shapes and rank tolerance remain partial.
- builtins/table.rs: options lowercase with last duplicate winning; agg drops NaN except count/nummissing, ties choose smallest; grouped table assembly uses stable key groups; joins hash right rows then stable key sort, strict join rejects duplicate right before missing left. Moving windows shrink endpoints, scalar even length looks backward more; file calls validate types/options then route through it.fs. Tests cover aggregate ties/missing, option overwrite, synthetic innerjoin and strict duplicate rejection; real FS remains unexecuted.
- table.rs: civil-day conversion offset 719529; ISO calendar dates round-trip validate but hour/minute/seconds are parsed without bounds; names normalize then dedupe; make validates name/duplicate/conversion/height in that order; scalar set_var broadcasts; sorting places missing last ascending and groups missing together, while row_key rejects missing and normalizes signed zero. Tests cover this important grouping-vs-join non-equivalence, dates and transactional set_var failure. Costs sort O(n log n*k), columns/output O(nw), date O(1) except text pattern.
- rng.rs: splitmix expands seed, wrapping xoshiro transforms four words; uniform uses top53 bits and normal caches polar spare. Predict identical seeded stream, reseed discards spare, bounded uniforms; no fixed rejection-loop bound. Tests inspect private state to distinguish spare consumption.
- mcgate.rs: eight scenarios times four ALPHAS yield32 rows, denominator actual retained ps; k=0 yields NaN fraction/infinite tolerance and false ok. Tiny k=2 exercises real RNG/stats wiring without claiming calibration. Cost O(k) with fixed small scenario sizes and300 chi2 draws per replicate; Gaussian loop remains expected-only.
- ext_boost.rs: table reads frozen files (not executed); measure filters reference normal values, calls actual interpreter, reports relative error/EPSILON, strict greater selects first worst tie. Synthetic measure tests use erf(0)=0, erfc(0)=1 and fault; read parser paths remain gap.
- ext_nist.rs: data_rows finds one-based inclusive header span and numeric tokens, num accepts D exponent, after chooses first matching numeric line, lre equality precedes finite check and zero uses absolute error. Synthetic header/helper and interpreter matrix fixtures test actual helpers; certified regression/ANOVA/descriptive datasets remain unexecuted.

Planned controls mutate independent assertions/contracts in each source, compile successfully, observe selected failure, restore exact source bytes and rerun. Paired debug/release test names and statuses must match. Models remain partial and unsupported executable-surrogate gaps explicit.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Previously uncovered legacy source modeled using source-scoped children. Synthetic tests added without changing production behavior. Assertions measure selected branches, not universal correctness or distribution calibration.

### 2026-10-31 · prospective

Heading `### Prospective statistics epoch — 2026-10-31` stays in prospective as `### Prospective statistics`.
