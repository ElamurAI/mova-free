# Nominal filtering and graph projection

## forward

With score-nouns disabled let K=[i | frames[i].how.by != Nominal], m(K[j])=j. Output copies frames at K and maps each quote/cause/next/link through m, deleting targets outside K. With score-nouns enabled return all frames unchanged.

## behavior-complement

Universe: input frame vectors and proposed projected vectors under fixed option. Exclude changed retained nonlink fields, retained nominal rows, or a removed nominal target still represented by its old index.

## reverse-reconstruction

Projected y admits arbitrary inserted nominal frames and edges targeting them, subject to the surviving remapping constraints. Reconstruct every such compatible original; deletion prevents a unique inverse.

## invariant

Retained order is stable; all retained original target indices are converted to ranks. Invalid targets also disappear via missing map entries.

## cost

O(F log F + L log F + cloned entity/output bytes), O(F+output) storage. No optimization claimed.

## edge-projection

A frame with quote=[nominal,event], cause=nominal and next=event becomes quote=[new_event], cause=None and next=new_event. link preserves relation/noun metadata while remapping parent. Native test asserts every field.

## composition

Projection must occur after read coordinates are global. Filtering frames without remapping would violate graph identity. Orphaned links vanish; semantic causal completion is not attempted.

## epoch-memory

Wave15. Prospective repair: reject unresolved right heads in identity with a constant six-item guard, preserving coordination membership and its nontransitivity. Old production failed unresolved_identity_is_symmetric_even_in_coordination; repaired code passes. Compiled guard-removal control fails the same named assertion and exact bytes are restored. Eight focused tests pass twice debug and twice release. No speed, full semantic coverage or universal proof claim.

## limitations

Partial syntax is not expanded semantic AST. No live parser, teacher, whole corpus, GPU or environment mutation. Acyclic input trees and fixed configuration/resource premises matter; malformed recursive ancestry, counter overflow, complete TAM/lexicon/quotation/repair semantics and all active-rule effects remain open. No executable approximant/error budget is claimed.

## History

### 2026-10-30 · epoch-memory

Wave15, 2026-10-30. Prospective repair: reject unresolved right heads in identity with a constant six-item guard, preserving coordination membership and its nontransitivity. Old production failed unresolved_identity_is_symmetric_even_in_coordination; repaired code passes. Compiled guard-removal control fails the same named assertion and exact bytes are restored. Eight focused tests pass twice debug and twice release. No speed, full semantic coverage or universal proof claim.

### Undated · document

Subject: prag/src/frames.rs SHA-256 bfbc2f7905d878080a7e7dd30ae7f1bfe8f07313c3a4cf8f0254be29e72a4b88. Partial source model, finite native evidence. Fixed configuration and admitted finite resource domains; source-derived inner exclusions and ambiguous preimages, no inverse solver.
