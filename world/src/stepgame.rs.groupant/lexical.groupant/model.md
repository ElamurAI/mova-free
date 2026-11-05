# world-stepgame-lexical

## descriptant

### world-stepgame-lexical

Reject clock; collect direction signs except intensifier right; reject conflicting axis signs; zero direction needs sameness. Ground chosen unique cue then has/have subject then a2; negate if ground is a1. Axis relation A(X)=None if (∃x>0)∧(∃x<0), else first(X) or0; lexical(s)=σ·(A(horizontal),A(vertical)), σ=+1 for ground=a2 else−1, after clock/zero-cue guards.

## approximant

### Forward relation

Reject clock; collect direction signs except intensifier right; reject conflicting axis signs; zero direction needs sameness. Ground chosen unique cue then has/have subject then a2; negate if ground is a1. Axis relation A(X)=None if (∃x>0)∧(∃x<0), else first(X) or0; lexical(s)=σ·(A(horizontal),A(vertical)), σ=+1 for ground=a2 else−1, after clock/zero-cue guards.

#### Domain and resource limits

Finite valid structures and allocation-success executions under the branch conditions in model.md#models; integer overflow is outside measured fixtures. Exact categorical relations have no supplied percent surrogate. Macro expansion, arbitrary model quality and external effects are not inferred from syntax.

## invariant

# Invariant

Each component in {-1,0,1}; repeated directions do not increase magnitude.

## Cost

O(B) tokens, O(T²) worst prefix walks; no weights/model required.

## composition

### Composition

This scope contributes lexical to its parent. Its forward, complement and preimage are restricted to the bound byte interval. The parent syntax projection is referenced once through ../syntax.json; no independent semantic AST is claimed.

## models

## behavior_complement

Exclude Some for clock/conflict/no-direction-no-sameness. Multiple ground cues fall back rather than rejecting.

## reverse_reconstruction

Some(0,-1) admits downward synonyms and A right under B; None combines several distinct failure fibers. Surface form unrecoverable. Pre(None)=Clock ∪ AxisConflict ∪ {no directions∧no sameness}; the guards are ordered operationally but the result forgets the reason.

## intensifier

right under gives below; right of keeps horizontal.

## ground_negation

B has A on its left gives B-relative-A right.

## sameness

same/coincid*/overlap*/share*/where/identical permit zero.

## failure_precedence

Clock before conflicting signs before sameness fallback.

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
