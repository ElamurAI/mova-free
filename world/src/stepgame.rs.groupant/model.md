# world-stepgame

## descriptant

### world-stepgame

load_txt resets at literal 1 and clones story on tabbed questions. JSON defaults absent fields, maps invalid numeric ids to zero, stably sorts numeric ids. read prioritizes perceptron for seen phrasing, otherwise lexical unless environment disables it. Scores tie by lowest class index. answer requires two distinct query agents, builds reciprocal edges, BFS from reference and projects displacement to sign class. train uses deterministic stride and integer accumulated weights; eval/contrast perform dataset/report effects. Equations: score_k=Σ_{f in feats}w[f,k], pred=min argmax_k score_k; p[q1]=0; vacant p[y]=p[x]+v_xy; label=LABELS[(sign p[q0].x, sign p[q0].y)]. lexical axes return None if both signs occur, otherwise first nonzero sign or zero; ground=a1 negates both axes.

## approximant

### Forward relation

load_txt resets at literal 1 and clones story on tabbed questions. JSON defaults absent fields, maps invalid numeric ids to zero, stably sorts numeric ids. read prioritizes perceptron for seen phrasing, otherwise lexical unless environment disables it. Scores tie by lowest class index. answer requires two distinct query agents, builds reciprocal edges, BFS from reference and projects displacement to sign class. train uses deterministic stride and integer accumulated weights; eval/contrast perform dataset/report effects. Equations: score_k=Σ_{f in feats}w[f,k], pred=min argmax_k score_k; p[q1]=0; vacant p[y]=p[x]+v_xy; label=LABELS[(sign p[q0].x, sign p[q0].y)]. lexical axes return None if both signs occur, otherwise first nonzero sign or zero; ground=a1 negates both axes.

#### Domain and resource limits

Finite valid structures and allocation-success executions under the branch conditions in model.md#models; integer overflow is outside measured fixtures. Exact categorical relations have no supplied percent surrogate. Macro expansion, arbitrary model quality and external effects are not inferred from syntax.

## invariant

# Invariant

LABELS order is tie order. Coordinates mean first agent relative second, not meters. Deduplicated single ASCII capital agents only. finish retains scaled sums, not divided average; avg is not a read switch.

## Cost

Features O(B) allocated ngrams; score O(9F) expected hash operations; lexical worst O(T²) backward cue scanning; BFS O((V+E)logV) plus clones. Training O(epochs*N*F*9); stride not a permutation if gcd(N,7919)>1.

## composition

### Composition

Scoped children: formats, lexical, graph. load_txt resets at literal 1 and clones story on tabbed questions. JSON defaults absent fields, maps invalid numeric ids to zero, stably sorts numeric ids. read prioritizes perceptron for seen phrasing, otherwise lexical unless environment disables it. Scores tie by lowest class index. answer requires two distinct query agents, builds reciprocal edges, BFS from reference and projects displacement to sign class. train uses deterministic stride and integer accumulated weights; eval/contrast perform dataset/report effects. Equations: score_k=Σ_{f in feats}w[f,k], pred=min argmax_k score_k; p[q1]=0; vacant p[y]=p[x]+v_xy; label=LABELS[(sign p[q0].x, sign p[q0].y)]. lexical axes return None if both signs occur, otherwise first nonzero sign or zero; ground=a1 negates both axes. Other functions and test additions are modeled only at this parent. bAbI Story and command World have different representations: command-language fixtures do not certify the UD parser adapter. StepGame lexical/load helpers connect directly to tests03; CLI helpers construct a separate world process but this wave does not spawn it. No composition-wide percent accuracy follows.

## models

## behavior_complement

Exclude lexical vectors for clock tokens, conflicting signs on an axis or directionless text without sameness; exclude answer for invalid agent count or unreachable target. Contradictory cycles still return first path. Zero perceptron intentionally yields left, not abstention. C_answer includes (q,G,Some(label)) whenever |agents(q)|≠2 or q0∉Reach_G(q1). C_lex includes (s,Some(v)) whenever clock(s) or axis_conflict(s).

## reverse_reconstruction

Left label admits every graph with first BFS displacement x<0,y=0; explanation narrows to exact d but never unique story. phrasing collapses letter renamings/punctuation/case. load_txt loses support fields and original numbers. Predicted class k admits weight states where k is smallest maximizing score. Pre(left)={ (R,I) | BFS_edges(R,I)(q1)[q0]=(x,0), x<0 }.

## tie_selection

Empty/default scores choose index0 left; repeated feature tokens count repeatedly.

## lexical_ground

Unique cue wins then possessive subject then a2; right before under is intensifier.

## training_schedule

t increments even for correct prediction; finish sums (t-last)*weight. Integer overflow is not certified.

## empty_evaluation

Printed raw percentages can NaN at zero; contrast modulo-zero possible with no eligible templates and samples>0. Live data paths excluded.

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

Read full assigned source before editing; prospective predictions retained in external wave29 receipts. Stored/live/parser-dependent test paths are excluded from execution. Evidence is limited to named bounded fixtures; registration is partial modeling. Production behavior unchanged; only inline synthetic tests added to four assigned sources.

### 2026-10-31 · tests

External receipt root: $MOVA_DATA/runs/octopus-wave29-worldeval-2026-10-31.
