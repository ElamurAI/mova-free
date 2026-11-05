# mlab-table

## descriptant

### mlab-table

Column-oriented Table names, Value columns and ColKind annotations, row indices and civil-day coordinates.

dates: Civil-day conversion maps Unix day0 to719529. datenum_ymd carries month and adds day offset; parse_iso validates calendar roundtrip but accepts unbounded parsed clock values. datestr rounds fractional day to seconds and carries midnight; nonfinite emits NaT.

columns: to_column rejects complex before char-row conversion or vector-shape check. make zips names/values, then name validity, duplicate, conversion, equal height. set_var broadcasts singleton to height and retains Date kind on numeric replacement.

keys: Ascending cmp orders NaN/None last and equal to themselves. Stable sorted_rows preserves tie indices; groups sorts by keys and restores appearance within each group. row_key returns None for any missing and canonicalizes signed zero.

display: cells_of formats by kind; format_body selects all rows <=30, otherwise head10 tail5, widths count Unicode chars and header. Text left aligns, others right; format_str_arr traverses column-major data into row text.

## approximant

### Forward approximant

dates: Civil-day conversion maps Unix day0 to719529. datenum_ymd carries month and adds day offset; parse_iso validates calendar roundtrip but accepts unbounded parsed clock values. datestr rounds fractional day to seconds and carries midnight; nonfinite emits NaT.

columns: to_column rejects complex before char-row conversion or vector-shape check. make zips names/values, then name validity, duplicate, conversion, equal height. set_var broadcasts singleton to height and retains Date kind on numeric replacement.

keys: Ascending cmp orders NaN/None last and equal to themselves. Stable sorted_rows preserves tie indices; groups sorts by keys and restores appearance within each group. row_key returns None for any missing and canonicalizes signed zero.

display: cells_of formats by kind; format_body selects all rows <=30, otherwise head10 tail5, widths count Unicode chars and header. Text left aligns, others right; format_str_arr traverses column-major data into row text.

#### Domain

Well-shaped public Table/Mat/StrArr, in-range zero-based internal indices, bounded i64 civil arithmetic. Malformed direct structs may panic.

#### Executable gap

No cheaper Rust surrogate has been measured for this source. Exact categorical/effect behavior is not a percentage metric; numerical surrogates need a separately specified domain, zero policy and supported1/2/5/10% cap. No speed or error-bound claim.

## invariant

### Invariants and cost

dates: Invalid February30 excludes Some date; hour24 is not excluded. Formatting is second-resolution and cannot preserve arbitrary fractional day bits.

columns: Failed set_var conversion/height excludes mutation; make does not enforce equal names/values lengths beyond zip. Logical var_list does not bounds-check true positions.

keys: Missing grouping does not imply missing join equality; join keys exclude missing. Numeric/text keys remain distinct tagged atoms.

display: Elided output excludes reconstruction of hidden rows; nonfinite Date displays NaT and string None displays <missing>. No ANSI width guarantee.

Column copies O(nw); stable key sort O(kn log n), groups plus per-group index sort; hashing keys O(k) plus text clone length. Display materializes all cells even though >30 rows shows15. Name validation in make costs O(w²) comparisons; dedupe suffix collisions can cost O(w³) string comparisons plus text work.

Allocations can fail; public malformed shapes and extreme integer arithmetic are outside the bounded checked domain. Floating calculations retain their source-specific NaN and tie policies; no invented global finite guarantee.

## composition

### Scoped composition

dates: Civil-day conversion maps Unix day0 to719529. datenum_ymd carries month and adds day offset; parse_iso validates calendar roundtrip but accepts unbounded parsed clock values. datestr rounds fractional day to seconds and carries midnight; nonfinite emits NaT.

columns: to_column rejects complex before char-row conversion or vector-shape check. make zips names/values, then name validity, duplicate, conversion, equal height. set_var broadcasts singleton to height and retains Date kind on numeric replacement.

keys: Ascending cmp orders NaN/None last and equal to themselves. Stable sorted_rows preserves tie indices; groups sorts by keys and restores appearance within each group. row_key returns None for any missing and canonicalizes signed zero.

display: cells_of formats by kind; format_body selects all rows <=30, otherwise head10 tail5, widths count Unicode chars and header. Text left aligns, others right; format_str_arr traverses column-major data into row text.

Children bind disjoint operation spans in the original source and share one syntax projection. Registration/reporting/glue between named spans remains represented by the parent projection.

dates: For datestr with a time-bearing pattern, an output admits datenum inputs rounding to the shown second, including midnight carry. iso_date first tests exact integrality: date-only output admits integral inputs only; nonintegral inputs may round to midnight but retain the datetime pattern. Valid civil roundtrip is unique only within the nonoverflow calendar domain.

