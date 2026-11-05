# Predicate and clause graph assembly

## forward

frames_of_tree delegates with no forced pronoun map. Predicate tokens in word order seed frames; subsequent passes resolve control/shared roles, clause relations, cause/next/quote and condition/factuality. The simple subject-verb domain yields one Event with the supplied verb and no questions/aliases.

## behavior-complement

Universe: default-config supplied simple subject-verb trees and candidate frame vectors. Exclude a frame count other than one, an agent id different from subject lemma, or a predicate anchor other than the verb token. More complex clauses remain outside this exclusion bound.

## reverse-reconstruction

One observed Event admits multiple lexical surface forms and tense encodings when TAM is omitted. Frame graphs alone do not recover all UD words, function words or antecedent salience.

## invariant

Predicate indices and frame ranks are separate. Clause graph referents are local until read_with offsets them.

## cost

Multiple predicate/word passes plus recursive entity construction and lexicon costs; rough O(P*W plus entity/lexicon work), not a measured bound.

## anchor-transport

In native dog/bark tree predicate token 1 maps to frame 0 and subject token 0 maps to Agent. Context anchors remain last-sentence observations and are not read to decide frame semantics.

## composition

Dependency helpers and signature resources participate in complex clauses. Existing factuality tests are not claimed as new wave15 execution; all branches are not covered.

## epoch-memory

The object's misplaced phrases include a conj noun with its own preposition (`pp_conj`, «8 eyes, usually in two or three rows» → place=row; PRAG_OFF=conj-pp); see the parent epoch note.

## limitations

Partial syntax is not expanded semantic AST. No live parser, teacher, whole corpus, GPU or environment mutation. Acyclic input trees and fixed configuration/resource premises matter; malformed recursive ancestry, counter overflow, complete TAM/lexicon/quotation/repair semantics and all active-rule effects remain open. No executable approximant/error budget is claimed.

## History

### 2026-10-30 · epoch-memory

Wave15, 2026-10-30. Prospective repair: reject unresolved right heads in identity with a constant six-item guard, preserving coordination membership and its nontransitivity. Old production failed unresolved_identity_is_symmetric_even_in_coordination; repaired code passes. Compiled guard-removal control fails the same named assertion and exact bytes are restored. Eight focused tests pass twice debug and twice release. No speed, full semantic coverage or universal proof claim.

### 2026-11-05 · epoch-memory

2026-11-05: the object's misplaced phrases include a conj noun with its own preposition (`pp_conj`, «8 eyes, usually in two or three rows» → place=row; PRAG_OFF=conj-pp); see the parent epoch note.

### Undated · document

Subject: prag/src/frames.rs SHA-256 bfbc2f7905d878080a7e7dd30ae7f1bfe8f07313c3a4cf8f0254be29e72a4b88. Partial source model, finite native evidence. Fixed configuration and admitted finite resource domains; source-derived inner exclusions and ambiguous preimages, no inverse solver.
