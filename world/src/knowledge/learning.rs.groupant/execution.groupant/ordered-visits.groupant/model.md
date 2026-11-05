# learning-memo-execution-ordered-visits

## model

# execution/ordered-visits fragment

Source coverage: UTF-8 bytes [2236, 4310) in learning.rs at the manifest digest. learn_lesson_with function, excluding the surrounding impl closing brace. This is a source-derived partial model, not a new runtime unit.

## Forward approximant

Return default work for P=0. Otherwise shuffle options once, allocate Empty memos, and traverse P passes in exercise order. A valid Inert increments inert only; valid Scored increments reused then learns; a miss evaluates, increments evaluated, snapshots before learning when reuse is enabled. Observe after every visit.

## Invariant

Under normal return and nonoverflowing counts, evaluated+reused+inert=P*E. Each call begins with Empty slots; its nonempty first pass evaluates every exercise. Reusing an Answer never bypasses the scored learning update.

## Cost approximation

O(E) preparation plus P*E validity/dispatch work and reached evaluator/learner costs; timing appends at most P values. E=0 still permits P timing samples.

## Behavior complement

Let U be finite helper traces with fixed inputs, valid indices, normal completion and nonoverflowing counters, and R the actual relation restricted to U. The candidate inner approximant C_hat to U \ R contains traces whose work counters do not sum to P*E, whose observer count differs from P*E, or whose scored snapshot occurs after its associated learning update. This is an explicit excluded relation, not merely a list of negative tests; it does not enumerate the entire complement or prove completeness.

## Reverse reconstruction

B(work,observed weights) contains all ordered visit branch sequences and input states compatible with those counts and vectors. Totals lose exercise attribution; P=0 and E=0 can both yield zero counters. Observations do not uniquely recover answers or lesson seeds.

## Specialized relation

The target relation equates every ordered weight vector to reuse=false recomputation for the same exercise order. It requires fixed evaluator dependencies and that Inert answers cannot update reference state; equal final scores alone are weaker.

## Epoch memory and limits

## tests

### Existing bounded oracles

Existing every_ordered_update_matches_recomputation_over_multiple_passes checks a nontrivial 30-visit trace. Existing bounded_cache_reference_traces_cover_weights_orders_seeds_and_passes compares ordered and final vectors over 270 cases, exercising pass counts, seeds, orders and initial weights.

Run the existing suite from the repository root:

`cargo test -p world knowledge::learning::tests --lib --locked`

Test definitions are in ../../../learning.rs. These are references to existing oracles; this fragment adds no executable test. Historical execution evidence is retained in ../../model.md#tests. The fixture's simplified evaluator does not establish full parser/inference correspondence.

## History

### 2026-10-29 · Epoch memory and limits

2026-10-29: decomposed existing source without moving code. Existing root observations and all twenty root obernants remain authoritative. Child anchors select source-bound partial syntax nodes; name resolution and macro expansion remain absent. No new runtime test or measured performance gain is claimed.
