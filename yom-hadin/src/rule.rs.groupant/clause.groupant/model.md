# court-rule-codec-clause

## descriptant

### Clause text encoder/decoder and display projection.

#### Descriptant

Prospective review preceded test edits. Runtime logic unchanged; three sources gain tests. Synthetic witnesses do not validate legal conclusions.

Composition: Encode neg marker, space-to-underscore verb, slash patient sentinel and optional comma args; decode first3 slash fields and normalize underscore to space.

Retained limits: partial syntax, opaque macros; no live corpus, teacher, parser model, production filesystem or frozen gold. Complete semantics and executable approximants remain open.

## approximant

### Forward model

Clause text encoder/decoder and display projection.

Encode neg marker, space-to-underscore verb, slash patient sentinel and optional comma args; decode first3 slash fields and normalize underscore to space.

Not an executable substitute. Percentage approximation is unsupported without a useful metric; future profiles limited to 1/2/5/10% require actual-reference comparison.

## models

# Typed models: Clause text encoder/decoder and display projection.

## behavior_complement

Empty decoded verb always errors; decoder cannot retain fourth slash field.

Operator: stated relation on this source projection. Assumptions: named domain and unchanged dependencies. Evidence: model.md#tests maps finite observations; other branches remain source review only.

## reverse_reconstruction

Patient missing/empty/'-' all reconstruct None; underscores normalize and extra slash fields vanish, yielding nonunique text preimages.

Operator: stated relation on this source projection. Assumptions: named domain and unchanged dependencies. Evidence: model.md#tests maps finite observations; other branches remain source review only.

## negation_prefix

Exactly one leading ! is stripped; a second ! belongs to verb text and can reencode.

Operator: stated relation on this source projection. Assumptions: named domain and unchanged dependencies. Evidence: model.md#tests maps finite observations; other branches remain source review only.

## argument_filter

Comma split filters empty args; empty and repeated commas are indistinguishable after decode.

Operator: stated relation on this source projection. Assumptions: named domain and unchanged dependencies. Evidence: model.md#tests maps finite observations; other branches remain source review only.

## verb_normalization

All underscores become spaces; literal underscore cannot roundtrip as underscore through encoding/decoding.

Operator: stated relation on this source projection. Assumptions: named domain and unchanged dependencies. Evidence: model.md#tests maps finite observations; other branches remain source review only.

## display_loss

show joins role words with spaces and no delimiter escaping, so text observation cannot identify patient versus argument boundaries uniquely.

Operator: stated relation on this source projection. Assumptions: named domain and unchanged dependencies. Evidence: model.md#tests maps finite observations; other branches remain source review only.


## tests

### Checks and evidence

19 exact selected native tests twice debug and twice release execute actual linked code on bounded hand-built cases. CLI covers argument helpers only. Full extraction/model initialization, actual jurisdiction loading, protocol disk errors, resource exhaustion and malformed public lattices are unexecuted. Module declarations have compile evidence only.

Three compiled source-behavior mutants: count weight, precedence tie, modal tie; each requires compile success, named assertion failure, exact byte/hash restoration and positive rerun. Legacy fixture interventions count separately. Final observations: external evidence-summary.md.

This node observes: Clause text encoder/decoder and display projection.. Unmapped models remain source-derived/prospective, not verified by the whole test count.

Parent source witness map: [source-evidence](../model.md#source-evidence). Only the described fragment observation is supported; parent test names do not imply every fragment branch executed.

## invariant-and-cost

# Invariant and cost

## Invariant

For a nonempty verb not beginning '!' and containing neither underscore nor slash, patient None or nonempty excluding slash and literal '-', and all arguments nonempty with neither comma nor slash, dec(enc(c))=c. Empty arguments and positive literal leading '!' verbs fall outside this sufficient roundtrip domain.

## Cost

O(clause bytes) allocation and traversal.

Symbolic bounds, not timings. Resource exhaustion is not tested.

## History

### 2026-10-31 · descriptant

Heading `#### Epoch 2026-10-31` stays in descriptant as `#### Descriptant`.

### 2026-10-31 · tests

External run label: octopus-wave31-court-2026-10-31; exact commands in native-commands.json, per-source observations in source-test-map.json, numeric exits and named outcomes in raw logs.

### Undated · document

Subject: yom-hadin/src/rule.rs, SHA-256 211396ab0efca1ad45c77dc3a3752ae335570464b18814d61837deae6d102a6a. Domain: bounded finite synthetic rules/cases; valid indices and successful allocation unless a section narrows this. Each relation observes implementation, not legal authority. Status: partial source-derived model, finite evidence mapped in model.md#tests.
