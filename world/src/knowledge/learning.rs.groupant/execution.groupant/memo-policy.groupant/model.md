# learning-memo-execution-memo-policy

## model

# execution/memo-policy fragment

Source coverage: UTF-8 bytes [886, 1702) in learning.rs at the manifest digest. Memo enum and remember/valid implementation. This is a source-derived partial model, not a new runtime unit.

## Forward approximant

remember yields Inert when !understood, count exists, or correct option has no best support; otherwise Scored stores the Answer and its current ask block. valid maps Empty to false, Inert to true, and Scored to exact equality of the stored/current selected block.

## Invariant

A supported currently-correct answer is still Scored. Inert eligibility is structural under the fixed-world assumption. Equality of a block is a validity predicate, not a proof about every dependency of an arbitrary evaluator.

## Cost approximation

Scored remember copies W weights and valid compares up to W entries; Empty/Inert operations are O(1). Answer and Vec allocations contribute storage.

## Behavior complement

Let U be normal remember/valid input-output observations with valid correct-option indices and readable weight blocks, and R the actual relation restricted to U. The candidate inner approximant C_hat to U \ R contains valid(Empty)=true, valid(Scored)=true for unequal selected blocks, or remember classifying a supported understood non-count answer as Inert. This is an explicit excluded relation, not merely a list of negative tests; it does not enumerate the entire complement or prove completeness.

## Reverse reconstruction

B(Inert) includes all answers satisfying at least one eligibility disjunct, without selecting which disjunct held. B(valid=true) includes Inert and matching Scored states. A boolean cannot reconstruct the answer, world or weight history.

## Specialized relation

For a fixed ask, w~v iff their VIAS.len()*NF selected blocks agree. Scored validity is constant on these classes; answer equivalence additionally requires evaluator dependence only on that block and fixed world/options.

## Epoch memory and limits

## tests

### Existing bounded oracles

Existing changed_winner_invalidates_but_another_weight_block_does_not probes an unchanged selected block, a changed winner and the stale cached-answer negative control. This is a concrete fixture, not a universal dependency proof.

Run the existing suite from the repository root:

`cargo test -p world knowledge::learning::tests --lib --locked`

Test definitions are in ../../../learning.rs. These are references to existing oracles; this fragment adds no executable test. Historical execution evidence is retained in ../../model.md#tests. The fixture's simplified evaluator does not establish full parser/inference correspondence.

## History

### 2026-10-29 · Epoch memory and limits

2026-10-29: decomposed existing source without moving code. Existing root observations and all twenty root obernants remain authoritative. Child anchors select source-bound partial syntax nodes; name resolution and macro expansion remain absent. No new runtime test or measured performance gain is claimed.
