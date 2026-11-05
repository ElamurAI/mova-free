# mlab-ext-boost

## descriptant

### mlab-ext-boost

Frozen file rows, interpreter text outputs and Stat(n,max_eps,worst_x).

parser: table searches exact array header and end delimiter, parses SC_ tokens; mlab_eval builds numeric vector, calls Interp::capture/run, parses Inf/-Inf/NaN and enforces output count.

metric: measure retains normal reference values only; e=abs((got-want)/want)/EPSILON with NaN got=>infinity, strict greater updates first worst. Existing CASES bound4eps; synthetic tests do not certify those datasets.

## approximant

### Forward approximant

parser: table searches exact array header and end delimiter, parses SC_ tokens; mlab_eval builds numeric vector, calls Interp::capture/run, parses Inf/-Inf/NaN and enforces output count.

metric: measure retains normal reference values only; e=abs((got-want)/want)/EPSILON with NaN got=>infinity, strict greater updates first worst. Existing CASES bound4eps; synthetic tests do not certify those datasets.

#### Domain

Synthetic bounded rectangular rows with valid col index; dataset suites deliberately not executed.

#### Executable gap

No cheaper Rust surrogate has been measured for this source. Exact categorical/effect behavior is not a percentage metric; numerical surrogates need a separately specified domain, zero policy and supported1/2/5/10% cap. No speed or error-bound claim.

## invariant

### Invariants and cost

parser: Missing array/file or malformed numeric token panics; no graceful empty fallback except no matching SC_ lines. Output length mismatch excludes successful helper return.

metric: Zero/subnormal/NaN/infinite reference values do not contribute. max_eps0 leaves worst_xNaN; ties do not update worst.

Parser O(bytes); interpreter work depends on function and point count; measure O(n) plus generated code/rows. No files loaded in selected test.

Allocations can fail; public malformed shapes and extreme integer arithmetic are outside the bounded checked domain. Floating calculations retain their source-specific NaN and tie policies; no invented global finite guarantee.

## composition

### Scoped composition

parser: table searches exact array header and end delimiter, parses SC_ tokens; mlab_eval builds numeric vector, calls Interp::capture/run, parses Inf/-Inf/NaN and enforces output count.

metric: measure retains normal reference values only; e=abs((got-want)/want)/EPSILON with NaN got=>infinity, strict greater updates first worst. Existing CASES bound4eps; synthetic tests do not certify those datasets.

Children bind disjoint operation spans in the original source and share one syntax projection. Registration/reporting/glue between named spans remains represented by the parent projection.

parser: Parsed rows admit whitespace and ignored text variants; emitted numeric output admits code representations rounding to same f64.

metric: Stat admits all row sets with same normal count and maximum relative discrepancy, including arbitrary excluded rows; worst_x alone not full error curve.

The mathematical combiner keeps source equations, branch exclusions, compatible antecedents and bounded tests distinct. Passing children does not prove caller preconditions; dimension, missingness and effect adapters must preserve their own contracts.

## models

## behavior_complement

parser: Missing array/file or malformed numeric token panics; no graceful empty fallback except no matching SC_ lines. Output length mismatch excludes successful helper return.

metric: Zero/subnormal/NaN/infinite reference values do not contribute. max_eps0 leaves worst_xNaN; ties do not update worst.

## reverse_reconstruction

parser: Parsed rows admit whitespace and ignored text variants; emitted numeric output admits code representations rounding to same f64.

metric: Stat admits all row sets with same normal count and maximum relative discrepancy, including arbitrary excluded rows; worst_x alone not full error curve.

## representation_transport

effect_boundary: table performs host filesystem read under fixed dataset directory and is unexecuted; selected mlab_eval in memory.
numeric_transport: Exponent formatting and %.17g output support finite roundtrip but error output panics rather than silently filtering.

zero_metric: Relative metric is undefined at0 and deliberately excluded; this is not a zero-reference accuracy claim.
fault_witness: erf fault adds1 to first output; synthetic erf0 confirms hook, without running frozen Boost points.

## failure_precedence

parser: Missing array/file or malformed numeric token panics; no graceful empty fallback except no matching SC_ lines. Output length mismatch excludes successful helper return.

metric: Zero/subnormal/NaN/infinite reference values do not contribute. max_eps0 leaves worst_xNaN; ties do not update worst.

## resource_cost

Parser O(bytes); interpreter work depends on function and point count; measure O(n) plus generated code/rows. No files loaded in selected test.

## composition_relation

parser: table searches exact array header and end delimiter, parses SC_ tokens; mlab_eval builds numeric vector, calls Interp::capture/run, parses Inf/-Inf/NaN and enforces output count.

metric: measure retains normal reference values only; e=abs((got-want)/want)/EPSILON with NaN got=>infinity, strict greater updates first worst. Existing CASES bound4eps; synthetic tests do not certify those datasets.

## tests

### Bounded evidence

Selected actual-source test: wave30_synthetic_measure. It uses only synthetic values. Retained native-commands.json specifies exact cargo selection; source-test-map.json maps logs, profiles and controls externally. Paired debug/release observations and compiled mutant status are recorded in receipt.json only after execution.

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
