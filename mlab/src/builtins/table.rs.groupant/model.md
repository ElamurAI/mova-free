# mlab-builtins-table

## descriptant

### mlab-builtins-table

Interpreter argument names/output/filesystem, immutable Rc tables and copied result columns.

options: Register installs table/date/file entrypoints. opts consumes name/value pairs, lowercase ASCII, opt searches reversed so last wins. table consumes it.arg_names before validating VariableNames. summary drops numeric NaN for summaries and prints via it.out.

sorting: Table sort resolves names/directions then sorted_rows; matrix sort interprets signed columns via abs as usize, copies real parts and class; second output is one-based original row positions.

grouping: agg filters NaNs, count counts original length, mode sorted runs strict greater chooses smallest tie. groupsummary appends GroupCount and method columns; grouptransform scatters group results into original rows. pivot groups row/column keys, count empty0, other empty NaN.

joins: right_index stores nonmissing key rows in appearance order. innerjoin enumerates all matches then stable sorts by left keys. join rejects any duplicate right key before scanning left and requires every left row match. combine omits right keys, suffixes colliding names.

missing-windows: ismissing maps numeric NaN except char to logical mask; rmmissing removes missing vector cells or whole matrix/table rows. moving scalar even k uses kb=k/2,kf=k/2-1; endpoints shrink, pair widths max0 then cast.

effects: read/write wrappers route through it.fs and contextualize errors. readtable validates file kind before read; readmatrix reads before delimiter validation. datenum accepts string/date components; JSON encodes through fileio; string converts NaN to missing.

## approximant

### Forward approximant

options: Register installs table/date/file entrypoints. opts consumes name/value pairs, lowercase ASCII, opt searches reversed so last wins. table consumes it.arg_names before validating VariableNames. summary drops numeric NaN for summaries and prints via it.out.

sorting: Table sort resolves names/directions then sorted_rows; matrix sort interprets signed columns via abs as usize, copies real parts and class; second output is one-based original row positions.

grouping: agg filters NaNs, count counts original length, mode sorted runs strict greater chooses smallest tie. groupsummary appends GroupCount and method columns; grouptransform scatters group results into original rows. pivot groups row/column keys, count empty0, other empty NaN.

joins: right_index stores nonmissing key rows in appearance order. innerjoin enumerates all matches then stable sorts by left keys. join rejects any duplicate right key before scanning left and requires every left row match. combine omits right keys, suffixes colliding names.

missing-windows: ismissing maps numeric NaN except char to logical mask; rmmissing removes missing vector cells or whole matrix/table rows. moving scalar even k uses kb=k/2,kf=k/2-1; endpoints shrink, pair widths max0 then cast.

effects: read/write wrappers route through it.fs and contextualize errors. readtable validates file kind before read; readmatrix reads before delimiter validation. datenum accepts string/date components; JSON encodes through fileio; string converts NaN to missing.

#### Domain

Well-shaped values and finite practical windows; effects delegated to it.fs. Physical filesystem not exercised.

#### Executable gap

No cheaper Rust surrogate has been measured for this source. Exact categorical/effect behavior is not a percentage metric; numerical surrogates need a separately specified domain, zero policy and supported1/2/5/10% cap. No speed or error-bound claim.

## invariant

### Invariants and cost

options: Malformed name/value pair excludes successful options; table failure does not restore consumed arg_names. Summary has output effect but returns empty list.

sorting: Stable equal keys exclude row reversal. Fractional matrix column indices truncate rather than reject; complex imaginary components are not copied.

grouping: Empty sum0 does not imply empty mean0 (NaN). grouptransform function result length mismatch excludes valid result; fixed zscore singleton divides by0.

joins: No NaN/None key matches. join excludes partial unmatched output and does not guarantee valid unique final names after suffix collisions.

missing-windows: NaN removal loses original positions. movmedian sorts with partial_cmp unwrap and can panic on NaN; no universal NaN propagation guarantee across window methods.

effects: Spreadsheet readtable excludes attempted read; readmatrix empty delimiter may follow successful read. No host FS isolation guaranteed by this module itself.

Aggregation O(n) or O(n log n) for sorting statistics; groups O(kn log n); joins expected O(nr*k+nl*k+matches) plus sorted innerjoin output O(m log m). Moving median O(n*w log w), other windows O(n*w). File costs include I/O bytes and parsing. Pivot/import name normalization can inherit table::dedupe O(c³) collision comparisons for c names, separate from numeric aggregation.