columns: Table columns admit row/column vector antecedents normalized to column shape; char rows and strings can agree. Scalar broadcast erases original orientation.

keys: Sorted output admits any permutation of unequal-key source rows with corresponding index mapping; groups discard original inter-group order but retain member indices.

display: Display string admits many matrices differing beyond precision, elided middle rows or real literal <missing> text.

The mathematical combiner keeps source equations, branch exclusions, compatible antecedents and bounded tests distinct. Passing children does not prove caller preconditions; dimension, missingness and effect adapters must preserve their own contracts.

## models

## behavior_complement

dates: Invalid February30 excludes Some date; hour24 is not excluded. Formatting is second-resolution and cannot preserve arbitrary fractional day bits.

columns: Failed set_var conversion/height excludes mutation; make does not enforce equal names/values lengths beyond zip. Logical var_list does not bounds-check true positions.

keys: Missing grouping does not imply missing join equality; join keys exclude missing. Numeric/text keys remain distinct tagged atoms.

display: Elided output excludes reconstruction of hidden rows; nonfinite Date displays NaT and string None displays <missing>. No ANSI width guarantee.

## reverse_reconstruction

dates: For datestr with a time-bearing pattern, an output admits datenum inputs rounding to the shown second, including midnight carry. iso_date first tests exact integrality: date-only output admits integral inputs only; nonintegral inputs may round to midnight but retain the datetime pattern. Valid civil roundtrip is unique only within the nonoverflow calendar domain.

columns: Table columns admit row/column vector antecedents normalized to column shape; char rows and strings can agree. Scalar broadcast erases original orientation.

keys: Sorted output admits any permutation of unequal-key source rows with corresponding index mapping; groups discard original inter-group order but retain member indices.

display: Display string admits many matrices differing beyond precision, elided middle rows or real literal <missing> text.

## representation_transport

calendar_transport: days_from_civil/civil_from_days form inverse on valid supported calendar dates; ISO parser additionally requires4-digit year layout.
clock_boundary: Time values are floating parsed, not range checked; timezone suffix unsupported; date range checked before calendar roundtrip.

column_transport: make has no scalar broadcast; set_var does. zero columns means height0. Direct take_rows/select_vars require valid indices.
name_precedence: Unicode alphabetic/underscore name starts accepted; make_valid_name prefixes x for leading underscore unlike valid_name.

missing_relation: NaNs group together but produce no hash key; +0 and -0 share numeric key0.
stable_ties: One-key numeric fast path adds original index as tie breaker; multi-key stable sort relies on sort stability.

display_cost: All cells generated before selection, so elision reduces output not source-format work.
layout_transport: Names and shown widths determine pad; trailing spaces stripped, four-space indent and three-space separators.

## failure_precedence

dates: Invalid February30 excludes Some date; hour24 is not excluded. Formatting is second-resolution and cannot preserve arbitrary fractional day bits.

columns: Failed set_var conversion/height excludes mutation; make does not enforce equal names/values lengths beyond zip. Logical var_list does not bounds-check true positions.

keys: Missing grouping does not imply missing join equality; join keys exclude missing. Numeric/text keys remain distinct tagged atoms.

display: Elided output excludes reconstruction of hidden rows; nonfinite Date displays NaT and string None displays <missing>. No ANSI width guarantee.

## resource_cost

Column copies O(nw); stable key sort O(kn log n), groups plus per-group index sort; hashing keys O(k) plus text clone length. Display materializes all cells even though >30 rows shows15. Name validation in make costs O(w²) comparisons; dedupe suffix collisions can cost O(w³) string comparisons plus text work.

## composition_relation

dates: Civil-day conversion maps Unix day0 to719529. datenum_ymd carries month and adds day offset; parse_iso validates calendar roundtrip but accepts unbounded parsed clock values. datestr rounds fractional day to seconds and carries midnight; nonfinite emits NaT.

columns: to_column rejects complex before char-row conversion or vector-shape check. make zips names/values, then name validity, duplicate, conversion, equal height. set_var broadcasts singleton to height and retains Date kind on numeric replacement.

keys: Ascending cmp orders NaN/None last and equal to themselves. Stable sorted_rows preserves tie indices; groups sorts by keys and restores appearance within each group. row_key returns None for any missing and canonicalizes signed zero.

display: cells_of formats by kind; format_body selects all rows <=30, otherwise head10 tail5, widths count Unicode chars and header. Text left aligns, others right; format_str_arr traverses column-major data into row text.

## tests

### Bounded evidence

Selected actual-source test: table::wave30::dates_keys_and_columns. It uses only synthetic values. Retained native-commands.json specifies exact cargo selection; source-test-map.json maps logs, profiles and controls externally. Paired debug/release observations and compiled mutant status are recorded in receipt.json only after execution.

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
