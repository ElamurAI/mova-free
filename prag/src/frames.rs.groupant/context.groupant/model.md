# Discourse salience and pronoun exclusion

## forward

mention_as ignores unresolved/personal heads, updates stored head identity, salience, recency and agreement hints. new_line halves salience and clears this-sentence weight. resolve_in filters agreement, excluded head and numeric/time candidates then maximizes salience plus role bonus, ties by recency; coref mode zero uses legacy rules.

## behavior-complement

Universe: context/mention/query transitions under fixed mode and representable counters. Exclude insertion of unresolved heads, resolution to the explicitly excluded id, or an unmentioned id.

## reverse-reconstruction

Resolved id constrains only winning compatible candidates. Many lower-salience histories, decayed scores, surface details and rejected candidates lead to the same answer. None admits empty context or incompatible/excluded contexts.

## invariant

Context keys are head ids. new_line is lossy and not invertible; summary(0) need not expose any retained entities. Counters require representability.

## cost

mention O(log E plus entity copying); resolve O(E times lexical/time predicates); summary O(E log E). Salience/time overflow is unmodeled.

## exclusion-fiber

Holding context and pronoun fixed, resolve_not removes exactly the named id before selection; it can reveal a runner-up or None. The box-only native fixture checks removal; unresolved mentions never seed that box.

## composition

Entity identity drives stored discourse; resolve feeds Reader. Full coreference sieve, mode permutations and lexical animacy resources are inspected only.

## epoch-memory

Wave15. Prospective repair: reject unresolved right heads in identity with a constant six-item guard, preserving coordination membership and its nontransitivity. Old production failed unresolved_identity_is_symmetric_even_in_coordination; repaired code passes. Compiled guard-removal control fails the same named assertion and exact bytes are restored. Eight focused tests pass twice debug and twice release. No speed, full semantic coverage or universal proof claim.

## limitations

Partial syntax is not expanded semantic AST. No live parser, teacher, whole corpus, GPU or environment mutation. Acyclic input trees and fixed configuration/resource premises matter; malformed recursive ancestry, counter overflow, complete TAM/lexicon/quotation/repair semantics and all active-rule effects remain open. No executable approximant/error budget is claimed.

## History

### 2026-10-30 · epoch-memory

Wave15, 2026-10-30. Prospective repair: reject unresolved right heads in identity with a constant six-item guard, preserving coordination membership and its nontransitivity. Old production failed unresolved_identity_is_symmetric_even_in_coordination; repaired code passes. Compiled guard-removal control fails the same named assertion and exact bytes are restored. Eight focused tests pass twice debug and twice release. No speed, full semantic coverage or universal proof claim.

### Undated · document

Subject: prag/src/frames.rs SHA-256 bfbc2f7905d878080a7e7dd30ae7f1bfe8f07313c3a4cf8f0254be29e72a4b88. Partial source model, finite native evidence. Fixed configuration and admitted finite resource domains; source-derived inner exclusions and ambiguous preimages, no inverse solver.
