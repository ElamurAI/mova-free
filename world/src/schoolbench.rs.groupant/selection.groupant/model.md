# world-schoolbench-selection

## forward
Partition by stars, compute stable digest from seed/course/n, lexically sort digest-index pairs, take ceil(n_s/d), sort global indices.

## invariant
Per nonempty stratum select at least one and at most n_s; Full selects every original index exactly once, even duplicate identities.

## cost
O(N identity bytes + sum n_s log n_s + K log K), O(N) stored pairs; hashes compared as strings.

## composition
Leaf projections share exact lesson identities, selected indices, text bytes and option ordering. Compose only with those shared values retained; isolated score/length observations cannot recover them. Source inspection identifies knowledge::main_args dispatch, schoolbench selected-lessons -> counts -> run_course_lessons -> trace scoring, and schooltools option_line -> canon -> tally/lint/edit. Tests execute native pure helpers and isolated snapshot/restore; no live course/parser/teacher call.

## epoch-memory
Initial coverage records tolerant parser/score semantics and per-file effect boundaries. Model review exposed the direct zero restore index panic, repaired after old-code failure. Benchmark score loop was extracted unchanged to a pure private helper to test actual classification and width/count gates. Lock constructor failure may leave a stale directory; argument failures can consume flags. These remain explicit source-inspected limitations.

## checks

Native module filters schoolbench:: and schooltools::, including eight new wave9 tests; independent finite rank/cadence/length/fraction oracles and a restore no-effect fixture. Raw evidence is external under (see History); root ../model.md#evidence binds outcomes. Macro bodies in syntax projection remain opaque; finite agreement does not certify unbounded input space.

## behavior-complement
Evidence/status: source-inspected relation with the bounded tests named under checks; unenumerated behavior remains unknown. Complement claims are inner constraints, reverse claims retain all compatible candidates rather than identifying a historical cause.

## reverse-reconstruction
Evidence/status: source-inspected relation with the bounded tests named under checks; unenumerated behavior remains unknown. Complement claims are inner constraints, reverse claims retain all compatible candidates rather than identifying a historical cause.

## rank-oracle
Evidence/status: source-inspected relation with the bounded tests named under checks; unenumerated behavior remains unknown. Complement claims are inner constraints, reverse claims retain all compatible candidates rather than identifying a historical cause.

## nested-subsets
Evidence/status: source-inspected relation with the bounded tests named under checks; unenumerated behavior remains unknown. Complement claims are inner constraints, reverse claims retain all compatible candidates rather than identifying a historical cause.

## History

### 2026-10-30 · document

Source-bound epoch 2026-10-30. Every view below uses this exact source revision and finite inputs with sufficient allocation resources. Forward/absent/reverse relations observe only the stated projection; filesystem models assume fixed supplied dependencies and distinguish successful returns from partial effects. Status: partial source model, with bounded native evidence in ../model.md#evidence at the root sidecar. No approximation percentage, universal proof or school accuracy claim.

### 2026-10-30 · checks

Native module filters schoolbench:: and schooltools::, including eight new wave9 tests; independent finite rank/cadence/length/fraction oracles and a restore no-effect fixture. Raw evidence is external under $MOVA_DATA/runs/octopus-wave9-school-2026-10-30; root ../model.md#evidence binds outcomes. Macro bodies in syntax projection remain opaque; finite agreement does not certify unbounded input space.

### 2026-10-30 · behavior-complement

Type: behavior_complement. Subject: world-schoolbench-selection, epoch 2026-10-30. Domain and observation: scoped inputs and projected outputs defined here. Operator/assumptions: For fixed inputs, inner absent results include reordered output, omissions in full, and selections violating per-stratum quota or rank. No assertion that different seeds necessarily yield different subsets.

### 2026-10-30 · reverse-reconstruction

Type: reverse_reconstruction. Subject: world-schoolbench-selection, epoch 2026-10-30. Domain and observation: scoped inputs and projected outputs defined here. Operator/assumptions: Selected indices constrain only quota and winning ranks. Any replacement lesson facts/questions with unchanged course/n/stars is compatible; rank ties preserve original indices. Hash preimage reconstruction is not implemented.

### 2026-10-30 · rank-oracle

Type: constraint_relation. Subject: world-schoolbench-selection, epoch 2026-10-30. Domain and observation: scoped inputs and projected outputs defined here. Operator/assumptions: Independent bounded oracle counts peers with smaller (digest,index) and includes i iff rank*d < stratum-size; 49 lengths times three seeds times three tiers =441 comparisons. It shares the digest primitive, so hash correctness is external.

### 2026-10-30 · nested-subsets

Type: order_relation. Subject: world-schoolbench-selection, epoch 2026-10-30. Domain and observation: scoped inputs and projected outputs defined here. Operator/assumptions: For same strata/ranks, ceil(n/8)<=ceil(n/3)<=n; prefixes compose into subset inclusion after chronological sorting. Changing lessons, stars or seed voids this premise.