Allocations can fail; public malformed shapes and extreme integer arithmetic are outside the bounded checked domain. Floating calculations retain their source-specific NaN and tie policies; no invented global finite guarantee.

## composition

### Scoped composition

options: Register installs table/date/file entrypoints. opts consumes name/value pairs, lowercase ASCII, opt searches reversed so last wins. table consumes it.arg_names before validating VariableNames. summary drops numeric NaN for summaries and prints via it.out.

sorting: Table sort resolves names/directions then sorted_rows; matrix sort interprets signed columns via abs as usize, copies real parts and class; second output is one-based original row positions.

grouping: agg filters NaNs, count counts original length, mode sorted runs strict greater chooses smallest tie. groupsummary appends GroupCount and method columns; grouptransform scatters group results into original rows. pivot groups row/column keys, count empty0, other empty NaN.

joins: right_index stores nonmissing key rows in appearance order. innerjoin enumerates all matches then stable sorts by left keys. join rejects any duplicate right key before scanning left and requires every left row match. combine omits right keys, suffixes colliding names.

missing-windows: ismissing maps numeric NaN except char to logical mask; rmmissing removes missing vector cells or whole matrix/table rows. moving scalar even k uses kb=k/2,kf=k/2-1; endpoints shrink, pair widths max0 then cast.

effects: read/write wrappers route through it.fs and contextualize errors. readtable validates file kind before read; readmatrix reads before delimiter validation. datenum accepts string/date components; JSON encodes through fileio; string converts NaN to missing.

Children bind disjoint operation spans in the original source and share one syntax projection. Registration/reporting/glue between named spans remains represented by the parent projection.

options: A parsed option list admits differently cased option names; final opt value admits any earlier duplicates. Table names alone cannot recover arg_names history.

sorting: Output with permutation admits original row order via inverse permutation only when full data is retained; real-only matrix output cannot recover imaginary values.

grouping: Aggregate values admit many groups including any insertion of NaNs except count/numel/nummissing; summaries do not reconstruct individual rows.

joins: Joined values admit multiple original unmatched rows and duplicate multiplicities; explicit source indices absent, so inverse is a relation. strict join admits right rows unused by left.

missing-windows: Removed rows admit arbitrary inserted missing rows; a moving sum admits kernel nullspaces and boundary constraints, not unique pointwise inversion.

effects: Read output admits every file representation decoded to same table; write result empty does not reconstruct previous filesystem contents.

The mathematical combiner keeps source equations, branch exclusions, compatible antecedents and bounded tests distinct. Passing children does not prove caller preconditions; dimension, missingness and effect adapters must preserve their own contracts.

## models

## behavior_complement

options: Malformed name/value pair excludes successful options; table failure does not restore consumed arg_names. Summary has output effect but returns empty list.

sorting: Stable equal keys exclude row reversal. Fractional matrix column indices truncate rather than reject; complex imaginary components are not copied.

grouping: Empty sum0 does not imply empty mean0 (NaN). grouptransform function result length mismatch excludes valid result; fixed zscore singleton divides by0.

joins: No NaN/None key matches. join excludes partial unmatched output and does not guarantee valid unique final names after suffix collisions.

missing-windows: NaN removal loses original positions. movmedian sorts with partial_cmp unwrap and can panic on NaN; no universal NaN propagation guarantee across window methods.

effects: Spreadsheet readtable excludes attempted read; readmatrix empty delimiter may follow successful read. No host FS isolation guaranteed by this module itself.

## reverse_reconstruction

options: A parsed option list admits differently cased option names; final opt value admits any earlier duplicates. Table names alone cannot recover arg_names history.

sorting: Output with permutation admits original row order via inverse permutation only when full data is retained; real-only matrix output cannot recover imaginary values.

grouping: Aggregate values admit many groups including any insertion of NaNs except count/numel/nummissing; summaries do not reconstruct individual rows.

joins: Joined values admit multiple original unmatched rows and duplicate multiplicities; explicit source indices absent, so inverse is a relation. strict join admits right rows unused by left.

missing-windows: Removed rows admit arbitrary inserted missing rows; a moving sum admits kernel nullspaces and boundary constraints, not unique pointwise inversion.

