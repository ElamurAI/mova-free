# world-mwe-judgment

Greedy gold matching, distinct overlap and metric projections.

## forward

For each prediction, scan gold order for the first unused gold whose distinct intersection with valid POS positions has size>=2 and contains a VERB. Mark it used and increment right, otherwise wrong. Return gold length unchanged; categories do not enter the predicate.

## invariant

r+w=P, r<=G. Repeating a token inside p cannot alone create a match. Repeating a whole prediction consumes another gold or increments wrong. Finite independent set-intersection oracle checks all 256 mask pairs with/without duplicated prediction members.

## cost

O(P*G*(K^2+K log K)) worst case and O(G+K) auxiliary memory, excluding input vectors. Every failed scan is bounded; no optimality claim.

## composition

Judge is independent of global dictionaries once predictions are supplied. score and pr consume its counts with different losses of information. The order fixture demonstrates maximum matching is not the implemented objective: [broad,narrow] scores one right, [narrow,broad] scores two.

## epoch-memory

First lexical epoch: old native witnesses returned raw heads [0,10,99], accepted noun-only overlap and duplicate token IDs, and panicked on an out-of-range head. Narrow fixes translate unambiguous IDs, omit invalid/missing arcs, and enforce the documented distinct/shared-verb condition. Prediction-order greedy matching remains intentionally unchanged; historical tuning scores can differ. No corpus retuning or measured speed claim.

## checks

Native filter mwe::wave10_models; see ../model.md#evidence for fragments or ../model.md#evidence at the root. Finite supplied fixtures and independent oracles; no live parser, corpus, teacher or global dictionary activation. Partial syntax has opaque macro bodies and no semantic AST. No executable percentage surrogate or speed claim.

## behavior-complement

Universe: bounded ordered prediction/gold lists with valid POS vocabulary. Inner excluded matches have no shared VERB, <2 distinct shared valid indices or already consumed gold. Finite fixtures reject noun-only and duplicate-verb false positives.

Subject: current source hash and this node's declared scope. Relation describes one terminating call over the finite inputs stated above, with fixed dependencies and successful allocation. Status: partial source-derived model; named native fixtures support only their finite domain. Unobserved effects remain unknown.

## reverse-reconstruction

For fixed sentence and observed counts, enumerate every ordered prediction list satisfying the greedy relation. Categories and duplicated members can vary without changing counts; order can also change counts. A score cannot uniquely reconstruct matched pairs, and a successful p does not identify one semantic interpretation.

Subject: current source hash and this node's declared scope. Relation describes one terminating call over the finite inputs stated above, with fixed dependencies and successful allocation. Status: partial source-derived model; named native fixtures support only their finite domain. Unobserved effects remain unknown.

## greedy-counterfactual

Fix gold [[0,1],[0,2]], with 0 verbal. Predictions [[0,1,2],[0,1]] yield (1,1,2), while reversed prediction order yields (2,0,2). Maximum bipartite matching would accept both orders; that counterfactual is explicitly not this API's semantics.

Subject: current source hash and this node's declared scope. Relation describes one terminating call over the finite inputs stated above, with fixed dependencies and successful allocation. Status: partial source-derived model; named native fixtures support only their finite domain. Unobserved effects remain unknown.

## invalid-index

Positions >=upos.len() are excluded from the overlap set, even if repeated in supplied gold/prediction vectors. This local tolerance does not validate the complete Sent arrays consumed by predict. The contract treats token positions as a set while prediction occurrences remain a sequence.

Subject: current source hash and this node's declared scope. Relation describes one terminating call over the finite inputs stated above, with fixed dependencies and successful allocation. Status: partial source-derived model; named native fixtures support only their finite domain. Unobserved effects remain unknown.

## History

### 2026-10-30 · epoch-memory

2026-10-30 first lexical epoch: old native witnesses returned raw heads [0,10,99], accepted noun-only overlap and duplicate token IDs, and panicked on an out-of-range head. Narrow fixes translate unambiguous IDs, omit invalid/missing arcs, and enforce the documented distinct/shared-verb condition. Prediction-order greedy matching remains intentionally unchanged; historical tuning scores can differ. No corpus retuning or measured speed claim.
