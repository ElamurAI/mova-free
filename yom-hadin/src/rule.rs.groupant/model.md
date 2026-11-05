# court-rule-codec

## descriptant

### Force vocabulary, clause encoding and compatible legacy/current rule rows.

#### Descriptant

Prospective review preceded test edits. Runtime logic unchanged; three sources gain tests. Synthetic witnesses do not validate legal conclusions.

Composition: Force parse matches exact names. Clause enc uses ! negation, underscore verb spaces, slash fields and comma args; dec consumes first3 fields and ignores rest. Rule from_line accepts15 columns or inserts empty reason into14-column legacy, then force/clauses; file reader ignores empty/comment/header rows.

Retained limits: partial syntax, opaque macros; no live corpus, teacher, parser model, production filesystem or frozen gold. Complete semantics and executable approximants remain open.

## approximant

### Forward model

Force vocabulary, clause encoding and compatible legacy/current rule rows.

Force parse matches exact names. Clause enc uses ! negation, underscore verb spaces, slash fields and comma args; dec consumes first3 fields and ignores rest. Rule from_line accepts15 columns or inserts empty reason into14-column legacy, then force/clauses; file reader ignores empty/comment/header rows.

Not an executable substitute. Percentage approximation is unsupported without a useful metric; future profiles limited to 1/2/5/10% require actual-reference comparison.

## models

# Typed models: Force vocabulary, clause encoding and compatible legacy/current rule rows.

## behavior_complement

Excluded results include accepting any row length except14/15, unknown exact force names, and empty clause verb. Empty file can validly yield zero rules; arbitrary delimiter-bearing inputs are not promised roundtrip.

Operator: stated relation on this source projection. Assumptions: named domain and unchanged dependencies. Evidence: model.md#tests maps finite observations; other branches remain source review only.

## reverse_reconstruction

dec preimage contains equivalent underscore/space normalization, absent/empty/dash patient and ignored suffix slash fields. Rule output cannot recover old14 versus current15 empty-reason origin or cleaned tabs/newlines.

Operator: stated relation on this source projection. Assumptions: named domain and unchanged dependencies. Evidence: model.md#tests maps finite observations; other branches remain source review only.

## codec_canonical_domain

For nonempty verbs not beginning '!' and without underscore/slash, optional patient either None or a nonempty string excluding slash and the literal '-' sentinel, and every argument nonempty and excluding comma/slash, dec(enc(c))=c. to_line adds additional semicolon/pipe/plus/tab restrictions; test witnesses only finite valid cases.

Operator: stated relation on this source projection. Assumptions: named domain and unchanged dependencies. Evidence: model.md#tests maps finite observations; other branches remain source review only.

## legacy_column_transport

A14-column row inserts '-' at reason index13 and moves original final text to14; all15-column inputs keep positions. Other lengths fail before force parsing.

Operator: stated relation on this source projection. Assumptions: named domain and unchanged dependencies. Evidence: model.md#tests maps finite observations; other branches remain source review only.

## failure_precedence

from_line validates length then force, acts, conds, excs in struct evaluation order. read_rules stops at first bad retained row; write failure may leave changed filesystem state.

Operator: stated relation on this source projection. Assumptions: named domain and unchanged dependencies. Evidence: model.md#tests maps finite observations; other branches remain source review only.

## group_boolean_representation

conds Vec<Vec<Clause>> means AND over groups of OR clauses; excs means OR over groups of AND clauses. Empty groups can exist through direct construction but '-' serialized sentinel denotes no groups, not a logical truth certificate.

Operator: stated relation on this source projection. Assumptions: named domain and unchanged dependencies. Evidence: model.md#tests maps finite observations; other branches remain source review only.


## tests

### Checks and evidence

19 exact selected native tests twice debug and twice release execute actual linked code on bounded hand-built cases. CLI covers argument helpers only. Full extraction/model initialization, actual jurisdiction loading, protocol disk errors, resource exhaustion and malformed public lattices are unexecuted. Module declarations have compile evidence only.

Three compiled source-behavior mutants: count weight, precedence tie, modal tie; each requires compile success, named assertion failure, exact byte/hash restoration and positive rerun. Legacy fixture interventions count separately. Final observations: external evidence-summary.md.

This node observes: Force vocabulary, clause encoding and compatible legacy/current rule rows.. Unmapped models remain source-derived/prospective, not verified by the whole test count.

The exact per-source names and limits are retained in [source-evidence](model.md#source-evidence).

## invariant-and-cost

# Invariant and cost

## Invariant

Roundtrip equality holds only delimiter-safe canonical fields and no literal underscore ambiguity/dash-empty ambiguity. Conditions encode alternatives with |, exceptions conjunction with +; serializer sanitizes selected text fields.

## Cost

Clause/Rule encoding and decoding O(total bytes) with allocations; read_rules reads entire file then collects until first error. write_rules buffers whole serialization then one non-atomic write.

Symbolic bounds, not timings. Resource exhaustion is not tested.

## source-evidence


### Source observation map

Actual codec; whole-file reader/writer unexecuted.

- rule::tests::line_round_trip
- wave31_codec_statistics_and_status_fibers

## History

### 2026-10-31 · descriptant

Heading `#### Epoch 2026-10-31` stays in descriptant as `#### Descriptant`.

#### Review correction 2026-10-31

Independent review supplied two counterexamples to an overbroad roundtrip domain: a positive verb beginning '!' changes decoded negation, and an empty argument is filtered. The sufficient domain now excludes leading '!' verbs and requires nonempty arguments and nonempty nonsentinel Some patients. Production code is unchanged; this corrects the model rather than calling normalization a runtime bug.

### 2026-10-31 · tests

External run label: octopus-wave31-court-2026-10-31; exact commands in native-commands.json, per-source observations in source-test-map.json, numeric exits and named outcomes in raw logs.

### Undated · document

Subject: yom-hadin/src/rule.rs, SHA-256 211396ab0efca1ad45c77dc3a3752ae335570464b18814d61837deae6d102a6a. Domain: bounded finite synthetic rules/cases; valid indices and successful allocation unless a section narrows this. Each relation observes implementation, not legal authority. Status: partial source-derived model, finite evidence mapped in model.md#tests.
