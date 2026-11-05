# world-story-knowledge

## forward

Initialize root; look up metadata; compute narrator changes recursively; known_since scans candidates with strict minimum replacement and stable first tie.

## invariant

Missing metadata is_real is true. Narrator level changes only across unequal observers. Valid parent trees are finite and levels fit u8.

## cost

known_since O(K+R) plus formatting; narr_depth can be O(F squared) on a chain.

## composition

Text sentence minima are not story chronology. knowledge_minimum_ties_and_narrator_depth checks ties, absent observers and distinct geometric/narrator depths.

## checks

Native story::wave9_models tests exercise the bounded domains described in ../model.md#evidence at the file root. Ordinary test bodies have partial syntax projections; macro token contents are opaque. The graph oracle uses a dense three-state Floyd closure, not the implementation BFS/bitset algorithm. Typed commands call the real public World::apply boundary.

## epoch-memory

Initial modeling epoch: distinguish admitted graph states from arbitrary public-field edits, and internal mutation from public commit. No production bug was established. In particular narr::relate restores its multi-edge batch, and World::apply discards a cloned failed command even when apply_root has already added a cause edge. Creation ordering has a deliberate 64-round fallback, not an unrestricted topological-sort guarantee. No percentage-error approximant or performance claim.

## behavior-complement

Subject: world-story-knowledge, source-bound revision. Domain and observation: the finite typed state/transition family in forward, with dependencies fixed. Operator: An unknown observer without a matching know/reveal/narration role receives no known_since result. A later candidate cannot replace an earlier one.

Assurance: source-derived partial relation, with finite native evidence where named in checks. Unknown paths remain unknown; no universal coverage or unique inverse is inferred. Complement claims are inner absent-behavior bounds under their stated premises; reverse relations retain all compatible antecedents in their declared family.

## reverse-reconstruction

Subject: world-story-knowledge, source-bound revision. Domain and observation: the finite typed state/transition family in forward, with dependencies fixed. Operator: Given earliest (sentence,explanation), retain all later knowledge/reveal histories plus tie alternatives consistent with first-candidate priority. The output never identifies a unique learning history.

Assurance: source-derived partial relation, with finite native evidence where named in checks. Unknown paths remain unknown; no universal coverage or unique inverse is inferred. Complement claims are inner absent-behavior bounds under their stated premises; reverse relations retain all compatible antecedents in their declared family.

## coupling

Subject: world-story-knowledge, source-bound revision. Domain and observation: the finite typed state/transition family in forward, with dependencies fixed. Operator: Text sentence minima are not story chronology. knowledge_minimum_ties_and_narrator_depth checks ties, absent observers and distinct geometric/narrator depths.

Assurance: source-derived partial relation, with finite native evidence where named in checks. Unknown paths remain unknown; no universal coverage or unique inverse is inferred. Complement claims are inner absent-behavior bounds under their stated premises; reverse relations retain all compatible antecedents in their declared family.

## History

### 2026-10-30 · document

Partial source-bound model, 2026-10-30. The reference is the exact sibling Rust source. The domain is valid finite typed values reachable through checked World::apply, except explicitly constructed graph/helper fixtures. Public mutable graph/frame fields are not defended against arbitrary corruption. No live parser, teacher, corpus, GPU or persistence is exercised.
