# world-mwe

## model

# world-mwe

Verbal multiword expression projection, prediction and greedy observation.

## forward

read_cupt filters rows and groups annotations, producing aligned dense Sent arrays. predict combines fixed global contiguous spans and dependency pairs under knobs. judge greedily consumes one unused gold for each prediction sharing at least two distinct valid token positions including a VERB. score=r-w; pr reports 100*r/max(1,r+w),100*r/max(1,g).

## invariant

On aligned Sent arrays, r+w equals prediction count and 0<=r<=min(prediction count,gold count). Parsed heads are zero or a dense position. These score invariants do not imply maximum matching or linguistic correctness.

## cost

Let B be input bytes, N retained rows, G gold groups, P predictions, K maximum set length. Parsing O(B+N log N+annotations log G), plus strings. judge O(P*G*(K^2+K log K)) worst case due to vector membership and distinct-set collection, memory O(G+K). Prediction includes opaque global lookup plus linear scans of previous predictions; no measured speed bound.

## composition

read_cupt creates the dense coordinate system consumed by predict and judge. Tune callers obtain defaults from knobs before score/pr; an empty cfg disables predictors. global::expressions_in_sentence merges lemma/form scans with overlap policy; global::idiom checks hot overrides before compiled data. Tests join actual read_cupt to Sent fields and actual judge to counts, plus zero-prediction score/pr. Enabled global lexicon matching is source-inspected only.

## epoch-memory

First lexical epoch: old native witnesses returned raw heads [0,10,99], accepted noun-only overlap and duplicate token IDs, and panicked on an out-of-range head. Narrow fixes translate unambiguous IDs, omit invalid/missing arcs, and enforce the documented distinct/shared-verb condition. Prediction-order greedy matching remains intentionally unchanged; historical tuning scores can differ. No corpus retuning or measured speed claim.

## checks

Native filter mwe::wave10_models; see model.md#evidence for fragments or model.md#evidence at the root. Finite supplied fixtures and independent oracles; no live parser, corpus, teacher or global dictionary activation. Partial syntax has opaque macro bodies and no semantic AST. No executable percentage surrogate or speed claim.

## behavior-complement

Universe: aligned sentence/configuration/ordered prediction/score transitions with fixed lexical state. Inner excluded set contains r>g, r+w!=P, a match with fewer than two distinct valid shared positions, and a match lacking a shared VERB. Absence of dictionary matches or semantic truth is not inferred.

Subject: current source hash and this node's declared scope. Relation describes one terminating call over the finite inputs stated above, with fixed dependencies and successful allocation. Status: partial source-derived model; named native fixtures support only their finite domain. Unobserved effects remain unknown.

## reverse-reconstruction

For observation (r,w,g), retain every ordered gold/prediction pair yielding those counts under greedy matching. For score d retain every r,w with r-w=d and count constraints. Token spelling, category, exact spans and prediction order cannot generally be recovered. Example equal score zero includes no predictions and one correct plus one wrong.

Subject: current source hash and this node's declared scope. Relation describes one terminating call over the finite inputs stated above, with fixed dependencies and successful allocation. Status: partial source-derived model; named native fixtures support only their finite domain. Unobserved effects remain unknown.

## metric-denominators

Fix ordered data and cfg. pr aggregates counts before division, so denominators are micro counts, not averages of sentence precision. Empty data and disabled predictors return (0,0), with max(1,denominator) preventing division by zero. score retains one signed difference per sentence and loses both counts.

Subject: current source hash and this node's declared scope. Relation describes one terminating call over the finite inputs stated above, with fixed dependencies and successful allocation. Status: partial source-derived model; named native fixtures support only their finite domain. Unobserved effects remain unknown.

## lexical-snapshot

Prediction is conditional on global's process snapshot and compiled expression table. Same supplied Sent/cfg with different hot idiom overrides can produce different arcs; no equivalence across snapshots is claimed. Knob booleans use >0.5, absent keys are zero; global overlapping-span selection can hide alternatives before local kind filtering.

Subject: current source hash and this node's declared scope. Relation describes one terminating call over the finite inputs stated above, with fixed dependencies and successful allocation. Status: partial source-derived model; named native fixtures support only their finite domain. Unobserved effects remain unknown.

## evidence

