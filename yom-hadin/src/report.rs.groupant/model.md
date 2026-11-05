# court-report

## descriptant

### Layered synthetic lattice reports, weighted summaries and deterministic protocol formatting.

#### Descriptant

Prospective review preceded test edits. Runtime logic unchanged; three sources gain tests. Synthetic witnesses do not validate legal conclusions.

Composition: judge_all groups admissible law reasons by jurisdiction index, filters only, optionally removes flagged rules, builds lattice/honest/tips. report constructs full and clean views per defendant, then selected sections; modern/historical references stay separate. protocol reads rules-file bytes, renders FNV fingerprints and rebuilt lattice records.

Retained limits: partial syntax, opaque macros; no live corpus, teacher, parser model, production filesystem or frozen gold. Complete semantics and executable approximants remain open.

## approximant

### Forward model

Layered synthetic lattice reports, weighted summaries and deterministic protocol formatting.

judge_all groups admissible law reasons by jurisdiction index, filters only, optionally removes flagged rules, builds lattice/honest/tips. report constructs full and clean views per defendant, then selected sections; modern/historical references stay separate. protocol reads rules-file bytes, renders FNV fingerprints and rebuilt lattice records.

Not an executable substitute. Percentage approximation is unsupported without a useful metric; future profiles limited to 1/2/5/10% require actual-reference comparison.

## models

# Typed models: Layered synthetic lattice reports, weighted summaries and deterministic protocol formatting.

## behavior_complement

Under finite nonnegative weights, excluded stats include positive-mass percentile outside0..100 and modal tie favoring greater severity. protocol cannot distinguish absent rules file from successfully read empty file by FNV alone.

Operator: stated relation on this source projection. Assumptions: named domain and unchanged dependencies. Evidence: model.md#tests maps finite observations; other branches remain source review only.

## reverse_reconstruction

Weighted summaries admit all severity-weight multisets with same aggregates/quantile crossings; row order is forgotten except floating summation effects. Protocol string reconstructs displayed options/counts but not complete files because64-bit fingerprints have collisions.

Operator: stated relation on this source projection. Assumptions: named domain and unchanged dependencies. Evidence: model.md#tests maps finite observations; other branches remain source review only.

## weighted_quantile_ties

Aggregate weights by severity; modal strict > picks first/lower tie. Quantile is first cumulative >=p*total-1e-9. With tiny total the epsilon affects usual quantile interpretation; NaN/negative/infinite weights outside meaningful statistical domain.

Operator: stated relation on this source projection. Assumptions: named domain and unchanged dependencies. Evidence: model.md#tests maps finite observations; other branches remain source review only.

## goal_order

serves convict means severity>=2, acquit0, reduce positive below honest, reclassify different verdict and changed winner act/force. goal_rows sorts tip count then goal-specific severity then jurisdiction index then Verdict.

Operator: stated relation on this source projection. Assumptions: named domain and unchanged dependencies. Evidence: model.md#tests maps finite observations; other branches remain source review only.

## reference_separation

clean view removes flagged rules unless opts.flagged; modern uses kind=='modern', historical every other kind. Choice of law is computed on original nonempty rules before cleaning, so a cleaned empty lattice can remain represented.

Operator: stated relation on this source projection. Assumptions: named domain and unchanged dependencies. Evidence: model.md#tests maps finite observations; other branches remain source review only.

## protocol_effect_fingerprint

fnv is wrapping64-bit FNV1a. protocol hashes current disk rules.tsv, not serialized in-memory j.rules, and silently substitutes empty bytes on read failure. Equal case/options/file bytes and computations give same text; race-free snapshot assumption required.

Operator: stated relation on this source projection. Assumptions: named domain and unchanged dependencies. Evidence: model.md#tests maps finite observations; other branches remain source review only.


## tests

### Checks and evidence

19 exact selected native tests twice debug and twice release execute actual linked code on bounded hand-built cases. CLI covers argument helpers only. Full extraction/model initialization, actual jurisdiction loading, protocol disk errors, resource exhaustion and malformed public lattices are unexecuted. Module declarations have compile evidence only.

Three compiled source-behavior mutants: count weight, precedence tie, modal tie; each requires compile success, named assertion failure, exact byte/hash restoration and positive rerun. Legacy fixture interventions count separately. Final observations: external evidence-summary.md.

This node observes: Layered synthetic lattice reports, weighted summaries and deterministic protocol formatting.. Unmapped models remain source-derived/prospective, not verified by the whole test count.

The exact per-source names and limits are retained in [source-evidence](model.md#source-evidence).

## invariant-and-cost

# Invariant and cost

## Invariant

For finite nonnegative weights with positive total, stats median/quartiles come from sorted observed severities, modal ties choose lower. Reporting does not write files; protocol does read and maps read errors to empty bytes.

## Cost

judge_all sums L+T over selected jurisdictions; report repeats full/clean work and device/absurd sections. stats O(N log K) space O(K); goal_rows O(Q log Q); protocol adds total rules-file read bytes plus another judge_all. Output allocation proportional rendered text.

Symbolic bounds, not timings. Resource exhaustion is not tested.

## source-evidence


### Source observation map

Actual stats/percentile/report/goals; protocol file reads not exercised.

- reference_statistics
- counting_is_deterministic
- no_discretion_means_no_sensitivity_and_the_goals_collapse
- wave31_codec_statistics_and_status_fibers

## History

### 2026-10-31 · descriptant

Heading `#### Epoch 2026-10-31` stays in descriptant as `#### Descriptant`.

### 2026-10-31 · tests

External run label: octopus-wave31-court-2026-10-31; exact commands in native-commands.json, per-source observations in source-test-map.json, numeric exits and named outcomes in raw logs.

### Undated · document

Subject: yom-hadin/src/report.rs, SHA-256 eb60c61212e8f64ad0b3c67ed5dfbd9623e384479ec54567ed762cc8504b58ff. Domain: bounded finite synthetic rules/cases; valid indices and successful allocation unless a section narrows this. Each relation observes implementation, not legal authority. Status: partial source-derived model, finite evidence mapped in model.md#tests.
