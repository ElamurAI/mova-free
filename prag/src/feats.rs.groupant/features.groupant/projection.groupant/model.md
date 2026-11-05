# prag-feats-features-projection

Project supplied words, terminal punctuation, UD relations, cues and metadata into 32 coordinates.

## forward

Content omits PUNCT and wholly nonalphanumeric forms. Terminal punctuation priority is ? then ! then ellipsis then period then dash then comma-like. Root is first head=0; subjects and tense auxiliaries are direct children. Inversion requires AUX before selected subject. Cue evaluation distinguishes pos/neg/mix; first prioritized act-like class becomes cue. ppos is only/first/last/mid; genre maps source.

## behavior-complement

Universe: finite Word vectors and valid text/frame/meta to 32-string rows. Excludes len outside six bucket labels, qm=no when a question mark occurs (value is yes or -), unknown source genre, neg=root when only a non-root/non-child negative exists, and terminal ! when terminal tail contains ?.

## reverse-reconstruction

A row retains all Word vectors with equal observed first/last/content bucket and selected root/child/cue facts. Length bucket 4-6 admits three content counts; punctuation tokens can vary without changing content slots. Unknown sperson '-' permits absent morphology even for you.

## invariant

First root and first qualifying child win; rootless input uses missing root slots. Head values are compared as indices, not validated as a tree. No semantic truth or correct parsing is inferred.

## cost

Constant-many linear word scans plus cue matching O(W*P*L); constant 32-slot output plus copied words and frame. No measured speedup or executable approximation error bound is claimed.

## coupling

Word forms determine normalization and lexical cues; UPOS/tag/feats/head/rel independently affect grammar slots. A token form alone cannot imply person morphology. ppos assumes sensible positions; extreme inconsistent public metadata remains outside tested domain.

## observation-fiber

Bucket and booleans collapse counts; first-hit wh/modal/subject discard later alternatives. qm scans entire utterance while end scans only final nonalphanumeric tail, so internal ? and terminal . can coexist.

## composition

Word forms determine normalization and lexical cues; UPOS/tag/feats/head/rel independently affect grammar slots. A token form alone cannot imply person morphology. ppos assumes sensible positions; extreme inconsistent public metadata remains outside tested domain. No child fragments; this is one scoped relation.

## checks

bucket_boundaries_and_terminal_priority checks twelve bucket boundaries and seven terminal strings; supplied UD checks eighteen named feature coordinates and empty-row test checks every slot. See the file-level ../../model.md#evidence for exact log hashes and repeated run outcomes.

## epoch-memory

Wave13: initial source-bound model. Prospective model preceded edits. Existing normalization and quote rules were retained; tests expose their information loss. Source identity witness justified qualifying predecessor keys by Src. Annotation is isolated behind a private closure used by the production row wrapper; fixtures do not call or certify tokenizer/parser. AST is a partial syntax projection, macro bodies opaque; semantic AST, live model/corpus and general proof remain absent.

## History

### 2026-10-30 · epoch-memory

2026-10-30 wave13: initial source-bound model. Prospective model preceded edits. Existing normalization and quote rules were retained; tests expose their information loss. Source identity witness justified qualifying predecessor keys by Src. Annotation is isolated behind a private closure used by the production row wrapper; fixtures do not call or certify tokenizer/parser. AST is a partial syntax projection, macro bodies opaque; semantic AST, live model/corpus and general proof remain absent.

### Undated · document

Source snapshot: 2a1a0bd6e43dd16f7284e993f30d0f4971c447c9e2080616033d19ac8693611f. All relations concern this snapshot on the stated admissible domain. Models are source-derived partial relations; finite fixtures are evidence, not general proofs. Complement is a conservative inner constraint model. Reverse reconstruction preserves all compatible antecedents conceptually; no enumerating solver is implemented.
