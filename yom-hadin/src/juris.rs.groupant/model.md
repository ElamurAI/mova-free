# court-jurisdiction

## descriptant

### Jurisdiction TSV loading, vocabulary covers and ordered lookup policies.

#### Descriptant

Prospective review preceded test edits. Runtime logic unchanged; three sources gain tests. Synthetic witnesses do not validate legal conclusions.

Composition: Load metadata/id/principles, merge common+own terms by head preserving first insertion order, remove overridden common penalties, read optional rules/authorities/flags/devices. load_all snapshots own rules then prepends direct inherited lists. cover selects Sure equality/narrow before Broad; severity max rank, authority first matching pattern.

Retained limits: partial syntax, opaque macros; no live corpus, teacher, parser model, production filesystem or frozen gold. Complete semantics and executable approximants remain open.

## approximant

### Forward model

Jurisdiction TSV loading, vocabulary covers and ordered lookup policies.

Load metadata/id/principles, merge common+own terms by head preserving first insertion order, remove overridden common penalties, read optional rules/authorities/flags/devices. load_all snapshots own rules then prepends direct inherited lists. cover selects Sure equality/narrow before Broad; severity max rank, authority first matching pattern.

Not an executable substitute. Percentage approximation is unsupported without a useful metric; future profiles limited to 1/2/5/10% require actual-reference comparison.

## models

# Typed models: Jurisdiction TSV loading, vocabulary covers and ordered lookup policies.

## behavior_complement

Excluded successful loads include absent jurisdiction id and unknown inheritance target. Cover cannot return No for equal words. Authority cannot select a later matching pattern over an earlier one; this does not validate external law tables.

Operator: stated relation on this source projection. Assumptions: named domain and unchanged dependencies. Evidence: model.md#tests maps finite observations; other branches remain source review only.

## reverse_reconstruction

Loaded state has all TSV antecedents with same trimmed rows, overwrite winners and direct inherited snapshots. Comments, whitespace and overridden rows are unrecoverable; equal severity observations allow many matching penalties and texts.

Operator: stated relation on this source projection. Assumptions: named domain and unchanged dependencies. Evidence: model.md#tests maps finite observations; other branches remain source review only.

## vocabulary_cover_lattice

Cover is max of No/Broad/Sure membership evidence, with equality Sure. Opposite is directional: narrow(rule) and opposite(case); reversing words need not preserve truth.

Operator: stated relation on this source projection. Assumptions: named domain and unchanged dependencies. Evidence: model.md#tests maps finite observations; other branches remain source review only.

## override_position

Replacing common term by same head changes definition without moving its first-seen order; term_of returns first broad-containing term even when another term supplies Sure cover. Plain reading can therefore depend on order.

Operator: stated relation on this source projection. Assumptions: named domain and unchanged dependencies. Evidence: model.md#tests maps finite observations; other branches remain source review only.

## lookup_tie_policy

severity lowercases penalty and selects maximal rank, last equal rank wins; authority lowercases name then first substring wins regardless of rank. Empty pattern matches any string.

Operator: stated relation on this source projection. Assumptions: named domain and unchanged dependencies. Evidence: model.md#tests maps finite observations; other branches remain source review only.

## inheritance_snapshot

load_all takes own map before extension. A inherits B and B inherits C does not imply A receives C; repeated inherits rows duplicate rules, and duplicate IDs overwrite own map entries. No transitive closure or dedup.

Operator: stated relation on this source projection. Assumptions: named domain and unchanged dependencies. Evidence: model.md#tests maps finite observations; other branches remain source review only.


## tests

### Checks and evidence

19 exact selected native tests twice debug and twice release execute actual linked code on bounded hand-built cases. CLI covers argument helpers only. Full extraction/model initialization, actual jurisdiction loading, protocol disk errors, resource exhaustion and malformed public lattices are unexecuted. Module declarations have compile evidence only.

Three compiled source-behavior mutants: count weight, precedence tie, modal tie; each requires compile success, named assertion failure, exact byte/hash restoration and positive rerun. Legacy fixture interventions count separately. Final observations: external evidence-summary.md.

This node observes: Jurisdiction TSV loading, vocabulary covers and ordered lookup policies.. Unmapped models remain source-derived/prospective, not verified by the whole test count.

The exact per-source names and limits are retained in [source-evidence](model.md#source-evidence).

## invariant-and-cost

# Invariant and cost

## Invariant

Load does not mutate files. Missing auxiliary TSV means empty; missing id or invalid principle/rank/device field errors. Direct inheritance preserves inherited rule jurisdiction IDs.

## Cost

Whole-file reads O(B) space/time plus parsing; term BTree inserts O(T log T), penalty override O(Pcommon*Pown), own-rule snapshot clones all rules. Cover O(total term-list bytes), severity/authority substring scan O(patterns*text length).

Symbolic bounds, not timings. Resource exhaustion is not tested.

## source-evidence


### Source observation map

Actual cover/opposite/authority/severity/flags; TSV loader/inheritance source review only.

- flagged_rules_leave_the_reference
- wave31_jurisdiction_cover_direction_and_lookup_order

## History

### 2026-10-31 · descriptant

Heading `#### Epoch 2026-10-31` stays in descriptant as `#### Descriptant`.

### 2026-10-31 · tests

External run label: octopus-wave31-court-2026-10-31; exact commands in native-commands.json, per-source observations in source-test-map.json, numeric exits and named outcomes in raw logs.

### Undated · document

Subject: yom-hadin/src/juris.rs, SHA-256 070c3346e18a26e34a47069d33438d78348dfcbe55d5464872d1df6a5119a4cc. Domain: bounded finite synthetic rules/cases; valid indices and successful allocation unless a section narrows this. Each relation observes implementation, not legal authority. Status: partial source-derived model, finite evidence mapped in model.md#tests.
