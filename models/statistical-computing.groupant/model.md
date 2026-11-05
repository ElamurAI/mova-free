# statistical-computing

## descriptant

### statistical-computing: wave30

Actual joins: stats imports builtins::table::{opts,opt}, table::groups, value::Table; mcgate imports stats and Rng; ext_boost/ext_nist import Interp whose registry resolves the same builtins. Invalid adapter example: group equality on NaN does not imply matching hash keys. Another: row-key signed zero canonicalization must agree with numerical sorting equality. Benchmark/calibration or full external dataset results are not inferred from helper tests. Paired ttest length compatibility precedes option parsing; only the minimum cleaned-sample-count check follows options. agg count and numel both count original values, so NaN insertion changes both. iso_date selects date-only formatting only for exactly integral inputs; rounded nonintegral midnight keeps a time-bearing pattern.

This source-bound partial group organizes existing Mova code. Native evidence and excluded effects are recorded in source sidecars and the root wave30 report. Registration is not a universal proof or whole-system runtime claim.

## model

# statistical-computing

## forward

Rng(seed) advances xoshiro state, feeding mcgate's eight H0 scenarios. stats::t_one/t_two/anova_one/corr_p/ols/chi2_ind produce p vectors; mcgate emits count(p<=a)/len with 4.5*sqrt(a(1-a)/len)+margin for four ALPHAS. Table comparison/groups supply sorted category identities to stats::crosstab and ANOVA; stats::mode delegates table builtin agg. Builtin joins use table::row_key, aggregate summaries use table::groups and scatter results. Boost/NIST helpers transport interpreter output into relative-EPS or LRE acceptance regions; selected synthetic tests call actual helpers without external data.

## invariant

For well-shaped bounded values, internal indices are zero-based and exposed sort permutations one-based; tables retain name/column/kind alignment when caller inputs satisfy representation preconditions. RNG cache distinguishes state-consuming polar generation from spare consumption. Missingness is operation-specific: groups coalesce NaNs, joins exclude them, t tests drop them, covariance propagates them. These policies must not be silently equated.

## cost

For n rows,w columns,k keys: grouping/sort O(k*n log n), copying O(nw), innerjoin output m can be nl*nr and costs expected hash matching plus O(m log m) ordering. OLS O(np²+p³), chi2 O(rc), t/ANOVA O(n). Tiny Monte Carlo scales O(K) fixed-sized simulations with300 chi2 draws per replicate and unbounded worst-case polar rejection. External parser cost O(bytes); no speed measurement. Table make name checks cost O(w²) comparisons; table::dedupe may retry suffixes and cost O(w³) string comparisons plus text work. Pivot/import callers inherit that collision cost.

## composition

Actual joins: stats imports builtins::table::{opts,opt}, table::groups, value::Table; mcgate imports stats and Rng; ext_boost/ext_nist import Interp whose registry resolves the same builtins. Invalid adapter example: group equality on NaN does not imply matching hash keys. Another: row-key signed zero canonicalization must agree with numerical sorting equality. Benchmark/calibration or full external dataset results are not inferred from helper tests. Paired ttest length compatibility precedes option parsing; only the minimum cleaned-sample-count check follows options. agg count and numel both count original values, so NaN insertion changes both. iso_date selects date-only formatting only for exactly integral inputs; rounded nonintegral midnight keeps a time-bearing pattern.

## domain

CPU-only bounded synthetic values, well-shaped Mat/Table structures, practical finite dimensions, seed42, Monte Carlo K=0 and2, synthetic2x2 NIST-like text and special-function known points. Host datasets, GPU and external services excluded.

## limitations

IEEE f64/transcendentals and statrs/faer behavior delegated. Direct public helper dimension preconditions and overflow-sensitive dates/windows are partial. No universal numerical accuracy or significance calibration claim. Physical file backend and frozen NIST/Boost suites unexecuted.

## checks

- Seven exact selected tests twice debug and twice release; native-commands.json
- Seven compiled mutants each failing actual selected assertion and exact restored positive rerun; controls.json
- Seven source syntax projections generated twice byte-identically; local-registry-entries.json

## catalog

## behavior-complement

Composition excludes missing-key join pairs even though group summaries include missing groups; excludes 1 from uniform draws; excludes cached-normal state advance; excludes rejection only on p==alpha because stats verdict is strict. Zero-K mcgate cannot claim ok rows. Synthetic helper success excludes no possible failure in unexecuted external data suites.

## reverse-reconstruction

Given gate fractions, antecedents are all p-vectors with matching threshold counts and scenario-compatible RNG/statistical states; original samples/seed generally not unique. Group sums and fitted beta admit many observation tables; missing rows and dropped columns cannot be recovered. Boost/NIST scalar error reports admit all curves beneath or attaining their maxima/caps.

## missingness-transport

table::groups(NaN,NaN) puts both rows together; row_key(NaN)=None removes both from join indexes. stats::clean drops NaNs while cov_matrix sums them; passing grouped data into stats requires method-specific policy, not blanket missing equivalence.

## random-calibration

Rng uniform consumes one integer transition; normal may consume cached spare without transition. mcgate reuses one RNG sequentially across scenarios, so modifying one scenario's draws changes later samples; per-row calibration is only a finite random experiment, and K=2 tests validate wiring only.

## table-statistic-shape

Table numeric columns are n-by1. categories groups keys into0-based IDs, crosstab counts into row/column categories then emits column-major Mat; stats tables wrap numeric summaries as aligned columns. sortrows output permutation adds1; it cannot be reused as an internal index without subtracting1.

## error-metric-boundaries

Boost e=abs((got-want)/want)/EPS excludes nonnormal reference values; NIST LRE=-log10(relative error) capped15 uses absolute error at certified zero. These metrics are not interchangeable, and neither finite synthetic observations nor source-bound models establishes a supported executable-surrogate percent cap.
