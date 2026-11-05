# prag-feats-previous

Index previous acts by source-qualified document ordinal, independently of paragraph and input order.

## forward

Build map M[(src,doc,ord)]=last input index for non-Tatoeba ord>0. Output None for Tatoeba or ord<=1, otherwise lookup M[(src,doc,ord-1)]. This is an identity lookup, not previous vector position.

## behavior-complement

Universe: Items to optional index vectors. Excludes cross-source/doc links, Tatoeba links, ord<=1 links, and selecting an earlier duplicate over the final matching predecessor. An edge may point forward in vector order; that is not excluded.

## reverse-reconstruction

Given edges retain all source/doc/ordinal assignments satisfying predecessor equality and last-wins duplicate constraints. Paragraphs, texts, titles and keys are unconstrained by this observation. Missing links permit absent predecessor, initial ordinal or Tatoeba.

## invariant

Returned indices lie within input; paragraph differences do not reset predecessor acts; guarded subtraction avoids i64 underflow. Same key collisions are explicitly last-wins, not rejected.

## cost

Expected O(N) hash operations and O(N) map space; doc strings are borrowed, source is a small enum. No measured speedup or executable approximation error bound is claimed.

## coupling

eval train_rows/predict_seq consume indices while separately deciding available labels/predictions. This function does not guarantee the predecessor was predicted earlier in input order.

## observation-fiber

Overwriting duplicate keys loses earlier indices. Distinct text/paragraph assignments give identical edges. Adding source to key removes accidental cross-source context without changing same-source duplicate policy.

## composition

eval train_rows/predict_seq consume indices while separately deciding available labels/predictions. This function does not guarantee the predecessor was predicted earlier in input order. No child fragments; this is one scoped relation.

## checks

prev_source_identity_and_last_duplicate failed old code with Tale/Gum collision, then passes source isolation, duplicate, paragraph and minimum ordinal checks. See the file-level ../model.md#evidence for exact log hashes and repeated run outcomes.

## epoch-memory

Wave13: initial source-bound model. Prospective model preceded edits. Existing normalization and quote rules were retained; tests expose their information loss. Source identity witness justified qualifying predecessor keys by Src. Annotation is isolated behind a private closure used by the production row wrapper; fixtures do not call or certify tokenizer/parser. AST is a partial syntax projection, macro bodies opaque; semantic AST, live model/corpus and general proof remain absent.

## History

### 2026-10-30 · epoch-memory

2026-10-30 wave13: initial source-bound model. Prospective model preceded edits. Existing normalization and quote rules were retained; tests expose their information loss. Source identity witness justified qualifying predecessor keys by Src. Annotation is isolated behind a private closure used by the production row wrapper; fixtures do not call or certify tokenizer/parser. AST is a partial syntax projection, macro bodies opaque; semantic AST, live model/corpus and general proof remain absent.

### Undated · document

Source snapshot: 2a1a0bd6e43dd16f7284e993f30d0f4971c447c9e2080616033d19ac8693611f. All relations concern this snapshot on the stated admissible domain. Models are source-derived partial relations; finite fixtures are evidence, not general proofs. Complement is a conservative inner constraint model. Reverse reconstruction preserves all compatible antecedents conceptually; no enumerating solver is implemented.
