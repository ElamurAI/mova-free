# world-stepgame-graph

## descriptant

### world-stepgame-graph

Validate two query agents; create v and -v for each two-agent sentence; seed q1 zero and assign vacant neighbors through BFS. Return sign label and deduplicated suspect-id count. BFS recurrence p[q1]=0 and p[y]=p[x]+v when y unassigned; answer label is sign(p[q0]); if q0 absent then None. Reader fallback uses pred=min argmax_k Σ_f w[f,k].

## approximant

### Forward relation

Validate two query agents; create v and -v for each two-agent sentence; seed q1 zero and assign vacant neighbors through BFS. Return sign label and deduplicated suspect-id count. BFS recurrence p[q1]=0 and p[y]=p[x]+v when y unassigned; answer label is sign(p[q0]); if q0 absent then None. Reader fallback uses pred=min argmax_k Σ_f w[f,k].

#### Domain and resource limits

Finite valid structures and allocation-success executions under the branch conditions in model.md#models; integer overflow is outside measured fixtures. Exact categorical relations have no supplied percent surrogate. Macro expansion, arbitrary model quality and external effects are not inferred from syntax.

## invariant

# Invariant

Adjacency insertion order breaks path ties; suspect count is distinct ids despite message saying sentences.

## Cost

Story reader cost plus O((V+E)logV), clones and suspect search; i32 addition unchecked.

## composition

### Composition

This scope contributes graph to its parent. Its forward, complement and preimage are restricted to the bound byte interval. The parent syntax projection is referenced once through ../syntax.json; no independent semantic AST is claimed.

## models

## behavior_complement

Exclude result for unreachable target or one deduplicated query agent. Inconsistent cycles are not reconciled.

## reverse_reconstruction

Left fiber comprises Reader/story states with negative x,zero y first-path target displacement; exact diagnostic offset leaves graph underdetermined. Pre(left)={(R,I):p[q0]=(x,0),x<0}; Pre(label,offset=d) additionally constrains p[q0]=d.

## origin

q[1] zero; edge a1→a0 adds read(a0,a1).

## empty_story

No valid edges leaves distinct query target unreachable.

## read_boundary

Fixture marks phrasing seen to force zero-weight perceptron regardless of lexical environment.

## sign_projection

-2,0 and -1,0 both left; explanation retains magnitude.

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
