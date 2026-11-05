# world-mwe-prediction

Configuration gates and fixed-lexicon span/pair assembly.

## forward

Contiguous flag activates global form/lemma matching. Accept spans of >=2 with allowed entry kind, a VERB and content count>=min(configured minimum,span length). Arc flag scans retained order: ignore zero, out-of-range and self heads; require obj/compound:prt/iobj plus verbal parent and enabled global idiom category. Sort each two-position pair and suppress it if already contained in an output span.

## invariant

Under aligned arrays and valid global spans, every output has at least two distinct indices and a VERB. Arc output pairs are sorted; global spans retain global ordering. The code assumes aligned public arrays, and cannot certify arbitrary manually malformed Sent values.

## cost

Beyond opaque global matching, O(M*N+N*P*K) conservative scanning bound for M candidate spans and P outputs of size K, plus ordered string lookup costs. Output size and allocation are unbounded by knobs alone.

## composition

The dense head field produced by read_cupt is consumed as h-1. Global filtering occurs before local gates, so rejecting a chosen longest global expression does not recover a suppressed shorter one. Tests cover disabled configuration and invalid-head rejection without activating global dictionaries.

## epoch-memory

First lexical epoch: old native witnesses returned raw heads [0,10,99], accepted noun-only overlap and duplicate token IDs, and panicked on an out-of-range head. Narrow fixes translate unambiguous IDs, omit invalid/missing arcs, and enforce the documented distinct/shared-verb condition. Prediction-order greedy matching remains intentionally unchanged; historical tuning scores can differ. No corpus retuning or measured speed claim.

## checks

Native filter mwe::wave10_models; see ../model.md#evidence for fragments or ../model.md#evidence at the root. Finite supplied fixtures and independent oracles; no live parser, corpus, teacher or global dictionary activation. Partial syntax has opaque macro bodies and no semantic AST. No executable percentage surrogate or speed claim.

## behavior-complement

Universe: aligned Sent, fixed lexical result and cfg, outputs. Inner absent transitions include arc emitted for zero/out-of-range/self head, nonverbal parent, unsupported relation or disabled category. Unknown lexical entries and contextual idiom senses remain outside the model.

Subject: current source hash and this node's declared scope. Relation describes one terminating call over the finite inputs stated above, with fixed dependencies and successful allocation. Status: partial source-derived model; named native fixtures support only their finite domain. Unobserved effects remain unknown.

## reverse-reconstruction

For a predicted pair retain any valid verbal-head/object-or-particle attachment with an enabled matching idiom category, and any accepted contiguous length-two match yielding that pair. The vector alone does not identify dictionary source or whether an arc was suppressed by a larger span.

Subject: current source hash and this node's declared scope. Relation describes one terminating call over the finite inputs stated above, with fixed dependencies and successful allocation. Status: partial source-derived model; named native fixtures support only their finite domain. Unobserved effects remain unknown.

## knob-threshold

Boolean controls compare >0.5; absent, zero, negative and NaN controls are false. min_content is numeric and capped at candidate span length, so a configured minimum above the length does not forbid that span if every token is content. Knobs() defaults differ from a missing-key cfg.

Subject: current source hash and this node's declared scope. Relation describes one terminating call over the finite inputs stated above, with fixed dependencies and successful allocation. Status: partial source-derived model; named native fixtures support only their finite domain. Unobserved effects remain unknown.

## suppression-order

Arc pair q is appended iff no previous output o contains every q member. Contiguous results are inserted first; later arcs may be blocked by them. This prevents some duplicate pairs but is not a general deduplicated set or maximal matching objective.

Subject: current source hash and this node's declared scope. Relation describes one terminating call over the finite inputs stated above, with fixed dependencies and successful allocation. Status: partial source-derived model; named native fixtures support only their finite domain. Unobserved effects remain unknown.

## History

### 2026-10-30 · epoch-memory

2026-10-30 first lexical epoch: old native witnesses returned raw heads [0,10,99], accepted noun-only overlap and duplicate token IDs, and panicked on an out-of-range head. Narrow fixes translate unambiguous IDs, omit invalid/missing arcs, and enforce the documented distinct/shared-verb condition. Prediction-order greedy matching remains intentionally unchanged; historical tuning scores can differ. No corpus retuning or measured speed claim.
