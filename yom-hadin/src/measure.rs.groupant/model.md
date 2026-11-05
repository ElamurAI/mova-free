# court-measure

## descriptant

### Own-rule selection and synthetic gold precision/recall accounting.

#### Descriptant

Prospective review preceded test edits. Runtime logic unchanged; three sources gain tests. Synthetic witnesses do not validate legal conclusions.

Composition: sample selects matching owner jurisdiction and sentence membership. judge_rule matches force and any act/condition verb, authority ASCII-insensitive. precision maps sentence through supplied order+shift, omits unavailable gold, excludes '?' authority denominator; recall counts each expected entry once if any sample matches.

Retained limits: partial syntax, opaque macros; no live corpus, teacher, parser model, production filesystem or frozen gold. Complete semantics and executable approximants remain open.

## approximant

### Forward model

Own-rule selection and synthetic gold precision/recall accounting.

sample selects matching owner jurisdiction and sentence membership. judge_rule matches force and any act/condition verb, authority ASCII-insensitive. precision maps sentence through supplied order+shift, omits unavailable gold, excludes '?' authority denominator; recall counts each expected entry once if any sample matches.

Not an executable substitute. Percentage approximation is unsupported without a useful metric; future profiles limited to 1/2/5/10% require actual-reference comparison.

## models

# Typed models: Own-rule selection and synthetic gold precision/recall accounting.

## behavior_complement

Excluded outputs include judging a sample absent from order, counting inherited rules as own when r.jur!=j.id, and unknown '?' authority increasing authority denominator. Metric correctness does not establish real legal precision.

Operator: stated relation on this source projection. Assumptions: named domain and unchanged dependencies. Evidence: model.md#tests maps finite observations; other branches remain source review only.

## reverse_reconstruction

Formatted precision recovers compatible integer counts up to rounding and supplied sample size, not unique rules/gold. Recall hits ignore duplicate extracted matches; many corpora share the same formatted percentage.

Operator: stated relation on this source projection. Assumptions: named domain and unchanged dependencies. Evidence: model.md#tests maps finite observations; other branches remain source review only.

## denominator_projection

judged counts only available ordered gold; fa uses judged, authority uses known gold among judged, both treats unknown authority as unconstrained. No single shared denominator is valid.

Operator: stated relation on this source projection. Assumptions: named domain and unchanged dependencies. Evidence: model.md#tests maps finite observations; other branches remain source review only.

## shift_control

For matching order position k, compare gold[order[(k+shift)%len]]; absence of position returns None before modulo, so empty order is safe. Duplicate order names use first occurrence.

Operator: stated relation on this source projection. Assumptions: named domain and unchanged dependencies. Evidence: model.md#tests maps finite observations; other branches remain source review only.

## recall_existential

For each expected(force,verbs), hit iff some same-sentence extracted rule has force and intersects verbs across acts+conditions. One extracted rule can satisfy multiple expected rows; no bijective matching.

Operator: stated relation on this source projection. Assumptions: named domain and unchanged dependencies. Evidence: model.md#tests maps finite observations; other branches remain source review only.

## gold_row_loss

read_gold overwrites duplicate sentence keys, ignores notes, missing second column means no rules, malformed force/verb errors. gold_order retains repeated sentence order independently; these representations can disagree.

Operator: stated relation on this source projection. Assumptions: named domain and unchanged dependencies. Evidence: model.md#tests maps finite observations; other branches remain source review only.


## tests

### Checks and evidence

19 exact selected native tests twice debug and twice release execute actual linked code on bounded hand-built cases. CLI covers argument helpers only. Full extraction/model initialization, actual jurisdiction loading, protocol disk errors, resource exhaustion and malformed public lattices are unexecuted. Module declarations have compile evidence only.

Three compiled source-behavior mutants: count weight, precedence tie, modal tie; each requires compile success, named assertion failure, exact byte/hash restoration and positive rerun. Legacy fixture interventions count separately. Final observations: external evidence-summary.md.

This node observes: Own-rule selection and synthetic gold precision/recall accounting.. Unmapped models remain source-derived/prospective, not verified by the whole test count.

The exact per-source names and limits are retained in [source-evidence](model.md#source-evidence).

## invariant-and-cost

# Invariant and cost

## Invariant

Metrics are descriptive formatted counts of supplied synthetic data; sample borrows original rules. Empty denominators format0%, and unknown authority can count jointly correct with force+act.

## Cost

sample O(R log G); precision O(S*O + S*V*Gverbs), recall O(Gentries*S*V); rows/strings allocated. shift addition must fit usize. No frozen assessment run.

Symbolic bounds, not timings. Resource exhaustion is not tested.

## source-evidence


### Source observation map

Actual in-memory sample/precision/recall/judge_rule; no frozen gold or read_gold filesystem.

- wave31_extraction_adjacency_disputes_and_metric_denominators

## History

### 2026-10-31 · descriptant

Heading `#### Epoch 2026-10-31` stays in descriptant as `#### Descriptant`.

### 2026-10-31 · tests

External run label: octopus-wave31-court-2026-10-31; exact commands in native-commands.json, per-source observations in source-test-map.json, numeric exits and named outcomes in raw logs.

### Undated · document

Subject: yom-hadin/src/measure.rs, SHA-256 c08d407318b3b1f8c70263d6ce08fd1325f4bbe5bb08c8bab82530a5c57511c7. Domain: bounded finite synthetic rules/cases; valid indices and successful allocation unless a section narrows this. Each relation observes implementation, not legal authority. Status: partial source-derived model, finite evidence mapped in model.md#tests.
