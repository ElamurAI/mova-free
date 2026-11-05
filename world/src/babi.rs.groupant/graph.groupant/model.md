# world-babi-graph

## descriptant

### world-babi-graph

holds canonicalizes relation then uses dot(offset,vec)>0 for vectors or forward reach before inverse reach for transitive relations. offset BFS subtracts signed adjacency vectors. path BFS retains first predecessor and reverses collected steps. Equations: pos[b]=(0,0); on first edge y→x carrying v set pos[x]=pos[y]−v. holds(a,r,b)=Some(pos[a]·vec(r)>0) when reachable. Transitive truth is reach_r(a,b), else false if reach_inverse(r)(a,b), else None.

## approximant

### Forward relation

holds canonicalizes relation then uses dot(offset,vec)>0 for vectors or forward reach before inverse reach for transitive relations. offset BFS subtracts signed adjacency vectors. path BFS retains first predecessor and reverses collected steps. Equations: pos[b]=(0,0); on first edge y→x carrying v set pos[x]=pos[y]−v. holds(a,r,b)=Some(pos[a]·vec(r)>0) when reachable. Transitive truth is reach_r(a,b), else false if reach_inverse(r)(a,b), else None.

#### Domain and resource limits

Finite valid structures and allocation-success executions under the branch conditions in model.md#models; integer overflow is outside measured fixtures. Exact categorical relations have no supplied percent surrogate. Macro expansion, arbitrary model quality and external effects are not inferred from syntax.

## invariant

# Invariant

offset(b,b)=(0,0), path(b,b)=[] even absent nodes; vertices assigned once.

## Cost

reach O(VE); ordered-map BFS O((V+E)log V), string/adjacency cloning extra; i32 overflow outside fixture domain.

## composition

### Composition

This scope contributes graph to its parent. Its forward, complement and preimage are restricted to the bound byte interval. The parent syntax projection is referenced once through ../syntax.json; no independent semantic AST is claimed.

## models

## behavior_complement

Exclude a displacement across disconnected nodes or nonempty self path. Contradictory graphs are not rejected; first path or forward reach wins.

## reverse_reconstruction

True vector predicate admits all reachable d with d·vec(r)>0. Path [e,n] admits all graphs with that first shortest-path labeling; other edges and lengths remain underdetermined. Pre(true;r,a,b)={G | BFS_G(b)[a]=d ∧ d·vec(r)>0}.

## axis_predicate

north_of requires positive y regardless of x; zero dot gives false when reachable.

## path_order

Relation insertion order decides competing equal-length paths.

## inverse_edges

A north_of B gives B→A north and reverse south.

## metadata_gate

Missing vec omits offset edges; missing step or inverse step omits path edges.

## tests

### Bounded checks

Named checks associated with this source (not every fragment branch):

- babi::wave29_tests::identity_and_inventory_without_parser
- babi::wave29_tests::graph_and_sentence_boundaries_without_parser

17 unique named tests in total passed twice debug and twice release; sorted named outcomes equal across all four runs. Four compiled controls cover bAbI identity, StepGame tie ordering, parser-command state removal and bench-command CPU selection; each exits101 at a runtime assertion, exact source restored then positive exit0. Other sources have finite adversarial fixtures but no separately compiled mutant.

CLI integration coverage is command construction and temporary cleanup only; legacy school/model/cadence paths are not executed. No speed benchmark, GPU, live teacher, datasets or held-out replay.

Limits:

- Partial syntax projection only; no macro expansion or proof
- No useful percentage surrogate for exact categorical/state/effect semantics; executable approximant unsupported
- No live parser, corpus, gap journal, training or held-out evaluation
- Unicode recasing byte slices, unbounded recursion/overflow not covered

## History

### 2026-10-31 · descriptant

#### Modeling epoch — 2026-10-31

Read full assigned source before editing; prospective predictions retained in external wave29 receipts. This child binds only its stated half-open UTF-8 byte range; other APIs remain parent scope. Evidence is limited to named bounded fixtures; registration is partial modeling. Production behavior unchanged; only inline synthetic tests added to four assigned sources.

### 2026-10-31 · tests

External receipt root: $MOVA_DATA/runs/octopus-wave29-worldeval-2026-10-31.
