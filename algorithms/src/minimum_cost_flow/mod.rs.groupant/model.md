# algorithm-minimum-cost-flow

## descriptant

### minimum_cost_flow mod

Original std-only Rust. Prospective model is retained beside implementation. Source citations identify algorithm provenance; no copied implementation. Exact bounded graph semantics are useful independently of any unmeasured optimization. Finite oracle checks and mutant discrimination improve confidence; groupant count alone proves no improvement.

## model

# minimum_cost_flow mod model

## Forward

Successive shortest augmenting paths, Bellman-Ford over residual arcs. Bound n<=64,m<=512,requested<=1024; reject any original positive-capacity negative cycle globally before even zero requested flow. Flow integral and bounded; shortest residual augmentation preserves minimum cost for current value. Maximize value<=requested then minimize signed i128 cost. Input edge order resolves strict relaxation ties. Checked arithmetic. Cost O((limit+1)V(E+V)), storage O(V+E). No speed claim.

## Invariant

Each residual pair sums to original capacity. Internal flow conservation holds. No negative residual cycle; each augmentation attains a shortest residual path and increases integer value.

## Cost

Successive shortest augmenting paths, Bellman-Ford over residual arcs. Bound n<=64,m<=512,requested<=1024; reject any original positive-capacity negative cycle globally before even zero requested flow. Flow integral and bounded; shortest residual augmentation preserves minimum cost for current value. Maximize value<=requested then minimize signed i128 cost. Input edge order resolves strict relaxation ties. Checked arithmetic. Cost O((limit+1)V(E+V)), storage O(V+E). No speed claim.

## nonbehavior

Type: behavior_complement. Let R be the deterministic relation specified by Forward including validation precedence and tie rules, X the declared input type and Y its Result type. Behavior complement is (X times Y) minus R. Specifically outputs violating Each residual pair sums to original capacity. Internal flow conservation holds. No negative residual cycle; each augmentation attains a shortest residual path and increases integer value. are excluded; a valid-input Err or a wrong successful witness is excluded. Resource, cycle and structural rejections are determined before algorithmic output.

## compatible_inputs

Type: reverse_reconstruction. Given (value,cost,edge_flows), antecedents satisfy capacity bounds, conservation, sum f_e*c_e=cost and lexicographic optimality under limit. Different unused capacities/costs yield the same output. InvalidEdge identifies first bad index; NegativeCycle recovers existence, not location.

## potential_transport

Type: potential_reweighting. Costs c'(u,v)=c(u,v)+h(u)-h(v), all fitting i64, preserve cycle sums and optimal flow value; optimal cost changes by value*(h(source)-h(sink)). Chosen witness can differ only among ties.

## zero_capacity

Type: irrelevant_input. Adding a zero-capacity edge cannot change value or cost, though edge_flows gains a zero. Its endpoints must still validate.

## capacity_certificate

Type: feasibility_certificate. For every output, 0<=f_e<=capacity_e; incoming minus outgoing equals value at sink, -value at source and zero elsewhere. Objective is exact sum f_e*c_e.

## limit_monotonicity

Type: monotone_extension. For limits L<=L' within 1024, achieved value cannot decrease. Cost need not increase when costs are negative. Capped value is min(L, maximum feasible flow).

## Limits

Partial syntax omits macro expansion and type resolution. No executable approximate profile: categorical graph semantics and feasibility have no justified percentage relaxation here. Integer operations are bounded by caps; flow uses checked i128/u64 arithmetic, with Overflow defensive under current caps. No callbacks. Allocator failure is outside typed status.

## evidence

Eleven tests across five algorithms, including independent exhaustive small-domain characterization. This file belongs to minimum_cost_flow. Two debug and two optimized runs matched normalized statuses. Strict clippy-driver --test -D warnings passed. A compiled production mutant failed a named assertion; exact bytes restored and positive rerun passed. Raw receipts: (see History). No timing or whole-system improvement claim. Source SHA-256: (see History).

### Evidence

## prospective

### Prospective model

Successive shortest augmenting paths, Bellman-Ford over residual arcs. Bound n<=64,m<=512,requested<=1024; reject any original positive-capacity negative cycle globally before even zero requested flow. Flow integral and bounded; shortest residual augmentation preserves minimum cost for current value. Maximize value<=requested then minimize signed i128 cost. Input edge order resolves strict relaxation ties. Checked arithmetic. Cost O((limit+1)VE), storage O(V+E). No speed claim.
Allocation failure is outside typed errors. Resource caps precede vertex/edge validation. No callbacks. Exact categorical output has no justified percentage surrogate; executable approximation profiles unsupported.

## History

### 2026-10-30 · evidence

Heading `### Evidence — 2026-10-30` stays in evidence as `### Evidence`.

Eleven tests across five algorithms, including independent exhaustive small-domain characterization. This file belongs to minimum_cost_flow. Two debug and two optimized runs matched normalized statuses. Strict clippy-driver --test -D warnings passed. A compiled production mutant failed a named assertion; exact bytes restored and positive rerun passed. Raw receipts: $MOVA_DATA/runs/octopus-wave25-graphs-2026-10-30/minimum_cost_flow-*.txt. No timing or whole-system improvement claim. Source SHA-256: a6de6ae849a183b1b508ebf475e7b1517b27bbf5c38a04dc6dbe1394e6885697.

### 2026-10-30 · prospective

Heading `### Prospective model — 2026-10-30` stays in prospective as `### Prospective model`.

### 2026-11-04 · document

Migration: consolidated into model.md, 2026-11-04, operator decision; fresh hash-bound references to the former files were rebound to the sections below.
