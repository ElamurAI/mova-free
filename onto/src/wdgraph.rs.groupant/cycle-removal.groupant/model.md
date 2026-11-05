# onto-wdgraph-cycle-removal

## descriptant

### onto-wdgraph-cycle-removal

Remove R={(s,o,p)∈E:comp(s)=comp(o)∧(size(comp)>1∨s=o)}; keep E\R, sort audit records and rebuild CSR. Kahn returns None iff all n vertices eliminated via indegree-zero queue.

## approximant

### Scoped forward relation

Remove R={(s,o,p)∈E:comp(s)=comp(o)∧(size(comp)>1∨s=o)}; keep E\R, sort audit records and rebuild CSR. Kahn returns None iff all n vertices eliminated via indegree-zero queue.

Scope bytes [6353,8449) only. Preconditions and exceptions in this scope's catalog; no claims for parent orchestration.

Executable approximant gap: no useful percentage metric established for these categorical/effectful observations. No surrogate or production substitution. Future caps 1/2/5/10% require a metric/domain and evidence; no numerical error bound follows here.

## invariant

# Scoped observations

No R edge remains in output for valid CSR; node preservation and minimal removal are excluded claims. Residual Kahn vertices may include downstream noncyclic ones.

## Cost

Tarjan/Kahn O(V+E); kept-graph reconstruction sorts O(Ekeep log Ekeep), O(V+E) storage. u32 sizes/indices require representability.

## composition

### Scope contribution

Remove R={(s,o,p)∈E:comp(s)=comp(o)∧(size(comp)>1∨s=o)}; keep E\R, sort audit records and rebuild CSR. Kahn returns None iff all n vertices eliminated via indegree-zero queue.

This fragment contributes only `cycle-removal`. Parent owns input acquisition, surrounding dispatch and downstream joins. Complement and compatible antecedents are scoped in model.md#models rather than copied parent statements.

## models

## behavior_complement

No R edge remains in output for valid CSR; node preservation and minimal removal are excluded claims. Residual Kahn vertices may include downstream noncyclic ones.

## reverse_reconstruction

E_unique=keep.edges∪⋃cycle.edges. Duplicate multiplicities unrecoverable; keep alone admits extra isolated cyclic components.

## self_loop

Singleton s→s produces cycle record.

## index_rebuild

Deleting4↔5 removes both if no other edges and shifts later indices.

## audit_order

Members/edges ascending; cycles size descending then members.

## gate_cost

Tarjan/Kahn O(V+E), rebuild O(Ekeep log Ekeep), storage O(V+E).

## tests

### Evidence boundary

cycles_gate covers removal/renumber, BFS, two-thread count-only closure and injected cycle. Deep200000 chain skipped, no dumps/materialization.

Parent model.md#tests names actual selected tests; this scope does not claim all its branches executed. The compiled control listed in parent receipt applies only to the named mutated boundary.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: source byte range [6353,8449) with source and range hashes, sharing parent lexical AST. Partial source-derived scope, no semantic proof.