effects: Read output admits every file representation decoded to same table; write result empty does not reconstruct previous filesystem contents.

## representation_transport

option_shadow: Alpha=1,ALPHA=2 resolves2; unknown options are retained and caller may ignore.
summary_missing: Text unique ignores None, numeric summary ignores NaN, logical true counts every nonzero including NaN.

index_transport: Internal zero-based order becomes idx+1 external indexing.
direction_policy: Single direction broadcasts; short direction vector defaults missing directions ascending.

aggregation_units: Sample variance denominator n-1 with singleton0; all-missing variance NaN; count includes missing.
pivot_transport: Pivot names derive key text then normalize/dedupe, so column labels lose exact original identifiers.

multiplicity: innerjoin matches count sums left multiplicity*right multiplicity for each key; strict join enforces right<=1 globally.
failure_precedence: Duplicate nonmissing right keys fail before first unmatched left; missing right rows never enter map.

window_coordinates: Output shape retained for numeric moving; row vectors traverse horizontally, matrices down columns.
empty_boundary: No loop executes for zero rows; extreme widths may overflow i+kf; finite practical widths only.

effect_order: writetable serializes before write, so serialization/options failure excludes write; backend partial writes remain backend contract.
date_adapter: datenum with one matrix >=3 columns uses Y/M/D only; scalar components add hour/minute/second fraction.

## failure_precedence

options: Malformed name/value pair excludes successful options; table failure does not restore consumed arg_names. Summary has output effect but returns empty list.

sorting: Stable equal keys exclude row reversal. Fractional matrix column indices truncate rather than reject; complex imaginary components are not copied.

grouping: Empty sum0 does not imply empty mean0 (NaN). grouptransform function result length mismatch excludes valid result; fixed zscore singleton divides by0.

joins: No NaN/None key matches. join excludes partial unmatched output and does not guarantee valid unique final names after suffix collisions.

missing-windows: NaN removal loses original positions. movmedian sorts with partial_cmp unwrap and can panic on NaN; no universal NaN propagation guarantee across window methods.

effects: Spreadsheet readtable excludes attempted read; readmatrix empty delimiter may follow successful read. No host FS isolation guaranteed by this module itself.

## resource_cost

Aggregation O(n) or O(n log n) for sorting statistics; groups O(kn log n); joins expected O(nr*k+nl*k+matches) plus sorted innerjoin output O(m log m). Moving median O(n*w log w), other windows O(n*w). File costs include I/O bytes and parsing. Pivot/import name normalization can inherit table::dedupe O(c³) collision comparisons for c names, separate from numeric aggregation.

## composition_relation

options: Register installs table/date/file entrypoints. opts consumes name/value pairs, lowercase ASCII, opt searches reversed so last wins. table consumes it.arg_names before validating VariableNames. summary drops numeric NaN for summaries and prints via it.out.

sorting: Table sort resolves names/directions then sorted_rows; matrix sort interprets signed columns via abs as usize, copies real parts and class; second output is one-based original row positions.

grouping: agg filters NaNs, count counts original length, mode sorted runs strict greater chooses smallest tie. groupsummary appends GroupCount and method columns; grouptransform scatters group results into original rows. pivot groups row/column keys, count empty0, other empty NaN.

joins: right_index stores nonmissing key rows in appearance order. innerjoin enumerates all matches then stable sorts by left keys. join rejects any duplicate right key before scanning left and requires every left row match. combine omits right keys, suffixes colliding names.

missing-windows: ismissing maps numeric NaN except char to logical mask; rmmissing removes missing vector cells or whole matrix/table rows. moving scalar even k uses kb=k/2,kf=k/2-1; endpoints shrink, pair widths max0 then cast.

effects: read/write wrappers route through it.fs and contextualize errors. readtable validates file kind before read; readmatrix reads before delimiter validation. datenum accepts string/date components; JSON encodes through fileio; string converts NaN to missing.

## tests

### Bounded evidence

Selected actual-source test: builtins::table::wave30::aggregation_join_and_options. It uses only synthetic values. Retained native-commands.json specifies exact cargo selection; source-test-map.json maps logs, profiles and controls externally. Paired debug/release observations and compiled mutant status are recorded in receipt.json only after execution.

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
