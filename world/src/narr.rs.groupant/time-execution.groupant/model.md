# world-narr-time-execution

## forward

Translate TRel to one/two endpoint inequalities; snapshot graph for batch rejection. register adds spans, placement/line, pending causes, bounds and metadata; unchain kills line/frame edges and stitches neighbors; causal_stamp deduplicates off-line real causes.

## invariant

relate rollback is local to its inserted relation. register is not independently atomic. Placement does not join line; soft causal contradictions record warnings.

## cost

relate clones graph and runs <=2 BFS; register BFS work per pending cause plus fixed edges; unchain scans stored edges and line ids.

## composition

relation_endpoint_oracle_and_atomic_pair enumerates interval assignments; unchain_stitches_and_pending_consumes_once exercises actual register/unchain/causal_stamp coupling.

## checks

Native narr::wave9_models tests exercise the bounded domains described in ../model.md#evidence at the file root. Ordinary test bodies have partial syntax projections; macro token contents are opaque. The graph oracle uses a dense three-state Floyd closure, not the implementation BFS/bitset algorithm. Typed commands call the real public World::apply boundary.

## epoch-memory

Initial modeling epoch: distinguish admitted graph states from arbitrary public-field edits, and internal mutation from public commit. No production bug was established. In particular narr::relate restores its multi-edge batch, and World::apply discards a cloned failed command even when apply_root has already added a cause edge. Creation ordering has a deliberate 64-round fallback, not an unrestricted topological-sort guarantee. No percentage-error approximant or performance claim.

## behavior-complement

Subject: world-narr-time-execution, source-bound revision. Domain and observation: the finite typed state/transition family in forward, with dependencies fixed. Operator: A failed second During inequality cannot leave its first inequality in relate's result. Removed middle line event cannot remain in line.main after successful unchain.

Assurance: source-derived partial relation, with finite native evidence where named in checks. Unknown paths remain unknown; no universal coverage or unique inverse is inferred. Complement claims are inner absent-behavior bounds under their stated premises; reverse relations retain all compatible antecedents in their declared family.

## reverse-reconstruction

Subject: world-narr-time-execution, source-bound revision. Domain and observation: the finite typed state/transition family in forward, with dependencies fixed. Operator: One observed order admits many line/placement/cause histories; unchain erases active line participation but retains dead-edge records. Pending cause dedup loses subsequent duplicate line stamps.

Assurance: source-derived partial relation, with finite native evidence where named in checks. Unknown paths remain unknown; no universal coverage or unique inverse is inferred. Complement claims are inner absent-behavior bounds under their stated premises; reverse relations retain all compatible antecedents in their declared family.

## coupling

Subject: world-narr-time-execution, source-bound revision. Domain and observation: the finite typed state/transition family in forward, with dependencies fixed. Operator: relation_endpoint_oracle_and_atomic_pair enumerates interval assignments; unchain_stitches_and_pending_consumes_once exercises actual register/unchain/causal_stamp coupling.

Assurance: source-derived partial relation, with finite native evidence where named in checks. Unknown paths remain unknown; no universal coverage or unique inverse is inferred. Complement claims are inner absent-behavior bounds under their stated premises; reverse relations retain all compatible antecedents in their declared family.

## History

### 2026-10-30 · document

Partial source-bound model, 2026-10-30. The reference is the exact sibling Rust source. The domain is valid finite typed values reachable through checked World::apply, except explicitly constructed graph/helper fixtures. Public mutable graph/frame fields are not defended against arbitrary corruption. No live parser, teacher, corpus, GPU or persistence is exercised.
