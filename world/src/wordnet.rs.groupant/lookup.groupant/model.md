# world-wordnet-lookup

Immutable runtime snapshot and ordered word/POS queries.

## forward

OnceLock captures load success or empty fallback. plain_forms lowercases input, then fixed suffix substitutions, then mentions::stem; duplicates and empty candidates are allowed. first_sense selects the first candidate with first matching exact POS. senses_up selects the first candidate with any matching POS and traverses each matching sense in its stored order.

## invariant

Candidate order, not linguistic probability, selects senses. Case variants share lowercased candidates if the stem dependency is fixed. POS is exact descriptive text (noun,verb,adjective,adverb,word), so NOUN does not equal noun.

## cost

With bounded suffix list, candidate generation O(word bytes+stem cost); lookup O(number of candidates*log lemmas+scanned senses), plus ancestry for each selected sense. Caches in callers are not bounded by this file.

## composition

This fragment covers public wrappers and private normalization helpers; its byte envelope overlaps ancestry, whose traversal semantics are modeled separately. Tests call the private helper against an explicit owned map.

## epoch-memory

First lexical epoch. Actual OEWN schema and knowledge/opchoice callers inspected read-only; native tests use only generated miniature JSON and explicit maps. Old traversal returned all 55 direct parents despite an outer out.len<40 guard. The repair stops at the fortieth distinct label without reordering. Pure snapshot helpers preserve public APIs. No live OEWN activation, cache reload, semantic quality or timing claim.

## checks

Native filter wordnet::wave10_models; see ../model.md#evidence for fragments or ../model.md#evidence at the root. Finite supplied fixtures and independent oracles; no live parser, corpus, teacher or global dictionary activation. Partial syntax has opaque macro bodies and no semantic AST. No executable percentage surrogate or speed claim.

## behavior-complement

Universe: fixed map, query spelling/POS and sense result. Inner absent set includes selecting a later sense when an earlier sense of the exact POS already matches the first viable candidate. No exclusion of linguistically wrong heuristic stems is asserted.

Subject: current source hash and this node's declared scope. Relation describes one terminating call over the finite inputs stated above, with fixed dependencies and successful allocation. Status: partial source-derived model; named native fixtures support only their finite domain. Unobserved effects remain unknown.

## reverse-reconstruction

Given the selected sense retain all case variants and inflected candidates whose first viable normalized form selects it. Three supplied cats/CATS/Cats spellings share n1 for noun but v for verb. The output cannot determine capitalization, original inflection, or whether alternate senses existed.

Subject: current source hash and this node's declared scope. Relation describes one terminating call over the finite inputs stated above, with fixed dependencies and successful allocation. Status: partial source-derived model; named native fixtures support only their finite domain. Unobserved effects remain unknown.

## normalization-order

Input itself precedes ies->y, es removal, s removal, ed removal, ed->e, d removal, ing removal, ing->e and irregular stem. TRIES starts [tries,try,tri,trie]; no deduplication. Native exact POS tests preserve first n1 rather than n2.

Subject: current source hash and this node's declared scope. Relation describes one terminating call over the finite inputs stated above, with fixed dependencies and successful allocation. Status: partial source-derived model; named native fixtures support only their finite domain. Unobserved effects remain unknown.

## failure-snapshot

OnceLock empty fallback makes missing directory, malformed JSON and valid empty source observationally equal to public lookup. Recovering cause requires external initialization evidence; later repairing files does not change W. Loading a separate owned snapshot can observe repairs but does not refresh global W.

Subject: current source hash and this node's declared scope. Relation describes one terminating call over the finite inputs stated above, with fixed dependencies and successful allocation. Status: partial source-derived model; named native fixtures support only their finite domain. Unobserved effects remain unknown.

## History

### 2026-10-30 · epoch-memory

2026-10-30 first lexical epoch. Actual OEWN schema and knowledge/opchoice callers inspected read-only; native tests use only generated miniature JSON and explicit maps. Old traversal returned all 55 direct parents despite an outer out.len()<40 guard. The repair stops at the fortieth distinct label without reordering. Pure snapshot helpers preserve public APIs. No live OEWN activation, cache reload, semantic quality or timing claim.
