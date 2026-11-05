# Entity construction and lexical roles

## forward

Reader builds entities from dependency noun heads, handles pronouns against context/forced ids, gathers modifiers and conjunctions, and projects argument fillers into lexicon roles. Scoped observation: supplied simple noun subject dog yields agent id dog.

## behavior-complement

Universe: supported simple noun subject trees and candidate roles under default fixed configuration. Exclude changing the subject head lemma when no pronoun/alias transformation applies, or adding a patient absent every supported argument route. Outside this small branch absence is unknown.

## reverse-reconstruction

Agent id dog admits arbitrary determiners/modifiers and surface spellings preserving the lemma. A supplied resolved pronoun and an explicit noun can share id while their pron field differs; retain both when only id is observed.

## invariant

Entities retain mention-level details; identity ignores poss/mods/appos. Recursive dependencies require acyclic bounded-depth trees.

## cost

Repeated subtree scans/copying can be quadratic in words and output; lexicon matching adds resource-dependent work. No performance measurement.

## role-head

For the tested simple Nsubj noun -> root verb tree, agent.id equals subject lemma; ctx anchor role token equals subject position. Predicate position is a different coordinate.

## composition

Entity recursion, nominal events and VerbNet alternatives are large unexecuted branches. This fragment is deliberately an observation model, not full lexical semantics.

## epoch-memory

`is_pred` excludes the second part of a hyphenated compound adjective (`hyphen_adj`), and a conj noun with its own preposition is not a coordinated entity (`pp_conj`); see the parent epoch note.

## limitations

Partial syntax is not expanded semantic AST. No live parser, teacher, whole corpus, GPU or environment mutation. Acyclic input trees and fixed configuration/resource premises matter; malformed recursive ancestry, counter overflow, complete TAM/lexicon/quotation/repair semantics and all active-rule effects remain open. No executable approximant/error budget is claimed.

## History

### 2026-10-30 · epoch-memory

Wave15, 2026-10-30. Prospective repair: reject unresolved right heads in identity with a constant six-item guard, preserving coordination membership and its nontransitivity. Old production failed unresolved_identity_is_symmetric_even_in_coordination; repaired code passes. Compiled guard-removal control fails the same named assertion and exact bytes are restored. Eight focused tests pass twice debug and twice release. No speed, full semantic coverage or universal proof claim.

### 2026-11-05 · epoch-memory

2026-11-05: `is_pred` excludes the second part of a hyphenated compound adjective (`hyphen_adj`), and a conj noun with its own preposition is not a coordinated entity (`pp_conj`); see the parent epoch note.

### Undated · document

Subject: prag/src/frames.rs SHA-256 bfbc2f7905d878080a7e7dd30ae7f1bfe8f07313c3a4cf8f0254be29e72a4b88. Partial source model, finite native evidence. Fixed configuration and admitted finite resource domains; source-derived inner exclusions and ambiguous preimages, no inverse solver.
