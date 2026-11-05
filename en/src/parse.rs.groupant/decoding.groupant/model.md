# en-parse-decoding

## descriptant

### parse: decoding

Beam expands legal constrained actions, sorts scores with gold lowered on ties, clamps width to1 and clones survivors. Labels are greedy except forced/gold labels. Parse trace replays winning path; kbest collapses equal trees; constrained returns None on empty beam.

## approximant

### decoding forward scope

Beam expands legal constrained actions, sorts scores with gold lowered on ties, clamps width to1 and clones survivors. Labels are greedy except forced/gold labels. Parse trace replays winning path; kbest collapses equal trees; constrained returns None on empty beam.

Composition rejoins the parent at its explicit state/representation boundaries; guarantees apply only to this byte range, not unrelated parent branches.

## invariant

### State and cost

Parser holds dense perceptron type/label tables, labels in encounter order, sorted known-word ids, training clock and beam. Config holds root0 stack, next buffer, heads/labels, extrema and saturating child counts.

A legal nonempty completed parse has n arcs and exactly one root attachment. No transition attaches root as dependent. Greedy label ties choose latest encountered label, not lexical order. Constraints do not guarantee existence with finite beam; nonprojective training may stop/skip. Tables do not validate all resource dimensions.

2n legal transitions; greedy scores O(n*F*(3+R)). Beam adds candidate sorting O(n*b log b) and clones O(n) configuration/path per surviving step, giving O(b*n^2) clone work. Constraints scan n for child readiness. Dense deserialization allocates rows*k with unchecked arithmetic.

## models

## behavior_complement

A legal nonempty completed parse has n arcs and exactly one root attachment. No transition attaches root as dependent. Greedy label ties choose latest encountered label, not lexical order. Constraints do not guarantee existence with finite beam; nonprojective training may stop/skip. Tables do not validate all resource dimensions.

## reverse_reconstruction

For fixed words/tags and resulting tree, compatible parser states are all weight tables/label orders whose selected transition path builds that tree. Multiple action paths can induce one tree and kbest collapses them; tree cannot recover scores or training history. Sparse table decoding reconstructs weights exactly but loses averaging accumulators.

## root_legality

Shift requires b<=n; Left requires stack>=2 and second-top!=0; Right requires stack>=2 and either second-top!=0 or b>n.

## tie_and_zero

Greedy total_cmp favors later ACTS; beam stable equal scores preserve enumeration except gold sorts below nongold. width0 clamps1; empty kbest returns one zero-score empty tree.

## sparse_transport

Checks length consistency, duplicate feature keys, then class bounds; duplicate class entries overwrite. Signed zero is serialized because to_bits()!=0; u accumulator is empty after load.

## training_boundary

Context count asserted before filtering; tagged-only inputs retained. Word frequency>=2 sorted for ids. EN_PX cached once; training seed is delegated to ctx. Invalid heads/lengths can panic.

## tests

### Evidence

Compiled source mutants cover construct rounded-floor boundary, parse root legality and store checksum gate. Other sources have no compiled source mutation claim. Preflight empty-model nonempty tagging failed in ptag; synthetic fixtures adjusted without production changes. Existing corpus/seeds tests not selected.

Partial source-bound model, not full proof. No useful executable percentage surrogate established for these categorical decisions/effects; budgets1/2/5/10% require a future observation metric. No corpus/API/GPU/teacher, allocation fault injection, adversarial size exhaustion or broad accuracy claim.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31. Exact byte range/hash in manifest. Partial lexical AST is shared with parent; no macro expansion, semantic proof or executable percent surrogate. Parent tests exercise selected reachable paths, not every statement in this fragment.

### 2026-10-31 · tests

Actual source test: parse::wave27::bounded_contract. Command: cargo test -p en --lib wave27 -- --test-threads=1 (debug twice, release twice). External evidence: $MOVA_DATA/runs/octopus-wave27-structure-2026-10-31. Nine focused tests, each in its owned production module; no copied implementation.