### Lexical evidence

Old-code witness: five named failures before fixes (raw sparse heads, duplicate-token intersection, missing shared verb, invalid-head panic and cap overshoot). WordNet private snapshot helper extraction preceded the witness without changing traversal. Production mutation common.len()>=2 to >=1 fails judge_finite_set_oracle and two literal tests. Production cap 40 to 39 fails ancestry_exact_output_cap. Both mutations compiled in one native run; failures are named assertions, not compile failures. Original bytes were restored and compared, then positive tests repeated. Final change thereafter only formatted the appended test modules; final bound tests repeat again.

Syntax generated twice per source and compared byte-for-byte. External registry scaffold preserves the base registry under a synthetic root and adds the two owned leaves; groupants repeats are identical and valid, changed reports exactly both sources with no missing registration. Root integration of authoritative registry/parents is separate. Narrow rustfmt covers only appended tests; git diff --check passes. No whole-world Clippy claim.

Actual joins exercised: read_cupt->Sent dense coordinates; judge->count observations plus zero-prediction score/pr; WordNet::load->first_sense_in; owned snapshot replacement vs existing snapshot; synset graph->label sequence. Global expression/idiom matching, global WordNet OnceLock, downstream caches and full school inference were inspected but not executed. No corpus, dataset download, teacher or GPU was used. No timing or percentage-error guarantee is asserted.

## History

### 2026-10-30 · epoch-memory

2026-10-30 first lexical epoch: old native witnesses returned raw heads [0,10,99], accepted noun-only overlap and duplicate token IDs, and panicked on an out-of-range head. Narrow fixes translate unambiguous IDs, omit invalid/missing arcs, and enforce the documented distinct/shared-verb condition. Prediction-order greedy matching remains intentionally unchanged; historical tuning scores can differ. No corpus retuning or measured speed claim.

### 2026-10-30 · evidence

Heading `### Lexical evidence, 2026-10-30` stays in evidence as `### Lexical evidence`.

Source SHA-256: 859883d4e16f6ed8cffba16cab81246f81f7c20c9fa3534190b91ea0fd21ed29. Partial syntax SHA-256: 32389963bc2ebfce052f716699fe1f221c105c9321aba4e657f39d265c4c1ab9.

Native filter: cargo test -p world --lib wave10_models -- --test-threads=1; repeat with --release. Eleven tests pass twice in each profile with identical normalized identities/results (SHA-256 83635ae7bd7b29bf6f2e86e7080b21891c03143b57417b374587989e06310ceb). Six MWE tests include 512 finite overlap/duplication cases, greedy-order/count fixtures, sparse/ambiguous/invalid ID files and invalid-head rejection. Five WordNet tests include 1536 independent transitive-closure cases over every three-node directed graph, label-collision/level fixtures, a 55-parent cap fixture, ordered normalization/POS fibers and actual load/lookup of synthetic temporary JSON.

Raw logs are external under $MOVA_DATA/runs/octopus-wave10-lexical-2026-10-30/. Digests:

- debug-bound-1.log: f55d682252ddf462967810cf9e5a3ade8fb05592ea1783ba448f1887b1132022

- debug-bound-2.log: da6ba581a4f22ddc5c85fef5ba0bbb29b24dd87b47dbd8d72367124427f937d2

- release-bound-1.log: 75167b14ddc6325a420d526251bbc34c6d74ef6cba1cd70501c6e1fd9f2c3120

- release-bound-2.log: b80c415943dbd34cc0eb61a745a7646213e131bdec14ffaad722cf6f78d7f069

- old-witness.log: 6df1d2d35ea996a3f0b37318dacebe487362400d627dbc9fa96fc8d6f74531cb

- mutants.log: ad9afebe67a7cd53cfcf90c857fb6103cc6e14917e49d290e5f20c3e55ad7ffe

- groupants-final-1.json: 9cd75ad8ca9f32b8acc26920cd9a57f169a3130296a61e79fc25171f39ffd6e0

- groupants-final-2.json: 9cd75ad8ca9f32b8acc26920cd9a57f169a3130296a61e79fc25171f39ffd6e0

- changed.json: bc99a5f762d597c352cbfbcd7cf3d7c7fdf8ff83795b1c8e476f9a55878a6aa0
