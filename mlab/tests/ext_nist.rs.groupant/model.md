# mlab-ext-nist

## descriptant

### mlab-ext-nist

NIST-like text, generated interpreter programs, LRE rows and acceptance thresholds.

helpers: data_rows locates Data+lines header, extracts first two integers and inclusive1-based span, keeps numeric tokens. num normalizes uppercaseD. after selects first matching line with parseable token scanning right. lre equality first=>15, nonfinite q=>0, zero c uses absolute error, otherwise relative, clamp0..15.

datasets: lls_case builds intercept/polynomial/multi-predictor design and checks fitlm plus backslash output length; ANOVA expects9 outputs and exact df; univ computes autocorrelation. suite aggregates threshold failures; detail env affects printing only.

## approximant

### Forward approximant

helpers: data_rows locates Data+lines header, extracts first two integers and inclusive1-based span, keeps numeric tokens. num normalizes uppercaseD. after selects first matching line with parseable token scanning right. lre equality first=>15, nonfinite q=>0, zero c uses absolute error, otherwise relative, clamp0..15.

datasets: lls_case builds intercept/polynomial/multi-predictor design and checks fitlm plus backslash output length; ANOVA expects9 outputs and exact df; univ computes autocorrelation. suite aggregates threshold failures; detail env affects printing only.

#### Domain

Well-formed one-based header spans and bounded synthetic data; frozen certified datasets unexecuted.

#### Executable gap

No cheaper Rust surrogate has been measured for this source. Exact categorical/effect behavior is not a percentage metric; numerical surrogates need a separately specified domain, zero policy and supported1/2/5/10% cap. No speed or error-bound claim.

## invariant

### Invariants and cost

helpers: Invalid span can panic; nonnumeric tokens skipped rather than rejected. Equal infinities yield15 by equality-before-finite, not a general finite accuracy claim.

datasets: Synthetic helper success does not certify Norris/Filip/Longley/ANOVA or UNIV thresholds. File-dependent negative controls remain unexecuted.

Parsing O(bytes) plus generated code; dataset regression design and numerical costs delegated to stats/linalg. Synthetic2x2 fixture only.

Allocations can fail; public malformed shapes and extreme integer arithmetic are outside the bounded checked domain. Floating calculations retain their source-specific NaN and tie policies; no invented global finite guarantee.

## composition

### Scoped composition

helpers: data_rows locates Data+lines header, extracts first two integers and inclusive1-based span, keeps numeric tokens. num normalizes uppercaseD. after selects first matching line with parseable token scanning right. lre equality first=>15, nonfinite q=>0, zero c uses absolute error, otherwise relative, clamp0..15.

datasets: lls_case builds intercept/polynomial/multi-predictor design and checks fitlm plus backslash output length; ANOVA expects9 outputs and exact df; univ computes autocorrelation. suite aggregates threshold failures; detail env affects printing only.

Children bind disjoint operation spans in the original source and share one syntax projection. Registration/reporting/glue between named spans remains represented by the parent projection.

helpers: data_rows output admits ignored labels and whitespace and arbitrary lines outside span; LRE cap15 admits exact match and sufficiently small errors.

datasets: Reported LRE rows admit many generated data sets and numerical errors; thresholds determine accepted region not unique reference values.

The mathematical combiner keeps source equations, branch exclusions, compatible antecedents and bounded tests distinct. Passing children does not prove caller preconditions; dimension, missingness and effect adapters must preserve their own contracts.

## models

## behavior_complement

helpers: Invalid span can panic; nonnumeric tokens skipped rather than rejected. Equal infinities yield15 by equality-before-finite, not a general finite accuracy claim.

datasets: Synthetic helper success does not certify Norris/Filip/Longley/ANOVA or UNIV thresholds. File-dependent negative controls remain unexecuted.

## reverse_reconstruction

helpers: data_rows output admits ignored labels and whitespace and arbitrary lines outside span; LRE cap15 admits exact match and sufficiently small errors.

datasets: Reported LRE rows admit many generated data sets and numerical errors; thresholds determine accepted region not unique reference values.

## representation_transport

header_coordinates: A=3,B=4 selects Rust lines[2..4], retaining both endpoints.
metric_zero: q=.001,c=0 yields3; q=infinity,c=1 yields0; min_lre empty starts15.

design_transport: One predictor degree depends on count of certified coefficients and B0; multiple predictors use original columns; no-intercept starts power1.
effect_boundary: read accesses fixed .dat paths; no selected test calls read, lls_case, anova_case, univ_case or dataset suite.

## failure_precedence

helpers: Invalid span can panic; nonnumeric tokens skipped rather than rejected. Equal infinities yield15 by equality-before-finite, not a general finite accuracy claim.

datasets: Synthetic helper success does not certify Norris/Filip/Longley/ANOVA or UNIV thresholds. File-dependent negative controls remain unexecuted.

## resource_cost

Parsing O(bytes) plus generated code; dataset regression design and numerical costs delegated to stats/linalg. Synthetic2x2 fixture only.

## composition_relation

helpers: data_rows locates Data+lines header, extracts first two integers and inclusive1-based span, keeps numeric tokens. num normalizes uppercaseD. after selects first matching line with parseable token scanning right. lre equality first=>15, nonfinite q=>0, zero c uses absolute error, otherwise relative, clamp0..15.

datasets: lls_case builds intercept/polynomial/multi-predictor design and checks fitlm plus backslash output length; ANOVA expects9 outputs and exact df; univ computes autocorrelation. suite aggregates threshold failures; detail env affects printing only.

## tests

### Bounded evidence

Selected actual-source test: wave30_synthetic_helpers. It uses only synthetic values. Retained native-commands.json specifies exact cargo selection; source-test-map.json maps logs, profiles and controls externally. Paired debug/release observations and compiled mutant status are recorded in receipt.json only after execution.

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
