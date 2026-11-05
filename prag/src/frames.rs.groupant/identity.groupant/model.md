# Tree indexing and entity head membership

## forward

kids[j] is the ascending list of i with words[i].head=j+1 inside 1..=W. root is first head-zero token. same(a,b)=notU(a.id) and notU(b.id) and (a.id=b.id or b.id in a.and or a.id in b.and). U is the six published unresolved heads.

## behavior-complement

Universe: word vectors and proposed index views, or entity pairs and proposed Boolean. Exclude out-of-range heads in kids, reversed child order, or true identity if either head belongs to U.

## reverse-reconstruction

Child lists alone lose token spelling and zero versus invalid heads; full words are retained by Tree. A true identity Boolean admits equal heads or either membership direction; false admits many distinct or unresolved pairs. Surface modifiers cannot be recovered.

## invariant

same is symmetric, but not transitive: dog and cat separately match dog+cat yet not each other. No equivalence-class quotient is valid.

## cost

Tree O(W) time/storage; identity O(bytes compared across both coordination lists), with constant six-head rejection checks.

## identity-fiber

Fix a=dog, b=cat and g=dog with and=[cat]: same(a,g)=same(g,b)=true, same(a,b)=false. Unresolved heads are rejected even if a coordination member names them. Tests cover all six unresolved heads in both directions.

## composition

Tree coordinates feed entity and predicate extraction. Tree accepts cycles/self-heads; recursive reader safety is outside this fragment.

## epoch-memory

Wave15. Prospective repair: reject unresolved right heads in identity with a constant six-item guard, preserving coordination membership and its nontransitivity. Old production failed unresolved_identity_is_symmetric_even_in_coordination; repaired code passes. Compiled guard-removal control fails the same named assertion and exact bytes are restored. Eight focused tests pass twice debug and twice release. No speed, full semantic coverage or universal proof claim.

## limitations

Partial syntax is not expanded semantic AST. No live parser, teacher, whole corpus, GPU or environment mutation. Acyclic input trees and fixed configuration/resource premises matter; malformed recursive ancestry, counter overflow, complete TAM/lexicon/quotation/repair semantics and all active-rule effects remain open. No executable approximant/error budget is claimed.

## History

### 2026-10-30 · epoch-memory

Wave15, 2026-10-30. Prospective repair: reject unresolved right heads in identity with a constant six-item guard, preserving coordination membership and its nontransitivity. Old production failed unresolved_identity_is_symmetric_even_in_coordination; repaired code passes. Compiled guard-removal control fails the same named assertion and exact bytes are restored. Eight focused tests pass twice debug and twice release. No speed, full semantic coverage or universal proof claim.

### Undated · document

Subject: prag/src/frames.rs SHA-256 bfbc2f7905d878080a7e7dd30ae7f1bfe8f07313c3a4cf8f0254be29e72a4b88. Partial source model, finite native evidence. Fixed configuration and admitted finite resource domains; source-derived inner exclusions and ambiguous preimages, no inverse solver.
