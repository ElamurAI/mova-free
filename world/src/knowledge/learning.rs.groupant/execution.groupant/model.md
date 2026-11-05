# learning-memo-execution

## model

# execution fragment

Source coverage: UTF-8 bytes [886, 4312) in learning.rs at the manifest digest. Memo states, configuration latch and PragStudent lesson execution implementation. This is a source-derived partial model, not a new runtime unit.

## Forward approximant

Resolve reuse once in the public wrapper; select each question's weight block; create a fresh memo vector and run ordered visits. Memo classification supplies eligibility, while ordered-visits supplies transition order and observations. Final work combines evaluated, reused and inert visits.

## Invariant

Within a call, world/options/labels stay fixed. A stored scored snapshot precedes the corresponding update. Every completed visit is observed; no memo crosses a call boundary.

## Cost approximation

For E exercises, P passes and block width W: preparation plus O(P*E*W) worst-case validity comparisons, reached evaluation/update costs, O(E*(W+answer/options footprints)+P) retained space when timing is enabled.

## Behavior complement

Let U be finite normal lesson-helper traces with valid right indices, stable world/options, evaluator block dependence and nonoverflowing counters, and R the actual relation restricted to U. The candidate inner approximant C_hat to U \ R contains traces that reuse an Empty slot, reuse a previous call's memo, or observe a visit when passes=0. This is an explicit excluded relation, not merely a list of negative tests; it does not enumerate the entire complement or prove completeness.

## Reverse reconstruction

B(final weights,work) is the set of initial weights, ordered exercises, pass counts, reuse choices and compatible evaluator answers producing that observation. Configuration is not uniquely recoverable: disabled reuse and all-invalid memos can both evaluate every visit.

## Specialized relation

New calls allocate Empty slots even when the previous call concluded Inert. New supporting facts may therefore change eligibility; unchanged weights alone cannot justify cross-call reuse.

## Epoch memory and limits

## composition

### Execution composition

Memo-policy classifies and validates answers; ordered-visits controls when those decisions affect evaluation and learning. The latter calls valid before the branch, and remember before learn_answer on a cache miss. Scored hits still update weights. These order constraints are essential: correct leaf classifiers alone do not justify a caller that snapshots after an update or suppresses learning on reuse.

The execution parent also contains enabled(), ask_weights() and the public wrapper, which are outside the two child ranges. The wrapper's OnceLock decision precedes the helper even when passes=0. ask_weights defines the block used by both remembering and validation. These bridges are modeled here, not silently attributed to a child.

Assume valid indices, stable world/options/labels during a call, evaluator dependence on the selected block, learner inactivity for Inert answers, and nonoverflowing arithmetic. Under those assumptions the composition targets equality of ordered weights with recomputation. It does not claim equivalence of evaluator side effects, timing, panic behavior, or arbitrary mutable caches. The 270-case existing fixture is bounded evidence for wiring; no universal semantic proof is supplied.

## tests

### Existing bounded oracles

Existing new_facts_cannot_reuse_an_inert_previous_lesson exercises fresh facts and zero passes. Existing bounded_cache_reference_traces_cover_weights_orders_seeds_and_passes checks 270 fixture configurations, not every real evaluator behavior.

Run the existing suite from the repository root:

`cargo test -p world knowledge::learning::tests --lib --locked`

Test definitions are in ../../learning.rs. These are references to existing oracles; this fragment adds no executable test. Historical execution evidence is retained in ../model.md#tests. The fixture's simplified evaluator does not establish full parser/inference correspondence.

## History

### 2026-10-29 · Epoch memory and limits

2026-10-29: decomposed existing source without moving code. Existing root observations and all twenty root obernants remain authoritative. Child anchors select source-bound partial syntax nodes; name resolution and macro expansion remain absent. No new runtime test or measured performance gain is claimed.

### 2026-11-04 · Epoch memory and limits

2026-11-04 (student-rules): the evaluator is `answer_by_weights`, the answer with the compiled rules left out, so a rule decision at answer time never changes which exercises update the weights (with rules in the learning path, the rows no rule decided moved +1/−9 on the ten courses).
