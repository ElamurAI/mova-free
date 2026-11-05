# world-stepgame-formats

## descriptant

### world-stepgame-formats

load branches by txt extension else BTreeMap JSON. Text skips no-space lines, clears at exact '1', emits question for >=2 tab fields else appends story. agents returns unique single ASCII capitals in encounter order.

## approximant

### Forward relation

load branches by txt extension else BTreeMap JSON. Text skips no-space lines, clears at exact '1', emits question for >=2 tab fields else appends story. agents returns unique single ASCII capitals in encounter order.

#### Domain and resource limits

Finite valid structures and allocation-success executions under the branch conditions in model.md#models; integer overflow is outside measured fixtures. Exact categorical relations have no supplied percent surrogate. Macro expansion, arbitrary model quality and external effects are not inferred from syntax.

## invariant

# Invariant

Text id is emitted index, not source line number; questions never append to story; empty text emits none.

## Cost

O(B+copied story bytes), quadratic duplication possible with many questions; JSON sort O(NlogN).

## composition

### Composition

This scope contributes formats to its parent. Its forward, complement and preimage are restricted to the bound byte interval. The parent syntax projection is referenced once through ../syntax.json; no independent semantic AST is claimed.

## models

## behavior_complement

Exclude retaining pre-reset story or duplicate agents; invalid JSON id keys are tolerated as zero.

## reverse_reconstruction

Items admit arbitrary ignored support fields; invalid numeric keys all map to zero and stable sort preserves lexical BTreeMap order among ties.

## reset_marker

1 clears, 01 does not.

## support_erasure

Third and later tab fields are discarded.

## agent_domain

AA and non-ASCII capitals are not single ASCII agents.

## failure_order

Read/UTF8 error before JSON parse; text malformed rows may be skipped rather than error.

## tests

### Bounded checks

Named checks associated with this source (not every fragment branch):

- stepgame::wave29_tests::lexical_ground_and_tie_boundaries
- stepgame::wave29_tests::graph_first_path_and_text_reset
- tests03::stepgame_lexical_reader_reads_new_phrasings
- tests03::stepgame_phrasing_and_corrected_format

17 unique named tests in total passed twice debug and twice release; sorted named outcomes equal across all four runs. Four compiled controls cover bAbI identity, StepGame tie ordering, parser-command state removal and bench-command CPU selection; each exits101 at a runtime assertion, exact source restored then positive exit0. Other sources have finite adversarial fixtures but no separately compiled mutant.

CLI integration coverage is command construction and temporary cleanup only; legacy school/model/cadence paths are not executed. No speed benchmark, GPU, live teacher, datasets or held-out replay.

Limits:

- Partial syntax projection only; no macro expansion or proof
- No useful percentage surrogate for exact categorical/state/effect semantics; executable approximant unsupported
- No dataset training, eval/contrast or GPU
- No overflow/allocation fault injection

## History

### 2026-10-31 · descriptant

#### Modeling epoch — 2026-10-31

Read full assigned source before editing; prospective predictions retained in external wave29 receipts. This child binds only its stated half-open UTF-8 byte range; other APIs remain parent scope. Evidence is limited to named bounded fixtures; registration is partial modeling. Production behavior unchanged; only inline synthetic tests added to four assigned sources.

### 2026-10-31 · tests

External receipt root: $MOVA_DATA/runs/octopus-wave29-worldeval-2026-10-31.
