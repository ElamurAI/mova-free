# world-tests-format

## descriptant

### world-tests-format

Require Err for 14 malformed command literals, Ok(None) for whitespace, and anchors [3,4] for valid speech.

## approximant

### Forward relation

Require Err for 14 malformed command literals, Ok(None) for whitespace, and anchors [3,4] for valid speech.

#### Domain and resource limits

Finite valid structures and allocation-success executions under the branch conditions in model.md#models; integer overflow is outside measured fixtures. Exact categorical relations have no supplied percent surrogate. Macro expansion, arbitrary model quality and external effects are not inferred from syntax.

## invariant

# Invariant

Pure command parser; no model/corpus or World execution.

## Cost

Finite literal bytes; delegated parse scan/allocation.

## composition

### Composition

This scope contributes format to its parent. Its forward, complement and preimage are restricted to the bound byte interval. The parent syntax projection is referenced once through ../syntax.json; no independent semantic AST is claimed.

## models

## behavior_complement

Exclude parse success on listed bad closed tags, duplicate keys, missing anchor, bad quote and registry mismatch.

## reverse_reconstruction

Err admits all diagnostics/failure stages; anchors [3,4] admit many valid payloads, not unique line text.

## closed_values

spaceship/hangry/threaten rejected.

## duplicate_fields

Two to fields cannot choose first or last.

## blank_identity

Whitespace is None, not empty executable Line.

## anchor_order

Two anchors preserve source sequence.

## tests

### Bounded checks

Named checks associated with this source (not every fragment branch):

- tests::format_is_strict
- tests::rejected_command_leaves_world_unchanged
- tests::old_states_stay_immutable
- tests::verdict_respects_order_and_alternatives

17 unique named tests in total passed twice debug and twice release; sorted named outcomes equal across all four runs. Four compiled controls cover bAbI identity, StepGame tie ordering, parser-command state removal and bench-command CPU selection; each exits101 at a runtime assertion, exact source restored then positive exit0. Other sources have finite adversarial fixtures but no separately compiled mutant.

CLI integration coverage is command construction and temporary cleanup only; legacy school/model/cadence paths are not executed. No speed benchmark, GPU, live teacher, datasets or held-out replay.

Limits:

- Only four synthetic tests executed; no model, stored VMM replay or held-out fixtures
- Finite assertions not executor proof; executable percentage approximant unsupported

## History

### 2026-10-31 · descriptant

#### Modeling epoch — 2026-10-31

Read full assigned source before editing; prospective predictions retained in external wave29 receipts. This child binds only its stated half-open UTF-8 byte range; other APIs remain parent scope. Evidence is limited to named bounded fixtures; registration is partial modeling. Production behavior unchanged; only inline synthetic tests added to four assigned sources.

### 2026-10-31 · tests

External receipt root: $MOVA_DATA/runs/octopus-wave29-worldeval-2026-10-31.
