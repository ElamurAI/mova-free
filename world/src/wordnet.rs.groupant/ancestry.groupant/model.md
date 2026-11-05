# world-wordnet-ancestry

Synset-identity traversal and capped label observation.

## forward

Initialize seen with starting ID and frontier=[start]. Scan each frontier ID and stored hypernym order; visit each new ID once, emit its first-member label if known and not previously emitted, queue its ID even if its label duplicates or target is missing. Return immediately after label 40; otherwise advance level until empty.

## invariant

Output labels are unique, length<=40, and near-before-far in traversal encounter order. ID deduplication terminates cycles in finite maps. Label deduplication does not prune distinct meanings: equal-label parents can still reach different deeper kinds. A missing node has no outgoing edges.

## cost

For reached V/E, O((V+E)log V+40E) ignoring label width; queue/seen storage O(V+E), output O(40). Cloning outgoing lists contributes E allocations/bytes. Forty emitted labels do not impose a 40-node or 40-edge work budget.

## composition

synset_up delegates to synset_up_in with the global immutable snapshot; test helper uses an owned snapshot with identical traversal. first_sense and senses_up supply synset IDs; following label text instead would conflate homonyms and is explicitly excluded.

## epoch-memory

First lexical epoch. Actual OEWN schema and knowledge/opchoice callers inspected read-only; native tests use only generated miniature JSON and explicit maps. Old traversal returned all 55 direct parents despite an outer out.len<40 guard. The repair stops at the fortieth distinct label without reordering. Pure snapshot helpers preserve public APIs. No live OEWN activation, cache reload, semantic quality or timing claim.

## checks

Native filter wordnet::wave10_models; see ../model.md#evidence for fragments or ../model.md#evidence at the root. Finite supplied fixtures and independent oracles; no live parser, corpus, teacher or global dictionary activation. Partial syntax has opaque macro bodies and no semantic AST. No executable percentage surrogate or speed claim.

## behavior-complement

Universe: finite directed maps and traversal observations. Inner absent outputs include duplicate labels, >40 labels, the start ID discovered only through a return cycle, or a later-level label before all earlier encountered unique labels. No semantic taxonomy truth or complete graph validation is claimed.

Subject: current source hash and this node's declared scope. Relation describes one terminating call over the finite inputs stated above, with fixed dependencies and successful allocation. Status: partial source-derived model; named native fixtures support only their finite domain. Unobserved effects remain unknown.

## reverse-reconstruction

For a label vector retain every start ID and graph whose capped traversal emits it. Equal-member IDs, missing labels, ordering-equivalent edges, cycles and all unvisited suffixes create ambiguity. The diamond fixture with two same-labeled parents still preserves both distinct descendants, demonstrating label observation is not node identity.

Subject: current source hash and this node's declared scope. Relation describes one terminating call over the finite inputs stated above, with fixed dependencies and successful allocation. Status: partial source-derived model; named native fixtures support only their finite domain. Unobserved effects remain unknown.

## finite-reachability

For all 512 three-node directed graphs with unique labels, transitive-closure oracle checks reachable nodes excluding start against output sets for all three starts (1536 cases). This tests cycles and missing self-return, but not arbitrary large graphs or label ordering; separate fixtures cover order/collisions.

Subject: current source hash and this node's declared scope. Relation describes one terminating call over the finite inputs stated above, with fixed dependencies and successful allocation. Status: partial source-derived model; named native fixtures support only their finite domain. Unobserved effects remain unknown.

## cap-prefix

Given traversal sequence L of first-seen distinct known labels, output is prefix_40(L). A 55-parent star must return h00..h39 in supplied edge order. Old outer-loop-only guard emitted 55; compiled cap mutant rejects. Unknown ancestry beyond the prefix is discarded, not evidence of absent hypernyms.

Subject: current source hash and this node's declared scope. Relation describes one terminating call over the finite inputs stated above, with fixed dependencies and successful allocation. Status: partial source-derived model; named native fixtures support only their finite domain. Unobserved effects remain unknown.

## History

### 2026-10-30 · epoch-memory

2026-10-30 first lexical epoch. Actual OEWN schema and knowledge/opchoice callers inspected read-only; native tests use only generated miniature JSON and explicit maps. Old traversal returned all 55 direct parents despite an outer out.len()<40 guard. The repair stops at the fortieth distinct label without reordering. Pure snapshot helpers preserve public APIs. No live OEWN activation, cache reload, semantic quality or timing claim.
