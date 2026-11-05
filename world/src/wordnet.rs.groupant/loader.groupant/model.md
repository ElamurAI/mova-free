# world-wordnet-loader

Sorted JSON files to lossy sense and synset maps.

## forward

Read only .json paths, sorted; entries-* objects accumulate lowercase lemma references in order, frames.json is ignored for synsets, other objects overwrite synsets by ID in sorted file order. Missing referenced synsets are skipped. Sense POS uses synset POS unless empty, then entry POS; kind takes two members per hypernym then three total, synonyms omit case-equivalent lemma and take five, examples take two.

## invariant

Actual loader returns owned strings and maps. Unknown POS maps to word. Missing/mistyped arrays yield empty projections; malformed JSON or read errors return Err. read_dir entry errors are silently dropped by flatten, so successful loading is not a completeness guarantee.

## cost

O(B+(V+E+S)log V+S*(H log V+M)) ignoring string costs, with H/M maximum per-synset hypernym/member lengths; stores decoded synsets, pending entries and materialized senses simultaneously. Upper bounds exclude filesystem latency and parsing implementation details.

## composition

All synsets are collected before entries are resolved, independent of whether the entries filename sorts before a target synset file. Last synset definition wins, while entry references accumulate; the synthetic fixture tests overwrite order, missing references, frames exclusion and projection truncation.

## epoch-memory

First lexical epoch. Actual OEWN schema and knowledge/opchoice callers inspected read-only; native tests use only generated miniature JSON and explicit maps. Old traversal returned all 55 direct parents despite an outer out.len<40 guard. The repair stops at the fortieth distinct label without reordering. Pure snapshot helpers preserve public APIs. No live OEWN activation, cache reload, semantic quality or timing claim.

## checks

Native filter wordnet::wave10_models; see ../model.md#evidence for fragments or ../model.md#evidence at the root. Finite supplied fixtures and independent oracles; no live parser, corpus, teacher or global dictionary activation. Partial syntax has opaque macro bodies and no semantic AST. No executable percentage surrogate or speed claim.

## behavior-complement

Universe: valid supplied JSON directory snapshots and returned maps. Inner absent relation excludes synonyms whose lowercase equals the normalized lemma, >5 synonyms, >2 examples and >3 kinds in any loaded sense. It does not exclude duplicate other synonyms or empty members; schema validation is intentionally tolerant.

Subject: current source hash and this node's declared scope. Relation describes one terminating call over the finite inputs stated above, with fixed dependencies and successful allocation. Status: partial source-derived model; named native fixtures support only their finite domain. Unobserved effects remain unknown.

## reverse-reconstruction

For a loaded sense retain every directory with the same surviving reference and projected fields. Earlier overwritten synsets, discarded examples/synonyms and missing-reference entries can vary freely without altering it. The snapshot cannot uniquely recover source file identity, complete senses or ignored metadata.

Subject: current source hash and this node's declared scope. Relation describes one terminating call over the finite inputs stated above, with fixed dependencies and successful allocation. Status: partial source-derived model; named native fixtures support only their finite domain. Unobserved effects remain unknown.

## file-priority

Sorted path order fixes duplicate synset replacement; later files win. Entries remain accumulated in lexical file/object/POS and declared sense order. The supplied a.json/z.json collision must yield new; frames.json cannot override it.

Subject: current source hash and this node's declared scope. Relation describes one terminating call over the finite inputs stated above, with fixed dependencies and successful allocation. Status: partial source-derived model; named native fixtures support only their finite domain. Unobserved effects remain unknown.

## projection-fiber

Fix the retained prefixes. Any third-or-later example, sixth-or-later non-self synonym and members beyond each first-two/overall-three kind limit can vary without changing the sense projection. Actual fixture observes [a,b], five synonyms and [H1,H2,J1]; discarded suffixes are not reconstructed.

Subject: current source hash and this node's declared scope. Relation describes one terminating call over the finite inputs stated above, with fixed dependencies and successful allocation. Status: partial source-derived model; named native fixtures support only their finite domain. Unobserved effects remain unknown.

## History

### 2026-10-30 · epoch-memory

2026-10-30 first lexical epoch. Actual OEWN schema and knowledge/opchoice callers inspected read-only; native tests use only generated miniature JSON and explicit maps. Old traversal returned all 55 direct parents despite an outer out.len()<40 guard. The repair stops at the fortieth distinct label without reordering. Pure snapshot helpers preserve public APIs. No live OEWN activation, cache reload, semantic quality or timing claim.
